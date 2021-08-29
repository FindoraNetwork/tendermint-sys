package main

import (
	"fmt"
	"github.com/spf13/viper"
	cfg "github.com/tendermint/tendermint/config"
    tmos "github.com/tendermint/tendermint/libs/os"
	tmrand "github.com/tendermint/tendermint/libs/rand"
	tmflags "github.com/tendermint/tendermint/libs/cli/flags"
	"github.com/tendermint/tendermint/libs/log"
	nm "github.com/tendermint/tendermint/node"
	"github.com/tendermint/tendermint/p2p"
	"github.com/tendermint/tendermint/privval"
	"github.com/tendermint/tendermint/proxy"
    "github.com/tendermint/tendermint/types"
	tmtime "github.com/tendermint/tendermint/types/time"
	"os"
	"path/filepath"
	"sync"
	"unsafe"
)

/*
#cgo CFLAGS: -g -Wall
#include<stdint.h>
#include<stddef.h>
typedef struct ByteBufferReturn {
    size_t len;
    uint8_t *data;
} ByteBufferReturn;
*/
import "C"

var mu sync.Mutex
var index int
var nodes = make(map[int]*nm.Node)

//export init_config
func init_config(config_c C.ByteBufferReturn) C.int32_t {
	configFile := string(C.GoBytes(unsafe.Pointer(config_c.data), C.int(config_c.len)))
	config := cfg.DefaultConfig()

    cfg.WriteConfigFile(configFile, config)

	root_dir := filepath.Dir(filepath.Dir(configFile))

	config.SetRoot(root_dir)

	logger := log.NewTMLogger(log.NewSyncWriter(os.Stdout))

	var err error
	logger, err = tmflags.ParseLogLevel(config.LogLevel, logger, cfg.DefaultLogLevel())
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to parse log level: %v\n", err)
		return -1
	}

    // init config
    privValKeyFile := config.PrivValidatorKeyFile()
	privValStateFile := config.PrivValidatorStateFile()
	var pv *privval.FilePV
	if tmos.FileExists(privValKeyFile) {
		pv = privval.LoadFilePV(privValKeyFile, privValStateFile)
		logger.Info("Found private validator", "keyFile", privValKeyFile,
			"stateFile", privValStateFile)
	} else {
		pv = privval.GenFilePV(privValKeyFile, privValStateFile)
		pv.Save()
		logger.Info("Generated private validator", "keyFile", privValKeyFile,
			"stateFile", privValStateFile)
	}

	nodeKeyFile := config.NodeKeyFile()
	if tmos.FileExists(nodeKeyFile) {
		logger.Info("Found node key", "path", nodeKeyFile)
	} else {
		if _, err := p2p.LoadOrGenNodeKey(nodeKeyFile); err != nil {
			return -2
		}
		logger.Info("Generated node key", "path", nodeKeyFile)
	}

	// genesis file
	genFile := config.GenesisFile()
	if tmos.FileExists(genFile) {
		logger.Info("Found genesis file", "path", genFile)
	} else {
		genDoc := types.GenesisDoc{
			ChainID:         fmt.Sprintf("test-chain-%v", tmrand.Str(6)),
			GenesisTime:     tmtime.Now(),
			ConsensusParams: types.DefaultConsensusParams(),
		}
		pubKey, err := pv.GetPubKey()
		if err != nil {
			return -3
		}
		genDoc.Validators = []types.GenesisValidator{{
			Address: pubKey.Address(),
			PubKey:  pubKey,
			Power:   10,
		}}

		if err := genDoc.SaveAs(genFile); err != nil {
			return -4
		}
		logger.Info("Generated genesis file", "path", genFile)
	}

    return 0
}

//export new_node
func new_node(config_c C.ByteBufferReturn, abci_ptr unsafe.Pointer, userdata unsafe.Pointer) C.int32_t {
	configFile := string(C.GoBytes(unsafe.Pointer(config_c.data), C.int(config_c.len)))
	config := cfg.DefaultConfig()

	root_dir := filepath.Dir(filepath.Dir(configFile))
	config.RootDir = root_dir
	viper.SetConfigFile(configFile)
	if err := viper.ReadInConfig(); err != nil {
		fmt.Fprintf(os.Stderr, "viper failed to read config file: %v\n", err)
		return -1
	}
	if err := viper.Unmarshal(config); err != nil {
		fmt.Fprintf(os.Stderr, "viper failed to unmarshal config: %v\n", err)
		return -1
	}
	if err := config.ValidateBasic(); err != nil {
		fmt.Fprintf(os.Stderr, "config is invalid: %v\n", err)
		return -1
	}

	config.SetRoot(root_dir)

	logger := log.NewTMLogger(log.NewSyncWriter(os.Stdout))

	var err error
	logger, err = tmflags.ParseLogLevel(config.LogLevel, logger, cfg.DefaultLogLevel())
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to parse log level: %v\n", err)
		return -4
	}

	pv := privval.LoadOrGenFilePV(
		config.PrivValidatorKeyFile(),
		config.PrivValidatorStateFile(),
	)

	nodeKey, err := p2p.LoadOrGenNodeKey(config.NodeKeyFile())
	if err != nil {
		// load node key error.
		fmt.Fprintf(os.Stderr, "%v", err)
		return -2
	}

	// Get index
	mu.Lock()
	defer mu.Unlock()
	index++

	for nodes[index] != nil {
		index++
	}

	app := NewABCFApplication(abci_ptr, index, userdata)

	node, err := nm.NewNode(
		config,
		pv,
		nodeKey,
		proxy.NewLocalClientCreator(app),
		nm.DefaultGenesisDocProviderFunc(config),
		nm.DefaultDBProvider,
		nm.DefaultMetricsProvider(config.Instrumentation),
		logger)

	if err != nil {
		fmt.Fprintf(os.Stderr, "%v", err)
		return -3
	}

	nodes[index] = node

	return C.int32_t(index)
}

//export start_node
func start_node(index C.int32_t) C.int32_t {
	app := nodes[int(index)]
	if app == nil {
		return -1
	}
	app.Start()
	return 0
}

//export stop_node
func stop_node(index C.int32_t) C.int32_t {
	app := nodes[int(index)]
	if app == nil {
		return -1
	}
	app.Stop()
	return 0
}

func main() {}

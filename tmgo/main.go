package main

import (
	cfg "github.com/tendermint/tendermint/config"
	"github.com/tendermint/tendermint/libs/log"
	nm "github.com/tendermint/tendermint/node"
	"github.com/tendermint/tendermint/p2p"
	"github.com/tendermint/tendermint/privval"
	"github.com/tendermint/tendermint/proxy"
	"os"
	"sync"
	"unsafe"
    "fmt"
    "github.com/spf13/viper"
    "path/filepath"
    tmflags "github.com/tendermint/tendermint/libs/cli/flags"
)

/*
#cgo CFLAGS: -g -Wall
#include<stdint.h>
typedef struct ByteBuffer {
    int64_t len;
    uint8_t *data;
} ByteBuffer;
*/
import "C"

var mu sync.Mutex
var index int
var nodes = make(map[int]*nm.Node)

//export new_node
func new_node(config_c C.ByteBuffer, abci_ptr unsafe.Pointer, userdata unsafe.Pointer) C.int32_t {
    configFile := string(C.GoBytes(unsafe.Pointer(config_c.data), C.int(config_c.len)))
    config := cfg.DefaultConfig()

    config.RootDir = filepath.Dir(filepath.Dir(configFile))
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

	logger := log.NewTMLogger(log.NewSyncWriter(os.Stdout))

    var err error
    logger, err = tmflags.ParseLogLevel(config.LogLevel, logger, cfg.DefaultLogLevel())
    if err != nil {
        fmt.Fprintf(os.Stderr, "failed to parse log level: %v\n", err)
        return -4
    }

	pv := privval.LoadFilePV(
		config.PrivValidatorKeyFile(),
		config.PrivValidatorStateFile(),
	)

	nodeKey, err := p2p.LoadNodeKey(config.NodeKeyFile())
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

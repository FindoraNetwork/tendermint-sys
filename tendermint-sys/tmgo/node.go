package main

import (
	"context"
	"fmt"
	"github.com/spf13/viper"
	tmclient "github.com/tendermint/tendermint/abci/client"
	cfg "github.com/tendermint/tendermint/config"
	"github.com/tendermint/tendermint/libs/log"
	tmos "github.com/tendermint/tendermint/libs/os"
	tmrand "github.com/tendermint/tendermint/libs/rand"
	"github.com/tendermint/tendermint/libs/service"
	tmtime "github.com/tendermint/tendermint/libs/time"
	"github.com/tendermint/tendermint/node"
	"github.com/tendermint/tendermint/privval"
	"github.com/tendermint/tendermint/types"
	"os"
	"path/filepath"
	"unsafe"
)

//#include "raw.h"
import "C"

var SERVER service.Service

//export init_config
func init_config(config_c C.ByteBufferReturn, node_type C.int32_t) C.int32_t {
    cgo_connfig := C.GoBytes(unsafe.Pointer(config_c.data), C.int(config_c.len))
	configFile := string(cgo_connfig)

	config := cfg.DefaultConfig()
	switch node_type {
	case 0: config.Mode = cfg.ModeFull
	case 1: config.Mode = cfg.ModeValidator
	case 2: config.Mode = cfg.ModeSeed
	default:
		return -8
	}

	err := cfg.WriteConfigFile(configFile, config)
	if err != nil {
		return -9
	}

	root_dir := configFile

	config.SetRoot(root_dir)

	//logger := log.NewTMLogger(log.NewSyncWriter(os.Stdout))
	logger,err := log.NewDefaultLogger(log.LogFormatPlain, "error", false)
	if err != nil {
		fmt.Println(err)
		return -7
	}

	// init config
	privValKeyFile := config.PrivValidator.KeyFile()
	privValStateFile := config.PrivValidator.StateFile()
	var pv *privval.FilePV

	if tmos.FileExists(privValKeyFile) {
		pv,err = privval.LoadFilePV(privValKeyFile, privValStateFile)
		if err != nil {
			return -6
		}
		logger.Info("Found private validator", "keyFile", privValKeyFile,
			"stateFile", privValStateFile)
	} else {
		pv,err = privval.GenFilePV(privValKeyFile, privValStateFile, types.ABCIPubKeyTypeEd25519)
		if err != nil {
			return -5
		}

		pv.Save()
		logger.Info("Generated private validator", "keyFile", privValKeyFile,
			"stateFile", privValStateFile)
	}

	nodeKeyFile := config.NodeKeyFile()
	if tmos.FileExists(nodeKeyFile) {
		logger.Info("Found node key", "path", nodeKeyFile)
	} else {
		if _, err := types.LoadOrGenNodeKey(nodeKeyFile); err != nil {
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
		pubKey, err := pv.GetPubKey(context.Background())
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
    cgo_connfig := C.GoBytes(unsafe.Pointer(config_c.data), C.int(config_c.len))
	configFile := string(cgo_connfig)
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

	logger,err := log.NewDefaultLogger(log.LogFormatPlain,"info", false)
	if err != nil {
		fmt.Println(err)
		return -3
	}

	app := NewABCFApplication(abci_ptr, userdata)

	client := tmclient.NewLocalCreator(app)
	server,err := node.New(config, logger, client,nil)
	if err != nil {
        fmt.Println(err)
		return -2
	}

	SERVER = server

	return 0
}

//export start_node
func start_node() C.int32_t {
	if SERVER == nil {
		return -1
	}
	err := SERVER.Start()
	if err != nil {
		fmt.Println(err)
		return -2
	}
	return 0
}

//export stop_node
func stop_node() C.int32_t {
	if SERVER == nil {
		return -1
	}
	err := SERVER.Stop()
	if err != nil {
		fmt.Println(err)
		return -2
	}
	return 0
}



package main

import (
    "fmt"
    "os"
    cfg "github.com/tendermint/tendermint/config"
    "github.com/tendermint/tendermint/libs/log"
    nm "github.com/tendermint/tendermint/node"
    "sync"
    "unsafe"
    "encoding/json"
    "github.com/tendermint/tendermint/p2p"
    "github.com/tendermint/tendermint/privval"
    "github.com/tendermint/tendermint/proxy"
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

func UnmarshalBB(data C.ByteBuffer, v interface{}) error {
    go_bytes := C.GoBytes(unsafe.Pointer(data.data), C.int(data.len))
    return json.Unmarshal(go_bytes, v)
}

//export new_node
func new_node(config_c C.ByteBuffer, abci_ptr unsafe.Pointer) C.int32_t {
    var config cfg.Config
    err := UnmarshalBB(config_c, &config)

    fmt.Println(config)

    if err != nil {
        // parse config error.
        return -1
    }

    pv := privval.LoadFilePV(
        config.PrivValidatorKeyFile(),
        config.PrivValidatorStateFile(),
    )

    nodeKey, err := p2p.LoadNodeKey(config.NodeKeyFile())
    if err != nil {
        // load node key error.
        return -2
    }

    logger := log.NewTMLogger(log.NewSyncWriter(os.Stdout))

    app := NewABCFApplication(abci_ptr)

    node, err := nm.NewNode(
        &config,
        pv,
        nodeKey,
        proxy.NewLocalClientCreator(app),
        nm.DefaultGenesisDocProviderFunc(&config),
        nm.DefaultDBProvider,
        nm.DefaultMetricsProvider(config.Instrumentation),
        logger)

    if err != nil {
        return -3
    }

    mu.Lock()
	defer mu.Unlock()
	index++
	for nodes[index] != nil {
		index++
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


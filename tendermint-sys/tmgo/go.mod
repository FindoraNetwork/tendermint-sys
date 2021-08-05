module tmgo

go 1.16

require (
	github.com/spf13/viper v1.7.1
	github.com/tendermint/tendermint v0.33.6
)

replace github.com/tendermint/tendermint => github.com/FindoraNetwork/tendermint v0.33.6-findora

replace runtime => github.com/tiannian/go v0.0.0-20210720115434-c6660865f46a
replace runtime/cgo => github.com/tiannian/go v0.0.0-20210720115434-c6660865f46a

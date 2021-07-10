# tendermint-sys

> This crate wrap tendermint Go version as Rust crate. You can start a tendermint node from Rust.

## Develop Plan (WIP)

- [X] Export tendermint(v0.34) node api using `CGO`.
  - [X] Compile tendermint as static library.
- [X] Wrap tendermint static library using Rust FFI.
- [X] Write `build.rs` to using as crate.
- [X] Make basic test.
- [ ] Add async support.
- [ ] Add api to init tendermint.(instead tendermint offical binary).

## Setup requirements

- Go >= v1.16

## Test

First, inital tendermint configure file.

``` bash
$ TMHOME="/tmp/example" tendermint init
```

Clone this repo.

Compile this crate.

Make test
``` bash
RUST_LOG=debug cargo run --example baseapp
```

## Usage

``` toml
tendermint-sys = { git = "https://github.com/FindoraNetwork/tendermint-sys.git" }
```



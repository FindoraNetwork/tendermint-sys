# tendermint-sys

> Wrap tendermint Go version as Rust crate. You can manage tendermint node in Rust.

![Lines of code](https://img.shields.io/tokei/lines/github/FindoraNetwork/tendermint-sys)

## Features

- Export tendermint(v0.34) node api using `CGO`.
  - Compile tendermint as static library.
- Wrap tendermint static library using Rust FFI.
- Write `build.rs` to use as crate.
- Async support.
- Add api to init tendermint.(instead tendermint offical binary).
- Abci types for `no_std`.
- Add `no_std` protos to tendermint-sys.

## Status

Supported platform:

- Linux (gnu)

## Setup requirements

- Go >= v1.16

## Test

First, inital tendermint configure file.

``` bash
$ TMHOME="/tmp/example" tendermint init
```

Clone this repo.

Test in async:
``` bash
RUST_LOG=debug cargo run --example baseapp
```

Test in sync:
``` bash
RUST_LOG=debug cargo run --example sync_baseapp --features="sync" --no-default-features
```

## Usage

Default this crate is `async`:

``` toml
tendermint-sys = { git = "ssh://git@github.com/FindoraNetwork/tendermint-sys.git" }
```

If you want to use `sync`:

``` toml
tendermint-sys = { git = "ssh://git@github.com/FindoraNetwork/tendermint-sys.git", default-features = false, features = ["sync"] }
```



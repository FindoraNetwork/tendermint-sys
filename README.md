# tendermint-sys

> Wrap tendermint Go version as Rust crate. You can start a tendermint node from Rust.

## Develop Plan

> Basic work, need more testing.

- [X] Export tendermint(v0.34) node api using `CGO`.
  - [X] Compile tendermint as static library.
- [X] Wrap tendermint static library using Rust FFI.
- [X] Write `build.rs` to use as crate.
- [X] Make basic test.
- [X] Add async support.
- [ ] Add api to init tendermint.(instead tendermint offical binary).
- [ ] Regenerate abci proto for `no_std`.

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



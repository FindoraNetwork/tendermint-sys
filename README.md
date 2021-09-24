# tendermint-sys

> Wrap tendermint Go version as Rust crate. You can manage tendermint node in Rust.

![GitHub top language](https://img.shields.io/github/languages/top/FindoraNetwork/tendermint-sys)
![GitHub go.mod Go version (subdirectory of monorepo)](https://img.shields.io/github/go-mod/go-version/FindoraNetwork/tendermint-sys?filename=tendermint-sys%2Ftmgo%2Fgo.mod)
![support](https://img.shields.io/badge/linux--gnu-support-success)
![support](https://img.shields.io/badge/MacOS-support-success)
![Lines of code](https://img.shields.io/tokei/lines/github/FindoraNetwork/tendermint-sys)
![GitHub](https://img.shields.io/github/license/FindoraNetwork/tendermint-sys)

## Packages

| name | description | crates.io | docs.rs |
| - | - | - | - |
| tendermint-sys | use tendermint in Rust | ![Crates.io](https://img.shields.io/crates/v/tendermint-sys) | ![docs.rs](https://img.shields.io/docsrs/tendermint-sys) |
| td-abci | ABCI interface in `no_std` | ![Crates.io](https://img.shields.io/crates/v/td-abci) | ![docs.rs](https://img.shields.io/docsrs/td-abci) |
| td-protos | ABCI types in `no_std` | ![Crates.io](https://img.shields.io/crates/v/td-protos) | ![docs.rs](https://img.shields.io/docsrs/td-protos) |

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
cargo run --example init
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



#![no_std]

pub mod abci {
    mod types {
        include!("protos/tendermint.abci.types.rs");
    }
    pub use types::*;
}

pub mod libs {
    pub mod kv {
        include!("protos/tendermint.libs.kv.rs");
    }
}

pub mod crypto {
    pub mod merkle {
        include!("protos/tendermint.crypto.merkle.rs");
    }
}

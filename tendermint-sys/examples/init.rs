use tendermint_sys::{init_home, NodeEnum};

fn main() {
    init_home("./target/tendermint", NodeEnum::Validator).unwrap();
}

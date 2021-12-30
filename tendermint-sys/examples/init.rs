use tendermint_sys::{init_home, NodeType};

fn main() {
    init_home("./target/tendermint", NodeType::Validator).unwrap();
}

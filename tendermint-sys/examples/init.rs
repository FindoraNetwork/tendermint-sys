use tendermint_sys::init_home;

fn main() {
    init_home("./target/tendermint").unwrap();
}

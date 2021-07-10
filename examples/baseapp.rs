use tendermint_sys::Node;

fn main() {
    env_logger::init();
    let mut app = ();
    let node = Node::new("/tmp/example/config/config.toml", &mut app).unwrap();
    node.start().unwrap();
    loop {}
}

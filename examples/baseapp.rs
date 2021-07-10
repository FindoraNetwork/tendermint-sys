use tendermint_sys::SyncNode;

fn main() {
    let mut app = ();
    let node = SyncNode::new("/tmp/example/config/config.toml", &mut app).unwrap();
    node.start().unwrap();
}

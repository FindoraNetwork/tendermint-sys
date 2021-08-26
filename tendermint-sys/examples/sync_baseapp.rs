use tendermint_sys::{Node, SyncApplication};
use tm_protos::abci::{RequestBeginBlock, RequestInfo, ResponseBeginBlock, ResponseInfo};

struct App {
    pub counter: u64,
}

impl SyncApplication for App {
    fn info(&mut self, _request: RequestInfo) -> ResponseInfo {
        log::info!("inner value is: {}", self.counter);
        Default::default()
    }

    fn begin_block(&mut self, _request: RequestBeginBlock) -> ResponseBeginBlock {
        log::info!("inner value is: {}", self.counter);
        self.counter += 1;
        panic!("asdsa");
        Default::default()
    }
}

fn main() {
    env_logger::init();
    let app = App { counter: 0 };
    let node = Node::new("./target/tendermint/config/config.toml", app).unwrap();
    node.start().unwrap();
    loop {}
}

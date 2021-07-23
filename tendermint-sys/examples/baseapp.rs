use tm_abci::Application;
use tendermint_sys::Node;
use tm_protos::abci::{RequestBeginBlock, RequestInfo, ResponseBeginBlock, ResponseInfo};

struct App {
    pub counter: u64,
}

#[async_trait::async_trait]
impl Application for App {
    async fn info(&mut self, _request: RequestInfo) -> ResponseInfo {
        log::info!("inner value is: {}", self.counter);
        Default::default()
    }

    async fn begin_block(&mut self, _request: RequestBeginBlock) -> ResponseBeginBlock {
        log::info!("inner value is: {}", self.counter);
        self.counter += 1;
        Default::default()
    }
}

fn main() {
    env_logger::init();
    let app = App { counter: 0 };
    let node = Node::new("/tmp/example/config/config.toml", app).unwrap();
    node.start().unwrap();
    loop {}
}

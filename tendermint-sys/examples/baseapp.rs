use std::time::Duration;

use smol::Timer;
use tendermint_sys::Node;
use tm_abci::Application;
use tm_protos::abci::{
    RequestBeginBlock, RequestDeliverTx, RequestInfo, ResponseBeginBlock, ResponseDeliverTx,
    ResponseInfo,
};

struct App {}

#[async_trait::async_trait]
impl Application for App {
    async fn info(&self, _request: RequestInfo) -> ResponseInfo {
        println!("--------------------------------------------------------info");
        Default::default()
    }

    async fn begin_block(&self, _request: RequestBeginBlock) -> ResponseBeginBlock {
        println!("--------------------------------------------------------begin_block");
        Default::default()
    }

    async fn deliver_tx(&self, _request: RequestDeliverTx) -> ResponseDeliverTx {
        println!("--------------------------------------------------------recv tx");

        let _ = Timer::after(Duration::from_secs(3)).await;
        Default::default()
    }
}

fn main() {
    env_logger::init();

    std::env::set_var("SMOL_THREADS", "10");

    let app = App {};
    let node = Node::new("./target/tendermint/config/config.toml", app).unwrap();
    node.start().unwrap();
    std::thread::park();
}

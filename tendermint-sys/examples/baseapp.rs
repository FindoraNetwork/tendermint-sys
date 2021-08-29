use std::sync::mpsc::channel;
use std::thread;

use tendermint_sys::Application;
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

fn run(node: Node) {
    node.start().unwrap();

    std::thread::park();

    node.stop().unwrap();
}

fn main() {
    env_logger::init();
    let app = App { counter: 0 };
    let node = Node::new("./target/tendermint/config/config.toml", app).unwrap();
    let thread = thread::Builder::new()
        .spawn(|| run(node))
        .unwrap();

    let (tx, rx) = channel();

    ctrlc::set_handler(move || {
        tx.send(()).unwrap();
    }).unwrap();

    rx.recv().unwrap();

    thread.thread().unpark();
    thread.join().unwrap();
}

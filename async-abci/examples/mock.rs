use async_abci::Server;
use std::{io, time::Duration};
use tm_abci::Application;
use tm_protos::abci::{
    RequestBeginBlock, RequestDeliverTx, RequestInfo, ResponseBeginBlock, ResponseDeliverTx,
    ResponseInfo,
};
use tokio::time::sleep;

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
        println!(
            "--------------------------------------------------------recv tx: {:?}",
            _request
        );

        sleep(Duration::from_secs(4)).await;

        Default::default()
    }

    async fn check_tx(
        &self,
        _request: tm_protos::abci::RequestCheckTx,
    ) -> tm_protos::abci::ResponseCheckTx {
        println!(
            "--------------------------------------------------------checj tx: {:?}",
            _request
        );

        Default::default()
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    env_logger::init();

    let app = App {};

    Server::new(app)
        .bind("127.0.0.1:26658")
        .await
        .unwrap()
        .run()
        .await
        .unwrap();
    Ok(())
}

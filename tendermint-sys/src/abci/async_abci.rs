//! Async abci application.
//!
//! Async version of abci.
pub use tm_protos::abci::{
    request, response, Request, RequestBeginBlock, RequestCheckTx, RequestDeliverTx, RequestEcho,
    RequestEndBlock, RequestInfo, RequestInitChain, RequestQuery, RequestSetOption, Response,
    ResponseBeginBlock, ResponseCheckTx, ResponseCommit, ResponseDeliverTx, ResponseEcho,
    ResponseEndBlock, ResponseFlush, ResponseInfo, ResponseInitChain, ResponseQuery,
    ResponseSetOption,
};

/// Async version application for ABCI.
#[async_trait::async_trait]
pub trait Application: Send {
    async fn echo(&mut self, request: RequestEcho) -> ResponseEcho {
        ResponseEcho {
            message: request.message,
        }
    }

    async fn info(&mut self, _request: RequestInfo) -> ResponseInfo {
        Default::default()
    }

    async fn init_chain(&mut self, _request: RequestInitChain) -> ResponseInitChain {
        Default::default()
    }

    async fn query(&mut self, _request: RequestQuery) -> ResponseQuery {
        Default::default()
    }

    async fn check_tx(&mut self, _request: RequestCheckTx) -> ResponseCheckTx {
        Default::default()
    }

    async fn begin_block(&mut self, _request: RequestBeginBlock) -> ResponseBeginBlock {
        Default::default()
    }

    async fn deliver_tx(&mut self, _request: RequestDeliverTx) -> ResponseDeliverTx {
        Default::default()
    }

    async fn end_block(&mut self, _request: RequestEndBlock) -> ResponseEndBlock {
        Default::default()
    }

    async fn flush(&mut self) -> ResponseFlush {
        ResponseFlush {}
    }

    async fn commit(&mut self) -> ResponseCommit {
        Default::default()
    }

    async fn set_option(&mut self, _request: RequestSetOption) -> ResponseSetOption {
        Default::default()
    }
}

impl Application for () {}

pub async fn dispatch<A>(app: &mut A, request: Request) -> Response
where
    A: Application + ?Sized,
{
    use request::Value;
    Response {
        value: Some(match request.value.unwrap() {
            Value::Echo(req) => response::Value::Echo(app.echo(req).await),
            Value::Flush(_) => response::Value::Flush(app.flush().await),
            Value::Info(req) => response::Value::Info(app.info(req).await),
            Value::SetOption(req) => response::Value::SetOption(app.set_option(req).await),
            Value::InitChain(req) => response::Value::InitChain(app.init_chain(req).await),
            Value::Query(req) => response::Value::Query(app.query(req).await),
            Value::BeginBlock(req) => response::Value::BeginBlock(app.begin_block(req).await),
            Value::CheckTx(req) => response::Value::CheckTx(app.check_tx(req).await),
            Value::DeliverTx(req) => response::Value::DeliverTx(app.deliver_tx(req).await),
            Value::EndBlock(req) => response::Value::EndBlock(app.end_block(req).await),
            Value::Commit(_) => response::Value::Commit(app.commit().await),
        }),
    }
}

//! Async abci application.
//!
//! Async version of abci.
pub use tendermint_proto::abci::{
    request, response, Request, RequestApplySnapshotChunk, RequestBeginBlock, RequestCheckTx,
    RequestDeliverTx, RequestEcho, RequestEndBlock, RequestInfo, RequestInitChain,
    RequestLoadSnapshotChunk, RequestOfferSnapshot, RequestQuery, RequestSetOption, Response,
    ResponseApplySnapshotChunk, ResponseBeginBlock, ResponseCheckTx, ResponseCommit,
    ResponseDeliverTx, ResponseEcho, ResponseEndBlock, ResponseFlush, ResponseInfo,
    ResponseInitChain, ResponseListSnapshots, ResponseLoadSnapshotChunk, ResponseOfferSnapshot,
    ResponseQuery, ResponseSetOption,
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

    async fn list_snapshots(&mut self) -> ResponseListSnapshots {
        Default::default()
    }

    async fn offer_snapshot(&mut self, _request: RequestOfferSnapshot) -> ResponseOfferSnapshot {
        Default::default()
    }

    async fn load_snapshot_chunk(
        &mut self,
        _request: RequestLoadSnapshotChunk,
    ) -> ResponseLoadSnapshotChunk {
        Default::default()
    }

    async fn apply_snapshot_chunk(
        &mut self,
        _request: RequestApplySnapshotChunk,
    ) -> ResponseApplySnapshotChunk {
        Default::default()
    }
}

impl Application for () {}

pub async fn dispatch<A>(app: &mut A, request: Request) -> Response
where
    A: Application,
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
            Value::ListSnapshots(_) => response::Value::ListSnapshots(app.list_snapshots().await),
            Value::OfferSnapshot(req) => {
                response::Value::OfferSnapshot(app.offer_snapshot(req).await)
            }
            Value::LoadSnapshotChunk(req) => {
                response::Value::LoadSnapshotChunk(app.load_snapshot_chunk(req).await)
            }
            Value::ApplySnapshotChunk(req) => {
                response::Value::ApplySnapshotChunk(app.apply_snapshot_chunk(req).await)
            }
        }),
    }
}

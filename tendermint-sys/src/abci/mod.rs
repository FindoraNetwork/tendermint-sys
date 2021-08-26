use tm_protos::abci::{request, response, Request, Response};

#[cfg(feature = "async")]
use tm_abci::Application;

#[cfg(feature = "sync")]
use tm_abci::SyncApplication;

#[cfg(feature = "async")]
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

#[cfg(feature = "sync")]
pub fn sync_dispatch<A>(app: &mut A, request: Request) -> Response
where
    A: SyncApplication + ?Sized,
{
    use request::Value;
    Response {
        value: Some(match request.value.unwrap() {
            Value::Echo(req) => response::Value::Echo(app.echo(req)),
            Value::Flush(_) => response::Value::Flush(app.flush()),
            Value::Info(req) => response::Value::Info(app.info(req)),
            Value::SetOption(req) => response::Value::SetOption(app.set_option(req)),
            Value::InitChain(req) => response::Value::InitChain(app.init_chain(req)),
            Value::Query(req) => response::Value::Query(app.query(req)),
            Value::BeginBlock(req) => response::Value::BeginBlock(app.begin_block(req)),
            Value::CheckTx(req) => response::Value::CheckTx(app.check_tx(req)),
            Value::DeliverTx(req) => response::Value::DeliverTx(app.deliver_tx(req)),
            Value::EndBlock(req) => response::Value::EndBlock(app.end_block(req)),
            Value::Commit(_) => response::Value::Commit(app.commit()),
            Value::ListSnapshots(_) => response::Value::ListSnapshots(app.list_snapshots()),
            Value::OfferSnapshot(req) => response::Value::OfferSnapshot(app.offer_snapshot(req)),
            Value::LoadSnapshotChunk(req) => {
                response::Value::LoadSnapshotChunk(app.load_snapshot_chunk(req))
            }
            Value::ApplySnapshotChunk(req) => {
                response::Value::ApplySnapshotChunk(app.apply_snapshot_chunk(req))
            }
        }),
    }
}

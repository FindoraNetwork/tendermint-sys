use crate::{Error, Result, codec::ICodec, OCodec};
use std::net::SocketAddr;
use std::sync::Arc;
use tm_abci::Application;
use tm_protos::abci::{request::Value, response, Request, Response};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

pub const DEFAULT_SERVER_READ_BUF_SIZE: usize = 1024 * 1024;

async fn conn_handle<A>(mut socket: TcpStream, addr: SocketAddr, app: Arc<A>)
where
    A: Application + 'static,
{
    let (reader, writer) = socket.split();

    let mut icodec = ICodec::new(reader, DEFAULT_SERVER_READ_BUF_SIZE);
    // let mut ocodec = OCodec::new(writer);

    let (req_tx, mut req_rx) = tokio::sync::mpsc::unbounded_channel();
    let (resp_tx, mut resp_rx) = tokio::sync::mpsc::unbounded_channel();
    
    tokio::spawn(async move {
        loop {
            let app = app.clone();

            if let Some(request) = req_rx.recv().await {
                let resp_tx = resp_tx.clone();

                let _ = tokio::spawn(async move {
                    let response = app.dispatch(request).await;
                    resp_tx.send(response).unwrap();
                });
            }
        }
    });

    loop {
        let request: Request = match icodec.next().await {
            Some(result) => match result {
                Ok(r) => r,
                Err(e) => {
                    log::info!(
                        "Failed to read incoming request from client {}: {:?}",
                        addr,
                        e
                    );
                    return;
                }
            },
            None => {
                log::info!("Client {} terminated stream", addr);
                return;
            }
        };

        req_tx.send(request).unwrap();

    }
}

pub async fn dispatch<A>(app: &A, request: Request) -> Response
where
    A: Application,
{
    Response {
        value: Some(match request.value.unwrap() {
            Value::Echo(req) => response::Value::Echo(app.echo(req).await),
            Value::Flush(_) => response::Value::Flush(app.flush().await),
            Value::Info(req) => response::Value::Info(app.info(req).await),
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

pub struct Server<A: Application> {
    listener: Option<TcpListener>,
    app: Arc<A>,
}

impl<A: Application + 'static> Server<A> {
    pub fn new(app: A) -> Self {
        Server {
            listener: None,
            app: Arc::new(app),
        }
    }

    pub async fn bind<Addr: ToSocketAddrs>(mut self, addr: Addr) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        self.listener = Some(listener);
        Ok(self)
    }

    pub async fn run(self) -> Result<()> {
        if self.listener.is_none() {
            return Err(Error::ServerNotBinding);
        }
        let listener = self.listener.unwrap();
        loop {
            let (socket, addr) = listener.accept().await?;
            log::info!("new connect from {}", addr);
            tokio::spawn(conn_handle(socket, addr, self.app.clone()));
        }
    }
}

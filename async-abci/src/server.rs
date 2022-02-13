use crate::{codec::ICodec, Error, OCodec, Result};
use std::net::SocketAddr;
use std::sync::Arc;
use tm_abci::Application;
use tm_protos::abci::{request::Value, response, Request, Response, ResponseFlush};
use tokio::{
    io::AsyncRead,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::mpsc::{unbounded_channel, UnboundedSender},
};

pub const DEFAULT_SERVER_READ_BUF_SIZE: usize = 1024 * 1024;

fn is_flush(req: &Request) -> bool {
    match req.value {
        Some(Value::Flush(_)) => true,
        _ => false,
    }
}

fn is_flush_reponse(resp: &Response) -> bool {
    match resp.value {
        Some(response::Value::Flush(_)) => true,
        _ => false,
    }
}

fn build_flush_resp() -> Response {
    Response {
        value: Some(response::Value::Flush(ResponseFlush {})),
    }
}

async fn read_to_flush<I: AsyncRead + Unpin, A: Application + 'static>(
    codec: &mut ICodec<I>,
    addr: SocketAddr,
    app: Arc<A>,
    resp_tx: UnboundedSender<Response>,
) -> Option<usize> {
    // Read packet to flush, return count of non-empty packet.
    let mut packet_num = 0;

    loop {
        let app = app.clone();
        let resp_tx = resp_tx.clone();

        match codec.next().await {
            Some(Ok(req)) => {
                if is_flush(&req) {
                    return Some(packet_num);
                } else {
                    packet_num += 1;
                    tokio::spawn(async move {
                        log::debug!("Recv request: {:?}", req);
                        let response = app.dispatch(req.clone()).await;
                        resp_tx.send(response).unwrap();
                    });
                }
            }
            Some(Err(e)) => {
                log::info!(
                    "Failed to read incoming request from client {}: {:?}",
                    addr,
                    e
                );
                return None;
            }
            None => return None,
        }
    }
}

async fn conn_handle<A>(socket: TcpStream, addr: SocketAddr, app: Arc<A>)
where
    A: Application + 'static,
{
    let (reader, writer) = socket.into_split();

    let mut icodec = ICodec::new(reader, DEFAULT_SERVER_READ_BUF_SIZE);
    let mut ocodec = OCodec::new(writer);

    let (resp_tx, mut resp_rx) = unbounded_channel();
    let (resp_event_tx, mut resp_event_rx) = unbounded_channel();

    tokio::spawn(async move {
        loop {
            if let Some(resp) = resp_rx.recv().await {
                let r: Response = resp;
                log::debug!("Send response {:?}", r);
                ocodec.send(r.clone()).await.expect("Send error.");

                if !is_flush_reponse(&r) {
                    // trigger a send event;
                    resp_event_tx.send(()).expect("send error");
                }
            }
        }
    });

    loop {
        let app = app.clone();
        let resp_tx = resp_tx.clone();

        if let Some(expect_packet_num) = read_to_flush(&mut icodec, addr, app, resp_tx.clone()).await {
            log::debug!("Recv {} packet before flush.", expect_packet_num);

            // Wait n packet.
            let mut packet_num = 0;

            while expect_packet_num != packet_num {
                if let Some(_) = resp_event_rx.recv().await {
                    packet_num += 1;
                    log::debug!("Packet {} already sent, expect {}", packet_num, expect_packet_num);
                }
            }

            resp_tx.send(build_flush_resp()).expect("Send error");

        } else {
            return;
        }
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

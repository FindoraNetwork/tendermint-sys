//! Tendermint Node.
//!
//! Create, start or stop tendermint node.

use crate::raw::{new_node, start_node, stop_node, ByteBufferReturn};
use crate::{Error, Result};
use prost::Message;
use smol::channel::{Receiver, Sender};
use smol::lock::Mutex;
use std::ffi::c_void;
use std::ptr::null_mut;
use std::slice::from_raw_parts;
use tm_protos::abci::{Request, Response};

#[cfg(feature = "sync")]
use tm_abci::SyncApplication;

#[cfg(feature = "async")]
use tm_abci::Application;

#[cfg(feature = "sync")]
lazy_static::lazy_static! {
    static ref APPLICATIONS: Mutex<RefCell<Box<dyn SyncApplication>>> = Mutex::new(RefCell::new(Box::new(())));
}

lazy_static::lazy_static! {
    static ref SENDERS: Mutex<Option<Sender<Request>>> = Mutex::new(None);
    static ref RECEIVER: Mutex<Option<Receiver<Response>>> = Mutex::new(None);
}

extern "C" fn abci_callback(
    argument: ByteBufferReturn,
    _userdata: *mut c_void,
) -> ByteBufferReturn {
    let abci_req_bytes = unsafe { from_raw_parts(argument.data, argument.len) };

    let abci_req: Request = Message::decode(abci_req_bytes).unwrap();

    unsafe {
        libc::free(argument.data as *mut c_void);
    }
    log::debug!("recv req: {:?}", abci_req);
    {
        smol::block_on(async move {
            let sender = SENDERS.lock().await.clone().unwrap();

            sender.send(abci_req).await.unwrap();
        });
    }

    let resp = smol::block_on(async move {
        let receiver = RECEIVER.lock().await.clone().unwrap();

        receiver.recv().await.expect("channel closed")
    });

    log::debug!("send resp: {:?}", resp);
    let mut r_bytes = Vec::new();
    resp.encode(&mut r_bytes).unwrap();

    let result_len = r_bytes.len();
    let result_ptr = r_bytes.as_ptr();

    unsafe {
        let bytes = libc::malloc(result_len);
        std::ptr::copy(result_ptr, bytes as *mut u8, result_len);
        drop(resp);
        ByteBufferReturn {
            len: result_len,
            data: bytes as *mut u8,
        }
    }
}

/// Tendermint node
pub struct Node {}

#[cfg(feature = "async")]
impl Node {
    /// Create tendermint node from config.
    pub fn new<A>(config: &str, application: A) -> Result<Self>
    where
        A: Application + 'static,
    {
        // local config
        let mut config_str = String::from(config);
        let config_bytes = ByteBufferReturn {
            len: config_str.len(),
            data: config_str.as_mut_ptr(),
        };

        let (req_tx, req_rx) = smol::channel::unbounded();
        let (resp_tx, resp_rx) = smol::channel::unbounded();

        smol::block_on(async move {
            let mut sender = SENDERS.lock().await;
            *sender = Some(req_tx);

            let mut receiver = RECEIVER.lock().await;
            *receiver = Some(resp_rx);
        });

        call_abci(application, req_rx, resp_tx);

        let ffi_res = unsafe { new_node(config_bytes, abci_callback, null_mut()) };
        if ffi_res < 0 {
            return Err(Error::from_new_node_error(ffi_res));
        }

        // release config_bytes here.

        Ok(Self {})
    }
}

#[cfg(feature = "async")]
fn call_abci<A>(app: A, req_rx: Receiver<Request>, resp_tx: Sender<Response>)
where
    A: Application + 'static,
{
    let _ = smol::spawn(async move {
        let a = std::sync::Arc::new(app);

        loop {
            let a = a.clone();
            let resp_tx = resp_tx.clone();

            let req = req_rx.recv().await.expect("channel closed");
            let _ = smol::spawn(async move {
                log::debug!("req {:?}:", req);

                let resp = a.dispatch(req).await;

                log::debug!("resp {:?}:", resp);
                resp_tx.send(resp).await.expect("channel close");
            });
        }
    });
}

#[cfg(feature = "sync")]
impl Node {
    /// Create tendermint node from config.
    pub fn new<A>(config: &str, application: A) -> Result<Self>
    where
        A: SyncApplication + 'static,
    {
        let mut config_str = String::from(config);
        let config_bytes = ByteBufferReturn {
            len: config_str.len(),
            data: config_str.as_mut_ptr(),
        };

        let mut app = APPLICATIONS.lock().expect("lock faild");
        // let index = INDEX.fetch_add(1, Ordering::SeqCst);
        // apps.insert(index, Box::new(application));
        app = Box::new(application);
        // release lock.
        drop(apps);

        let (req_tx, req_rx) = std::sync::mpsc::channel();
        let (resp_tx, resp_rx) = std::sync::mpsc::channel();

        let mut sender = SENDERS.lock().expect("lock failed");
        *sender = Some(req_tx);
        drop(sender);

        let mut receiver = RECEIVER.lock().expect("lock failed");
        *receiver = Some(resp_rx);
        drop(receiver);

        std::thread::spawn(move || loop {
            let req = req_rx.recv().expect("receive failed");
            let resp = call_abci(index, req);
            resp_tx.send(resp).expect("send failed");
        });

        let ffi_res = unsafe { new_node(config_bytes, abci_callback, null_mut()) };
        if ffi_res < 0 {
            return Err(Error::from_new_node_error(ffi_res));
        }

        // release config_bytes here.

        assert_eq!(ffi_res, index);

        Ok(Self {})
    }
}

#[cfg(feature = "sync")]
fn call_abci(req: Request) -> Response {
    let app = APPLICATIONS.lock().expect("lock faild");
    app.borrow_mut().dispatch(req)
}

impl Node {
    /// Start node
    pub fn start(&self) -> Result<()> {
        let ffi_res = unsafe { start_node() };
        match ffi_res {
            0 => Ok(()),
            -1 => Err(Error::NoNodeIndex),
            _ => Err(Error::Unknown),
        }
    }

    /// Stop node.
    pub fn stop(&self) -> Result<()> {
        let ffi_res = unsafe { stop_node() };
        match ffi_res {
            0 => Ok(()),
            -1 => Err(Error::NoNodeIndex),
            _ => Err(Error::Unknown),
        }
    }
}

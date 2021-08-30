//!
//! Tendermint Node.
//!
//! Create, start or stop tendermint node.
//!

use crate::raw::{ByteBufferReturn, NodeIndex, new_node, start_node, stop_node};
use crate::{Error, Result};
use lazy_static::lazy_static;
use prost::Message;
use std::slice::from_raw_parts;
use std::{
    collections::BTreeMap,
    ffi::c_void,
    ptr::null_mut,
    sync::atomic::{AtomicI32, Ordering},
    sync::Mutex,
};
use tm_protos::abci::{Request, Response};

#[cfg(feature = "sync")]
use crate::{sync_dispatch, SyncApplication};

#[cfg(feature = "async")]
use crate::{dispatch, Application};

#[cfg(all(feature = "async", not(feature = "sync")))]
lazy_static! {
    static ref APPLICATIONS: Mutex<BTreeMap<i32, Box<dyn Application>>> =
        Mutex::new(BTreeMap::new());
}

#[cfg(all(feature = "sync", not(feature = "async")))]
lazy_static! {
    static ref APPLICATIONS: Mutex<BTreeMap<i32, Box<dyn SyncApplication>>> =
        Mutex::new(BTreeMap::new());
    static ref SENDERS: Mutex<Option<std::sync::mpsc::Sender<Request>>> = Mutex::new(None);
    static ref RECEIVER: Mutex<Option<std::sync::mpsc::Receiver<Response>>> = Mutex::new(None);
}

lazy_static! {
    static ref INDEX: AtomicI32 = AtomicI32::new(1);
}

#[cfg(feature = "async")]
fn call_abci(index: i32, req: Request) -> Response {
    let mut apps = APPLICATIONS.lock().expect("lock faild");
    log::debug!("index from go is: {}", index);
    let app = apps.get_mut(&index).expect("index from go error");
    smol::block_on(async { dispatch(app.as_mut(), req).await })
}

#[cfg(feature = "sync")]
fn call_abci(index: i32, req: Request) -> Response {
    let mut apps = APPLICATIONS.lock().expect("lock faild");
    log::debug!("index from go is: {}", index);
    let app = apps.get_mut(&index).expect("index from go error");
    sync_dispatch(app.as_mut(), req)
}

extern "C" fn abci_callback(
    argument: ByteBufferReturn,
    _index: i32,
    _userdata: *mut c_void,
) -> ByteBufferReturn {
    let abci_req_bytes = unsafe {
        from_raw_parts(argument.data, argument.len)
    };
    
    let abci_req: Request = Message::decode(abci_req_bytes).unwrap();

    unsafe {
        libc::free(argument.data as *mut c_void);
    }
    log::debug!("recv req: {:?}", abci_req);

    let sender = SENDERS.lock().expect("lock failed");
    sender.as_ref().unwrap().send(abci_req).expect("send failed");
    drop(sender);

    let receiver = RECEIVER.lock().expect("lock failed");
    let resp = receiver.as_ref().unwrap().recv().expect("recv failed");
    drop(receiver);

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
pub struct Node {
    index: NodeIndex,
}

#[cfg(feature = "async")]
impl Node {
    /// Create tendermint node from config.
    pub fn new<A>(config: &str, application: A) -> Result<Self>
    where
        A: Application + 'static,
    {
        let mut config_str = String::from(config);
        let config_bytes = ByteBufferReturn {
            len: config_str.len(),
            data: config_str.as_mut_ptr(),
        };

        let mut apps = APPLICATIONS.lock().expect("lock faild");
        let index = INDEX.fetch_add(1, Ordering::SeqCst);
        apps.insert(index, Box::new(application));
        // release lock.
        drop(apps);

        let ffi_res = unsafe { new_node(config_bytes, abci_callback, null_mut()) };
        if ffi_res < 0 {
            return Err(Error::from_new_node_error(ffi_res));
        }

        assert_eq!(ffi_res, index);

        Ok(Self { index })
    }
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

        let mut apps = APPLICATIONS.lock().expect("lock faild");
        let index = INDEX.fetch_add(1, Ordering::SeqCst);
        apps.insert(index, Box::new(application));
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

        std::thread::spawn(move || {
            loop {
                let req = req_rx.recv().expect("receive failed");
                let resp = call_abci(index, req);
                resp_tx.send(resp).expect("send failed");
            }
        });

        let ffi_res = unsafe { new_node(config_bytes, abci_callback, null_mut()) };
        if ffi_res < 0 {
            return Err(Error::from_new_node_error(ffi_res));
        }

        assert_eq!(ffi_res, index);

        Ok(Self {
            index,
        })
    }
}

impl Node {
    /// Start node
    pub fn start(&self) -> Result<()> {
        let ffi_res = unsafe { start_node(self.index) };
        match ffi_res {
            0 => Ok(()),
            -1 => Err(Error::NoNodeIndex),
            _ => Err(Error::Unknown),
        }
    }

    /// Stop node.
    pub fn stop(&self) -> Result<()> {
        let ffi_res = unsafe { stop_node(self.index) };
        match ffi_res {
            0 => Ok(()),
            -1 => Err(Error::NoNodeIndex),
            _ => Err(Error::Unknown),
        }
    }
}

//! Tendermint Node.
//!
//! Create, start or stop tendermint node.

use crate::raw::{new_node, start_node, stop_node, NodeIndex};
use crate::{closure, dispatch, Error, Result, SyncApplication};
use ffi_support::ByteBuffer;
use prost::Message;
use tendermint_proto::abci::Request;

/// Tendermint node
pub struct SyncNode {
    index: NodeIndex,
    // app: Arc<RwLock<A>>,
}

impl SyncNode {
    /// Create tendermint node from config.
    pub fn new<A: SyncApplication>(config: &str, application: &mut A) -> Result<Self> {
        let config_str = String::from(config);
        let config_bytes = ByteBuffer::from_vec(config_str.into_bytes());
        let handle = move |request: ByteBuffer| -> ByteBuffer {
            let abci_req_bytes = request.as_slice();
            let abci_req: Request = Message::decode(abci_req_bytes).unwrap();
            log::debug!("recv req: {:?}", abci_req);
            let resp = dispatch(application, abci_req);
            let mut r_bytes = Vec::new();
            resp.encode(&mut r_bytes).unwrap();
            ByteBuffer::from_vec(r_bytes)
        };
        let (fptr, uptr) = closure::convert_closure_abci_callback_ptr(handle);
        let ffi_res = unsafe { new_node(config_bytes, fptr, uptr) };
        if ffi_res < 0 {
            return Err(Error::from_new_node_error(ffi_res));
        }
        Ok(SyncNode {
            index: ffi_res,
            // app,
        })
    }

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

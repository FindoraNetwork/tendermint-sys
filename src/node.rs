//! Tendermint Node.
//!
//! Create, start or stop tendermint node.

use crate::raw::{new_node, start_node, stop_node, NodeIndex};
use crate::{Error, Result};
use tendermint::config::TendermintConfig;
// use tendermint_abci::Application as SyncApplication;
use crate::closure;
use ffi_support::ByteBuffer;

/// Tendermint node
pub struct Node {
    index: NodeIndex,
    // app: A,
}

impl Node {
    /// Create tendermint node from config.
    pub fn new(config: TendermintConfig) -> Result<Self> {
        let config_json = serde_json::to_vec(&config).unwrap();
        let config_bytes = ByteBuffer::from_vec(config_json);
        let handle = move |_request| {
            log::debug!("recv req");
            ByteBuffer::default()
        };
        let (fptr, uptr) = closure::convert_closure_abci_callback_ptr(handle);
        let ffi_res = unsafe { new_node(config_bytes, fptr, uptr) };
        if ffi_res != 0 {
            return Err(Error::from_new_node_error(ffi_res));
        }
        // let
        // let index = unsafe {};
        Ok(Node { index: ffi_res })
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

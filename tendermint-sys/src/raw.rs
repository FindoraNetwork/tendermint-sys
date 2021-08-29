//! `tmgo`'s FFI wrap.
//!
//! This module write by hand, no use bindgen.

use std::ffi::c_void;

#[repr(C)]
pub struct ByteBufferReturn {
    pub len: usize,
    pub data: *mut u8,
}

/// Tendermint node index.
///
/// If value > 0, is a valid index.
pub type NodeIndex = i32;

/// This function pointer will called when abci messages are trigged.
///
/// ABCI Request and Response are encode by protobuf.
pub type AbciCallbackPtr = extern "C" fn(ByteBufferReturn, NodeIndex, *mut c_void) -> ByteBufferReturn;

extern "C" {
    /// Creat a tendermint node from configure.
    ///
    /// This function receive configure path. Then return `NodeIndex`.
    /// If NodeIndex >= 0, meaning node create success.
    /// If NodeIndex == -1, meaning configure parse failed.
    /// If NodeIndex == -2, meaning load node key from configure file failed.
    /// If NodeIndex == -3, meaning node crate failed.
    /// If NodeIndex == -4, meaning logger init failed.
    pub fn new_node(
        config_bytes: ByteBufferReturn,
        abci_ptr: AbciCallbackPtr,
        userdata: *mut c_void,
    ) -> i32;

    /// Start tendermint node.
    ///
    /// If return 0, start success.
    /// Or return -1, node index don't exist.
    pub fn start_node(index: NodeIndex) -> i32;

    /// Stop tendermint node.
    ///
    /// If return 0, start success.
    /// Or return -1, node index don't exist.
    pub fn stop_node(index: NodeIndex) -> i32;

    /// Init config file
    ///
    /// This function receive configure path. Then return `StatusCode`.
    /// If StatusCode == 0, meaning config file create success.
    /// If StatusCode == -1, meaning logger init failed.
    /// If StatusCode == -2, meaning node key init failed.
    /// If StatusCode == -3, meaning public key get failed.
    /// If StatusCode == -4, meaning genesis save failed.
    pub fn init_config(config_bytes: ByteBufferReturn) -> i32;
}

// #[no_mangle]
// pub extern "C" fn tmgo_callback(request: ByteBuffer) -> ByteBuffer {
//     log::debug!("recv request: {:?}", ByteBuffer);
// }

//! `tmgo`'s FFI wrap.
//!
//! This module write by hand, no use bindgen.

use ffi_support::ByteBuffer;

/// Tendermint node index.
///
/// If value > 0, is a valid index.
pub type NodeIndex = i32;

/// This function pointer will called when abci messages are trigged.
///
/// ABCI Request and Response are encode by protobuf.
pub type AbciCallbackPtr = extern "C" fn(ByteBuffer) -> ByteBuffer;

extern "C" {
    /// Creat a tendermint node from configure.
    ///
    /// This function receive configure string as json. Then return `NodeIndex`.
    /// If NodeIndex >= 0, meaning node create success.
    /// If NodeIndex == -1, meaning configure parse failed.
    /// If NodeIndex == -2, meaning load node key from configure file failed.
    /// If NodeIndex == -3, meaning node crate failed.
    pub fn new_node(config_bytes: ByteBuffer, abci_ptr: AbciCallbackPtr) -> i32;

    /// Start tendermint node.
    ///
    /// If return 0, start success.
    /// Or return -1, node index don't exist.
    pub fn star_node(index: NodeIndex) -> i32;

    /// Stop tendermint node.
    ///
    /// If return 0, start success.
    /// Or return -1, node index don't exist.
    pub fn stop_node(index: NodeIndex) -> i32;
}

// #[no_mangle]
// pub extern "C" fn tmgo_callback(request: ByteBuffer) -> ByteBuffer {
//     log::debug!("recv request: {:?}", ByteBuffer);
// }

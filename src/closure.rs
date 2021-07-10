//! Convert Rust closure as function pointer.

use ffi_support::ByteBuffer;
use std::ffi::c_void;

unsafe extern "C" fn hook<F>(userdata: *mut c_void, result: ByteBuffer)
where
    F: FnMut(ByteBuffer),
{
    (*(userdata as *mut F))(result)
}

#[no_mangle]
pub extern "C" fn tmgo_callback(request: ByteBuffer) -> ByteBuffer {
    log::debug!("recv request: {:?}", ByteBuffer);
}


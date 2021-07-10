//! Convert Rust closure as function pointer.

use ffi_support::ByteBuffer;
use std::ffi::c_void;

unsafe extern "C" fn callback<Fuserdata: *mut c_void, result: ByteBuffer)
where
    F: FnMut(ByteBuffer),
{
    (*(userdata as *mut F))(result)
}



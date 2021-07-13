//! Convert Rust closure as function pointer.

use crate::raw::AbciCallbackPtr;
use ffi_support::ByteBuffer;
use std::ffi::c_void;

extern "C" fn callback<F>(argument: ByteBuffer, _index: i32, userdata: *mut c_void) -> ByteBuffer
where
    F: FnMut(ByteBuffer) -> ByteBuffer,
{
    let closure = unsafe { &mut *(userdata as *mut F) };
    closure(argument)
}

/// Convert closure to function pointer and userdata pointer.
pub fn convert_closure_abci_callback_ptr<F>(mut f: F) -> (AbciCallbackPtr, *mut c_void)
where
    F: FnMut(ByteBuffer) -> ByteBuffer,
{
    let user_data = &mut f as *mut _ as *mut c_void;
    (callback::<F>, user_data)
}

use ewwii_plugin_api::FutureResult;
use std::ffi::{c_char, CString};
use crate::HostHandle;

struct SendFutureU64(FutureResult<u64>);
struct SendFutureStr(FutureResult<String>);
unsafe impl Send for SendFutureU64 {}
unsafe impl Send for SendFutureStr {}

struct SendHandle(*const HostHandle);
unsafe impl Send for SendHandle {}

pub fn start_u64_worker(
    handle: *const HostHandle,
    callback: extern "C" fn(*const HostHandle, *mut u64), 
    fut: FutureResult<u64>
) {
    let send_handle = SendHandle(handle);
    let send_fut = SendFutureU64(fut);

    std::thread::spawn(move || {
   let send_handle = send_handle;
        let send_fut = send_fut;

        let fut = send_fut.0;
        let handle = send_handle.0;

        let mut result = fut.resolve().expect("Failed to resolve future");
        callback(handle, &mut result as *mut u64);
    });
}

pub fn start_string_worker(
    handle: *const HostHandle,
    callback: extern "C" fn(*const HostHandle, *const c_char), 
    fut: FutureResult<String>
) {
    let send_handle = SendHandle(handle);
    let send_fut = SendFutureStr(fut);

    std::thread::spawn(move || {
        let send_handle = send_handle;
        let send_fut = send_fut;

        let fut = send_fut.0;
        let handle = send_handle.0;

        let result = fut.resolve().expect("Failed to resolve future");
        let c_str = CString::new(result).expect("Failed to cast to CString");
        let c_str_ptr = c_str.as_ptr();

        callback(handle, c_str_ptr);
    });
}


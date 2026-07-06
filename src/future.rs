use crate::HostHandle;
use ewwii_plugin_api::{FutureResult, RuntimePaths as RsRuntimePaths};
use std::ffi::{CString, c_char};

struct SendHandle(*const HostHandle);

struct SendFutureU64(FutureResult<u64>);
struct SendFutureStr(FutureResult<String>);
struct SendFutureRtPaths(FutureResult<RsRuntimePaths>);

unsafe impl Send for SendHandle {}
unsafe impl Send for SendFutureU64 {}
unsafe impl Send for SendFutureStr {}
unsafe impl Send for SendFutureRtPaths {}

pub fn start_u64_worker(
    handle: *const HostHandle,
    callback: extern "C" fn(*const HostHandle, *mut u64),
    fut: FutureResult<u64>,
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
    fut: FutureResult<String>,
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

/// Paths to important dirs/files like confiuration directory.
#[repr(C)]
pub struct CRuntimePaths {
    pub log_file: *const c_char,
    pub log_dir: *const c_char,
    pub ipc_socket_file: *const c_char,
    pub config_dir: *const c_char,
}

pub fn start_rt_paths_worker(
    handle: *const HostHandle,
    callback: extern "C" fn(*const HostHandle, *const CRuntimePaths),
    fut: FutureResult<RsRuntimePaths>,
) {
    let send_handle = SendHandle(handle);
    let send_fut = SendFutureRtPaths(fut);

    std::thread::spawn(move || {
        let send_handle = send_handle;
        let send_fut = send_fut;

        let fut = send_fut.0;
        let handle = send_handle.0;

        let result = fut.resolve().expect("Failed to resolve future");
        // result = RsRuntimePaths
        let crt_paths = CRuntimePaths {
            log_file: CString::new(result.log_file).expect("Failed to cast to CString").into_raw(),
            log_dir: CString::new(result.log_dir).expect("Failed to cast to CString").into_raw(),
            ipc_socket_file: CString::new(result.ipc_socket_file)
                .expect("Failed to cast to CString")
                .into_raw(),
            config_dir: CString::new(result.config_dir)
                .expect("Failed to cast to CString")
                .into_raw(),
        };

        callback(handle, &crt_paths as *const CRuntimePaths);
    });
}

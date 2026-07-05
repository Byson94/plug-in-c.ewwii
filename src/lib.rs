#![allow(clippy::missing_safety_doc)]

mod entry;
mod future;
mod macro_defs;
mod utils;

use ewwii_plugin_api::{EwwiiAPI, ListenHandleFn, ListenHandleFnExt};
use future::CRuntimePaths;
use std::ffi::{CStr, c_char, c_void};

#[repr(C)]
pub struct HostHandle {
    pub inner: *const c_void,
}

impl HostHandle {
    pub unsafe fn as_api(&self) -> &dyn EwwiiAPI {
        use ewwii_plugin_api::proxy::HostProxy;
        let host_ptr = self.inner as *const HostProxy;
        unsafe { &*host_ptr }
    }
}

#[repr(C)]
pub struct RawMetadata {
    pub id: *const c_char,
    pub version: *const c_char,
}

// === General API ===

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_log(handle: *const HostHandle, msg: *const c_char) {
    call!({
        let host = unsafe { (*handle).as_api() };
        let c_str = unsafe { CStr::from_ptr(msg) };
        if let Ok(s) = c_str.to_str() {
            host.log(s);
        }
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_warn(handle: *const HostHandle, msg: *const c_char) {
    call!({
        let host = unsafe { (*handle).as_api() };
        let c_str = unsafe { CStr::from_ptr(msg) };
        if let Ok(s) = c_str.to_str() {
            host.warn(s);
        }
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_error(handle: *const HostHandle, msg: *const c_char) {
    call!({
        let host = unsafe { (*handle).as_api() };
        let c_str = unsafe { CStr::from_ptr(msg) };
        if let Ok(s) = c_str.to_str() {
            host.error(s);
        }
    });
}

// === Injections ===

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_inject_css(
    handle: *const HostHandle,
    css: *const c_char,
    future_handler: extern "C" fn(*const HostHandle, *mut u64),
) {
    let result = call!({
        let host = unsafe { (*handle).as_api() };
        let c_str = unsafe { CStr::from_ptr(css) };
        c_str.to_str().ok().and_then(|s| Some(host.inject_css(s)))
    });

    match result {
        Some(val) => {
            future::start_u64_worker(handle, future_handler, val);
        }
        None => {}
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_remove_css(handle: *const HostHandle, idx_ptr: *mut u64) {
    call!({
        if !idx_ptr.is_null() {
            let idx = unsafe { *idx_ptr };
            let host = unsafe { (*handle).as_api() };
            host.remove_css(idx);
        }
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_inject_nbcl(handle: *const HostHandle, nbcl: *const c_char) {
    call!({
        let host = unsafe { (*handle).as_api() };
        let c_str = unsafe { CStr::from_ptr(nbcl) };
        if let Ok(s) = c_str.to_str() {
            host.inject_nbcl_bootstrap(s);
        }
    });
}

// === Getters ===

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_get_runtime_paths(
    handle: *const HostHandle,
    future_handler: extern "C" fn(*const HostHandle, *const CRuntimePaths),
) {
    let result = call!({
        let host = unsafe { (*handle).as_api() };
        Some(host.get_runtime_paths())
    });

    match result {
        Some(val) => {
            future::start_rt_paths_worker(handle, future_handler, val);
        }
        None => {}
    }
}

// === Emissions & Listening ===
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_emit(
    handle: *const HostHandle,
    signal: *const c_char,
    data: *const c_char,
) {
    call!({
        let host = unsafe { (*handle).as_api() };

        let sig_c_str = unsafe { CStr::from_ptr(signal).to_string_lossy().into_owned() };
        let data_c_str = unsafe { CStr::from_ptr(data).to_string_lossy().into_owned() };

        host.emit(&sig_c_str, data_c_str);
    });
}

pub type CListenCallback = unsafe extern "C" fn(*const c_char, *const c_char);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_listen(
    handle: *const HostHandle,
    signal: *const c_char,
    callback: CListenCallback,
) {
    call!({
        let host = unsafe { (*handle).as_api() };
        let sig_str = unsafe { CStr::from_ptr(signal).to_string_lossy() };

        host.listen(
            &sig_str,
            ListenHandleFn::new(move |info| {
                let c_pid = std::ffi::CString::new(info.pid).unwrap_or_default();
                let c_data = std::ffi::CString::new(info.data).unwrap_or_default();

                unsafe {
                    callback(c_pid.as_ptr(), c_data.as_ptr());
                }
            }),
        );
    });
}

// === Signals API ===

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_register_signal(
    handle: *const HostHandle,
    name: *const c_char,
    initial: *const c_char,
) {
    call!({
        let host = unsafe { (*handle).as_api() };
        let name_str = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
        let init_str = unsafe { CStr::from_ptr(initial).to_string_lossy().into_owned() };
        host.register_signal(&name_str, init_str);
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_update_signal(
    handle: *const HostHandle,
    name: *const c_char,
    value: *const c_char,
) {
    call!({
        let host = unsafe { (*handle).as_api() };
        let name_str = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
        let val_str = unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() };
        host.update_signal(&name_str, val_str);
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_signal_value(
    handle: *const HostHandle,
    name: *const c_char,
    future_handler: extern "C" fn(*const HostHandle, *const c_char),
) {
    let result = call!({
        let host = unsafe { (*handle).as_api() };
        let name_str = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
        Some(host.signal_value(&name_str))
    });

    match result {
        Some(val) => {
            future::start_string_worker(handle, future_handler, val);
        }
        None => {}
    }
}

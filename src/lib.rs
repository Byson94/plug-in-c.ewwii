#![allow(clippy::missing_safety_doc)]

mod entry;
mod future;
mod macro_defs;
mod utils;

use ewwii_plugin_api::{EwwiiAPI, ListenHandleFn, ListenHandleFnExt};
use future::CRuntimePaths;
use std::ffi::{CStr, c_char, c_void};

/// Host handle required for calling back to ewwii.
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

/// Metadata of the plugin to register.
#[repr(C)]
pub struct RawMetadata {
    pub id: *const c_char,
    pub version: *const c_char,
}

// === General API ===

/// Log a message to ewwii with the appropriate plugin ID visible.
///
/// @param handle The host handle 
/// @param msg The message to log
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

/// Log a warning to ewwii with the appropriate plugin ID visible.
///
/// @param handle The host handle 
/// @param msg The message to log
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

/// Log an error to ewwii with the appropriate plugin ID visible.
///
/// @param handle The host handle 
/// @param msg The message to log
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

/// Inject CSS into the core ewwii engine and handle the resulting CSS ID.
///
/// @param handle The host handle 
/// @param css The css string to inject 
/// @future_handler A function to call when the CSS ID is resolved
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

/// Remove an injected CSS from ewwii using the resolved CSS ID.
///
/// @param handle The host handle 
/// @param idx_ptr The pointer to the resolved CSS ID which is to be removed
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

/// Inject nbcl into ewwii.
///
/// @param handle The host handle 
/// @param nbcl The nbcl code to inject
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

/// Get the runtime paths like the configuration directory, socket file, etc.
///
/// @param handle The host handle 
/// @future_handler The function to call when the CRuntimePaths are resolved
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

/// Emit a message which other plugins can see and work with the provided data.
///
/// @param handle The host handle 
/// @signal The signal to emit 
/// @data The data to attach with the signal
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

/// Listen to emissions made by other plugins and ewwii itself.
///
/// @param handle The host handle
/// @param signal The signal to listen to 
/// @param callback The function to call when emission is found
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

/// Register a signal (GlobalVar) to ewwii which can be accessed from configuration.
///
/// @param handle The host handle 
/// @param name The name of the signal 
/// @param initial The initial value of the signal
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

/// Update the value of a signal (Global).
///
/// @param handle The host handle 
/// @param name The name of the signal to update
/// @param value The value to set
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

/// Get the value of a signal (GlobalVar) and do callback.
///
/// @param handle The host handle 
/// @param name The name of the signal to get value of 
/// @param future_handler The function to call back to after resolving value
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

#![allow(clippy::missing_safety_doc)]

mod entry;
mod utils;
mod macro_defs;

use std::ffi::CStr;
use std::os::raw::c_char;
use ewwii_plugin_api::EwwiiAPI;

#[repr(C)]
pub struct HostHandle {
    pub inner: *const std::ffi::c_void,
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
        if let Ok(s) = c_str.to_str() { host.log(s); }
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_warn(handle: *const HostHandle, msg: *const c_char) {
    call!({
        let host = unsafe { (*handle).as_api() };
        let c_str = unsafe { CStr::from_ptr(msg) };
        if let Ok(s) = c_str.to_str() { host.warn(s); }
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_error(handle: *const HostHandle, msg: *const c_char) {
    call!({
        let host = unsafe { (*handle).as_api() };
        let c_str = unsafe { CStr::from_ptr(msg) };
        if let Ok(s) = c_str.to_str() { host.error(s); }
    });
}

// === Injections ===

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_inject_css(handle: *const HostHandle, css: *const c_char) -> *mut u64 {
    let result = call!({
        let host = unsafe { (*handle).as_api() };
        let c_str = unsafe { CStr::from_ptr(css) };
        c_str.to_str().ok().and_then(|s| {
            host.inject_css(s).resolve().ok() 
        })
    });

    match result {
        Some(val) => Box::into_raw(Box::new(val)),
        None => std::ptr::null_mut(),
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
        if let Ok(s) = c_str.to_str() { host.inject_nbcl_bootstrap(s); }
    });
}

// === Signals API ===

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_register_signal(handle: *const HostHandle, name: *const c_char, initial: *const c_char) {
    call!({
        let host = unsafe { (*handle).as_api() };
        let name_str = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
        let init_str = unsafe { CStr::from_ptr(initial).to_string_lossy().into_owned() };
        host.register_signal(&name_str, init_str);
    });
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ewwii_update_signal(handle: *const HostHandle, name: *const c_char, value: *const c_char) {
    call!({
        let host = unsafe { (*handle).as_api() };
        let name_str = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
        let val_str = unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() };
        host.update_signal(&name_str, val_str);
    });
}

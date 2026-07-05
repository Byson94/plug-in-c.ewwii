mod entry;
mod utils;
mod macro_defs;

use std::ffi::CStr;
use std::os::raw::c_char;
use ewwii_plugin_api::EwwiiAPI;

#[repr(C)]
pub struct HostHandle {
    pub inner: *const dyn EwwiiAPI,
}

#[repr(C)]
pub struct RawMetadata {
    pub id: *const c_char,
    pub version: *const c_char,
}

// === General API ===

#[unsafe(no_mangle)]
pub extern "C" fn ewwii_log(handle: *const HostHandle, msg: *const c_char) {
    call!({
        let host = unsafe { &*(*handle).inner };
        let c_str = unsafe { CStr::from_ptr(msg) };
        if let Ok(s) = c_str.to_str() { host.log(s); }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn ewwii_warn(handle: *const HostHandle, msg: *const c_char) {
    call!({
        let host = unsafe { &*(*handle).inner };
        let c_str = unsafe { CStr::from_ptr(msg) };
        if let Ok(s) = c_str.to_str() { host.warn(s); }
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn ewwii_error(handle: *const HostHandle, msg: *const c_char) {
    call!({
        let host = unsafe { &*(*handle).inner };
        let c_str = unsafe { CStr::from_ptr(msg) };
        if let Ok(s) = c_str.to_str() { host.error(s); }
    });
}

// === Signals API ===

#[unsafe(no_mangle)]
pub extern "C" fn ewwii_register_signal(handle: *const HostHandle, name: *const c_char, initial: *const c_char) {
    call!({
        let host = unsafe { &*(*handle).inner };
        let name_str = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
        let init_str = unsafe { CStr::from_ptr(initial).to_string_lossy().into_owned() };
        host.register_signal(&name_str, init_str);
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn ewwii_update_signal(handle: *const HostHandle, name: *const c_char, value: *const c_char) {
    call!({
        let host = unsafe { &*(*handle).inner };
        let name_str = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };
        let val_str = unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() };
        host.update_signal(&name_str, val_str);
    });
}

use ewwii_plugin_api::{PluginInfo, API_VERSION, EwwiiAPI};
use crate::RawMetadata;
use crate::HostHandle;
use std::ffi::CStr;

#[unsafe(no_mangle)]
pub extern "C" fn ewwii_api_version() -> *const std::os::raw::c_char {
    API_VERSION.as_ptr() as *const std::ffi::c_char
}

unsafe extern "C" {
    fn plugin_metadata() -> RawMetadata;
    fn plugin_init(host: *const HostHandle);
}

#[unsafe(no_mangle)]
pub unsafe fn ewwii_plugin_create() -> PluginInfo {
    let raw = unsafe { plugin_metadata() };

    let id_str = unsafe { CStr::from_ptr(raw.id).to_str().unwrap_or("unknown") };
    let ver_str = unsafe { CStr::from_ptr(raw.version).to_str().unwrap_or("0.0.0") };

    PluginInfo {
        id: id_str,
        version: ver_str,
    }
}

#[unsafe(no_mangle)]
pub unsafe fn ewwii_plugin_init(id_ptr: *const u8, id_len: usize) {
    let id_bytes = unsafe { ::std::slice::from_raw_parts(id_ptr, id_len) };

    let id_cow = ::std::string::String::from_utf8_lossy(id_bytes);
    let id_str: &str = &id_cow;

    // leaking as it should exist for the lifetime
    let proxy = Box::new(ewwii_plugin_api::proxy::HostProxy::new(id_str));
    let proxy_ptr: *const dyn EwwiiAPI = Box::leak(proxy);
    
    let handle = HostHandle { 
        inner: proxy_ptr as *const std::ffi::c_void 
    };

    unsafe { plugin_init(&handle) };
}


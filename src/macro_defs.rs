#[macro_export]
macro_rules! call {
    ($body:block) => {
        $crate::utils::safe_call(std::panic::AssertUnwindSafe(|| {
            $body
        }))
    }
}

#[macro_export]
macro_rules! get_host {
    ($handle:expr) => {{
        unsafe {
            use ewwii_plugin_api::proxy::HostProxy;
            let raw_ptr = (*$handle).inner;
            let proxy_ptr = raw_ptr as *const HostProxy;
            &*proxy_ptr
        }
    }};
}

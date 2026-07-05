#[macro_export]
macro_rules! call {
    ($body:block) => {
        $crate::utils::safe_call(std::panic::AssertUnwindSafe(|| {
            $body
        }))
    }
}



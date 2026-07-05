use std::panic::UnwindSafe;
use std::panic::catch_unwind;

pub fn safe_call<F, R>(f: F) -> R 
where F: FnOnce() -> R + UnwindSafe, R: Default 
{
    catch_unwind(f).unwrap_or_default()
}


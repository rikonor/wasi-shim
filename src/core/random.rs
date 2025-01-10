use wasi::{Errno, Size};

// Types

pub type RandomGetFn = fn(buf: *mut u8, buf_len: Size) -> Errno;

// Polyfills

struct RandomPolyfills {
    get: Option<RandomGetFn>,
}

static mut POLYFILLS: RandomPolyfills = RandomPolyfills { get: None };

pub mod set {
    use super::*;

    pub unsafe fn random_get(f: RandomGetFn) {
        POLYFILLS.get = Some(f);
    }
}

// Shims

pub mod shims {
    use super::*;

    #[no_mangle]
    unsafe extern "C" fn __shim_random_get(buf: *mut u8, buf_len: Size) -> Errno {
        match POLYFILLS.get {
            Some(f) => f(buf, buf_len),
            None => unimplemented!("random_get"),
        }
    }
}

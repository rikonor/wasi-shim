use wasi::{Errno, Size};

// Types

pub type EnvironGetFn = fn(environ: *mut *mut u8, environ_buf: *mut u8) -> Errno;
pub type EnvironSizesGetFn = fn(rp0: *mut Size, rp1: *mut Size) -> Errno;

// Polyfills

struct EnvironPolyfills {
    get: Option<EnvironGetFn>,
    sizes_get: Option<EnvironSizesGetFn>,
}

static mut POLYFILLS: EnvironPolyfills = EnvironPolyfills {
    get: None,
    sizes_get: None,
};

pub mod set {
    use super::*;

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn environ_get(f: EnvironGetFn) {
        POLYFILLS.get = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn environ_sizes_get(f: EnvironSizesGetFn) {
        POLYFILLS.sizes_get = Some(f);
    }
}

// Shims

pub mod shims {
    use super::*;

    #[no_mangle]
    unsafe extern "C" fn __shim_environ_get(environ: *mut *mut u8, environ_buf: *mut u8) -> Errno {
        match POLYFILLS.get {
            Some(f) => f(environ, environ_buf),
            None => unimplemented!("environ_get"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_environ_sizes_get(rp0: *mut Size, rp1: *mut Size) -> Errno {
        match POLYFILLS.sizes_get {
            Some(f) => f(rp0, rp1),
            None => unimplemented!("environ_sizes_get"),
        }
    }
}

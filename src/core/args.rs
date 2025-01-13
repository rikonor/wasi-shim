use wasi::{Errno, Size};

// Types

pub type ArgsGetFn = fn(argv: *mut *mut u8, argv_buf: *mut u8) -> Errno;
pub type ArgsSizesGetFn = fn(rp0: *mut Size, rp1: *mut Size) -> Errno;

// Polyfills

struct ArgsPolyfills {
    get: Option<ArgsGetFn>,
    sizes_get: Option<ArgsSizesGetFn>,
}

static mut POLYFILLS: ArgsPolyfills = ArgsPolyfills {
    get: None,
    sizes_get: None,
};

pub mod set {
    use super::*;

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn args_get(f: ArgsGetFn) {
        POLYFILLS.get = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn args_sizes_get(f: ArgsSizesGetFn) {
        POLYFILLS.sizes_get = Some(f);
    }
}

// Shims

pub mod shims {
    use super::*;

    #[no_mangle]
    unsafe extern "C" fn __shim_args_get(argv: *mut *mut u8, argv_buf: *mut u8) -> Errno {
        match POLYFILLS.get {
            Some(f) => f(argv, argv_buf),
            None => unimplemented!("args_get"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_args_sizes_get(rp0: *mut Size, rp1: *mut Size) -> Errno {
        match POLYFILLS.sizes_get {
            Some(f) => f(rp0, rp1),
            None => unimplemented!("args_sizes_get"),
        }
    }
}

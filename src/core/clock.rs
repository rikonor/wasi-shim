use wasi::{Clockid, Errno, Timestamp};

// Types

pub type ClockResGetFn = fn(id: Clockid, rp0: *mut Timestamp) -> Errno;
pub type ClockTimeGetFn = fn(id: Clockid, precision: Timestamp, rp0: *mut Timestamp) -> Errno;

// Polyfills

struct ClockPolyfills {
    res_get: Option<ClockResGetFn>,
    time_get: Option<ClockTimeGetFn>,
}

static mut POLYFILLS: ClockPolyfills = ClockPolyfills {
    res_get: None,
    time_get: None,
};

pub mod set {
    use super::*;

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn clock_res_get(f: ClockResGetFn) {
        POLYFILLS.res_get = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn clock_time_get(f: ClockTimeGetFn) {
        POLYFILLS.time_get = Some(f);
    }
}

// Shims

pub mod shims {
    use super::*;

    #[no_mangle]
    unsafe extern "C" fn __shim_clock_res_get(id: Clockid, rp0: *mut Timestamp) -> Errno {
        match POLYFILLS.res_get {
            Some(f) => f(id, rp0),
            None => unimplemented!("clock_res_get"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_clock_time_get(
        id: Clockid,
        precision: Timestamp,
        rp0: *mut Timestamp,
    ) -> Errno {
        match POLYFILLS.time_get {
            Some(f) => f(id, precision, rp0),
            None => unimplemented!("clock_time_get"),
        }
    }
}

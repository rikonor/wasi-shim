use wasi::{Errno, Exitcode, Signal};

// Types

pub type ProcExitFn = fn(rval: Exitcode) -> !;
pub type ProcRaiseFn = fn(sig: Signal) -> Errno;

// Polyfills

struct ProcPolyfills {
    exit: Option<ProcExitFn>,
    raise: Option<ProcRaiseFn>,
}

static mut POLYFILLS: ProcPolyfills = ProcPolyfills {
    exit: None,
    raise: None,
};

pub mod set {
    use super::*;

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn proc_exit(f: ProcExitFn) {
        POLYFILLS.exit = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn proc_raise(f: ProcRaiseFn) {
        POLYFILLS.raise = Some(f);
    }
}

// Shims

pub mod shims {
    use super::*;

    #[no_mangle]
    unsafe extern "C" fn __shim_proc_exit(rval: Exitcode) -> ! {
        match POLYFILLS.exit {
            Some(f) => f(rval),
            None => unimplemented!("proc_exit"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_proc_raise(sig: Signal) -> Errno {
        match POLYFILLS.raise {
            Some(f) => f(sig),
            None => unimplemented!("proc_raise"),
        }
    }
}

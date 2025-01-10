use wasi::Errno;

// Types

pub type SchedYieldFn = fn() -> Errno;

// Polyfills

struct SchedPolyfills {
    sched_yield: Option<SchedYieldFn>,
}

static mut POLYFILLS: SchedPolyfills = SchedPolyfills { sched_yield: None };

pub mod set {
    use super::*;

    pub unsafe fn sched_yield(f: SchedYieldFn) {
        POLYFILLS.sched_yield = Some(f);
    }
}

// Shims

pub mod shims {
    use super::*;

    #[no_mangle]
    unsafe extern "C" fn __shim_sched_yield() -> Errno {
        match POLYFILLS.sched_yield {
            Some(f) => f(),
            None => unimplemented!("sched_yield"),
        }
    }
}

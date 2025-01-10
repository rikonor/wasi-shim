use wasi::{Errno, Event, Size, Subscription};

// Types

pub type PollOneoffFn =
    fn(in_: *const Subscription, out: *mut Event, nsubscriptions: Size, rp0: *mut Size) -> Errno;

// Polyfills

struct PollPolyfills {
    oneoff: Option<PollOneoffFn>,
}

static mut POLYFILLS: PollPolyfills = PollPolyfills { oneoff: None };

pub mod set {
    use super::*;

    pub unsafe fn poll_oneoff(f: PollOneoffFn) {
        POLYFILLS.oneoff = Some(f);
    }
}

// Shims

pub mod shims {
    use super::*;

    #[no_mangle]
    unsafe extern "C" fn __shim_poll_oneoff(
        in_: *const Subscription,
        out: *mut Event,
        nsubscriptions: Size,
        rp0: *mut Size,
    ) -> Errno {
        match POLYFILLS.oneoff {
            Some(f) => f(in_, out, nsubscriptions, rp0),
            None => unimplemented!("poll_oneoff"),
        }
    }
}

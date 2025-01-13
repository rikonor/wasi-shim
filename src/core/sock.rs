use wasi::{Ciovec, Errno, Fd, Fdflags, Iovec, Riflags, Roflags, Sdflags, Siflags, Size};

// Types

pub type SockAcceptFn = fn(fd: Fd, flags: Fdflags, rp0: *mut Fd) -> Errno;

pub type SockRecvFn = fn(
    fd: Fd,
    ri_data: *const Iovec,
    ri_data_len: i32,
    ri_flags: Riflags,
    rp0: *mut Size,
    rp1: *mut Roflags,
) -> Errno;

pub type SockSendFn = fn(
    fd: Fd,
    si_data: *const Ciovec,
    si_data_len: i32,
    si_flags: Siflags,
    rp0: *mut Size,
) -> Errno;

pub type SockShutdownFn = fn(fd: Fd, how: Sdflags) -> Errno;

// Polyfills

struct SockPolyfills {
    accept: Option<SockAcceptFn>,
    recv: Option<SockRecvFn>,
    send: Option<SockSendFn>,
    shutdown: Option<SockShutdownFn>,
}

static mut POLYFILLS: SockPolyfills = SockPolyfills {
    accept: None,
    recv: None,
    send: None,
    shutdown: None,
};

pub mod set {
    use super::*;

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn sock_accept(f: SockAcceptFn) {
        POLYFILLS.accept = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn sock_recv(f: SockRecvFn) {
        POLYFILLS.recv = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn sock_send(f: SockSendFn) {
        POLYFILLS.send = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn sock_shutdown(f: SockShutdownFn) {
        POLYFILLS.shutdown = Some(f);
    }
}

// Shims

pub mod shims {
    use super::*;

    #[no_mangle]
    unsafe extern "C" fn __shim_sock_accept(fd: Fd, flags: Fdflags, rp0: *mut Fd) -> Errno {
        match POLYFILLS.accept {
            Some(f) => f(fd, flags, rp0),
            None => unimplemented!("sock_accept"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_sock_recv(
        fd: Fd,
        ri_data: *const Iovec,
        ri_data_len: i32,
        ri_flags: Riflags,
        rp0: *mut Size,
        rp1: *mut Roflags,
    ) -> Errno {
        match POLYFILLS.recv {
            Some(f) => f(fd, ri_data, ri_data_len, ri_flags, rp0, rp1),
            None => unimplemented!("sock_recv"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_sock_send(
        fd: Fd,
        si_data: *const Ciovec,
        si_data_len: i32,
        si_flags: Siflags,
        rp0: *mut Size,
    ) -> Errno {
        match POLYFILLS.send {
            Some(f) => f(fd, si_data, si_data_len, si_flags, rp0),
            None => unimplemented!("sock_send"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_sock_shutdown(fd: Fd, how: Sdflags) -> Errno {
        match POLYFILLS.shutdown {
            Some(f) => f(fd, how),
            None => unimplemented!("sock_shutdown"),
        }
    }
}

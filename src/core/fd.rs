use wasi::{
    Advice, Dircookie, Errno, Fd, Fdflags, Fdstat, Filedelta, Filesize, Filestat, Fstflags, Iovec,
    Prestat, Rights, Size, Timestamp, Whence,
};

// Types

pub type FdAdviseFn = fn(fd: Fd, offset: Filesize, len: Filesize, advice: Advice) -> Errno;
pub type FdAllocateFn = fn(fd: Fd, offset: Filesize, len: Filesize) -> Errno;
pub type FdCloseFn = fn(fd: Fd) -> Errno;
pub type FdDatasyncFn = fn(fd: Fd) -> Errno;
pub type FdFdstatGetFn = fn(fd: Fd, rp0: *mut Fdstat) -> Errno;
pub type FdFdstatSetFlagsFn = fn(fd: Fd, flags: Fdflags) -> Errno;
pub type FdFdstatSetRightsFn =
    fn(fd: Fd, fs_rights_base: Rights, fs_rights_inheriting: Rights) -> Errno;
pub type FdFilestatGetFn = fn(fd: Fd, rp0: *mut Filestat) -> Errno;
pub type FdFilestatSetSizeFn = fn(fd: Fd, size: Filesize) -> Errno;
pub type FdFilestatSetTimesFn =
    fn(fd: Fd, atim: Timestamp, mtim: Timestamp, fst_flags: Fstflags) -> Errno;
pub type FdPreadFn =
    fn(fd: Fd, iovs: *const Iovec, len: i32, offset: Filesize, rp0: *mut Size) -> Errno;
pub type FdPrestatDirNameFn = fn(fd: Fd, path: *mut u8, path_len: Size) -> Errno;
pub type FdPrestatGetFn = fn(fd: Fd, rp0: *mut Prestat) -> Errno;
pub type FdPwriteFn = fn(fd: Fd, iovs: *const Iovec, offset: Filesize, rp0: *mut Size) -> Errno;
pub type FdReadFn = fn(fd: Fd, iovs: *const Iovec, iovs_len: i32, rp0: *mut Size) -> Errno;
pub type FdReaddirFn =
    fn(fd: Fd, buf: *mut u8, buf_len: Size, cookie: Dircookie, rp0: *mut Size) -> Errno;
pub type FdRenumberFn = fn(fd: Fd, to: Fd) -> Errno;
pub type FdSeekFn = fn(fd: Fd, offset: Filedelta, whence: Whence, rp0: *mut Filesize) -> Errno;
pub type FdSyncFn = fn(fd: Fd) -> Errno;
pub type FdTellFn = fn(fd: Fd, rp0: *mut Filesize) -> Errno;
pub type FdWriteFn = fn(fd: Fd, iovs: *const Iovec, iovs_len: i32, rp0: *mut Size) -> Errno;

// Polyfills

struct FdPolyfills {
    advise: Option<FdAdviseFn>,
    allocate: Option<FdAllocateFn>,
    close: Option<FdCloseFn>,
    datasync: Option<FdDatasyncFn>,
    fdstat_get: Option<FdFdstatGetFn>,
    fdstat_set_flags: Option<FdFdstatSetFlagsFn>,
    fdstat_set_rights: Option<FdFdstatSetRightsFn>,
    filestat_get: Option<FdFilestatGetFn>,
    filestat_set_size: Option<FdFilestatSetSizeFn>,
    filestat_set_times: Option<FdFilestatSetTimesFn>,
    pread: Option<FdPreadFn>,
    prestat_dir_name: Option<FdPrestatDirNameFn>,
    prestat_get: Option<FdPrestatGetFn>,
    pwrite: Option<FdPwriteFn>,
    read: Option<FdReadFn>,
    readdir: Option<FdReaddirFn>,
    renumber: Option<FdRenumberFn>,
    seek: Option<FdSeekFn>,
    sync: Option<FdSyncFn>,
    tell: Option<FdTellFn>,
    write: Option<FdWriteFn>,
}

static mut POLYFILLS: FdPolyfills = FdPolyfills {
    advise: None,
    allocate: None,
    close: None,
    datasync: None,
    fdstat_get: None,
    fdstat_set_flags: None,
    fdstat_set_rights: None,
    filestat_get: None,
    filestat_set_size: None,
    filestat_set_times: None,
    pread: None,
    prestat_dir_name: None,
    prestat_get: None,
    pwrite: None,
    read: None,
    readdir: None,
    renumber: None,
    seek: None,
    sync: None,
    tell: None,
    write: None,
};

pub mod set {
    use super::*;

    pub unsafe fn fd_advise(f: FdAdviseFn) {
        POLYFILLS.advise = Some(f);
    }

    pub unsafe fn fd_allocate(f: FdAllocateFn) {
        POLYFILLS.allocate = Some(f);
    }

    pub unsafe fn fd_close(f: FdCloseFn) {
        POLYFILLS.close = Some(f);
    }

    pub unsafe fn fd_datasync(f: FdDatasyncFn) {
        POLYFILLS.datasync = Some(f);
    }

    pub unsafe fn fd_fdstat_get(f: FdFdstatGetFn) {
        POLYFILLS.fdstat_get = Some(f);
    }

    pub unsafe fn fd_fdstat_set_flags(f: FdFdstatSetFlagsFn) {
        POLYFILLS.fdstat_set_flags = Some(f);
    }

    pub unsafe fn fd_fdstat_set_rights(f: FdFdstatSetRightsFn) {
        POLYFILLS.fdstat_set_rights = Some(f);
    }

    pub unsafe fn fd_filestat_get(f: FdFilestatGetFn) {
        POLYFILLS.filestat_get = Some(f);
    }

    pub unsafe fn fd_filestat_set_size(f: FdFilestatSetSizeFn) {
        POLYFILLS.filestat_set_size = Some(f);
    }

    pub unsafe fn fd_filestat_set_times(f: FdFilestatSetTimesFn) {
        POLYFILLS.filestat_set_times = Some(f);
    }

    pub unsafe fn fd_pread(f: FdPreadFn) {
        POLYFILLS.pread = Some(f);
    }

    pub unsafe fn fd_prestat_dir_name(f: FdPrestatDirNameFn) {
        POLYFILLS.prestat_dir_name = Some(f);
    }

    pub unsafe fn fd_prestat_get(f: FdPrestatGetFn) {
        POLYFILLS.prestat_get = Some(f);
    }

    pub unsafe fn fd_pwrite(f: FdPwriteFn) {
        POLYFILLS.pwrite = Some(f);
    }

    pub unsafe fn fd_read(f: FdReadFn) {
        POLYFILLS.read = Some(f);
    }

    pub unsafe fn fd_readdir(f: FdReaddirFn) {
        POLYFILLS.readdir = Some(f);
    }

    pub unsafe fn fd_renumber(f: FdRenumberFn) {
        POLYFILLS.renumber = Some(f);
    }

    pub unsafe fn fd_seek(f: FdSeekFn) {
        POLYFILLS.seek = Some(f);
    }

    pub unsafe fn fd_sync(f: FdSyncFn) {
        POLYFILLS.sync = Some(f);
    }

    pub unsafe fn fd_tell(f: FdTellFn) {
        POLYFILLS.tell = Some(f);
    }

    pub unsafe fn fd_write(f: FdWriteFn) {
        POLYFILLS.write = Some(f);
    }
}

// Shims

pub mod shims {
    use super::*;

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_advise(
        fd: Fd,
        offset: Filesize,
        len: Filesize,
        advice: Advice,
    ) -> Errno {
        match POLYFILLS.advise {
            Some(f) => f(fd, offset, len, advice),
            None => unimplemented!("fd_advise"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_allocate(fd: Fd, offset: Filesize, len: Filesize) -> Errno {
        match POLYFILLS.allocate {
            Some(f) => f(fd, offset, len),
            None => unimplemented!("fd_allocate"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_close(fd: Fd) -> Errno {
        match POLYFILLS.close {
            Some(f) => f(fd),
            None => unimplemented!("fd_close"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_datasync(fd: Fd) -> Errno {
        match POLYFILLS.datasync {
            Some(f) => f(fd),
            None => unimplemented!("fd_datasync"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_fdstat_get(fd: Fd, rp0: *mut Fdstat) -> Errno {
        match POLYFILLS.fdstat_get {
            Some(f) => f(fd, rp0),
            None => unimplemented!("fd_fdstat_get"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_fdstat_set_flags(fd: Fd, flags: Fdflags) -> Errno {
        match POLYFILLS.fdstat_set_flags {
            Some(f) => f(fd, flags),
            None => unimplemented!("fd_fdstat_set_flags"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_fdstat_set_rights(
        fd: Fd,
        fs_rights_base: Rights,
        fs_rights_inheriting: Rights,
    ) -> Errno {
        match POLYFILLS.fdstat_set_rights {
            Some(f) => f(fd, fs_rights_base, fs_rights_inheriting),
            None => unimplemented!("fd_fdstat_set_rights"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_filestat_get(fd: Fd, rp0: *mut Filestat) -> Errno {
        match POLYFILLS.filestat_get {
            Some(f) => f(fd, rp0),
            None => unimplemented!("fd_filestat_get"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_filestat_set_size(fd: Fd, size: Filesize) -> Errno {
        match POLYFILLS.filestat_set_size {
            Some(f) => f(fd, size),
            None => unimplemented!("fd_filestat_set_size"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_filestat_set_times(
        fd: Fd,
        atim: Timestamp,
        mtim: Timestamp,
        fst_flags: Fstflags,
    ) -> Errno {
        match POLYFILLS.filestat_set_times {
            Some(f) => f(fd, atim, mtim, fst_flags),
            None => unimplemented!("fd_filestat_set_times"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_pread(
        fd: Fd,
        iovs: *const Iovec,
        len: i32,
        offset: Filesize,
        rp0: *mut Size,
    ) -> Errno {
        match POLYFILLS.pread {
            Some(f) => f(fd, iovs, len, offset, rp0),
            None => unimplemented!("fd_pread"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_prestat_dir_name(
        fd: Fd,
        path: *mut u8,
        path_len: Size,
    ) -> Errno {
        match POLYFILLS.prestat_dir_name {
            Some(f) => f(fd, path, path_len),
            None => unimplemented!("fd_prestat_dir_name"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_prestat_get(fd: Fd, rp0: *mut Prestat) -> Errno {
        match POLYFILLS.prestat_get {
            Some(f) => f(fd, rp0),
            None => unimplemented!("fd_prestat_get"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_pwrite(
        fd: Fd,
        iovs: *const Iovec,
        offset: Filesize,
        rp0: *mut Size,
    ) -> Errno {
        match POLYFILLS.pwrite {
            Some(f) => f(fd, iovs, offset, rp0),
            None => unimplemented!("fd_pwrite"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_read(
        fd: Fd,
        iovs: *const Iovec,
        iovs_len: i32,
        rp0: *mut Size,
    ) -> Errno {
        match POLYFILLS.read {
            Some(f) => f(fd, iovs, iovs_len, rp0),
            None => unimplemented!("fd_read"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_readdir(
        fd: Fd,
        buf: *mut u8,
        buf_len: Size,
        cookie: Dircookie,
        rp0: *mut Size,
    ) -> Errno {
        match POLYFILLS.readdir {
            Some(f) => f(fd, buf, buf_len, cookie, rp0),
            None => unimplemented!("fd_readdir"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_renumber(fd: Fd, to: Fd) -> Errno {
        match POLYFILLS.renumber {
            Some(f) => f(fd, to),
            None => unimplemented!("fd_renumber"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_seek(
        fd: Fd,
        offset: Filedelta,
        whence: Whence,
        rp0: *mut Filesize,
    ) -> Errno {
        match POLYFILLS.seek {
            Some(f) => f(fd, offset, whence, rp0),
            None => unimplemented!("fd_seek"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_sync(fd: Fd) -> Errno {
        match POLYFILLS.sync {
            Some(f) => f(fd),
            None => unimplemented!("fd_sync"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_tell(fd: Fd, rp0: *mut Filesize) -> Errno {
        match POLYFILLS.tell {
            Some(f) => f(fd, rp0),
            None => unimplemented!("fd_tell"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_fd_write(
        fd: Fd,
        iovs: *const Iovec,
        iovs_len: i32,
        rp0: *mut Size,
    ) -> Errno {
        match POLYFILLS.write {
            Some(f) => f(fd, iovs, iovs_len, rp0),
            None => unimplemented!("fd_write"),
        }
    }
}

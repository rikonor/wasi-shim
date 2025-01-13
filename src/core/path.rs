use wasi::{Errno, Fd, Fdflags, Filestat, Fstflags, Lookupflags, Oflags, Rights, Size, Timestamp};

// Types

pub type PathCreateDirectoryFn = fn(fd: Fd, path: *const u8, path_len: i32) -> Errno;
pub type PathFilestatGetFn =
    fn(fd: Fd, flags: Lookupflags, path: *const u8, path_len: i32, rp0: *mut Filestat) -> Errno;
pub type PathFilestatSetTimesFn = fn(
    fd: Fd,
    flags: Lookupflags,
    path: *const u8,
    path_len: i32,
    atim: Timestamp,
    mtim: Timestamp,
    fst_flags: Fstflags,
) -> Errno;
pub type PathLinkFn = fn(
    old_fd: Fd,
    old_flags: Lookupflags,
    old_path: *const u8,
    old_path_len: i32,
    new_fd: Fd,
    new_path: *const u8,
    new_path_len: i32,
) -> Errno;
pub type PathOpenFn = fn(
    fd: Fd,
    dirflags: Lookupflags,
    path: *const u8,
    path_len: i32,
    oflags: Oflags,
    fs_rights_base: Rights,
    fs_rights_inheriting: Rights,
    fdflags: Fdflags,
    rp0: *mut Fd,
) -> Errno;
pub type PathReadlinkFn = fn(
    fd: Fd,
    path: *const u8,
    path_len: i32,
    buf: *mut u8,
    buf_len: Size,
    rp0: *mut Size,
) -> Errno;
pub type PathRemoveDirectoryFn = fn(fd: Fd, path: *const u8, path_len: i32) -> Errno;
pub type PathRenameFn = fn(
    fd: Fd,
    old_path: *const u8,
    old_path_len: i32,
    new_fd: Fd,
    new_path: *const u8,
    new_path_len: i32,
) -> Errno;
pub type PathSymlinkFn = fn(
    old_path: *const u8,
    old_path_len: i32,
    fd: Fd,
    new_path: *const u8,
    new_path_len: i32,
) -> Errno;
pub type PathUnlinkFileFn = fn(fd: Fd, path: *const u8, path_len: i32) -> Errno;

// Polyfills

struct PathPolyfills {
    create_directory: Option<PathCreateDirectoryFn>,
    filestat_get: Option<PathFilestatGetFn>,
    filestat_set_times: Option<PathFilestatSetTimesFn>,
    link: Option<PathLinkFn>,
    open: Option<PathOpenFn>,
    readlink: Option<PathReadlinkFn>,
    remove_directory: Option<PathRemoveDirectoryFn>,
    rename: Option<PathRenameFn>,
    symlink: Option<PathSymlinkFn>,
    unlink_file: Option<PathUnlinkFileFn>,
}

static mut POLYFILLS: PathPolyfills = PathPolyfills {
    create_directory: None,
    filestat_get: None,
    filestat_set_times: None,
    link: None,
    open: None,
    readlink: None,
    remove_directory: None,
    rename: None,
    symlink: None,
    unlink_file: None,
};

pub mod set {
    use super::*;

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn path_create_directory(f: PathCreateDirectoryFn) {
        POLYFILLS.create_directory = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn path_filestat_get(f: PathFilestatGetFn) {
        POLYFILLS.filestat_get = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn path_filestat_set_times(f: PathFilestatSetTimesFn) {
        POLYFILLS.filestat_set_times = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn path_link(f: PathLinkFn) {
        POLYFILLS.link = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn path_open(f: PathOpenFn) {
        POLYFILLS.open = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn path_readlink(f: PathReadlinkFn) {
        POLYFILLS.readlink = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn path_remove_directory(f: PathRemoveDirectoryFn) {
        POLYFILLS.remove_directory = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn path_rename(f: PathRenameFn) {
        POLYFILLS.rename = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn path_symlink(f: PathSymlinkFn) {
        POLYFILLS.symlink = Some(f);
    }

    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn path_unlink_file(f: PathUnlinkFileFn) {
        POLYFILLS.unlink_file = Some(f);
    }
}

// Shims

pub mod shims {
    use super::*;

    #[no_mangle]
    unsafe extern "C" fn __shim_path_create_directory(
        fd: Fd,
        path: *const u8,
        path_len: i32,
    ) -> Errno {
        match POLYFILLS.create_directory {
            Some(f) => f(fd, path, path_len),
            None => unimplemented!("path_create_directory"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_path_filestat_get(
        fd: Fd,
        flags: Lookupflags,
        path: *const u8,
        path_len: i32,
        rp0: *mut Filestat,
    ) -> Errno {
        match POLYFILLS.filestat_get {
            Some(f) => f(fd, flags, path, path_len, rp0),
            None => unimplemented!("path_filestat_get"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_path_filestat_set_times(
        fd: Fd,
        flags: Lookupflags,
        path: *const u8,
        path_len: i32,
        atim: Timestamp,
        mtim: Timestamp,
        fst_flags: Fstflags,
    ) -> Errno {
        match POLYFILLS.filestat_set_times {
            Some(f) => f(fd, flags, path, path_len, atim, mtim, fst_flags),
            None => unimplemented!("path_filestat_set_times"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_path_link(
        old_fd: Fd,
        old_flags: Lookupflags,
        old_path: *const u8,
        old_path_len: i32,
        new_fd: Fd,
        new_path: *const u8,
        new_path_len: i32,
    ) -> Errno {
        match POLYFILLS.link {
            Some(f) => f(
                old_fd,
                old_flags,
                old_path,
                old_path_len,
                new_fd,
                new_path,
                new_path_len,
            ),
            None => unimplemented!("path_link"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_path_open(
        fd: Fd,
        dirflags: Lookupflags,
        path: *const u8,
        path_len: i32,
        oflags: Oflags,
        fs_rights_base: Rights,
        fs_rights_inheriting: Rights,
        fdflags: Fdflags,
        rp0: *mut Fd,
    ) -> Errno {
        match POLYFILLS.open {
            Some(f) => f(
                fd,
                dirflags,
                path,
                path_len,
                oflags,
                fs_rights_base,
                fs_rights_inheriting,
                fdflags,
                rp0,
            ),
            None => unimplemented!("path_open"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_path_readlink(
        fd: Fd,
        path: *const u8,
        path_len: i32,
        buf: *mut u8,
        buf_len: Size,
        rp0: *mut Size,
    ) -> Errno {
        match POLYFILLS.readlink {
            Some(f) => f(fd, path, path_len, buf, buf_len, rp0),
            None => unimplemented!("path_readlink"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_path_remove_directory(
        fd: Fd,
        path: *const u8,
        path_len: i32,
    ) -> Errno {
        match POLYFILLS.remove_directory {
            Some(f) => f(fd, path, path_len),
            None => unimplemented!("path_remove_directory"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_path_rename(
        fd: Fd,
        old_path: *const u8,
        old_path_len: i32,
        new_fd: Fd,
        new_path: *const u8,
        new_path_len: i32,
    ) -> Errno {
        match POLYFILLS.rename {
            Some(f) => f(fd, old_path, old_path_len, new_fd, new_path, new_path_len),
            None => unimplemented!("path_rename"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_path_symlink(
        old_path: *const u8,
        old_path_len: i32,
        fd: Fd,
        new_path: *const u8,
        new_path_len: i32,
    ) -> Errno {
        match POLYFILLS.symlink {
            Some(f) => f(old_path, old_path_len, fd, new_path, new_path_len),
            None => unimplemented!("path_symlink"),
        }
    }

    #[no_mangle]
    unsafe extern "C" fn __shim_path_unlink_file(fd: Fd, path: *const u8, path_len: i32) -> Errno {
        match POLYFILLS.unlink_file {
            Some(f) => f(fd, path, path_len),
            None => unimplemented!("path_unlink_file"),
        }
    }
}

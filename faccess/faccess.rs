//! Checks accessibility of a file.

use std::ffi::OsStr;
use std::io;

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Mode: libc::c_int {
        const EXISTS  = libc::F_OK;
        const EXECUTE = libc::X_OK;
        const WRITE   = libc::W_OK;
        const READ    = libc::R_OK;
    }
}

/// Perform accessibility check using the effecitive user ID and group ID.
/// By default, libc::faccessat uses the real IDs.
pub fn at<P: AsRef<OsStr>>(path: P, mode: Mode) -> io::Result<()> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    #[cfg(not(target_os = "android"))]
    use libc::AT_EACCESS;
    // Android does not support AT_EACCESS.
    // https://android.googlesource.com/platform/bionic/+/master/libc/bionic/faccessat.cpp#45
    #[cfg(target_os = "android")]
    const AT_EACCESS: libc::c_int = 0;

    let cstr = CString::new(path.as_ref().as_bytes())?;
    let path = cstr.as_ptr() as *const libc::c_char;

    if unsafe { libc::faccessat(libc::AT_FDCWD, path, mode.bits(), AT_EACCESS) } == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

//! Checks accessibility of a file.

use std::ffi::OsStr;
use std::io;

use bitflags::bitflags;

bitflags! {
    pub struct Mode: libc::c_int {
        const EXISTS  = libc::F_OK;
        const EXECUTE = libc::X_OK;
        const WRITE   = libc::W_OK;
        const READ    = libc::R_OK;
    }
}

#[cfg(unix)]
pub fn check<P: AsRef<OsStr>>(path: P, mode: Mode) -> io::Result<()> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    use libc::faccessat;
    // Perform access checks using the effective user and group IDs.
    // By default, faccessat() uses the real IDs.
    #[cfg(not(target_os = "android"))]
    use libc::AT_EACCESS;

    // Android does not support AT_EACCESS.
    // https://android.googlesource.com/platform/bionic/+/master/libc/bionic/faccessat.cpp#45
    #[cfg(target_os = "android")]
    const AT_EACCESS: libc::c_int = 0;

    let cstr = CString::new(path.as_ref().as_bytes())?;
    let path = cstr.as_ptr() as *const libc::c_char;

    if unsafe { faccessat(libc::AT_FDCWD, path, mode.bits(), AT_EACCESS) } == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

#[cfg(test)]
mod test {
    use testing::rlocation;

    use super::*;

    #[test]
    fn faccessat_runfiles() {
        assert!(check(rlocation("symplectix/.rustfmt.toml"), Mode::EXISTS).is_ok());
        assert!(check(rlocation("symplectix/.clippy.toml"), Mode::EXISTS).is_ok());
        assert!(check(rlocation("symplectix/path/faccess/faccess.rs"), Mode::EXISTS).is_ok());

        assert!(check(rlocation("symplectix/.rustfmt.toml"), Mode::READ).is_ok());
        assert!(check(rlocation("symplectix/.clippy.toml"), Mode::READ).is_ok());
        assert!(check(rlocation("symplectix/path/faccess/faccess.rs"), Mode::READ).is_ok());

        // See about --spawn_strategy at .bazelrc.
        // assert!(check(rlocation("symplectix/.rustfmt.toml"), Mode::WRITE).is_err());
        // assert!(check(rlocation("symplectix/.clippy.toml"), Mode::WRITE).is_err());
        // assert!(check(rlocation("symplectix/path/faccess/faccess.rs"), Mode::WRITE).is_err());

        assert!(check(rlocation("symplectix/.rustfmt.toml"), Mode::EXECUTE).is_err());
        assert!(check(rlocation("symplectix/.clippy.toml"), Mode::EXECUTE).is_err());
        assert!(check(rlocation("symplectix/path/faccess/faccess.rs"), Mode::EXECUTE).is_err());
    }
}

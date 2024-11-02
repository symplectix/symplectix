//! Checks accessibility of a file.

use bitflags::bitflags;
use std::ffi::OsStr;
use std::io;

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
    use libc::faccessat;
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

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
    use super::*;
    use runfiles::Runfiles;

    #[test]
    fn rustfmt_toml() {
        let r = Runfiles::create().expect("runfiles can not be created");

        assert!(check(r.rlocation("trunk/.rustfmt.toml"), Mode::EXISTS).is_ok());
        assert!(check(r.rlocation("trunk/.clippy.toml"), Mode::EXISTS).is_ok());
        assert!(check(r.rlocation("trunk/path/faccess/faccess_unix.rs"), Mode::EXISTS).is_ok());

        assert!(check(r.rlocation("trunk/.rustfmt.toml"), Mode::READ).is_ok());
        assert!(check(r.rlocation("trunk/.clippy.toml"), Mode::READ).is_ok());
        assert!(check(r.rlocation("trunk/path/faccess/faccess_unix.rs"), Mode::READ).is_ok());

        assert!(check(r.rlocation("trunk/.rustfmt.toml"), Mode::WRITE).is_err());
        assert!(check(r.rlocation("trunk/.clippy.toml"), Mode::WRITE).is_err());
        assert!(check(r.rlocation("trunk/path/faccess/faccess_unix.rs"), Mode::WRITE).is_err());

        assert!(check(r.rlocation("trunk/.rustfmt.toml"), Mode::EXECUTE).is_err());
        assert!(check(r.rlocation("trunk/.clippy.toml"), Mode::EXECUTE).is_err());
        assert!(check(r.rlocation("trunk/path/faccess/faccess_unix.rs"), Mode::EXECUTE).is_err());
    }
}

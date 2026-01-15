#![allow(missing_docs)]
use std::io;

use runfiles::{
    Runfiles,
    rlocation,
};

fn check_ok(result: io::Result<()>) {
    result.unwrap_or_else(|err| panic!("check_ok: {err}"));
}

fn check_err(result: io::Result<()>) {
    result.expect_err("expect error but got ok");
}

#[test]
fn runfiles() {
    let r = Runfiles::create().expect("failed to create Runfiles");
    let path = rlocation!(r, "_main/.rustfmt.toml").unwrap();

    check_ok(byc::faccess().at(&path));
    check_ok(byc::faccess().r_ok().at(&path));
    check_err(byc::faccess().r_ok().w_ok().at(&path));
    check_err(byc::faccess().r_ok().x_ok().at(&path));

    check_ok(byc::faccess().real().at(&path));
    check_ok(byc::faccess().real().r_ok().at(&path));
    check_err(byc::faccess().real().w_ok().at(&path));
    check_err(byc::faccess().real().x_ok().at(&path));
}

#[test]
fn rootfs() {
    check_ok(byc::faccess().r_ok().at("/bin/bash"));
    check_ok(byc::faccess().r_ok().x_ok().at("/bin/bash"));
    check_err(byc::faccess().w_ok().at("/bin/bash"));
}

#![allow(missing_docs)]
use std::io;

fn check_ok(result: io::Result<()>) {
    result.unwrap_or_else(|err| panic!("check_ok: {err}"));
}

fn check_err(result: io::Result<()>) {
    result.expect_err("expect error but got ok");
}

#[test]
fn cargo() {
    let bin = env!("CARGO");
    check_ok(byc::faccess().at(bin));
    check_ok(byc::faccess().r_ok().at(bin));
    check_ok(byc::faccess().r_ok().w_ok().at(bin));
    check_ok(byc::faccess().r_ok().w_ok().x_ok().at(bin));

    check_ok(byc::faccess().real().at(bin));
    check_ok(byc::faccess().real().r_ok().at(bin));
    check_ok(byc::faccess().real().r_ok().w_ok().at(bin));
    check_ok(byc::faccess().real().r_ok().w_ok().x_ok().at(bin));
}

#[test]
fn rootfs() {
    check_ok(byc::faccess().r_ok().at("/bin/bash"));
    check_ok(byc::faccess().r_ok().x_ok().at("/bin/bash"));
    check_err(byc::faccess().w_ok().at("/bin/bash"));
}

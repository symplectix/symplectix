use std::io;
use std::path::Path;

use faccess::faccess;

fn check_ok(result: io::Result<()>) {
    result.unwrap_or_else(|err| panic!("check_ok: {err}"));
}

fn check_err(result: io::Result<()>) {
    result.expect_err("expect error but got ok");
}

#[test]
fn cargo_manifest() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");

    check_ok(faccess().at(&path));
    check_ok(faccess().r_ok().at(&path));
    check_ok(faccess().real().at(&path));
    check_ok(faccess().real().r_ok().at(&path));

    check_ok(faccess().r_ok().w_ok().at(&path));
    check_ok(faccess().real().w_ok().at(&path));

    check_err(faccess().x_ok().at(&path));
    check_err(faccess().real().x_ok().at(&path));
}

#[test]
fn bin_bash() {
    check_ok(faccess().r_ok().at("/bin/bash"));
    check_ok(faccess().r_ok().x_ok().at("/bin/bash"));
    check_err(faccess().w_ok().at("/bin/bash"));
}

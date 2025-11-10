use std::path::Path;

use faccess::Mode;
use testing::rlocation;

fn check_ok<P: AsRef<Path>>(path: P, mode: Mode) {
    let path = path.as_ref();
    faccess::at(path, mode).unwrap_or_else(|err| panic!("check_ok: {err} {path}", path = path.display()));
}

fn check_err<P: AsRef<Path>>(path: P, mode: Mode) {
    let path = path.as_ref();
    faccess::at(path, mode).expect_err(&format!("check_err failed: {path}", path = path.display()));
}

#[test]
fn ruff_whl_init_py() {
    let path = rlocation("_main/faccess/ruff_whl/ruff/__init__.py");
    check_ok(&path, Mode::EXISTS | Mode::READ);
    check_err(&path, Mode::WRITE | Mode::EXECUTE);
}

#[test]
fn ruff_whl_ruff() {
    let path = rlocation("_main/faccess/ruff_whl/ruff-0.14.3.data/scripts/ruff");
    check_ok(&path, Mode::EXISTS | Mode::READ | Mode::EXECUTE);
    check_err(&path, Mode::WRITE);
}

#[test]
fn ruff_whl_lisence() {
    let path = rlocation("_main/faccess/ruff_whl/ruff-0.14.3.dist-info/licenses/LICENSE");
    check_ok(&path, Mode::EXISTS | Mode::READ);
    check_err(&path, Mode::WRITE);
}

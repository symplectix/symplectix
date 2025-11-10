use faccess::Mode;
use testing::rlocation;

#[test]
fn ruff_whl() {
    let check_ok = |path, mode: Mode| {
        assert!(faccess::at(path, mode).is_ok());
    };
    let check_err = |path, mode: Mode| {
        assert!(faccess::at(path, mode).is_err());
    };

    let path = rlocation("_main/faccess/ruff_whl/ruff/__init__.py");
    check_ok(&path, Mode::EXISTS | Mode::READ);
    check_err(&path, Mode::WRITE | Mode::EXECUTE);
    let path = rlocation("_main/faccess/ruff_whl/ruff-0.14.3.data/scripts/ruff");
    check_ok(&path, Mode::EXISTS | Mode::READ | Mode::EXECUTE);
    check_err(&path, Mode::WRITE);
    let path = rlocation("_main/faccess/ruff_whl/ruff-0.14.3.dist-info/licenses/LICENSE");
    check_ok(&path, Mode::EXISTS | Mode::READ);
    check_err(&path, Mode::WRITE);

    let path = rlocation("rules_python++pip+pypi_313_ruff/bin/ruff");
    check_ok(&path, Mode::EXISTS | Mode::READ | Mode::EXECUTE);
    check_err(&path, Mode::WRITE);
}

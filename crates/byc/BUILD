load("//bazel:rust.bzl", "rust")

rust.library(
    name = "byc",
    srcs = glob(["src/*.rs"]),
    deps = [
        "@crates//:bitflags",
        "@crates//:bon",
        "@crates//:libc",
    ],
)

rust.test(
    name = "byc_test",
    size = "small",
    crate = ":byc",
)

rust.test_suite(
    name = "byc_test_suite",
    size = "small",
    srcs = glob([
        "tests/*.rs",
    ]),
    data = [
        "//:.rustfmt.toml",
    ],
    deps = [
        ":byc",
        "@rules_rust//rust/runfiles",
    ],
)

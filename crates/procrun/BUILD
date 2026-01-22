load("//build:cc.bzl", "cc")
load("//build:rust.bzl", "rust")

package(default_visibility = ["//:__subpackages__"])

rust.library(
    name = "procrun",
    srcs = [
        "src/lib.rs",
        "src/tests.rs",
    ],
    deps = [
        "//crates/reaper",
        "@crates//:anyhow",
        "@crates//:arc-swap",
        "@crates//:clap",
        "@crates//:futures",
        "@crates//:humantime",
        "@crates//:libc",
        "@crates//:thiserror",
        "@crates//:tokio",
        "@crates//:tracing",
        "@crates//:tracing-subscriber",
    ],
)

rust.binary(
    name = "procrun_cli",
    srcs = ["bin/procrun.rs"],
    binary_name = "procrun",
    deps = [
        ":procrun",
    ],
)

cc.binary(
    name = "orphan",
    srcs = ["bin/orphan.c"],
)

rust.test(
    name = "procrun_test",
    size = "small",
    crate = ":procrun",
    deps = [
        "//crates/testing",
    ],
)

rust.test_suite(
    name = "procrun_test_suite",
    size = "small",
    srcs = [
        "tests/run.rs",
    ],
    data = [
        ":orphan",
        ":procrun_cli",
    ],
    deps = [
        "//crates/byc",
        "@crates//:libc",
        "@rules_rust//rust/runfiles",
    ],
)

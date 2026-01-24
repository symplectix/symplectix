load("//bazel:cc.bzl", "cc")
load("//bazel:rust.bzl", "rust")

package(default_visibility = ["//:__subpackages__"])

rust.binary(
    name = "procrun",
    srcs = ["bin/procrun.rs"],
    deps = [":proclib"],
)

rust.library(
    name = "proclib",
    srcs = [
        "src/lib.rs",
        "src/tests.rs",
    ],
    deps = [
        "//reaper",
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

cc.binary(
    name = "orphan",
    srcs = ["bin/orphan.c"],
)

rust.test(
    name = "proclib_test",
    size = "small",
    crate = ":proclib",
    deps = [
        "//crates/testing",
    ],
)

# nb. using procrun instead of proclib to test.
rust.test_suite(
    name = "procrun_test_suite",
    size = "small",
    srcs = [
        "tests/run.rs",
    ],
    data = [
        ":orphan",
        ":procrun",
    ],
    deps = [
        "//crates/byc",
        "@crates//:libc",
        "@rules_rust//rust/runfiles",
    ],
)

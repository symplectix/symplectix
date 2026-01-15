load("//bazel:rust.bzl", "rust")

rust.library(
    name = "ano",
    srcs = glob(["src/*.rs"]),
)

rust.test(
    name = "ano_test",
    size = "small",
    crate = ":ano",
)

rust.test_suite(
    name = "ano_test_suite",
    size = "small",
    srcs = [
        "tests/simple.rs",
        "tests/thread.rs",
    ],
    shared_srcs = glob(["tests/helper/*"]),
    deps = [":ano"],
)

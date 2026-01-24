load("//bazel:rust.bzl", "rust")

rust.library(
    name = "plying",
    srcs = glob(["src/*.rs"]),
    visibility = ["//:__subpackages__"],
)

rust.test(
    name = "plying_test",
    size = "small",
    crate = ":plying",
)

rust.test_suite(
    name = "plying_test_suite",
    size = "small",
    srcs = [
        "tests/simple.rs",
        "tests/thread.rs",
    ],
    shared_srcs = glob(["tests/helper/*"]),
    deps = [":plying"],
)

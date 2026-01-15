load("//bazel:rust.bzl", "rust")

rust.library(
    name = "bit",
    srcs = ["bit.rs"],
)

rust.test(
    name = "bit_test",
    size = "small",
    crate = ":bit",
)

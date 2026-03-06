load("//bazel:rust.bzl", "rust")

rust.library(
    name = "unionfind",
    srcs = ["src/lib.rs"],
    visibility = ["//:__subpackages__"],
)

rust.test(
    name = "unionfind_test",
    size = "small",
    crate = ":unionfind",
)

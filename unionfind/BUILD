load("//bazel:rust.bzl", "rust")

rust.library(
    name = "unionfind",
    srcs = ["unionfind.rs"],
    visibility = ["//:__subpackages__"],
)

rust.test(
    name = "unionfind_test",
    size = "small",
    crate = ":unionfind",
)

load("//bazel:rust.bzl", "rust")

package(default_visibility = ["//:__subpackages__"])

rust.library(
    name = "bit",
    srcs = ["bit.rs"],
)

rust.test(
    name = "bit_test",
    size = "small",
    crate = ":bit",
)

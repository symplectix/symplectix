load("//bazel:rust.bzl", "rust")

package(default_visibility = ["//:__subpackages__"])

rust.library(
    name = "bits",
    srcs = glob([
        "src/*.rs",
        "src/mask/*.rs",
        "src/block/*.rs",
    ]),
    deps = [
        "//bit",
        "@crates//:num",
    ],
)

rust.test(
    name = "bits_test",
    size = "small",
    crate = ":bits",
)

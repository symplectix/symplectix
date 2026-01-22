load("//build:rust.bzl", "rust")

rust.library(
    name = "testing",
    testonly = True,
    srcs = ["testing.rs"],
    visibility = ["//:__subpackages__"],
    deps = [
        "@crates//:tempfile",
    ],
)

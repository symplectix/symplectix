load("//bazel:rust.bzl", "rust")

package(default_visibility = ["//:__subpackages__"])

rust.library(
    name = "reaper",
    srcs = ["src/lib.rs"],
    deps = [
        "@crates//:libc",
        "@crates//:tokio",
        "@crates//:tracing",
    ],
)

load("//bazel:rust.bzl", "rust")

package(default_visibility = ["//:__subpackages__"])

rust.library(
    name = "devtool",
    srcs = glob(
        ["src/*.rs"],
        exclude = ["src/main.rs"],
    ),
    deps = [
        "@crates//:anyhow",
        "@crates//:clap",
    ],
)

rust.binary(
    name = "devtool_cli",
    srcs = ["src/main.rs"],
    binary_name = "devtool",
    deps = [
        ":devtool",
        "@crates//:anyhow",
        "@crates//:clap",
    ],
)

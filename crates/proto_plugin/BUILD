load("//build:rust.bzl", "rust")

package(default_visibility = ["//:__subpackages__"])

rust.library(
    name = "proto_plugin",
    srcs = ["proto_plugin.rs"],
    deps = [
        "@crates//:anyhow",
        "@crates//:prost",
        "@crates//:prost-reflect",
        "@crates//:prost-types",
    ],
)

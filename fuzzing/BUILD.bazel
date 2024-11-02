load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "fuzz",
    srcs = ["main.rs"] + glob(["cmd/*.rs"]),
    visibility = ["//visibility:public"],
    deps = [
        "//process",
        "//process/entrypoint",
        "@crates//:anyhow",
        "@crates//:clap",
        "@crates//:thiserror",
        "@crates//:tokio",
        "@crates//:tracing",
        "@crates//:tracing-subscriber",
    ],
)

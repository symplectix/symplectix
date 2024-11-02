load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "geckodriver",
    srcs = ["geckodriver.rs"],
    data = [
        "@geckodriver",
    ],
    deps = [
        "//process",
        "//process/entrypoint",
        "@crates//:anyhow",
        "@crates//:tokio",
        "@crates//:tracing",
        "@crates//:tracing-subscriber",
        "@rules_rust//tools/runfiles",
    ],
)

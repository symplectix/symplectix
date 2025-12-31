load("@rules_cc//cc:cc_binary.bzl", "cc_binary")
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "run",
    srcs = glob(["run/*.rs"]),
    visibility = ["//visibility:public"],
    deps = [
        "//process/run",
        "@crates//:anyhow",
        "@crates//:tokio",
        "@crates//:tracing",
        "@crates//:tracing-subscriber",
    ],
)

cc_binary(
    name = "orphan",
    srcs = glob(["orphan/*.c"]),
    visibility = ["//process/run:__subpackages__"],
)

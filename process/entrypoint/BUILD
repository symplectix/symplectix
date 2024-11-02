load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_doc_test", "rust_library", "rust_test")

rust_library(
    name = "entrypoint",
    srcs = glob(["src/**/*.rs"]),
    # will not work as expected on windows
    target_compatible_with = select({
        "@platforms//os:linux": [],
        "@platforms//os:macos": [],
        "//conditions:default": ["@platforms//:incompatible"],
    }),
    visibility = ["//visibility:public"],
    deps = [
        "//process/fsutil",
        "@crates//:anyhow",
        "@crates//:clap",
        "@crates//:humantime",
        "@crates//:libc",
        "@crates//:thiserror",
        "@crates//:tokio",
        "@crates//:tracing",
    ],
)

rust_test(
    name = "entrypoint_unit_test",
    size = "small",
    crate = ":entrypoint",
    deps = [
        "//testing",
    ],
)

rust_doc_test(
    name = "entrypoint_doc_test",
    size = "small",
    crate = ":entrypoint",
)

cc_binary(
    name = "orphan",
    srcs = ["orphan.c"],
)

cc_binary(
    name = "zombie",
    srcs = ["zombie.c"],
)

rust_binary(
    name = "coordinator",
    srcs = ["bin/coordinator.rs"],
    deps = [
        ":entrypoint",
        "//process/fsutil",
        "@crates//:anyhow",
        "@crates//:clap",
        "@crates//:futures",
        "@crates//:tokio",
        "@crates//:tracing",
        "@crates//:tracing-subscriber",
    ],
)

load("@rules_rust//rust:defs.bzl", "rust_library")

rust_library(
    name = "reaper",
    srcs = ["reaper.rs"],
    crate_name = "reaper",
    crate_root = "reaper.rs",
    # will not work as expected on windows
    target_compatible_with = select({
        "@platforms//os:linux": [],
        "@platforms//os:macos": [],
        "//conditions:default": ["@platforms//:incompatible"],
    }),
    visibility = ["//visibility:public"],
    deps = [
        "@crates//:libc",
        "@crates//:tokio",
        "@crates//:tracing",
    ],
)

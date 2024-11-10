load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test")

rust_library(
    name = "bits_pack",
    srcs = ["pack.rs"],
    visibility = ["//bits:__subpackages__"],
    deps = [
        "//bits/bits_core",
    ],
)

rust_test(
    name = "bits_pack_lib_test",
    timeout = "short",
    crate = ":bits_pack",
)

rust_doc_test(
    name = "bits_pack_doc_test",
    timeout = "short",
    crate = ":bits_pack",
)

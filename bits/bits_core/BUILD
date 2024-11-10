load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test")

rust_library(
    name = "bits_core",
    srcs = glob(
        ["**/*.rs"],
        exclude = ["**/*_test.rs"],
    ),
    visibility = ["//bits:__subpackages__"],
    deps = [
        "//bits/bit",
        "@crates//:num",
    ],
)

rust_test(
    name = "bits_core_lib_test",
    timeout = "short",
    crate = ":bits_core",
)

rust_doc_test(
    name = "bits_core_doc_test",
    timeout = "short",
    crate = ":bits_core",
)

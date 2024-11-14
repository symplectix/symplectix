load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test")

rust_library(
    name = "bits_buf",
    srcs = glob(
        ["*.rs"],
        exclude = ["*_test.rs"],
    ),
    visibility = ["//bits:__subpackages__"],
    deps = [
        "//bits/bit",
        "//bits/bits_core",
        "@crates//:num",
        "@crates//:smallvec",
    ],
)

rust_test(
    name = "bits_buf_lib_test",
    timeout = "short",
    crate = ":bits_buf",
)

rust_doc_test(
    name = "bits_buf_doc_test",
    timeout = "short",
    crate = ":bits_buf",
)

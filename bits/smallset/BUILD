load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test")

rust_library(
    name = "smallset",
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
    name = "smallset_lib_test",
    timeout = "short",
    crate = ":smallset",
)

rust_doc_test(
    name = "smallset_doc_test",
    timeout = "short",
    crate = ":smallset",
)

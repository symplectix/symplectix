load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test")

rust_library(
    name = "bits_aux",
    srcs = glob(
        ["*.rs"],
        exclude = ["*_test.rs"],
    ),
    crate_root = "aux.rs",
    visibility = ["//bits:__subpackages__"],
    deps = [
        "//bits/bit",
        "//bits/bits_core",
        "//bits/fenwicktree",
        "@crates//:num",
    ],
)

rust_test(
    name = "bits_aux_lib_test",
    timeout = "short",
    crate = ":bits_aux",
)

rust_doc_test(
    name = "bits_aux_doc_test",
    timeout = "short",
    crate = ":bits_aux",
)

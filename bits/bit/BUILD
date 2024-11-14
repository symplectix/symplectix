load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test")

rust_library(
    name = "bit",
    srcs = ["bit.rs"],
    visibility = ["//bits:__subpackages__"],
)

rust_test(
    name = "bit_lib_test",
    timeout = "short",
    crate = ":bit",
)

rust_doc_test(
    name = "bit_doc_test",
    timeout = "short",
    crate = ":bit",
)

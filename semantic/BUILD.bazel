load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "semantic_test",
    srcs = ["main.rs"],
    deps = [
        "@crates//:tree-sitter",
        "@crates//:tree-sitter-rust",
    ],
)

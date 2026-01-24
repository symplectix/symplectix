load("//bazel:rust.bzl", "rust")

rust.binary(
    name = "print_md_cst",
    srcs = glob(["src/*.rs"]),
    deps = [
        "@crates//:clap",
        "@crates//:tree-sitter",
        "@crates//:tree-sitter-md",
    ],
)

load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test")

rust_library(
    name = "roaring_block",
    srcs = glob(
        ["*.rs"],
        exclude = ["*_test.rs"],
    ),
    visibility = ["//bits:__subpackages__"],
    deps = [
        "//bits/bit",
        "//bits/bits_core",
        "//bits/bits_mask",
    ],
)

rust_test(
    name = "roaring_block_lib_test",
    timeout = "short",
    crate = ":roaring_block",
)

rust_doc_test(
    name = "roaring_block_doc_test",
    timeout = "short",
    crate = ":roaring_block",
)

# rust_test_suite(
#     name = "roaring_block_tests",
#     timeout = "short",
#     srcs = glob(["*_test.rs"]),
#     proc_macro_deps = [
#         "@crates//:quickcheck_macros",
#     ],
#     deps = [
#         ":roaring_block",
#         "//bits/bits_core",
#         "@crates//:quickcheck",
#     ],
# )

load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test", "rust_test_suite")

rust_library(
    name = "fenwicktree",
    srcs = ["fenwicktree.rs"],
    visibility = ["//bits:__subpackages__"],
    deps = [
        "//bits/bits_core",
    ],
)

rust_test(
    name = "fenwicktree_lib_test",
    timeout = "short",
    crate = ":fenwicktree",
)

rust_doc_test(
    name = "fenwicktree_doc_test",
    timeout = "short",
    crate = ":fenwicktree",
)

rust_test_suite(
    name = "fenwicktree_tests",
    timeout = "short",
    srcs = glob(["*_test.rs"]),
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    rustc_flags = [
        "-Coverflow-checks=off",
    ],
    deps = [
        ":fenwicktree",
        "//bits/bits_core",
        "@crates//:quickcheck",
    ],
)

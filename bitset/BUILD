load("//bazel:rust.bzl", "rust")

rust.library(
    name = "bitset",
    srcs = glob(["*.rs"]),
    visibility = ["//:__subpackages__"],
    deps = ["//bits"],
)

rust.test(
    name = "bitset_test",
    size = "small",
    crate = ":bitset",
)

rust.doc_test(
    name = "bitset_doc_test",
    size = "small",
    crate = ":bitset",
    # The root cause is unknown, but in CI, doc_test fails
    # due to an LLVM version mismatch. Skip the test for now.
    tags = ["no-ci"],
)

rust.test_suite(
    name = "bitset_integration",
    size = "small",
    srcs = glob(["tests/*.rs"]),
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    deps = [
        ":bitset",
        "//bits",
        "@crates//:quickcheck",
    ],
)

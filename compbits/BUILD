load("//bazel:rust.bzl", "rust")

rust.library(
    name = "compbits",
    srcs = glob(["*.rs"]),
    visibility = ["//:__subpackages__"],
    deps = ["@crates//:num"],
)

rust.test(
    name = "compbits_test",
    size = "small",
    crate = ":compbits",
)

rust.doc_test(
    name = "compbits_doc_test",
    size = "small",
    crate = ":compbits",
    # The root cause is unknown, but in CI, doc_test fails
    # due to an LLVM version mismatch. Skip the test for now.
    tags = ["no-ci"],
)

rust.test_suite(
    name = "compbits_integration",
    size = "small",
    srcs = glob(["tests/*.rs"]),
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    deps = [
        ":compbits",
        "@crates//:quickcheck",
    ],
)

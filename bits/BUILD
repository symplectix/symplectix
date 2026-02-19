load("//bazel:rust.bzl", "rust")

rust.library(
    name = "bits",
    srcs = glob(["*.rs"]),
    visibility = ["//:__subpackages__"],
    deps = ["@crates//:num"],
)

rust.test(
    name = "bits_test",
    size = "small",
    crate = ":bits",
)

rust.doc_test(
    name = "bits_doc_test",
    size = "small",
    crate = ":bits",
    # The root cause is unknown, but in CI, doc_test fails
    # due to an LLVM version mismatch. Skip the test for now.
    tags = ["no-ci"],
)

rust.test_suite(
    name = "bits_integration",
    size = "small",
    srcs = glob(["tests/*.rs"]),
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    deps = [
        ":bits",
        "@crates//:quickcheck",
    ],
)

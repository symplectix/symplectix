load("//bazel:rust.bzl", "rust")

rust.library(
    name = "setop",
    srcs = glob(["*.rs"]),
    visibility = ["//:__subpackages__"],
    deps = ["//compbits/bitop"],
)

rust.test(
    name = "setop_test",
    size = "small",
    crate = ":setop",
)

rust.doc_test(
    name = "setop_doc_test",
    size = "small",
    crate = ":setop",
    # The root cause is unknown, but in CI, doc_test fails
    # due to an LLVM version mismatch. Skip the test for now.
    tags = ["no-ci"],
)

rust.test_suite(
    name = "setop_integration",
    size = "small",
    srcs = glob(["tests/*.rs"]),
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    deps = [
        ":setop",
        "//compbits/bitop",
        "@crates//:quickcheck",
    ],
)

load("//bazel:rust.bzl", "rust")

rust.library(
    name = "bitop",
    srcs = glob(["*.rs"]),
    visibility = ["//:__subpackages__"],
    deps = ["@crates//:num"],
)

rust.test(
    name = "bitop_test",
    size = "small",
    crate = ":bitop",
)

rust.doc_test(
    name = "bitop_doc_test",
    size = "small",
    crate = ":bitop",
    # The root cause is unknown, but in CI, doc_test fails
    # due to an LLVM version mismatch. Skip the test for now.
    tags = ["no-ci"],
)

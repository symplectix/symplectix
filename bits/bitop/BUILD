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
)

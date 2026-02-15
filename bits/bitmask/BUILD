load("//bazel:rust.bzl", "rust")

rust.library(
    name = "bitmask",
    srcs = glob(["*.rs"]),
    visibility = ["//:__subpackages__"],
    deps = ["//bits/bitop"],
)

rust.test(
    name = "bitmask_test",
    size = "small",
    crate = ":bitmask",
)

rust.doc_test(
    name = "bitmask_doc_test",
    size = "small",
    crate = ":bitmask",
)

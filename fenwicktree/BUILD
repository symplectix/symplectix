load("//bazel:rust.bzl", "rust")

rust.library(
    name = "fenwicktree",
    srcs = glob(["src/*.rs"]),
    visibility = ["//:__subpackages__"],
    deps = [
        "//bits",
    ],
)

rust.test(
    name = "fenwicktree_test",
    size = "small",
    crate = ":fenwicktree",
)

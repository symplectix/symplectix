load("//bazel:rust.bzl", "rust")

rust.library(
    name = "bits_aux",
    srcs = glob(["src/*.rs"]),
    visibility = ["//:__subpackages__"],
    deps = [
        "//compbits/bitop",
        "//fenwicktree",
        "@crates//:num",
    ],
)

rust.test(
    name = "bits_aux_test",
    size = "small",
    crate = ":bits_aux",
)

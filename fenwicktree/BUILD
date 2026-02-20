load("//bazel:rust.bzl", "rust")

rust.library(
    name = "fenwicktree",
    srcs = glob(["src/*.rs"]),
    visibility = ["//:__subpackages__"],
    deps = [
        "//compbits/bitop",
    ],
)

rust.test(
    name = "fenwicktree_test",
    size = "small",
    crate = ":fenwicktree",
)

rust.test_suite(
    name = "fenwick_integration",
    size = "small",
    srcs = glob(["tests/*.rs"]),
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    # depends on overflowing for now.
    # could be removed.
    rustc_flags = [
        "-Coverflow-checks=off",
    ],
    deps = [
        ":fenwicktree",
        "//compbits/bitop",
        "@crates//:quickcheck",
    ],
)

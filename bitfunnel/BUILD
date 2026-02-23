load("//bazel:rust.bzl", "rust")

rust.library(
    name = "bitfunnel",
    srcs = glob([
        "*.rs",
    ]),
    visibility = ["//:__subpackages__"],
    deps = [
        "//bits:roaring",
    ],
)

rust.test(
    name = "bitfunnel_test",
    size = "small",
    crate = ":bitfunnel",
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    deps = [
        "@crates//:quickcheck",
        "@crates//:rand",
    ],
)

rust.test_suite(
    name = "bitfunnel_integration",
    size = "small",
    srcs = glob(["tests/*.rs"]),
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    deps = [
        ":bitfunnel",
        "@crates//:quickcheck",
        "@crates//:rand",
    ],
)

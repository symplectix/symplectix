load("//bazel:rust.bzl", "rust")

rust.library(
    name = "compbits",
    srcs = glob([
        "src/*.rs",
        "src/bit_set/*.rs",
        "src/bit_set/mask/*.rs",
        "src/bit_set/ops/*.rs",
        "src/bit_set/tests/*.rs",
    ]),
    visibility = ["//:__subpackages__"],
    deps = [
        "//bits",
    ],
)

rust.test(
    name = "compbits_test",
    size = "small",
    crate = ":compbits",
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    deps = [
        "@crates//:quickcheck",
    ],
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
        "//bits",
        "@crates//:lazy_static",
        "@crates//:quickcheck",
        "@crates//:rand",
    ],
)

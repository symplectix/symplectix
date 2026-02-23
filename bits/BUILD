load("//bazel:rust.bzl", "rust")

rust.library(
    name = "bits",
    srcs = glob(["bits/*.rs"]),
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

rust.library(
    name = "poppy",
    srcs = glob(["poppy/*.rs"]),
    visibility = ["//:__subpackages__"],
    deps = [
        ":bits",
        "//fenwicktree",
        "@crates//:num",
    ],
)

rust.test(
    name = "poppy_test",
    size = "small",
    crate = ":poppy",
)

rust.library(
    name = "roaring",
    srcs = glob([
        "roaring/*.rs",
        "roaring/bit_set/*.rs",
        "roaring/bit_set/mask/*.rs",
        "roaring/bit_set/ops/*.rs",
        "roaring/bit_set/tests/*.rs",
    ]),
    visibility = ["//:__subpackages__"],
    deps = [
        ":bits",
    ],
)

rust.test(
    name = "roaring_test",
    size = "small",
    crate = ":roaring",
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    deps = [
        "@crates//:quickcheck",
    ],
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

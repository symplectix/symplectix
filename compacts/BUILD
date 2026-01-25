load("//bazel:rust.bzl", "rust")

rust.library(
    name = "compacts",
    srcs = glob([
        "src/*.rs",
        "src/bits/*.rs",
        "src/bits/roaring/*.rs",
        "src/ops/*.rs",
        "src/text/*.rs",
        "src/text/wavelet_matrix/*.rs",
    ]),
    visibility = ["//:__subpackages__"],
    deps = [":build_script"],
)

rust.build_script(
    name = "build_script",
    srcs = ["build.rs"],
)

rust.test(
    name = "compacts_test",
    size = "small",
    crate = ":compacts",
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    deps = [
        "@crates//:quickcheck",
    ],
)

rust.test_suite(
    name = "compacts_test_suite",
    size = "small",
    srcs = glob(["tests/*.rs"]),
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    deps = [
        ":compacts",
        "@crates//:lazy_static",
        "@crates//:quickcheck",
        "@crates//:rand",
    ],
)

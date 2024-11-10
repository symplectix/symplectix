load("@rules_go//go:def.bzl", "go_library", "go_test")
load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test", "rust_test_suite")

rust_library(
    name = "bits",
    srcs = glob(
        ["src/**/*.rs"],
        exclude = ["*_test.rs"],
    ),
    visibility = ["//visibility:public"],
    deps = [
        "//bits/bits_aux",
        "//bits/bits_core",
        "//bits/bits_mask",
        "//bits/roaring_block",
        "@crates//:num",
    ],
)

rust_test(
    name = "bits_lib_test",
    timeout = "short",
    crate = ":bits",
)

rust_doc_test(
    name = "bits_doc_test",
    timeout = "short",
    crate = ":bits",
)

rust_test_suite(
    name = "bits_tests",
    timeout = "short",
    srcs = glob(["src/**/*_test.rs"]),
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    deps = [
        ":bits",
        "//bits/bits_pack",
        "@crates//:quickcheck",
    ],
)

go_library(
    name = "bits_go",
    srcs = glob(
        ["*.go"],
        exclude = ["*_test.go"],
    ),
    importpath = "github.com/symplectix/symplectix/bits",
    visibility = ["//bits:__subpackages__"],
)

go_test(
    name = "bits_go_test",
    timeout = "short",
    srcs = glob(["*_test.go"]),
    deps = [
        ":bits_go",
        "@rules_go//go/runfiles",
    ],
)

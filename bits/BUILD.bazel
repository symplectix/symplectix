load("@rules_go//go:def.bzl", "go_library", "go_test")
load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test", "rust_test_suite")

go_library(
    name = "bits_go",
    srcs = glob(
        ["*.go"],
        exclude = ["*_test.go"],
    ),
    importpath = "github.com/symplectix/symplectix/bits",
    visibility = ["//visibility:public"],
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

rust_library(
    name = "bits_rs",
    srcs = glob(
        ["*.rs"],
        exclude = ["*_test.rs"],
    ),
    crate_name = "bits",
    crate_root = "bits.rs",
    visibility = ["//visibility:public"],
    deps = [
        "//bits/bit:bit_rs",
        "@crates//:num",
    ],
)

rust_test(
    name = "bits_rs_lib_test",
    timeout = "short",
    crate = ":bits_rs",
)

rust_doc_test(
    name = "bits_rs_doc_test",
    timeout = "short",
    crate = ":bits_rs",
)

rust_test_suite(
    name = "bits_rs_tests",
    timeout = "short",
    srcs = glob(["*_test.rs"]),
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    rustc_flags = [
        "-Copt-level=3",
    ],
    deps = [
        ":bits_rs",
        "//bits/bitaux:bitaux_rs",
        "//bits/bitpacking:bitpacking_rs",
        "@crates//:quickcheck",
    ],
)

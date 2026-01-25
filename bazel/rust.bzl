load("@rules_rust//cargo:defs.bzl", "cargo_build_script")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_binary",
    "rust_doc_test",
    "rust_library",
    "rust_test",
    "rust_test_suite",
)
load(
    "//bazel/internal:rust_fuzz_binary.bzl",
    "rust_fuzz_binary",
)

rust = struct(
    binary = rust_binary,
    build_script = cargo_build_script,
    library = rust_library,
    doc_test = rust_doc_test,
    test = rust_test,
    test_suite = rust_test_suite,
    fuzz_binary = rust_fuzz_binary,
)

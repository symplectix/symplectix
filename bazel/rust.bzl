load(
    "@rules_rust//rust:defs.bzl",
    "rust_binary",
    "rust_doc_test",
    "rust_library",
    "rust_test",
    "rust_test_suite",
)

rust = struct(
    binary = rust_binary,
    library = rust_library,
    doc_test = rust_doc_test,
    test = rust_test,
    test_suite = rust_test_suite,
)

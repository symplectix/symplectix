load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test")

rust_library(
    name = "bitaux",
    srcs = glob(["src/**/*.rs"]),
    visibility = ["//bits:__subpackages__"],
    deps = [
        "//bits",
        "//bits/bit",
        "//bits/bitpacking",
        "//fenwicktree",
        "//num",
    ],
)

rust_test(
    name = "bitaux_unit_test",
    crate = ":bitaux",
)

rust_doc_test(
    name = "bitaux_doc_test",
    crate = ":bitaux",
)

load("//bazel:rust.bzl", "rust")

# CRITERION_HOME=/tmp/compint \
# bazel run --config=release compint:compint_bench -- --bench
rust.binary(
    name = "compint_bench",
    srcs = [
        "compint_bench.rs",
    ],
    deps = [
        ":streamvbyte",
        ":varint",
        "@crates//:bitpacking",
        "@crates//:criterion",
        "@crates//:rand",
    ],
)

rust.library(
    name = "varint",
    srcs = [
        "varint/varint.rs",
        "varint/varint_test.rs",
    ],
    visibility = ["//:__subpackages__"],
)

rust.test(
    name = "varint_test",
    size = "small",
    crate = ":varint",
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    deps = [
        "@crates//:quickcheck",
    ],
)

rust.library(
    name = "streamvbyte",
    srcs = [
        "streamvbyte/streamvbyte.rs",
        "streamvbyte/streamvbyte_test.rs",
    ],
    visibility = ["//:__subpackages__"],
    deps = ["@crates//:byteorder"],
)

rust.test(
    name = "streamvbyte_test",
    size = "small",
    crate = ":streamvbyte",
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    deps = [
        "@crates//:quickcheck",
    ],
)

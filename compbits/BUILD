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
        ":rrr15",
        ":rrr31",
        ":rrr63",
        "@crates//:lazy_static",
        "@crates//:quickcheck",
        "@crates//:rand",
    ],
)

# TODO: Consider not to share the build script and source files.
# Possibly, but this setup seems to confuse rust-analyzer.

[
    rust.library(
        name = rrr,
        srcs = ["rrr/{}.rs".format(rrr)],
        visibility = ["//:__subpackages__"],
        deps = [":{}_build_script".format(rrr)],
    )
    for rrr in [
        "rrr15",
        "rrr31",
        "rrr63",
    ]
]

[
    rust.build_script(
        name = "{}_build_script".format(rrr),
        srcs = ["rrr/{}_build.rs".format(rrr)],
    )
    for rrr in [
        "rrr15",
        "rrr31",
        "rrr63",
    ]
]

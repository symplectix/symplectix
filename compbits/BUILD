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
        "//compbits/rrr:rrr4",
        "//compbits/rrr:rrr15",
        "//compbits/rrr:rrr31",
        "//compbits/rrr:rrr63",
        "@crates//:lazy_static",
        "@crates//:quickcheck",
        "@crates//:rand",
    ],
)

[
    [
        rust.library(
            name = rrr,
            srcs = ["{}/{}.rs".format(rrr, rrr)],
            visibility = ["//:__subpackages__"],
            deps = [":{}_build_script".format(rrr)],
        ),
        rust.build_script(
            name = "{}_build_script".format(rrr),
            srcs = ["{}/build.rs".format(rrr)],
        ),
    ]
    for rrr in [
        "rrr4",
        "rrr15",
        "rrr31",
        "rrr63",
    ]
]

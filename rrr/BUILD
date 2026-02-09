load("//bazel:rust.bzl", "rust")

rust.library(
    name = "rrrbuild",
    srcs = glob(["rrrbuild/*.rs"]),
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
            deps = [":rrrbuild"],
        ),
    ]
    for rrr in [
        "rrr15",
        "rrr31",
        "rrr63",
    ]
]

rust.library(
    name = "rrr4",
    testonly = True,
    srcs = ["rrr4/rrr4.rs"],
    visibility = ["//:__subpackages__"],
    deps = [":rrr4_build_script"],
)

rust.build_script(
    name = "rrr4_build_script",
    srcs = ["rrr4/build.rs"],
    deps = [":rrrbuild"],
)

rust.test_suite(
    name = "rrr_integration",
    size = "small",
    srcs = glob(["tests/*.rs"]),
    proc_macro_deps = [
        "@crates//:quickcheck_macros",
    ],
    deps = [
        ":rrr15",
        ":rrr31",
        ":rrr4",
        ":rrr63",
        "@crates//:quickcheck",
    ],
)

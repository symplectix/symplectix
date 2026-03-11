load("//bazel:lib.bzl", "lib")
load("//bazel:py.bzl", "py")
load("//bazel:rust.bzl", "rust")

[
    [
        rust.library(
            name = rrr,
            srcs = ["{}/{}.rs".format(rrr, rrr)],
            visibility = ["//:__subpackages__"],
            deps = [
                ":{}_build_script".format(rrr),
                ":rrrutil",
            ],
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
    deps = [
        ":rrr4_build_script",
        ":rrrutil",
    ],
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

rust.library(
    name = "rrrutil",
    srcs = glob(["rrrutil/*.rs"]),
)

rust.library(
    name = "rrrbuild",
    srcs = glob(["rrrbuild/*.rs"]),
)

rust.test(
    name = "rrrbuild_test",
    size = "small",
    crate = ":rrrbuild",
    data = [
        ":rrrbuild_testdata",
    ],
    deps = [
        "@crates//:serde",
        "@crates//:serde_json",
        "@rules_rust//rust/runfiles",
    ],
)

filegroup(
    name = "rrrbuild_testdata",
    srcs = [
        ":comb_table_16_json",
        ":comb_table_32_json",
    ],
)

py.binary(
    name = "comb",
    srcs = ["comb.py"],
    deps = ["@pypi//pydantic"],
)

lib.run_binary(
    name = "comb_table_16_json",
    outs = ["comb_table_16.json"],
    args = [
        "--size",
        "16",
        "--path",
        "$(execpath :comb_table_16.json)",
    ],
    tool = ":comb",
)

lib.run_binary(
    name = "comb_table_32_json",
    outs = ["comb_table_32.json"],
    args = [
        "--size",
        "32",
        "--path",
        "$(execpath :comb_table_32.json)",
    ],
    tool = ":comb",
)

load("//bazel:rust.bzl", "rust")

package(default_visibility = ["//:__subpackages__"])

rust.library(
    name = "shtok",
    srcs = ["src/lib.rs"],
    deps = ["@crates//:itertools"],
)

rust.test(
    name = "shtok_test",
    size = "small",
    crate = ":shtok",
)

rust.test_suite(
    name = "shtok_test_suite",
    size = "small",
    srcs = [
        "tests/tests.rs",
    ],
    deps = [":shtok"],
)

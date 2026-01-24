load("@aspect_bazel_lib//lib:jq.bzl", "jq")
load("//bazel:rust.bzl", "rust")
load("//bazel:py.bzl", "py")

package(default_visibility = ["//:__subpackages__"])

rust.library(
    name = "devtool",
    srcs = glob(
        ["*.rs"],
        exclude = ["main.rs"],
    ),
    deps = [
        "@crates//:anyhow",
        "@crates//:clap",
    ],
)

rust.binary(
    name = "devtool_cli",
    srcs = ["main.rs"],
    binary_name = "devtool",
    deps = [
        ":devtool",
        "@crates//:anyhow",
        "@crates//:clap",
    ],
)

py.binary(
    name = "workspace_status",
    srcs = ["workspace_status.py"],
    main = "workspace_status.py",
)

jq(
    name = "workspace_status_json",
    srcs = [],
    args = [
        "--sort-keys",
    ],
    filter = "|".join([
        "$ARGS.named.STAMP as $stamp",
        "$stamp // []",
        "reduce .[] as $x ({}; . * $x)",
    ]),
)

genrule(
    name = "version",
    srcs = [":workspace_status_json"],
    outs = ["version.txt"],
    cmd = "$(JQ_BIN) -r '.STABLE_VERSION' $(location :workspace_status_json) > $@",
    toolchains = ["@jq_toolchains//:resolved_toolchain"],
)

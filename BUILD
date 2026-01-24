load("@aspect_bazel_lib//lib:jq.bzl", "jq")
load("//bazel:uv.bzl", "uv")

exports_files([
    "MODULE.bazel",
    ".clippy.toml",
    ".rustfmt.toml",
])

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

uv.pip_compile(
    name = "pip_compile",
    requirements_in = ":pyproject.toml",
    requirements_txt = ":requirements.txt",
)

uv.create_venv(
    name = "create_venv",
    destination_folder = ".venv",
    requirements_txt = ":requirements.txt",
)

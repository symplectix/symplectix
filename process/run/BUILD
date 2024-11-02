load("@container_structure_test//:defs.bzl", "container_structure_test")
load("@rules_go//go:def.bzl", "go_test")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_load")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")
load("@rules_rust//rust:defs.bzl", "rust_doc_test", "rust_library", "rust_test")

rust_library(
    name = "run",
    srcs = glob(["*.rs"]),
    crate_name = "run",
    crate_root = "run.rs",
    # will not work as expected on windows
    target_compatible_with = select({
        "@platforms//os:linux": [],
        "@platforms//os:macos": [],
        "//conditions:default": ["@platforms//:incompatible"],
    }),
    visibility = ["//visibility:public"],
    deps = [
        "//process/fsutil:fsutil_rs",
        "//process/reaper",
        "@crates//:clap",
        "@crates//:futures",
        "@crates//:humantime",
        "@crates//:libc",
        "@crates//:thiserror",
        "@crates//:tokio",
        "@crates//:tracing",
    ],
)

rust_test(
    name = "run_lib_test",
    size = "small",
    crate = ":run",
    deps = [
        "//testing:testing_rs",
    ],
)

rust_doc_test(
    name = "run_doc_test",
    size = "small",
    crate = ":run",
)

go_test(
    name = "run_test",
    srcs = ["run_test.go"],
    data = [
        "//process/run/cmd:orphan",
        "//process/run/cmd:run",
    ],
    deps = [
        "@org_golang_x_exp//constraints",
        "@rules_go//go/runfiles:go_default_library",
    ],
)

pkg_tar(
    name = "usr_bin",
    srcs = [
        "//process/run/cmd:orphan",
        "//process/run/cmd:run",
    ],
    package_dir = "/usr/bin",
)

oci_image(
    name = "image",
    base = "@distroless_cc_debian12_nonroot",
    entrypoint = ["/usr/bin/run"],
    target_compatible_with = [
        "@platforms//os:linux",
    ],
    tars = [
        ":usr_bin",
    ],
)

# To load on macOS:
#
#     devcontainer exec --workspace-folder $(bazel info workspace) bazel run :load
oci_load(
    name = "load",
    image = ":image",
    repo_tags = [package_name() + ":image"],
)

container_structure_test(
    name = "container_structure_test",
    configs = ["testdata/container_structure_test.yaml"],
    image = ":image",
    tags = [
        "manual",
    ],
    target_compatible_with = [
        "@platforms//os:linux",
    ],
)

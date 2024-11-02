# The definition of @local_config_platform//:host could be found at
# [workspace_root]/bazel-out/../../../external/local_config_platform/constraints.bzl

platform(
    name = "stable",
    constraint_values = [
        "@rules_rust//rust/platform/channel:stable",
    ],
    parents = ["@local_config_platform//:host"],
)

platform(
    name = "nightly",
    constraint_values = [
        "@rules_rust//rust/platform/channel:nightly",
    ],
    parents = ["@local_config_platform//:host"],
)

config_setting(
    name = "linux_x86_64",
    constraint_values = [
        "@platforms//os:linux",
        "@platforms//cpu:x86_64",
    ],
)

config_setting(
    name = "macos_x86_64",
    constraint_values = [
        "@platforms//os:macos",
        "@platforms//cpu:x86_64",
    ],
)

config_setting(
    name = "macos_arm64",
    constraint_values = [
        "@platforms//os:macos",
        "@platforms//cpu:arm64",
    ],
)

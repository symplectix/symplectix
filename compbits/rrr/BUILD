load("//bazel:rust.bzl", "rust")

# TODO: Consider not to share the build script and source files.
# Possibly, but this setup seems to confuse rust-analyzer.

[
    rust.library(
        name = rrr,
        srcs = ["rrr.rs"],
        crate_features = [rrr],
        visibility = ["//:__subpackages__"],
        deps = [":build_script"],
    )
    for rrr in [
        "rrr4",
        "rrr15",
        "rrr31",
        "rrr63",
    ]
]

rust.build_script(
    name = "build_script",
    srcs = ["build.rs"],
)

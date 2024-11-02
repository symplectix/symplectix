load("@bazel_skylib//rules:build_test.bzl", "build_test")

exports_files([
    "BUILD.zlib.bazel",
])

build_test(
    name = "build_test",
    targets = [
        "@zlib//:zlib",
    ],
)

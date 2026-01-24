load("//bazel:py.bzl", "py")

package(default_visibility = ["//:__subpackages__"])

exports_files(
    [
        "fuzztest.py",
    ],
)

py.binary(
    name = "workspace_status",
    srcs = ["workspace_status.py"],
    main = "workspace_status.py",
)

py.binary(
    name = "fuzztest",
    srcs = ["fuzztest.py"],
    main = "fuzztest.py",
)

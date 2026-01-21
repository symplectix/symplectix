load("@rules_python//python:defs.bzl", "py_binary")

package(default_visibility = ["//:__subpackages__"])

exports_files([
    "exec_fuzz_target.py",
])

py_binary(
    name = "exec_fuzz_target",
    srcs = ["exec_fuzz_target.py"],
)

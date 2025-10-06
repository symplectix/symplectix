load(
    "@rules_python//python:defs.bzl",
    _py_binary = "py_binary",
    _py_library = "py_library",
    _py_test = "py_test",
)

py = struct(
    binary = _py_binary,
    library = _py_library,
    test = _py_test,
)

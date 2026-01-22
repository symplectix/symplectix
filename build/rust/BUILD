load("@rules_python//python:defs.bzl", "py_binary")
load(":rust_fuzz_binary.bzl", "rust_fuzz_binary")

exports_files(
    [
        "exec_fuzz_target.py",
    ],
    visibility = ["//:__subpackages__"],
)

py_binary(
    name = "fuzztest",
    srcs = ["fuzztest.py"],
)

rust_fuzz_binary(
    name = "should_not_fail",
    srcs = ["examples/should_not_fail.rs"],
    tags = ["manual"],
    deps = [
        "@crates//:libfuzzer-sys",
    ],
)

rust_fuzz_binary(
    name = "buffer_overflow",
    srcs = ["examples/buffer_overflow.rs"],
    tags = ["manual"],
    deps = [
        "@crates//:libfuzzer-sys",
    ],
)

rust_fuzz_binary(
    name = "memory_leaks",
    srcs = ["examples/memory_leaks.rs"],
    tags = ["manual"],
    deps = [
        "@crates//:libfuzzer-sys",
    ],
)

rust_fuzz_binary(
    name = "use_after_scope",
    srcs = ["examples/use_after_scope.rs"],
    env = {
        "ASAN_OPTIONS": ":".join([
            "detact_stack_use_after_return=1",
        ]),
    },
    tags = ["manual"],
    deps = [
        "@crates//:libfuzzer-sys",
    ],
)

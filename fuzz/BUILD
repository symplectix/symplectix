load("//bazel:rust.bzl", "rust")

package(default_visibility = ["//:__subpackages__"])

rust.fuzz_target(
    name = "should_not_fail",
    testonly = True,
    srcs = ["should_not_fail.rs"],
    tags = ["manual"],
    deps = [
        "@crates//:libfuzzer-sys",
    ],
)

rust.fuzz_target(
    name = "buffer_overflow",
    testonly = True,
    srcs = ["buffer_overflow.rs"],
    tags = ["manual"],
    deps = [
        "@crates//:libfuzzer-sys",
    ],
)

# ASAN_OPTIONS="detect_leaks=1:detact_stack_use_after_return=1" \
# bazel run ...
rust.fuzz_target(
    name = "memory_leaks",
    testonly = True,
    srcs = ["memory_leaks.rs"],
    tags = ["manual"],
    deps = [
        "@crates//:libfuzzer-sys",
    ],
)

rust.fuzz_target(
    name = "stack_use_after_scope",
    testonly = True,
    srcs = ["stack_use_after_scope.rs"],
    tags = ["manual"],
    deps = [
        "@crates//:libfuzzer-sys",
    ],
)

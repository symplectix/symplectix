load("@rules_rust//rust:defs.bzl", "rust_binary")

def rust_fuzzing(
        name,
        sanitizer = None,
        **bin_kwargs):
    """Helps to fuzzing.
    """

    rustc_flags = [
        "--cfg=fuzzing",
        "-Cinstrument-coverage",
        "-Cpasses=sancov-module",
        "-Cllvm-args=-sanitizer-coverage-level=4",
        "-Cllvm-args=-sanitizer-coverage-inline-8bit-counters",
        "-Cllvm-args=-sanitizer-coverage-pc-table",
        "-Cllvm-args=-sanitizer-coverage-trace-compares",
        "-Zsanitizer={}".format(sanitizer),
    ]

    rust_binary(
        name = name,
        rustc_flags = select({
            "@rules_rust//rust/toolchain/channel:nightly": rustc_flags,
            "//conditions:default": [],
        }),
        # target_compatible_with = [
        #     "@rules_rust//rust/platform/channel:nightly",
        # ],
        **bin_kwargs
    )

load(
    "@rules_rust//rust:defs.bzl",
    "rust_binary",
)

visibility("//bazel")

def _rust_fuzz_target_impl(
        name,
        sanitizer = "address",
        no_cfg_fuzzing = False,
        no_strip_dead_code = False,
        trace_compares = True,
        trace_divs = True,
        trace_geps = True,
        disable_branch_folding = True,
        coverage = False,
        **kwargs):
    # https://github.com/rust-fuzz/cargo-fuzz
    # https://github.com/rust-fuzz/libfuzzer

    rustc_flags = []
    tags = ["fuzzing"]

    if not no_cfg_fuzzing:
        rustc_flags.append("--cfg=fuzzing")

    if not no_strip_dead_code:
        rustc_flags.append("-Clink-dead-code")

    rustc_flags.extend([
        "-Cpasses=sancov-module",
        "-Cllvm-args=-sanitizer-coverage-level=4",
        "-Cllvm-args=-sanitizer-coverage-inline-8bit-counters",
        "-Cllvm-args=-sanitizer-coverage-pc-table",
    ])

    # if os == "linux":
    rustc_flags.append("-Cllvm-args=-sanitizer-coverage-stack-depth")

    if trace_compares:
        rustc_flags.append("-Cllvm-args=-sanitizer-coverage-trace-compares")

    if trace_divs:
        rustc_flags.append("-Cllvm-args=-sanitizer-coverage-trace-divs")

    if trace_geps:
        rustc_flags.append("-Cllvm-args=-sanitizer-coverage-trace-geps")

    if disable_branch_folding:
        rustc_flags.append("-Cllvm-args=-simplifycfg-branch-fold-threshold=0")

    if coverage:
        rustc_flags.append("-Cinstrument-coverage")

    if sanitizer == "memory":
        rustc_flags.extend([
            "-Zsanitizer=memory",
            "-Zsanitizer-memory-track-origins",
        ])
    else:
        rustc_flags.append("-Zsanitizer={}".format(sanitizer))

    _rustc_flags = kwargs.pop("rustc_flags")
    if _rustc_flags == None:
        _rustc_flags = []

    _tags = kwargs.pop("tags")
    if _tags == None:
        _tags = []

    rust_binary(
        name = name,
        rustc_flags = _rustc_flags + rustc_flags,
        tags = _tags + tags,
        **kwargs
    )

rust_fuzz_target = macro(
    inherit_attrs = rust_binary,
    implementation = _rust_fuzz_target_impl,
    attrs = {
        "sanitizer": attr.string(
            configurable = False,
            doc = """
            Use a specific sanitizer, 'address' by default.
              https://doc.rust-lang.org/unstable-book/compiler-flags/sanitizer.html
            """,
            default = "address",
        ),
        "no_cfg_fuzzing": attr.bool(
            configurable = False,
            default = False,
        ),
        "no_strip_dead_code": attr.bool(
            configurable = False,
            default = False,
        ),
        "trace_compares": attr.bool(
            configurable = False,
            default = True,
        ),
        "trace_divs": attr.bool(
            configurable = False,
            doc = """
            Enables `sanitizer-coverage-trace-divs` LLVM instrumentation.
              When set to True, the compiler will instrument integer division instructions
              to capture the right argument of division.
            """,
            default = True,
        ),
        "trace_geps": attr.bool(
            configurable = False,
            doc = """
            trace_geps: Enables `sanitizer-coverage-trace-geps` LLVM instrumentation
              When set to True, instruments GetElementPtr (GEP) instructions to track
              pointer arithmetic operations to capture array indices.
            """,
            default = True,
        ),
        "disable_branch_folding": attr.bool(
            configurable = False,
            doc = """
            disable_branch_folding: Disable transformation of if-statements into `cmov` instructions.
              When set to True, we get no coverage feedback for that branch. This is done by
              setting the `-simplifycfg-branch-fold-threshold=0` LLVM arg.
            """,
            default = True,
        ),
        "coverage": attr.bool(
            configurable = False,
            doc = "Instrument program code with source-based code coverage information.",
            default = True,
        ),
    },
)

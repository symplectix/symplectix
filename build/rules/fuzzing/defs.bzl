load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//build/rules/fuzzing/private:fuzzing.bzl", "fuzz_binary", "fuzz_corpus")

def rust_fuzz_binary(
        name,
        sanitizer,
        corpus = None,
        envs = None,
        **kwargs):
    """Helps to fuzzing.
    """

    target_name = name + "_target"
    corpus_name = name + "_corpus"

    fuzz_binary(
        name = name,
        envs = envs,
        corpus = corpus_name,
        target = target_name,
        tags = ["fuzzing"],
    )

    fuzz_corpus(
        name = corpus_name,
        srcs = corpus,
        tags = ["fuzzing", "manual"],
    )

    kwargs.setdefault("rustc_flags", []).extend([
        "--cfg=fuzzing",
        "-Cinstrument-coverage",
        "-Cpasses=sancov-module",
        "-Cllvm-args=-sanitizer-coverage-level=4",
        "-Cllvm-args=-sanitizer-coverage-inline-8bit-counters",
        "-Cllvm-args=-sanitizer-coverage-pc-table",
        "-Cllvm-args=-sanitizer-coverage-trace-compares",
        "-Zsanitizer={}".format(sanitizer),
    ])

    kwargs.setdefault("tags", []).extend([
        "fuzzing",
    ])

    rust_binary(
        name = target_name,
        target_compatible_with = [
            "@rules_rust//rust/platform/channel:nightly",
        ],
        **kwargs
    )

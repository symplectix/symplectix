def _fuzz_test_impl(ctx):
    entrypoint = ctx.actions.declare_file(ctx.label.name)

    entrypoint_template = """\
{exports}
RUNFILES_DIR="$0.runfiles" \
exec "{fuzz_test}" \
{time_to_run} \
{envs} \
-- \
"{executable}" \
"{corpus}" \
"$@"
"""

    if hasattr(ctx.attr, "time_to_run") and ctx.attr.time_to_run != "":
        time_to_run = "--timeout {}".format(ctx.attr.time_to_run)
    else:
        time_to_run = ""

    entrypoint_content = entrypoint_template.format(
        exports = "\n".join([
            "export %s='%s'" % (key, val)
            for key, val in ctx.attr.envs.items()
        ]),
        fuzz_test = ctx.executable._fuzz_test.short_path,
        command = ctx.attr.command,
        corpus = ctx.file.corpus.short_path,
        time_to_run = time_to_run,
        envs = " ".join([
            "\"--env\" '%s'" % key
            for key in ctx.attr.envs.keys()
        ]),
        executable = ctx.executable.executable.short_path,
    )

    ctx.actions.write(
        entrypoint,
        entrypoint_content,
        is_executable = True,
    )

    runfiles = ctx.runfiles(files = [ctx.file.corpus]) \
        .merge(ctx.attr._fuzz_test[DefaultInfo].default_runfiles) \
        .merge(ctx.attr._fuzz_test[DefaultInfo].data_runfiles) \
        .merge(ctx.attr.executable[DefaultInfo].default_runfiles) \
        .merge(ctx.attr.executable[DefaultInfo].data_runfiles)

    return [
        DefaultInfo(
            executable = entrypoint,
            runfiles = runfiles,
        ),
    ]

_fuzz_test = rule(
    implementation = _fuzz_test_impl,
    test = True,
    attrs = {
        "_fuzz_test": attr.label(
            default = Label("@//fuzzing:fuzz_test"),
            executable = True,
            cfg = "exec",
        ),
        "time_to_run": attr.string(
            default = "",
            mandatory = True,
        ),
        "command": attr.string(
            mandatory = True,
        ),
        "corpus": attr.label(
            mandatory = True,
            allow_single_file = True,
        ),
        "envs": attr.string_dict(
            default = {},
            mandatory = False,
        ),
        "executable": attr.label(
            doc = "The executable of the fuzz test to run.",
            executable = True,
            allow_single_file = True,
            cfg = "target",
            mandatory = True,
        ),
    },
)

def fuzz_test(**kwargs):
    _fuzz_test(
        command = "test",
        time_to_run = "30s",
        **kwargs
    )

def _fuzz_corpus_impl(ctx):
    output = ctx.actions.declare_directory(ctx.attr.name)

    corpus_list_args = ctx.actions.args()
    corpus_list_args.set_param_file_format("multiline")
    corpus_list_args.use_param_file("--corpus-list=%s", use_always = True)
    corpus_list_args.add_all(ctx.files.srcs)

    output_args = ctx.actions.args()
    output_args.add("--output=" + output.path)

    ctx.actions.run(
        executable = ctx.executable._fuzz_corpus,
        arguments = [output_args, corpus_list_args],
        inputs = ctx.files.srcs,
        outputs = [output],
    )

    return [DefaultInfo(
        runfiles = ctx.runfiles(files = [output]),
        files = depset([output]),
    )]

fuzz_corpus = rule(
    implementation = _fuzz_corpus_impl,
    doc = """
This rule creates a directory collecting all the corpora files
specified in the srcs attribute.
""",
    attrs = {
        "_fuzz_corpus": attr.label(
            default = Label("@//fuzzing:fuzz_corpus"),
            executable = True,
            cfg = "exec",
        ),
        "srcs": attr.label_list(
            doc = "The corpus files for the fuzzing test.",
            allow_files = True,
        ),
    },
)

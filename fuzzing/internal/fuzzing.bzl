def _fuzz_test_impl(ctx):
    entrypoint = ctx.actions.declare_file(ctx.label.name)

    entrypoint_template = """\
{exports}
RUNFILES_DIR="$0.runfiles" \
exec "{run}" \
{envs} \
--kill-after {time_to_run} \
--timeout-is-ok \
-- \
"{executable}" \
"{corpus}" \
"$@"
"""

    if hasattr(ctx.attr, "time_to_run") and ctx.attr.time_to_run != "":
        time_to_run = "{}".format(ctx.attr.time_to_run)
    else:
        time_to_run = ""

    entrypoint_content = entrypoint_template.format(
        exports = "\n".join([
            "export %s='%s'" % (key, val)
            for key, val in ctx.attr.envs.items()
        ]),
        run = ctx.executable._run.short_path,
        envs = " ".join([
            "\"--env\" '%s'" % key
            for key in ctx.attr.envs.keys()
        ]),
        time_to_run = time_to_run,
        executable = ctx.executable.executable.short_path,
        corpus = ctx.file.corpus.short_path,
    )

    ctx.actions.write(
        entrypoint,
        entrypoint_content,
        is_executable = True,
    )

    runfiles = ctx.runfiles(files = [ctx.file.corpus]) \
        .merge(ctx.attr._run[DefaultInfo].default_runfiles) \
        .merge(ctx.attr._run[DefaultInfo].data_runfiles) \
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
        "_run": attr.label(
            default = Label("@//process/run/cmd:run"),
            executable = True,
            cfg = "exec",
        ),
        "envs": attr.string_dict(
            default = {},
            mandatory = False,
        ),
        "time_to_run": attr.string(
            default = "",
            mandatory = True,
        ),
        "executable": attr.label(
            doc = "The executable of the fuzz test to run.",
            executable = True,
            allow_single_file = True,
            cfg = "target",
            mandatory = True,
        ),
        "corpus": attr.label(
            mandatory = True,
            allow_single_file = True,
        ),
    },
)

def fuzz_test(**kwargs):
    _fuzz_test(
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

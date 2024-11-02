load("@aspect_bazel_lib//lib:jq.bzl", "jq")

jq(
    name = "status",
    srcs = [],
    filter = "|".join([
        "$ARGS.named.STAMP as $stamp",
        """
        {
            gitCommit: ($stamp.STABLE_GIT_COMMIT // \"none\"),
            githubRunId: ($stamp.STABLE_GITHUB_RUN_ID // \"none\"),
            githubRunNumber: ($stamp.STABLE_GITHUB_RUN_NUMBER // \"none\"),
        }
        """,
    ]),
)

genrule(
    name = "git_commit",
    srcs = [":status.json"],
    outs = ["git_commit.txt"],
    cmd = "$(JQ_BIN) -r '.gitCommit' $(location :status.json) > $@",
    toolchains = ["@jq_toolchains//:resolved_toolchain"],
)

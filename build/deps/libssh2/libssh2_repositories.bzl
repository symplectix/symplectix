load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

def libssh2_repositories():
    maybe(
        http_archive,
        name = "libssh2",
        urls = [
            "https://mirror.bazel.build/github.com/libssh2/libssh2/releases/download/libssh2-1.10.0/libssh2-1.10.0.tar.gz",
            "https://github.com/libssh2/libssh2/releases/download/libssh2-1.10.0/libssh2-1.10.0.tar.gz",
        ],
        type = "tar.gz",
        sha256 = "2d64e90f3ded394b91d3a2e774ca203a4179f69aebee03003e5a6fa621e41d51",
        strip_prefix = "libssh2-1.10.0",
        build_file = Label("//build/deps/libssh2:BUILD.libssh2.bazel"),
    )

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

_LIBGIT2_VERSION = "1.5.0"

def libgit2_repositories():
    maybe(
        http_archive,
        name = "libgit2",
        build_file = Label("//thirdparty/libgit2:BUILD.libgit2.bazel"),
        sha256 = "8de872a0f201b33d9522b817c92e14edb4efad18dae95cf156cf240b2efff93e",
        strip_prefix = "libgit2-{}".format(_LIBGIT2_VERSION),
        urls = [url.format(version = _LIBGIT2_VERSION) for url in [
            "https://mirror.bazel.build/github.com/libgit2/libgit2/archive/refs/tags/v{version}.tar.gz",
            "https://github.com/libgit2/libgit2/archive/refs/tags/v{version}.tar.gz",
        ]],
    )

"""A module defining the third party dependency OpenSSL"""

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")

def openssl_repositories():
    maybe(
        http_archive,
        name = "openssl",
        build_file = Label("//build/deps/openssl:BUILD.openssl.bazel"),
        sha256 = "e2f8d84b523eecd06c7be7626830370300fbcc15386bf5142d72758f6963ebc6",
        strip_prefix = "openssl-1.1.1u",
        urls = [
            "https://mirror.bazel.build/www.openssl.org/source/openssl-1.1.1u.tar.gz",
            "https://www.openssl.org/source/openssl-1.1.1u.tar.gz",
            "https://github.com/openssl/openssl/archive/OpenSSL_1_1_1u.tar.gz",
        ],
    )

    maybe(
        http_archive,
        name = "nasm",
        build_file = Label("//build/deps/openssl:BUILD.nasm.bazel"),
        sha256 = "f5c93c146f52b4f1664fa3ce6579f961a910e869ab0dae431bd871bdd2584ef2",
        strip_prefix = "nasm-2.15.05",
        urls = [
            "https://mirror.bazel.build/www.nasm.us/pub/nasm/releasebuilds/2.15.05/win64/nasm-2.15.05-win64.zip",
            "https://www.nasm.us/pub/nasm/releasebuilds/2.15.05/win64/nasm-2.15.05-win64.zip",
        ],
    )

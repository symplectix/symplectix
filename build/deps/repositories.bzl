load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")
load("@rules_rust//crate_universe:defs.bzl", "splicing_config")
load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")
load("@rules_rust//proto/prost:repositories.bzl", "rust_prost_dependencies")
load("@rules_rust//proto/prost:transitive_repositories.bzl", "rust_prost_transitive_repositories")
load("//build/deps:versions.bzl", "RUST_STABLE_VERSION")
load("//build/deps/crates:defs.bzl", "bin_crates", "lib_crates")

def build_deps_repositories():
    # openssl
    maybe(
        http_archive,
        name = "openssl",
        build_file = Label("//build/deps/openssl:BUILD.openssl.bazel"),
        sha256 = "cf3098950cb4d853ad95c0841f1f9c6d3dc102dccfcacd521d93925208b76ac8",
        strip_prefix = "openssl-1.1.1w",
        urls = [
            "https://mirror.bazel.build/www.openssl.org/source/openssl-1.1.1w.tar.gz",
            "https://www.openssl.org/source/openssl-1.1.1w.tar.gz",
            "https://github.com/openssl/openssl/archive/OpenSSL_1_1_1w.tar.gz",
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

    # zlib
    maybe(
        http_archive,
        name = "zlib",
        build_file = Label("//build/deps/zlib:BUILD.zlib.bazel"),
        sha256 = "b3a24de97a8fdbc835b9833169501030b8977031bcb54b3b3ac13740f846ab30",
        strip_prefix = "zlib-1.2.13",
        urls = [
            "https://zlib.net/zlib-1.2.13.tar.gz",
            "https://storage.googleapis.com/mirror.tensorflow.org/zlib.net/zlib-1.2.13.tar.gz",
        ],
    )

    # For prost and tonic.
    rust_prost_dependencies()
    rust_prost_transitive_repositories()

    # If the current version of rules_rust is not a release artifact,
    # you may need to set additional flags such as bootstrap = True.
    crate_universe_dependencies()

    bin_crates.dependencies()

    # CARGO_BAZEL_REPIN=1 CARGO_BAZEL_REPIN_ONLY=bin_crates bazel sync --only=bin_crates
    bin_crates.repository()

    # CARGO_BAZEL_REPIN=1 CARGO_BAZEL_REPIN_ONLY=lib_crates bazel sync --only=lib_crates
    lib_crates.repository(
        # The version of Rust the currently registered toolchain is using.
        rust_version = RUST_STABLE_VERSION,
        splicing_config = splicing_config(
            resolver_version = "2",
        ),
    )

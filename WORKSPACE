workspace(name = "trunk")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "platforms",
    sha256 = "3a561c99e7bdbe9173aa653fd579fe849f1d8d67395780ab4770b1f381431d51",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/platforms/releases/download/0.0.7/platforms-0.0.7.tar.gz",
        "https://github.com/bazelbuild/platforms/releases/download/0.0.7/platforms-0.0.7.tar.gz",
    ],
)

http_archive(
    name = "bazel_skylib",
    sha256 = "66ffd9315665bfaafc96b52278f57c7e2dd09f5ede279ea6d39b2be471e7e3aa",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.4.2/bazel-skylib-1.4.2.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.4.2/bazel-skylib-1.4.2.tar.gz",
    ],
)

http_archive(
    name = "aspect_bazel_lib",
    sha256 = "09b51a9957adc56c905a2c980d6eb06f04beb1d85c665b467f659871403cf423",
    strip_prefix = "bazel-lib-1.34.5",
    url = "https://github.com/aspect-build/bazel-lib/releases/download/v1.34.5/bazel-lib-v1.34.5.tar.gz",
)

http_archive(
    name = "rules_cc",
    sha256 = "2037875b9a4456dce4a79d112a8ae885bbc4aad968e6587dca6e64f3a0900cdf",
    strip_prefix = "rules_cc-0.0.9",
    urls = ["https://github.com/bazelbuild/rules_cc/releases/download/0.0.9/rules_cc-0.0.9.tar.gz"],
)

http_archive(
    name = "rules_foreign_cc",
    sha256 = "2a4d07cd64b0719b39a7c12218a3e507672b82a97b98c6a89d38565894cf7c51",
    strip_prefix = "rules_foreign_cc-0.9.0",
    url = "https://github.com/bazelbuild/rules_foreign_cc/archive/refs/tags/0.9.0.tar.gz",
)

http_archive(
    name = "zlib",
    build_file = "//build/deps/zlib:BUILD.zlib.bazel",
    sha256 = "b3a24de97a8fdbc835b9833169501030b8977031bcb54b3b3ac13740f846ab30",
    strip_prefix = "zlib-1.2.13",
    urls = [
        "https://zlib.net/zlib-1.2.13.tar.gz",
        "https://storage.googleapis.com/mirror.tensorflow.org/zlib.net/zlib-1.2.13.tar.gz",
    ],
)

http_archive(
    name = "openssl",
    build_file = "//build/deps/openssl:BUILD.openssl.bazel",
    sha256 = "cf3098950cb4d853ad95c0841f1f9c6d3dc102dccfcacd521d93925208b76ac8",
    strip_prefix = "openssl-1.1.1w",
    urls = [
        "https://mirror.bazel.build/www.openssl.org/source/openssl-1.1.1w.tar.gz",
        "https://www.openssl.org/source/openssl-1.1.1w.tar.gz",
        "https://github.com/openssl/openssl/archive/OpenSSL_1_1_1w.tar.gz",
    ],
)

http_archive(
    name = "nasm",
    build_file = "//build/deps/openssl:BUILD.nasm.bazel",
    sha256 = "f5c93c146f52b4f1664fa3ce6579f961a910e869ab0dae431bd871bdd2584ef2",
    strip_prefix = "nasm-2.15.05",
    urls = [
        "https://mirror.bazel.build/www.nasm.us/pub/nasm/releasebuilds/2.15.05/win64/nasm-2.15.05-win64.zip",
        "https://www.nasm.us/pub/nasm/releasebuilds/2.15.05/win64/nasm-2.15.05-win64.zip",
    ],
)

http_archive(
    name = "rules_perl",
    sha256 = "391edb08802860ba733d402c6376cfe1002b598b90d2240d9d302ecce2289a64",
    strip_prefix = "rules_perl-7f10dada09fcba1dc79a6a91da2facc25e72bd7d",
    urls = [
        "https://github.com/bazelbuild/rules_perl/archive/7f10dada09fcba1dc79a6a91da2facc25e72bd7d.tar.gz",
    ],
)

http_archive(
    name = "rules_rust",
    sha256 = "c46bdafc582d9bd48a6f97000d05af4829f62d5fee10a2a3edddf2f3d9a232c1",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.28.0/rules_rust-v0.28.0.tar.gz"],
)

http_archive(
    name = "io_bazel_rules_go",
    sha256 = "278b7ff5a826f3dc10f04feaf0b70d48b68748ccd512d7f98bf442077f043fe3",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.41.0/rules_go-v0.41.0.zip",
        "https://github.com/bazelbuild/rules_go/releases/download/v0.41.0/rules_go-v0.41.0.zip",
    ],
)

http_archive(
    name = "rules_proto",
    sha256 = "dc3fb206a2cb3441b485eb1e423165b231235a1ea9b031b4433cf7bc1fa460dd",
    strip_prefix = "rules_proto-5.3.0-21.7",
    urls = [
        "https://github.com/bazelbuild/rules_proto/archive/refs/tags/5.3.0-21.7.tar.gz",
    ],
)

http_archive(
    name = "rules_proto_grpc",
    sha256 = "928e4205f701b7798ce32f3d2171c1918b363e9a600390a25c876f075f1efc0a",
    strip_prefix = "rules_proto_grpc-4.4.0",
    urls = ["https://github.com/rules-proto-grpc/rules_proto_grpc/releases/download/4.4.0/rules_proto_grpc-4.4.0.tar.gz"],
)

http_archive(
    name = "bazel_gazelle",
    sha256 = "29218f8e0cebe583643cbf93cae6f971be8a2484cdcfa1e45057658df8d54002",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-gazelle/releases/download/v0.32.0/bazel-gazelle-v0.32.0.tar.gz",
        "https://github.com/bazelbuild/bazel-gazelle/releases/download/v0.32.0/bazel-gazelle-v0.32.0.tar.gz",
    ],
)

http_archive(
    name = "com_github_bazelbuild_buildtools",
    sha256 = "b7187e0856280feb0658ab9d629c244e638022819ded8243fb02e0c1d4db8f1c",
    strip_prefix = "buildtools-6.3.2",
    urls = ["https://github.com/bazelbuild/buildtools/archive/refs/tags/v6.3.2.tar.gz"],
)

load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")

rules_proto_dependencies()

rules_proto_toolchains()

load("@rules_proto_grpc//:repositories.bzl", "rules_proto_grpc_repos", "rules_proto_grpc_toolchains")

rules_proto_grpc_toolchains()

rules_proto_grpc_repos()

load("//build/deps:repositories.bzl", "build_dependencies")

build_dependencies()

load("//build/deps:setup.bzl", "build_dependencies_setup")

build_dependencies_setup()

# OCI
http_archive(
    name = "rules_oci",
    sha256 = "a3b6f4c0051938940ccf251a7bdcdf7ac5a93ae00e63ad107c9c6d3bfe20885b",
    strip_prefix = "rules_oci-1.3.1",
    url = "https://github.com/bazel-contrib/rules_oci/releases/download/v1.3.1/rules_oci-v1.3.1.tar.gz",
)

load("@rules_oci//oci:dependencies.bzl", "rules_oci_dependencies")

rules_oci_dependencies()

load("@rules_oci//oci:repositories.bzl", "LATEST_CRANE_VERSION", "oci_register_toolchains")

oci_register_toolchains(
    name = "oci",
    crane_version = LATEST_CRANE_VERSION,
    # Uncommenting the zot toolchain will cause it to be used instead of crane for some tasks.
    # Note that it does not support docker-format images.
    # zot_version = LATEST_ZOT_VERSION,
)

load("@rules_oci//oci:pull.bzl", "oci_pull")

# The image contains:
# - ca-certificates
# - A /etc/passwd entry for a root user
# - A /tmp directory
# - tzdata
# - glibc
# - libssl
# - openssl
oci_pull(
    name = "distroless_base_nonroot",
    digest = "sha256:c62385962234a3dae5c9e9777dedc863d99f676b7202cd073e90b06e46021994",
    image = "gcr.io/distroless/base",
    platforms = [
        "linux/amd64",
        "linux/arm64",
    ],
)

# The image contains everything in the base image, plus:
# libgcc1 and its dependencies.
oci_pull(
    name = "distroless_cc_nonroot",
    digest = "sha256:880bcf2ca034ab5e8ae76df0bd50d700e54eb44e948877244b130e3fcd5a1d66",
    image = "gcr.io/distroless/cc",
    platforms = [
        "linux/amd64",
        "linux/arm64",
    ],
)

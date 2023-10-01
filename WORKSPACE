workspace(name = "trunk")

load("//:workspace.bzl", "repo")

repo.http_archives()

repo.http_files()

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")

bazel_skylib_workspace()

load("@aspect_bazel_lib//lib:repositories.bzl", "aspect_bazel_lib_dependencies", "register_jq_toolchains", "register_yq_toolchains")

aspect_bazel_lib_dependencies()

register_jq_toolchains()

register_yq_toolchains()

load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")

rules_proto_dependencies()

rules_proto_toolchains()

load("@rules_proto_grpc//:repositories.bzl", "rules_proto_grpc_repos", "rules_proto_grpc_toolchains")

rules_proto_grpc_toolchains()

rules_proto_grpc_repos()

load("//3rdparty:repositories.bzl", "build_dependencies")

build_dependencies()

load("//3rdparty:setup.bzl", "build_dependencies_setup")

build_dependencies_setup()

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

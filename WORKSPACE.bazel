workspace(name = "trunk")

load("//:workspace.bzl", "versions", "workspace_dependencies", "workspace_toolchains")

workspace_dependencies()

workspace_toolchains()

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")

bazel_skylib_workspace()

load("@aspect_bazel_lib//lib:repositories.bzl", "aspect_bazel_lib_dependencies", "register_jq_toolchains", "register_yq_toolchains")

aspect_bazel_lib_dependencies()

register_jq_toolchains()

register_yq_toolchains()

# This sets up some common toolchains for building targets. For more details, please see
# https://bazelbuild.github.io/rules_foreign_cc/0.9.0/flatten.html#rules_foreign_cc_dependencies
load("@rules_foreign_cc//foreign_cc:repositories.bzl", "rules_foreign_cc_dependencies")

rules_foreign_cc_dependencies()

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = versions.rust.edition,
    versions = versions.rust.versions,
)

load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_dependencies")

# Load the dependencies for the rust-project.json generator tool.
# To regenerate the rust-project.json file:
#   bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
rust_analyzer_dependencies()

load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")

go_rules_dependencies()

go_register_toolchains(
    version = versions.go.version,
)

load("@rules_perl//perl:deps.bzl", "perl_register_toolchains", "perl_rules_dependencies")

perl_rules_dependencies()

perl_register_toolchains()

load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")

rules_proto_dependencies()

rules_proto_toolchains()

load("@rules_proto_grpc//:repositories.bzl", "rules_proto_grpc_repos", "rules_proto_grpc_toolchains")

rules_proto_grpc_toolchains()

rules_proto_grpc_repos()

# For prost and tonic.
load("@rules_rust//proto/prost:repositories.bzl", "rust_prost_dependencies")
load("@rules_rust//proto/prost:transitive_repositories.bzl", "rust_prost_transitive_repositories")

rust_prost_dependencies()

rust_prost_transitive_repositories()

load("@rules_rust//crate_universe:defs.bzl", "splicing_config")

# If the current version of rules_rust is not a release artifact,
# you may need to set additional flags such as bootstrap = True.
load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("//x/bin_crates:defs.bzl", "bin_crates")
load("//x/crates:defs.bzl", "crates")

# Cargo packages that contain a library. To generate Bazel targets for binaries,
# you must annotate on the package. See defs.bzl for working examples.
#
# CARGO_BAZEL_REPIN=1 CARGO_BAZEL_REPIN_ONLY=crates bazel sync --only=crates
crates.repository(
    splicing_config = splicing_config(
        # The resolver version to use in generated Cargo manifests.
        # This flag is only used when splicing a manifest from direct package definitions.
        # https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
        resolver_version = "2",
    ),
)

# The bin_crates repository is for cargo packages that contain binaries but no library.
# https://bazelbuild.github.io/rules_rust/crate_universe.html#binary-dependencies
#
# CARGO_BAZEL_REPIN=1 CARGO_BAZEL_REPIN_ONLY=bin_crates bazel sync --only=bin_crates
bin_crates.repository()

load("@bin_crates//:defs.bzl", bin_crates_repositories = "crate_repositories")
load("@crates//:defs.bzl", crates_repositories = "crate_repositories")

bin_crates_repositories()

crates_repositories()

load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies")

gazelle_dependencies()

load("@rules_pkg//:deps.bzl", "rules_pkg_dependencies")

rules_pkg_dependencies()

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

load("@container_structure_test//:repositories.bzl", "container_structure_test_register_toolchain")

container_structure_test_register_toolchain(name = "cst")

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

load("@bin_crates//:defs.bzl", bin_crates_repositories = "crate_repositories")
load("@crates//:defs.bzl", crates_repositories = "crate_repositories")

def build_dependencies_setup():
    bin_crates_repositories()
    crates_repositories()

    native.register_toolchains("//build/toolchains:prost_toolchain")

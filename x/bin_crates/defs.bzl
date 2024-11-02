load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

def _bin_crates_repository(**kwargs):
    http_archive(
        name = "geckodriver",
        build_file = "//x/bin_crates:BUILD.geckodriver.bazel",
        sha256 = "6847d9046206c0f0189857d356991b9b225554045241cb0d33b43c1c83d732b7",
        strip_prefix = "geckodriver-0.33.0",
        type = "tar.gz",
        urls = ["https://crates.io/api/v1/crates/geckodriver/0.33.0/download"],
    )

    crates_repository(
        name = "bin_crates",
        cargo_lockfile = "//x/bin_crates:Cargo.lock",
        lockfile = "//x/bin_crates:Cargo.Bazel.lock",
        manifests = [
            "@geckodriver//:Cargo.toml",
        ],
        **kwargs
    )

bin_crates = struct(
    repository = _bin_crates_repository,
)

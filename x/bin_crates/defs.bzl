load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

def _bin_crates_repository(**kwargs):
    http_archive(
        name = "geckodriver",
        build_file = "//x/bin_crates:BUILD.geckodriver.bazel",
        sha256 = "457dd896a5962cdc143acc7dbce147d32ec6578599e7874c5ee6fb445f12fc02",
        strip_prefix = "geckodriver-0.34.0",
        type = "tar.gz",
        urls = ["https://crates.io/api/v1/crates/geckodriver/0.34.0/download"],
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

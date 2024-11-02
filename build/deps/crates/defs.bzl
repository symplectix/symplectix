load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository")

_lib_crates_annotations = {
    "openssl-sys": [crate.annotation(
        build_script_data = [
            "@openssl//:openssl_dir",
            "@openssl//:openssl",
        ],
        # https://github.com/sfackler/rust-openssl/tree/master/openssl-sys/build
        build_script_data_glob = ["build/**/*.c"],
        build_script_env = {
            "OPENSSL_DIR": "$(execpath @openssl//:openssl_dir)",
            "OPENSSL_STATIC": "1",
        },
        data = ["@openssl"],
        deps = ["@openssl"],
    )],
    "libz-sys": [crate.annotation(
        gen_build_script = False,
        deps = ["@zlib"],
    )],
    "cargo-audit": [crate.annotation(
        gen_binaries = ["cargo-audit"],
    )],
    "protoc-gen-prost": [crate.annotation(
        gen_binaries = ["protoc-gen-prost"],
    )],
    "protoc-gen-tonic": [crate.annotation(
        gen_binaries = ["protoc-gen-tonic"],
    )],
}

_lib_crates_packages = {
    "libc": crate.spec(
        version = "0.2",
    ),
    "openssl": crate.spec(
        version = "0.10.55",
    ),
    "openssl-sys": crate.spec(
        version = "0.9.85",
    ),
    "libz-sys": crate.spec(
        version = "1.1.0",
        features = ["libc"],
    ),
    "serde": crate.spec(
        version = "1.0.188",
        features = ["derive"],
    ),

    # Proroc plugins for prost/tonic
    "protoc-gen-prost": crate.spec(
        version = "0",
    ),
    "protoc-gen-tonic": crate.spec(
        version = "0",
    ),

    # Protobuf support
    "prost": crate.spec(
        version = "0.12",
    ),
    # Protobuf well-known types
    "prost-types": crate.spec(
        version = "0.12",
    ),
    "prost-reflect": crate.spec(
        version = "0.12",
    ),
    "tonic": crate.spec(
        version = "0.10",
    ),
    "tonic-types": crate.spec(
        version = "0.10",
    ),
    "tonic-health": crate.spec(
        version = "0.10",
    ),
    "tonic-reflection": crate.spec(
        version = "0.10",
    ),

    # For fuzzing
    "arbitrary": crate.spec(
        version = "1",
        features = ["derive"],
    ),
    "libfuzzer-sys": crate.spec(
        version = "0.4",
    ),

    # Result/Error helpers
    "anyhow": crate.spec(
        version = "1",
    ),
    "thiserror": crate.spec(
        version = "1",
    ),

    # Futures extensions
    "futures": crate.spec(
        version = "0.3",
    ),
    # Async runtime
    "tokio": crate.spec(
        version = "1.29.1",
        features = ["full"],
    ),
    # Async fn in traits
    # https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html
    "async-trait": crate.spec(
        version = "0.1",
    ),

    # Tracing
    "tracing": crate.spec(
        version = "0.1",
    ),
    "tracing-subscriber": crate.spec(
        version = "0.3",
    ),

    # Arguments parsing
    "clap": crate.spec(
        version = "4.3",
        features = ["derive"],
    ),

    # Includes formatters and parsers for std::time::SystemTime/std::time::Duration
    "humantime": crate.spec(
        version = "2",
    ),

    # Provides a macro to generate structures which behave like a set of bitflags
    "bitflags": crate.spec(
        version = "2",
    ),

    # TODO: Use std::cell::OnceCell
    # https://doc.rust-lang.org/std/cell/struct.OnceCell.html
    "once_cell": crate.spec(
        version = "1.18",
    ),
    "tempfile": crate.spec(
        version = "3",
    ),

    # "rand": crate.spec(
    #     version = "0.8.5",
    # ),
    "quickcheck": crate.spec(
        version = "1",
    ),
    "quickcheck_macros": crate.spec(
        version = "1",
    ),

    # Audit
    "cargo-audit": crate.spec(
        version = "0.18.1",
    ),
}

def _lib_crates_repository(**kwargs):
    crates_repository(
        name = "lib_crates",
        annotations = _lib_crates_annotations,
        cargo_lockfile = "//build/deps/crates/lib_crates:Cargo.lock",
        lockfile = "//build/deps/crates/lib_crates:Cargo.Bazel.lock",
        packages = _lib_crates_packages,
        **kwargs
    )

lib_crates = struct(
    repository = _lib_crates_repository,
)

# In order to depend on a Cargo package that contains binaries but no library:
# * use http_archive to import its source code,
# * use crates_repository to make build targets for its dependencies
# * create build targets for the binary
#
# https://bazelbuild.github.io/rules_rust/crate_universe.html#binary-dependencies
def _bin_crates_dependencies():
    http_archive(
        name = "geckodriver",
        build_file = "//build/deps/crates/bin_crates:BUILD.geckodriver.bazel",
        sha256 = "6847d9046206c0f0189857d356991b9b225554045241cb0d33b43c1c83d732b7",
        strip_prefix = "geckodriver-0.33.0",
        type = "tar.gz",
        urls = ["https://crates.io/api/v1/crates/geckodriver/0.33.0/download"],
    )

def _bin_crates_repository(**kwargs):
    crates_repository(
        name = "bin_crates",
        cargo_lockfile = "//build/deps/crates/bin_crates:Cargo.lock",
        lockfile = "//build/deps/crates/bin_crates:Cargo.Bazel.lock",
        manifests = [
            "@geckodriver//:Cargo.toml",
        ],
        **kwargs
    )

bin_crates = struct(
    dependencies = _bin_crates_dependencies,
    repository = _bin_crates_repository,
)

load("@//:toolchain.bzl", "RUST_STABLE_VERSION")
load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository", "splicing_config")
load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

_annotations = {
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
    "libgit2-sys": [crate.annotation(
        gen_build_script = False,
        deps = [
            "@libgit2",
            "@//build/deps/crates:build_libgit2_sys",
        ],
    )],
    "libssh2-sys": [crate.annotation(
        gen_build_script = False,
        deps = ["@libssh2"],
    )],
    "libz-sys": [crate.annotation(
        gen_build_script = False,
        deps = ["@zlib"],
    )],
    "cargo-audit": [crate.annotation(
        gen_binaries = ["cargo-audit"],
    )],
}

_packages = {
    "libc": crate.spec(
        version = "0.2",
    ),
    "openssl": crate.spec(
        version = "0.10.48",
    ),
    "openssl-sys": crate.spec(
        version = "0.9.83",
    ),
    "ssh2": crate.spec(
        version = "0.9",
    ),
    "libssh2-sys": crate.spec(
        version = "0.2",
    ),
    "git2": crate.spec(
        version = "0.16.1",
    ),
    "libgit2-sys": crate.spec(
        version = "0.14.2+1.5.1",
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
        version = "1.24",
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
        version = "4.1",
        features = ["derive"],
    ),

    # Includes formatters and parsers for std::time::SystemTime/std::time::Duration
    "humantime": crate.spec(
        version = "2",
    ),

    # Provides a macro to generate structures which behave like a set of bitflags
    "bitflags": crate.spec(
        version = "2.0.0-rc.1",
    ),

    # TODO: Use std::cell::OnceCell
    # https://doc.rust-lang.org/std/cell/struct.OnceCell.html
    "once_cell": crate.spec(
        version = "1.17",
    ),
    "tempfile": crate.spec(
        version = "3.4",
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
        version = "0.17",
    ),
}

def crates_repositories():
    crate_universe_dependencies(
        rust_version = RUST_STABLE_VERSION,
    )

    # Repinning
    # https://bazelbuild.github.io/rules_rust/crate_universe.html#repinning--updating-dependencies
    crates_repository(
        name = "crates",
        cargo_lockfile = "//build/deps/crates:cargo.lock",
        lockfile = "//build/deps/crates:cargo-bazel-lock.json",
        annotations = _annotations,
        packages = _packages,
        # The version of Rust the currently registered toolchain is using.
        rust_version = RUST_STABLE_VERSION,
        splicing_config = splicing_config(
            resolver_version = "2",
        ),
    )

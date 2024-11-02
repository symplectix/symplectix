load("@rules_rust//crate_universe:defs.bzl", "crate")

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
    "protoc-gen-prost": [crate.annotation(
        gen_binaries = ["protoc-gen-prost"],
    )],
    "protoc-gen-tonic": [crate.annotation(
        gen_binaries = ["protoc-gen-tonic"],
    )],
}

_packages = {
    "libc": crate.spec(
        version = "0.2",
    ),
    "openssl": crate.spec(
        version = "0.10.55",
    ),
    "openssl-sys": crate.spec(
        version = "0.9.85",
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
    "serde": crate.spec(
        version = "1.0.188",
        features = ["derive"],
    ),

    # Protobuf support
    "prost": crate.spec(
        version = "0",
    ),
    # Protobuf well-known types
    "prost-types": crate.spec(
        version = "0",
    ),
    "prost-reflect": crate.spec(
        version = "0",
    ),
    "protoc-gen-prost": crate.spec(
        version = "0",
    ),
    "protoc-gen-tonic": crate.spec(
        version = "0",
    ),
    "tonic": crate.spec(
        version = "0",
    ),
    "tonic-types": crate.spec(
        version = "0",
    ),
    "tonic-health": crate.spec(
        version = "0",
    ),
    "tonic-reflection": crate.spec(
        version = "0",
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
        version = "0.17.6",
    ),
}

crates = struct(
    annotations = _annotations,
    packages = _packages,
)

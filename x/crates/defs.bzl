load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository")

_crates = {
    "libc": struct(
        spec = crate.spec(
            version = "0.2",
        ),
    ),
    "libz-sys": struct(
        spec = crate.spec(
            version = "1.1.0",
            features = ["libc"],
        ),
        annotations = [crate.annotation(
            gen_build_script = False,
            deps = ["@zlib"],
        )],
    ),
    "openssl": struct(
        spec = crate.spec(
            version = "0.10.55",
        ),
    ),
    "openssl-sys": struct(
        spec = crate.spec(
            version = "0.9.85",
        ),
        annotations = [crate.annotation(
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
    ),

    # A framework for serializing and deserializing Rust data structures.
    "serde": struct(
        spec = crate.spec(
            version = "1.0.188",
            features = ["derive"],
        ),
    ),

    # Futures extensions
    "futures": struct(
        spec = crate.spec(
            version = "0.3",
        ),
    ),
    # Async runtime
    "tokio": struct(
        spec = crate.spec(
            version = "1.29.1",
            features = ["full"],
        ),
    ),

    # Async fn in traits
    # https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html
    "async-trait": struct(
        spec = crate.spec(
            version = "0.1",
        ),
    ),

    # Tracing
    "tracing": struct(
        spec = crate.spec(
            version = "0.1",
        ),
    ),
    "tracing-subscriber": struct(
        spec = crate.spec(
            version = "0.3",
        ),
    ),

    # Protobuf support for Rust.
    "prost": struct(
        spec = crate.spec(
            version = "0.12",
        ),
    ),
    "prost-types": struct(
        spec = crate.spec(
            version = "0.12",
        ),
    ),
    "prost-reflect": struct(
        spec = crate.spec(
            version = "0.12",
        ),
    ),
    "tonic": struct(
        spec = crate.spec(
            version = "0.10",
        ),
    ),
    "tonic-types": struct(
        spec = crate.spec(
            version = "0.10",
        ),
    ),
    "tonic-health": struct(
        spec = crate.spec(
            version = "0.10",
        ),
    ),
    "tonic-reflection": struct(
        spec = crate.spec(
            version = "0.10",
        ),
    ),

    # Proroc plugins for prost/tonic.
    "protoc-gen-prost": struct(
        spec = crate.spec(
            version = "0",
        ),
        annotations = [crate.annotation(
            gen_binaries = ["protoc-gen-prost"],
        )],
    ),
    "protoc-gen-tonic": struct(
        spec = crate.spec(
            version = "0",
        ),
        annotations = [crate.annotation(
            gen_binaries = ["protoc-gen-tonic"],
        )],
    ),

    # A tree-sitter binding.
    # Tree-sitter is a parser generator tool and an incremental parsing library.
    #
    # https://tree-sitter.github.io/tree-sitter/
    "tree-sitter": struct(
        spec = crate.spec(
            version = "0.20",
        ),
    ),
    "tree-sitter-cli": struct(
        spec = crate.spec(
            version = "0.20",
        ),
        annotations = [crate.annotation(
            gen_binaries = ["tree-sitter"],
        )],
    ),
    # Rust grammar for tree-sitter.
    "tree-sitter-rust": struct(
        spec = crate.spec(
            version = "0.20",
        ),
    ),

    # Arguments parsing.
    "clap": struct(
        spec = crate.spec(
            version = "4.3",
            features = ["derive"],
        ),
    ),

    # Includes formatters and parsers for std::time::SystemTime and std::time::Duration.
    "humantime": struct(
        spec = crate.spec(
            version = "2",
        ),
    ),

    # Result/Error helpers.
    "anyhow": struct(
        spec = crate.spec(
            version = "1",
        ),
    ),
    "thiserror": struct(
        spec = crate.spec(
            version = "1",
        ),
    ),

    # Provides a macro to generate structures which behave like a set of bitflags
    "bitflags": struct(
        spec = crate.spec(
            version = "2",
        ),
    ),

    # Temporary files and directories.
    "tempfile": struct(
        spec = crate.spec(
            version = "3",
        ),
    ),

    # Single assignment cells and lazy statics without macros.
    #
    # TODO: Use std::cell::OnceCell
    # Parts of once_cell API are included into std as of Rust 1.70.0.
    # https://doc.rust-lang.org/std/cell/struct.OnceCell.html
    "once_cell": struct(
        spec = crate.spec(
            version = "1.18",
        ),
    ),

    # For testing.
    "quickcheck": struct(
        spec = crate.spec(
            version = "1",
        ),
    ),
    "quickcheck_macros": struct(
        spec = crate.spec(
            version = "1",
        ),
    ),
    "arbitrary": struct(
        spec = crate.spec(
            version = "1",
            features = ["derive"],
        ),
    ),
    "libfuzzer-sys": struct(
        spec = crate.spec(
            version = "0.4",
        ),
    ),

    # For auditing Rust packages.
    "cargo-audit": struct(
        spec = crate.spec(
            version = "0.18.2",
        ),
        annotations = [crate.annotation(
            gen_binaries = ["cargo-audit"],
        )],
    ),
}

def _crates_repository(**kwargs):
    _crates_annotations = {
        name: c.annotations
        for (name, c) in _crates.items()
        if hasattr(c, "annotations")
    }

    _crates_packages = {
        name: c.spec
        for (name, c) in _crates.items()
        if hasattr(c, "spec")
    }

    crates_repository(
        name = "crates",
        annotations = _crates_annotations,
        cargo_lockfile = "//x/crates:Cargo.lock",
        lockfile = "//x/crates:Cargo.Bazel.lock",
        packages = _crates_packages,
        **kwargs
    )

crates = struct(
    repository = _crates_repository,
)

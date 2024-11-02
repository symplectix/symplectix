load("@//:toolchain.bzl", "RUST_VERSION")
load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository", "splicing_config")
load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

def crates_repositories():
    crate_universe_dependencies(
        rust_version = RUST_VERSION,
    )

    crates_repository(
        name = "crates",
        annotations = {
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
                    "@//thirdparty/crates:build_libgit2_sys",
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
        },
        cargo_lockfile = "//thirdparty/crates:cargo.lock",
        lockfile = "//thirdparty/crates:cargo-bazel-lock.json",
        packages = {
            "cargo-audit": crate.spec(
                version = "0.17",
                # gen_binaries = True,
            ),
            "openssl": crate.spec(
                version = "0.10",
            ),
            "openssl-sys": crate.spec(
                version = "0.9",
            ),
            "ssh2": crate.spec(
                version = "0.9",
            ),
            "libssh2-sys": crate.spec(
                version = "0.2",
            ),
            "git2": crate.spec(
                version = "0.15",
            ),
            "libgit2-sys": crate.spec(
                version = "0.14.0+1.5.0",
            ),

            # "arbitrary": crate.spec(
            #     version = "1",
            #     features = ["derive"],
            # },
            "quickcheck": crate.spec(
                version = "1",
            ),
            "quickcheck_macros": crate.spec(
                version = "1",
            ),

            # "rand": crate.spec(
            #     version = "0.8.5",
            # ),
        },
        rust_version = RUST_VERSION,
        splicing_config = splicing_config(
            resolver_version = "2",
        ),
    )

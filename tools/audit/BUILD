sh_binary(
    name = "audit",
    srcs = ["audit.sh"],
    args = [
        "$(rootpath //3rdparty/crates:Cargo.lock)",
        "$(rootpath //3rdparty/crates/bin_crates:Cargo.lock)",
    ],
    data = [
        "@//3rdparty/crates:Cargo.lock",
        "@//3rdparty/crates/bin_crates:Cargo.lock",
        "@crates//:cargo-audit__cargo-audit",
    ],
    env = {
        "CARGO_AUDIT_BIN": "$(rootpath @crates//:cargo-audit__cargo-audit)",
    },
)

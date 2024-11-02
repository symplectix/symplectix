sh_binary(
    name = "audit",
    srcs = ["audit.sh"],
    args = [
        "$(rootpath //x/crates:Cargo.lock)",
        "$(rootpath //x/bin_crates:Cargo.lock)",
    ],
    data = [
        "@//x/bin_crates:Cargo.lock",
        "@//x/crates:Cargo.lock",
        "@crates//:cargo-audit__cargo-audit",
    ],
    env = {
        "CARGO_AUDIT_BIN": "$(rootpath @crates//:cargo-audit__cargo-audit)",
    },
)

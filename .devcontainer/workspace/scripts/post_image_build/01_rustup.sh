#!/usr/bin/env bash
set -euo pipefail
shopt -s inherit_errexit

export RUSTUP_HOME="${1:-"/home/nonroot/.rustup"}"
export CARGO_HOME="${2:-"/home/nonroot/.cargo"}"

# Install rustup
if ! type rustup >/dev/null 2>&1; then
    mkdir -p $RUSTUP_HOME $CARGO_HOME
    curl --tlsv1.2 https://sh.rustup.rs -sSf | bash -s -- -y --no-modify-path --profile minimal 2>&1
fi

"$CARGO_HOME"/bin/rustup update 2>&1
"$CARGO_HOME"/bin/rustup component add rust-src rustfmt clippy rls rust-analysis 2>&1

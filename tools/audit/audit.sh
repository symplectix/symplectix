#!/usr/bin/env bash
set -euo pipefail -o posix

CARGO_AUDIT_BIN="${CARGO_AUDIT_BIN:-cargo-audit}"

LIB_CRATES_LOCK_FILE="${1:?Path to the crates lock file}"
BIN_CRATES_LOCK_FILE="${2:?Path to the bin_crates lock file}"
shift 2

"${CARGO_AUDIT_BIN}" audit --file "${LIB_CRATES_LOCK_FILE}" --db ./advisory-db "$@" && \
"${CARGO_AUDIT_BIN}" audit --file "${BIN_CRATES_LOCK_FILE}" --db ./advisory-db "$@" && \
echo OK

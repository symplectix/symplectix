#!/usr/bin/env bash
set -euo pipefail -o posix

BIN="$(pwd)/${1}"
shift

(cd "$BUILD_WORKING_DIRECTORY" && exec "$BIN" "$@")

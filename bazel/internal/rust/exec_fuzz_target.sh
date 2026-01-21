#!/usr/bin/env bash

set -euo pipefail

fuzz_target="$(readlink -f "${1:?missing fuzz_target}")"
shift 1

artifact_prefix="$(mktemp -d -t "fuzzing.XXXXXXXXXX")"

cd "$BUILD_WORKING_DIRECTORY"
exec "$fuzz_target" -artifact_prefix="${artifact_prefix}/" $@

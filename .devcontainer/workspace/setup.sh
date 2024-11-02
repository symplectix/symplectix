#!/usr/bin/env bash
set -eu -o pipefail -o posix
shopt -s inherit_errexit

scripts_dir="$1"

for f in "${scripts_dir}"/*.sh; do
  bash "$f"
done

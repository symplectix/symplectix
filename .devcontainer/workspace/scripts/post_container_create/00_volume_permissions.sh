#!/usr/bin/env bash
set -euo pipefail
shopt -s inherit_errexit

sudo chown nonroot:nonroot ~/.cache ~/.config ~/.local/share

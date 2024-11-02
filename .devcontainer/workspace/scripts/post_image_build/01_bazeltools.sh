#!/usr/bin/env bash
set -euo pipefail
shopt -s inherit_errexit

arch=$(uname -m | sed s/aarch64/arm64/ | sed s/x86_64/amd64/)

wget --no-hsts "https://github.com/bazelbuild/bazelisk/releases/download/v1.11.0/bazelisk-linux-${arch}"
chmod +x "bazelisk-linux-${arch}"
sudo mv "bazelisk-linux-${arch}" /usr/local/bin/bazel

wget --no-hsts "https://github.com/bazelbuild/buildtools/releases/download/5.1.0/buildifier-linux-${arch}"
chmod +x "buildifier-linux-${arch}"
sudo mv "buildifier-linux-${arch}" /usr/local/bin/buildifier

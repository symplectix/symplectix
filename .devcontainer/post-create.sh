#!/usr/bin/env bash
set -euo pipefail

main() {
    chown_volumes
    locale_gen
    install_bazel
}

chown_volumes() {
    sudo chown nonroot:nonroot ~/.cache ~/.config ~/.local
}

locale_gen() {
    sudo sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen
    sudo locale-gen
}

install_bazel() {
    local arch="$(uname -m | sed s/aarch64/arm64/ | sed s/x86_64/amd64/)"
    wget --no-hsts "https://github.com/bazelbuild/bazelisk/releases/download/v1.19.0/bazelisk-linux-${arch}"
    chmod +x "bazelisk-linux-${arch}"
    sudo mv "bazelisk-linux-${arch}" /usr/local/bin/bazel
}

main "$@"

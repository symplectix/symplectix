#!/usr/bin/env bash
set -euo pipefail
shopt -s inherit_errexit

sudo sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen
sudo locale-gen

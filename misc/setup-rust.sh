#!/usr/bin/env bash

# Installs the specific rust toolchain with the minimal profile
#
# Basic usage:
# ```bash
# $ bash setup-rust.sh
# ```

set -eux
set -o pipefail

if command -v rustup; then
    echo "rust already installed"
else
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.87.0 --profile minimal
    echo "Successfully installed rust"
fi

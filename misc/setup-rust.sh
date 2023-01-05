#!bash

# Script that install the specific rust toolchain
#
# It will install rust with the minimal profile
#
# Basic usage:
#
# ```bash
# $ bash setup-rust.sh # Will install rust-1.65 (The default)
# $ bash setup-rust.sh 1.65.0 # Will install rust-1.65.0
# ```

set -eux
set -o pipefail

RUST_TOOLCHAIN=${1:-1.65}

if command -v rustc && [[ "$(rustc --version)" == *"${RUST_TOOLCHAIN}"* ]]; then
    echo "rust-${RUST_TOOLCHAIN} already installed"
else
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --default-toolchain 1.65 --profile minimal ${@:2}
    echo "Successfully installed rust-${RUST_TOOLCHAIN}"
fi

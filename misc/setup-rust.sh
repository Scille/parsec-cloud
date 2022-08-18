#!bash

# Script that install the specific rust toolchain
#
# It will install rust with the minimal profile
#
# Basic usage:
#
# ```bash
# $ bash setup-rust.sh # Will install rust-1.63 (The default)
# $ bash setup-rust.sh 1.63.1 # Will install rust-1.63.1
# ```

set -eux
set -o pipefail

RUST_TOOLCHAIN=${1:-1.63}

if command -v rustc && [[ "$(rustc --version)" == *"${RUST_TOOLCHAIN}"* ]]; then
    echo "rust-${RUST_TOOLCHAIN} already installed"
else
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --default-toolchain 1.63 --profile minimal ${@:2}
    echo "Successfully installed rust-${RUST_TOOLCHAIN}"
fi

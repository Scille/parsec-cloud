#!bash

# Script that install the specific rust toolchain
#
# It will install rust with the minimal profile
#
# Basic usage:
#
# ```bash
# $ bash setup-rust.sh # Will install rust-1.62 (The default)
# $ bash setup-rust.sh 1.62.1 # Will install rust-1.62.1
# ```

set -eux
set -o pipefail

RUST_TOOLCHAIN=${1:-1.62}

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --default-toolchain 1.62 --profile minimal ${@:2}

echo "Successfully installed rust-${RUST_TOOLCHAIN}"

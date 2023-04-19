#!bash

# Script that install the specific rust toolchain
#
# It will install rust with the minimal profile
#
# Basic usage:
#
# ```bash
# $ bash setup-rust.sh # Will install rust-1.68.0 (The default)
# $ bash setup-rust.sh 1.68.0 # Will install rust-1.68.0
# ```

set -eux
set -o pipefail

if command -v rustc; then
    echo "rust already installed"
else
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --default-toolchain none --profile minimal
    echo "Successfully installed rust"
fi

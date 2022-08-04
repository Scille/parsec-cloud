#!bash
set -eux

RUST_TOOLCHAIN=${1:-1.62}

TMP_DIR=$(mktemp -d -t rust-install.XXXXXXXXXX)

trap "rm -vrf $TMP_DIR" EXIT INT TERM

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > $TMP_DIR/rust-install.sh

bash $TMP_DIR/rust-install.sh -y --default-toolchain 1.62 --profile minimal ${@:2}

echo "export PATH=$PATH:$HOME/.cargo/bin"

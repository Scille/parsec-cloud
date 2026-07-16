#!/bin/bash

set -xe

# Install Rust (actual toolchain is going to be installed by maturin according to `rust-toolchain.toml`)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.96.0
export PATH="/root/.cargo/bin:$PATH"
cargo --version

# Install uv
curl -LsSf https://astral.sh/uv/0.11.29/install.sh | sh
export PATH="/root/.local/bin:$PATH"
uv --version

# Install parsec in virtual env
uv venv --project=server venv

VIRTUAL_ENV=$PWD/venv MATURIN_PEP517_ARGS="$(python make.py --quiet python-release-libparsec-cargo-flags)" \
    uv pip install --directory=server . --verbose

# Basic check to see if the wheel looks like it's well built.
./venv/bin/parsec --version

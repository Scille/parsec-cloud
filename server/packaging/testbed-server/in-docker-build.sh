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

uv venv --project=server venv

VIRTUAL_ENV=$PWD/venv MATURIN_PEP517_ARGS="$(python make.py --quiet python-ci-libparsec-cargo-flags)" \
    uv pip install --directory=server --group testbed-server . --verbose

# Boto3/Botocore are pretty big dependencies and won't be used (given the testbed
# server only uses the memory storage)
rm -rf ./venv/lib/python3.12/site-packages/{boto3,botocore,pip,setuptools}

# Basic check to see if the wheel looks like it's well built.
./venv/bin/parsec --version

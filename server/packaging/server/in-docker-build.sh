#!/bin/bash

set -xe

# Install Rust (actual toolchain is going to be installed by maturin according to `rust-toolchain.toml`)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.85.0
export PATH="/root/.cargo/bin:$PATH"
cargo --version

# Install UV
curl -LsSL https://astral.sh/uv/0.8.4/install.sh | sh
export PATH="/root/.local/bin:$PATH"
uv --version

# Install parsec in virtual env
python -m venv venv
. ./venv/bin/activate

# Compile in release mode
# Also don't bundle OpenSSL shared library (it is already in the Docker image !)
UV_LIBPARSEC_BUILD_PROFILE=release \
UV_LIBPARSEC_BUNDLE_EXTRA_SHARED_LIBRARIES=false \
pip install ./server

# Basic check to see if the wheel looks like it's well built.
./venv/bin/parsec --version

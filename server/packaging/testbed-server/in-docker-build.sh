#!/bin/bash

set -xe

# Install Rust (actual toolchain is going to be installed by maturin according to `rust-toolchain.toml`)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.85.0
export PATH="/root/.cargo/bin:$PATH"
cargo --version

# Install Poetry
curl -LsSL https://astral.sh/uv/0.8.4/install.sh | sh
export PATH="/root/.local/bin:$PATH"
uv --version

# Install parsec in virtual env
python -m venv venv
. ./venv/bin/activate

# Compile in CI mode to reduce size while still retain `test-utils` feature
# Also don't bundle OpenSSL shared library (it is already in the Docker image !)
UV_LIBPARSEC_BUILD_PROFILE=ci \
UV_LIBPARSEC_BUNDLE_EXTRA_SHARED_LIBRARIES=false \
pip install ./server

# Boto3/Botocore are pretty big dependencies and won't be used (given the testbed
# server only uses the memory storage)
rm -rf ./venv/lib/python3.12/site-packages/{boto3,botocore,pip,setuptools}

# Basic check to see if the wheel looks like it's well built.
./venv/bin/parsec --version

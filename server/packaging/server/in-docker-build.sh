#!/bin/bash

set -xe

# Remove bindings crates that aren't needed for backend-server
sed -i -e '\;bindings;d' Cargo.toml

# Install Rust (actual toolchain is going to be installed by maturin according to `rust-toolchain.toml`)
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain none

export PATH="/root/.cargo/bin:$PATH"

# Install parsec in virtual env
python -m venv venv

# Compile in release mode
POETRY_LIBPARSEC_BUILD_PROFILE=release ./venv/bin/python -m pip install ./server

# Boto3/Botocore are pretty big dependencies and won't be used (given the testbed
# server only uses the memory storage)
rm -rf ./venv/lib/python3.9/site-packages/{boto3,botocore,pip,setuptools}

# Basic to see if the wheel look like it's well built.
(cd / && /work/venv/bin/python -m parsec.cli --version)

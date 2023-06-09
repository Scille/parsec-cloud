#!/bin/bash

set -xe

# Remove bindings crates that aren't needed for testbed-server
sed -i -e '\;bindings;d' Cargo.toml

# Install Rust (actual toolchain is going to be installed by maturin according to `rust-toolchain.toml`)
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain none

export PATH="/root/.cargo/bin:$PATH"

# Install parsec in virtual env
python -m venv venv

# Compile in CI mode to reduce size while still retain `test-utils` feature
POETRY_LIBPARSEC_BUILD_PROFILE=ci ./venv/bin/python -m pip install ./server

# PSutil is among the dev requirements, retrieve it version and install it manually
VERSION=$(grep "psutil-" ./server/poetry.lock | head -n 1 | sed -E 's/.*psutil-([0-9.]+).*/\1/') \
    && ./venv/bin/python -m pip install psutil=="$VERSION"

# Boto3/Botocore are pretty big dependencies and won't be used (given the testbed
# server only uses the memory storage)
rm -rf ./venv/lib/python3.9/site-packages/{boto3,botocore,pip,setuptools}

(cd / && /work/venv/bin/python -m parsec.cli --version)

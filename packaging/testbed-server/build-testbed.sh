#!/bin/bash

set -xe

# Remove bindings crates that aren't needed for testbed-server
sed -i -e '\;oxidation/bindings;d' Cargo.toml

# Remove the line that generate pyqt code in the build script since the backend don't need it.
sed -i -e '\;misc/generate_pyqt.py;d' build.py

# Install Rust (actual toolchain is going to be installed by maturin according to `rust-toolchain.toml`)
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain none

export PATH="/root/.cargo/bin:$PATH"

# Install parsec in virtual env
python -m venv venv

# Compile in CI mode to reduce size while still retain `test-utils` feature
POETRY_LIBPARSEC_BUILD_PROFILE=ci ./venv/bin/python -m pip install .[backend]

# PSutil is among the dev requirements, retrieve it version and install it manually
VERSION=$(grep "psutil-" ./poetry.lock | head -n 1 | sed -E 's/.*psutil-([0-9.]+).*/\1/') \
    && ./venv/bin/python -m pip install psutil=="$VERSION"

rm -rf ./venv/lib/python3.9/site-packages/{boto3,botocore,pip,setuptools}

(cd / && /work/venv/bin/python -m parsec.cli --version)

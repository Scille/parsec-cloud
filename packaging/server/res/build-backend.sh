#!/bin/bash

set -xe

# Remove bindings crates that aren't needed for backend-server
sed -i -e '\;oxidation/bindings;d' Cargo.toml

# Remove the line that generate pyqt code in the build script since the backend don't need it.
sed -i -e '\;misc/generate_pyqt.py;d' build.py

# Install Rust (actual toolchain is going to be installed by maturin according to `rust-toolchain.toml`)
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain none

export PATH="/root/.cargo/bin:$PATH"

# Install parsec in virtual env
python -m venv venv

# Compile in release mode
POETRY_LIBPARSEC_BUILD_PROFILE=release ./venv/bin/python -m pip install .[backend]

# Remove some python package that aren't needed
rm -rf ./venv/lib/python3.9/site-packages/{boto3,botocore,pip,setuptools}

# Basic to see if the wheel look like it's well built.
(cd / && /work/venv/bin/python -m parsec.cli --version)

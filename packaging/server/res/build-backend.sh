#!/bin/bash

set -xe

# Remove bindings crates that aren't needed for backend-server
sed -i -e '\;oxidation/bindings;d' Cargo.toml

# Install Rust (actual toolchain is going to be installed by maturin according to `rust-toolchain.toml`)
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain none
export PATH="/root/.cargo/bin:$PATH"
cargo --version

# Install Poetry
curl -sSL https://install.python-poetry.org | python - --version=1.5.1
export PATH="/root/.local/bin:$PATH"
poetry --version

# Install parsec in virtual env
python -m venv venv
. ./venv/bin/activate

# Installing the Python project is a bit tricky:
# - `pip install .` ignores dependency pinning :(
# - `poetry install` install the main project as a symlink in the virtualenv :(
# So instead we ask poetry to generate a `requirements.txt` that contains the
# pinned dependencies, then pip install this, and finally do a regular
# `pip install .` that will only install our project given all dependencies are
# already installed.

# Only install backend extras, core and dev are not necessary
poetry --directory ./server export --output requirements.txt --extras backend
# Install the dependencies...
pip install -r ./requirements.txt
# ...and our project
# Compile in CI mode to reduce size while still retain `test-utils` feature
# Also don't bundle OpenSSL shared library (it is already in the Docker image !)
POETRY_LIBPARSEC_BUILD_PROFILE=release \
POETRY_PYQT_BUILD_STRATEGY=no_build \
pip install .

# Boto3/Botocore are pretty big dependencies and won't be used (given the testbed
# server only uses the memory storage)
rm -rf ./venv/lib/python3.9/site-packages/{boto3,botocore,pip,setuptools}

(cd / && /server/venv/bin/parsec --version)

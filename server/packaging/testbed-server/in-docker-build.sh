#!/bin/bash

set -xe

# Install Rust (actual toolchain is going to be installed by maturin according to `rust-toolchain.toml`)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.85.0
export PATH="/root/.cargo/bin:$PATH"
cargo --version

# Install Poetry
curl -sSL https://install.python-poetry.org | python - --version=2.1.1
export PATH="/root/.local/bin:$PATH"
poetry --version
# Install plugin for poetry export
poetry self add poetry-plugin-export

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

# Only keep dependencies from `main` group (i.e. the base dependencies) and
# testbed-server, as other groups are for dev
poetry --project ./server export --output requirements.txt --with=main,testbed-server
# Install the dependencies...
pip install -r ./requirements.txt
# ...and our project
# Compile in CI mode to reduce size while still retain `test-utils` feature
# Also don't bundle OpenSSL shared library (it is already in the Docker image !)
POETRY_LIBPARSEC_BUILD_PROFILE=ci \
POETRY_LIBPARSEC_BUNDLE_EXTRA_SHARED_LIBRARIES=false \
pip install ./server

# Boto3/Botocore are pretty big dependencies and won't be used (given the testbed
# server only uses the memory storage)
rm -rf ./venv/lib/python3.12/site-packages/{boto3,botocore,pip,setuptools}

# Basic check to see if the wheel looks like it's well built.
./venv/bin/parsec --version

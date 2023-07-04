#!/bin/bash

set -xe

# Remove bindings crates that aren't needed for testbed-server
sed -i -e '\;bindings;d' Cargo.toml

# Install Rust (actual toolchain is going to be installed by maturin according to `rust-toolchain.toml`)
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain none
export PATH="/root/.cargo/bin:$PATH"
cargo --version

# Install Poetry
curl -sSL https://install.python-poetry.org | python - --version=1.3.2
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

# Only keep dependencies from `main` group (i.e. the base dependencies) and
# testbed-server, as other groups are for dev
poetry --directory ./server export --output requirements.txt --with=main,testbed-server
# Install the dependencies...
pip install -r ./requirements.txt
# ...and our project
# Compile in CI mode to reduce size while still retain `test-utils` feature
POETRY_LIBPARSEC_BUILD_PROFILE=ci pip install ./server

# Boto3/Botocore are pretty big dependencies and won't be used (given the testbed
# server only uses the memory storage)
rm -rf ./venv/lib/python3.9/site-packages/{boto3,botocore,pip,setuptools}

(cd / && /work/venv/bin/python -m parsec.cli --version)

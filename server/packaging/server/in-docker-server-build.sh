#!/bin/bash

set -xe

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

# Only keep dependencies from `main` group (i.e. the base dependencies), as other groups are for dev
poetry --directory ./server export --output requirements.txt --with=main
# Install the dependencies...
pip install -r ./requirements.txt
# ...and our project
# Compile in release mode
# Also don't bundle OpenSSL shared library (it is already in the Docker image !)
POETRY_LIBPARSEC_BUILD_PROFILE=release \
POETRY_LIBPARSEC_BUNDLE_EXTRA_SHARED_LIBRARIES=false \
pip install ./server

# Basic check to see if the wheel looks like it's well built.
./venv/bin/parsec --version

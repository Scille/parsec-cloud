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
python3.9 -m venv venv
. ./venv/bin/activate

# Installing the Python project is a bit tricky:
# - `pip install .` ignores dependency pinning :(
# - `poetry install` install the main project as a symlink in the virtualenv :(
# So instead we ask poetry to generate a `requirements.txt` that contains the
# pinned dependencies, then pip install this, and finally do a regular
# `pip install .` that will only install our project given all dependencies are
# already installed.

# Only install backend extras, core and dev are not necessary
poetry export --output requirements.txt --extras backend

# Install the dependencies
pip install -r requirements.txt

# And install our project (release mode, no need for PyQt)
POETRY_LIBPARSEC_BUILD_PROFILE=release \
POETRY_PYQT_BUILD_STRATEGY=no_build \
pip install .

(cd / && /server/venv/bin/parsec --version)

# syntax=docker/dockerfile:1.4

#
# 1) Build stage
#

FROM python:3.9 AS builder
WORKDIR /work

# Map source, cannot just do `ADD --link . .` otherwise modifying the current
# file will invalidate the cache.
# Dockerfile must be move in the root directory prior to being run
ADD --link build.py .
ADD --link Cargo.lock .
ADD --link Cargo.toml .
ADD --link HISTORY.rst .
ADD --link make.py .
ADD --link misc ./misc/
ADD --link mypy.ini .
ADD --link oxidation/ ./oxidation/
ADD --link parsec ./parsec/
ADD --link poetry.lock .
ADD --link pyproject.toml .
ADD --link README.rst .
ADD --link rust-toolchain.toml .
ADD --link setup.cfg .
ADD --link src ./src/
ADD --link tests ./tests/

# Install Rust (actual toolchain is going to be installed by maturin according to `rust-toolchain.toml`)
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain none
# RUN RUST_CHANNEL=$(grep channel ./rust-toolchain.toml | sed 's/channel\s=\s"\(.*\)"/\1/') \
#     curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain $RUST_CHANNEL
# RUN rm ./rust-toolchain.toml
ENV PATH="/root/.cargo/bin:${PATH}"

# Install parsec in virtual env
RUN python -m venv venv
# Compile in CI mode to reduce size while still retain `test-utils` feature
RUN POETRY_LIBPARSEC_BUILD_PROFILE=ci ./venv/bin/python -m pip install .[backend]
# PSutil is among the dev requirements, retrieve it version and install it manually
RUN VERSION=$(grep "psutil-" ./poetry.lock | head -n 1 | sed -E 's/.*psutil-([0-9.]+).*/\1/') && ./venv/bin/python -m pip install psutil=="$VERSION"
# Remove unneeded and beefy stuff: boto3&botocore take ~80mo, pip ~15mo, setuptools ~4mo
RUN cd ./venv/lib/python3.*/site-packages && rm -rf boto3 botocore pip setuptools
# Make sure installation in venv went smoothly
RUN cd / && /work/venv/bin/python -m parsec.cli --version

#
# 2) Bundle stage
#

FROM python:3.9-slim
WORKDIR /

COPY --from=builder /work/tests/scripts/run_testbed_server.py .
COPY --from=builder /work/parsec/ ./parsec/
COPY --from=builder /work/venv ./venv
# Make sure `run_testbed_server.py` is correctly available
RUN /venv/bin/python run_testbed_server.py --help

EXPOSE 6777
ENTRYPOINT ["/venv/bin/python", "/run_testbed_server.py", "--port", "6777"]
CMD []

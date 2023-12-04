# syntax=docker/dockerfile:1.5

#
# 1) Build stage
#

FROM python:3.9 AS builder

WORKDIR /server

# Map source, cannot just do `ADD --link . .` otherwise modifying the current
# file will invalidate the cache.
# Dockerfile must be move in the root directory prior to being run
ADD --link \
    README.rst \
    rust-toolchain.toml \
    Cargo.lock \
    Cargo.toml \
    make.py \
    server/packaging/server/in-docker-build.sh \
    .
ADD --link libparsec/ libparsec/
ADD --link server/ server/
ADD --link bindings/ bindings/
ADD --link cli/ cli/

RUN bash in-docker-build.sh

#
# 2) Bundle stage
#

FROM python:3.9-slim

LABEL org.opencontainers.image.source=https://github.com/Scille/parsec-cloud
LABEL org.opencontainers.image.description="Run the Parsec backend server."

RUN groupadd --gid=1234 parsec && useradd --home-dir=/home/parsec --create-home --uid=1234 --gid=1234 parsec

# Create parsec user and group
USER parsec:parsec

# Copy the venv from the builder
# Important: Use the same path as the builder so the venv scripts can run
WORKDIR /server
COPY --chown=1234:1234 --from=builder /server/venv /server/venv

# Add venv/bin to PATH to make `parsec` available
ENV PATH "/server/venv/bin:$PATH"

# Suppress those annoying TrioDeprecationWarnings
ENV PYTHONWARNINGS "ignore:::quart_trio.app"

# Define entry point
EXPOSE 6777
ENTRYPOINT ["parsec", "backend"]
CMD ["run", "--port=6777"]

# syntax=docker/dockerfile:1.5

#
# 1) Build stage
#

FROM python:3.12 AS builder

WORKDIR /work

# Map source, cannot just do `ADD --link . .` otherwise modifying the current
# file will invalidate the cache.
# Dockerfile must be move in the root directory prior to being run
ADD --link \
    README.rst \
    rust-toolchain.toml \
    Cargo.lock \
    Cargo.toml \
    make.py \
    server/packaging/testbed-server/in-docker-build.sh \
    ./
ADD --link libparsec/ libparsec/
ADD --link server/ server/
ADD --link bindings/ bindings/
ADD --link cli/ cli/

RUN bash in-docker-build.sh

#
# 2) Bundle stage
#

FROM python:3.12-slim

LABEL org.opencontainers.image.source=https://github.com/Scille/parsec-cloud
LABEL org.opencontainers.image.description="Run a testbed parsec server to simplify mockup of an existing organization."

USER 1234:1234
WORKDIR /testbed

COPY --chown=1234:1234 --from=builder /work/venv /testbed/venv

EXPOSE 6777
ENTRYPOINT ["parsec", "testbed", "--host", "0.0.0.0", "--port", "6777"]
CMD []

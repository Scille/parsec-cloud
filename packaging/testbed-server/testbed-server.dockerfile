# syntax=docker/dockerfile:1.5

#
# 1) Build stage
#

FROM python:3.9 AS builder

WORKDIR /work

# Map source, cannot just do `ADD --link . .` otherwise modifying the current
# file will invalidate the cache.
# Dockerfile must be move in the root directory prior to being run
ADD --link \
    build.py \
    Cargo.lock \
    Cargo.toml \
    make.py \
    poetry.lock \
    pyproject.toml \
    rust-toolchain.toml \
    setup.cfg \
    README.rst \
    packaging/testbed-server/build-testbed.sh \
    .
ADD --link oxidation/ oxidation/
ADD --link parsec/ parsec/
ADD --link src/ src/

RUN bash build-testbed.sh

# #
# # 2) Bundle stage
# #

FROM python:3.9-slim

USER 1234:1234
WORKDIR /testbed

COPY --chown=1234:1234 tests/scripts/run_testbed_server.py /testbed/.
COPY --chown=1234:1234 --from=builder /work/venv /testbed/venv

EXPOSE 6777
ENTRYPOINT ["/testbed/venv/bin/python", "/testbed/run_testbed_server.py", "--host", "0.0.0.0", "--port", "6777"]
CMD []

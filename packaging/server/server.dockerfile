FROM python:3.9 as builder

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
    packaging/server/res/build-backend.sh \
    .

ADD --link oxidation/libparsec/ oxidation/libparsec/
ADD --link parsec/ parsec/
ADD --link src/ src/

RUN bash build-backend.sh

#
# 2) Bundle stage
#

FROM python:3.9-slim

LABEL org.opencontainers.image.source=https://github.com/Scille/parsec-cloud
LABEL org.opencontainers.image.description="Run the Parsec backend server."

USER 1234:1234
WORKDIR /backend

COPY --chown=1234:1234 --from=builder /work/venv /backend/venv

EXPOSE 6777

ENTRYPOINT ["/backend/venv/bin/python", "parsec", "backend"]
CMD ["run", "--port=6777"]

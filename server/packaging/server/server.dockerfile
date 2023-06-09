FROM python:3.9 as builder

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
    server/packaging/server/in-docker-build.sh \
    .

ADD --link libparsec/ libparsec/
ADD --link server/ server/

RUN bash in-docker-build.sh

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

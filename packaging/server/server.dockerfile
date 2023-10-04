FROM python:3.9 as builder

WORKDIR /server

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

RUN groupadd --gid=1234 parsec && useradd --home-dir=/home/parsec --create-home --uid=1234 --gid=1234 parsec

USER parsec:parsec

# Important: Use the same name as the builder so the venv scripts can run
WORKDIR /server

COPY --chown=1234:1234 --from=builder /server/venv /server/venv

EXPOSE 6777

ENV PATH "/server/venv/bin:$PATH"
ENV PYTHONWARNINGS "ignore:::quart_trio.app"
ENTRYPOINT ["parsec", "backend"]
CMD ["run", "--port=6777"]

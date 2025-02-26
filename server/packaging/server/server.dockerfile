# syntax=docker/dockerfile:1.5

# #
# # 1) Build stage 1: build the client app web
# #

# FROM node:18.12.2 AS client-web-app-builder

# WORKDIR /client

# # Map source, cannot just do `ADD --link . .` otherwise modifying the current
# # file will invalidate the cache.
# # Dockerfile must be moved in the root directory prior to being run
# ADD --link \
#     README.rst \
#     rust-toolchain.toml \
#     Cargo.lock \
#     Cargo.toml \
#     make.py \
#     server/packaging/server/in-docker-client-web-app-build.sh \
#     .
# ADD --link libparsec/ libparsec/
# ADD --link client/ client/
# ADD --link bindings/ bindings/

# RUN bash in-docker-client-web-app-build.sh

#
# 2) Build stage 2: build the server wheel 
#

FROM python:3.12 AS server-builder

WORKDIR /server

# Install node (to build the client web app)
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash
RUN . "$HOME/.nvm/nvm.sh" && nvm install 18

# Map source, cannot just do `ADD --link . .` otherwise modifying the current
# file will invalidate the cache.
# Dockerfile must be moved in the root directory prior to being run
ADD --link \
    README.rst \
    rust-toolchain.toml \
    Cargo.lock \
    Cargo.toml \
    make.py \
    server/packaging/testbed-server/in-docker-server-build.sh \
    ./
ADD --link libparsec/ libparsec/
ADD --link server/ server/
ADD --link client/ client/
ADD --link bindings/ bindings/
ADD --link cli/ cli/

RUN bash in-docker-server-build.sh

#
# 3) Bundle stage
#

FROM python:3.12-slim

LABEL org.opencontainers.image.source=https://github.com/Scille/parsec-cloud
LABEL org.opencontainers.image.description="Run the Parsec server."

RUN groupadd --gid=1234 parsec && useradd --home-dir=/home/parsec --create-home --uid=1234 --gid=1234 parsec

# Create parsec user and group
USER parsec:parsec

# Copy the venv from the builder
# Important: Use the same path as the builder so the venv scripts can run
WORKDIR /server
COPY --chown=1234:1234 --from=server-builder /server/venv /server/venv

# Add venv/bin to PATH to make `parsec` available
ENV PATH "/server/venv/bin:$PATH"

# Define entry point
EXPOSE 6777
ENTRYPOINT ["parsec"]
CMD ["run", "--port=6777"]

# cspell:words inotify

FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive
SHELL [ "/bin/bash", "-c" ]

RUN useradd --uid=1000 --user-group --home-dir=/home/parsec --create-home parsec

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    make \
    python3 \
    python3-venv \
    python3-pip \
    # Install additional fonts for latex
    fonts-roboto \
    fonts-firacode \
    # Required by `sphinx.ext.imgconverter`.
    imagemagick \
    librsvg2-bin \
    # We use lualatex as latex engine.
    texlive-luatex \
    # Install requirement for latex.
    texlive-latex-base \
    texlive-latex-recommended \
    texlive-latex-extra \
    # The pdf is build using latexmk
    latexmk \
    ca-certificates \
    curl \
    git \
    inotify-tools

RUN mkdir /data && chown parsec:parsec /data

USER parsec:parsec

COPY poetry.lock pyproject.toml /data/

WORKDIR /data

RUN python3 -m venv /home/parsec/poetry-venv && \
    source /home/parsec/poetry-venv/bin/activate && \
    pip install poetry && \
    poetry install && \
    echo 'source /home/parsec/poetry-venv/bin/activate' >> /home/parsec/.bashrc

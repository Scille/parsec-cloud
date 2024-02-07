# cspell:words inotify

FROM python:3.12-slim

SHELL [ "/bin/bash", "-c" ]

RUN useradd --uid=1000 --user-group --home-dir=/home/parsec --create-home parsec

RUN apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    make \
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
    # The PDF is built using latexmk
    latexmk \
    ca-certificates \
    curl \
    git \
    gettext \
    inotify-tools \
    # Remove unnecessary apt cache
    && rm -rf /var/lib/apt/lists/*

USER parsec:parsec

WORKDIR /data/docs

COPY poetry.lock pyproject.toml conf.py /data/docs/

RUN python -m venv /home/parsec/poetry-venv && \
    source /home/parsec/poetry-venv/bin/activate && \
    pip install poetry && \
    poetry install && \
    echo 'source /home/parsec/poetry-venv/bin/activate' >> /home/parsec/.bashrc

CMD [ "/bin/bash" ]

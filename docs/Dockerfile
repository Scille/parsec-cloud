# cspell:words inotify

# Using bookworm because it provides the same version of gettext (v0.21) used in the CI.
FROM python:3.12-slim-bookworm

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

COPY uv.lock pyproject.toml conf.py /data/docs/

ENV VIRTUAL_ENV="/home/parsec/venv"
RUN python -m venv $VIRTUAL_ENV && \
    source $VIRTUAL_ENV/bin/activate && \
    pip install uv && \
    uv sync --active && \
    echo "source ${VIRTUAL_ENV}/bin/activate" >> /home/parsec/.bashrc

ENV PATH="${VIRTUAL_ENV}/bin:${PATH}"

CMD [ "/bin/bash" ]

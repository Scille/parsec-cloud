FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    make \
    python3 \
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
    latexmk

# Generate with `poetry export --format=requirements.txt --only docs --output=docs/requirements.txt`
# Then you need to remove the `python_version` requirement in the
COPY requirements.txt /requirements.txt

RUN python3 -m pip install -r requirements.txt

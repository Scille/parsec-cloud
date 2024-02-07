<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Parsec documentation

This directory contains the sources for Parsec docs.

## Introduction

Parsec docs are written in [reStructuredText (.rst)](https://www.sphinx-doc.org/en/master/usage/restructuredtext/basics.html) format.

[Sphinx](https://www.sphinx-doc.org/en/master/index.html) is used to build the docs in multiple output formats (such as HTML and PDF).

## Requirements

There are two ways to install dependencies:

- Using [Poetry](https://python-poetry.org/docs/#installation)

  ```bash
  poetry install
  ```

> **Note**
> You'll need extra dependencies to build PDF docs (via `latexmk`), see Dockerfile.

- Using [Docker](https://docs.docker.com/engine/install/)

    1. Build Docker image:

        ```bash
        docker build -t parsec-docs .
        ```

    2. Run a container:

        ```bash
        docker run --rm -it -v $(pwd):/data parsec-docs
        ```

        > **Note**
        > The current working directory must be the local path to parsec-cloud repo.
        > It is mounted in `/data` in the container. This means that builds started from
        > the container will be available in your local repo under `_build` directory.

## Build docs

Build HTML docs with:

```bash
make html
```

To see the list of available output formats:

```bash
make
```

## Translations

Translations are based on [Sphinx Internationalization](http://www.sphinx-doc.org/en/master/usage/advanced/intl.html).

1. Extract text from docs

    ```bash
    make gettext
    ```

2. Create/update `.po` files for translation

    ```bash
    sphinx-intl update -p _build/locale -l fr
    ```

3. Translate `.po` files (manually or with a tool like [Poedit](https://poedit.net/))

4. Build docs `.po -> .mo`

    ```bash
    make -e SPHINXOPTS="-D language='fr'" html
    ```

# How to generate the documentation website

## Requirements

Install dependencies with `poetry install --with docs`

## Update translation

see <http://www.sphinx-doc.org/en/master/usage/advanced/intl.html>

1. Extract text and create/update `.po` files

    ```shell
    make gettext
    sphinx-intl update -p _build/locale -l fr
    ```

2. Translate `.po` files

3. Build `.po -> .mo`

    ```shell
    make -e SPHINXOPTS="-D language='fr'" html
    ```

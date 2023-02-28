# Oxidation in prod - Poetry/Maturin based pyproject.toml for main parsec Python project

From [ISSUE-2489](https://github.com/Scille/parsec-cloud/issues/2489)

depends of [PR-2371](https://github.com/Scille/parsec-cloud/pull/2371)

## Overview

Once Rust extension are mandatory to run Parsec there would be no need to separate the python bindings from the regular parsec codebase.
In fact this separation makes things more complex for packaging given parsec requires libparsec (i.e. the rust extension) but must specify the relation with a relative path, which is not allowed when building a wheel (On top of that dependency means dealing version number which we also would like to avoid).

So the solution is simply to merge together parsec and libparsec into a single hybrid Python package.

## Architecture

The switch to Poetry is under way (see [PR-2371](https://github.com/Scille/parsec-cloud/pull/2371)), once it is done we should be able to add maturin to it by [taking example of this project](https://github.com/ArniDagur/python-adblock/blob/master/pyproject.toml)

## Command workflow

First use:

```shell
poetry install  # Create a new virtual env and install all dependencies within it
poetry shell  # Jump into the virtual env
maturin develop  # Build the Rust extensions and install them within the parsec source code so that they can be imported
```

First use wth a custom virtual env (letting poetry handle the venv should be the preferred way, but this might come handy):

```shell
python -m venv my_venv
. ./my_venv/bin/activate
poetry install  # Poetry detects it is already in a venv and reuse it
maturin develop
```

After that, switching into another branch (considering the venv is already activated):

```shell
poetry install && maturin develop
```

## Rust Release vs Debug build ?

In theory we want to have a debug build when doing maturin develop, but switch to release when installing the project with `pip install .` (or when building the wheel, which is equivalent since `pip install` build a wheel behind the scene).
So we should make sure this is the case !

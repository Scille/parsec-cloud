# Hacking Quickstart

- [Hacking Quickstart](#hacking-quickstart)
  - [Get the sources](#get-the-sources)
  - [Required environment configurations](#required-environment-configurations)
  - [Run the tests](#run-the-tests)
  - [Run Parsec in local](#run-parsec-in-local)

## Get the sources

Source code is available on [github](https://github.com/Scille/parsec-cloud).

## Required environment configurations

1. Your text editor need to be configured with [`editorconfig`](https://editorconfig.org/)
2. You need to have the following tools available:
    - [`git`](https://git-scm.com/)
    - [`python v3.9`](https://www.python.org/)
    - [`poetry >=1.2.0`](https://python-poetry.org/docs/#installation)
    - [`Rust v1.68.0`](https://www.rust-lang.org/fr/learn/get-started)

Consider using [`pyenv`](https://github.com/pyenv/pyenv#installation) to install a specific version of `Python`.

**TL,DR:**

1. Install the required tools:

    ```shell
    # Install Poetry (Does not require a specific version of Python)
    curl --proto '=https' --tlsv1.2 -sSL https://install.python-poetry.org/ | python - --version=1.2.0
    # Install Rust
    curl --proto '=https' --tlsv1.2 -sSL https://sh.rustup.rs | sh -s -- --default-toolchain 1.68.0
    # Install Pyenv
    curl --proto '=https' --tlsv1.2 -sSL https://pyenv.run | bash
    # Compile Python
    pyenv install 3.9.10
    # Create the project virtual with the correct version of Python
    poetry env use $(pyenv prefix 3.9.10)/bin/python
    ```

2. You must first clone it with ``git`` and move to the project dir

    ```shell
    git clone git@github.com:Scille/parsec-cloud.git
    cd parsec-cloud
    ```

3. Initialize a poetry env

    ```shell
    poetry install -E core -E backend
    ```

4. Start a shell with the initialized virtual env

    ```shell
    poetry shell
    ```

5. To run parsec do

    ```shell
    poetry run parsec
    ```

## Run the tests

Run the tests with `pytest`:

```shell
poetry run py.test tests
```

On top of that, multiple options are available:

| Option              | Description                                               |
| ------------------- | --------------------------------------------------------- |
| ``--runmountpoint`` | Include mountpoint tests                                  |
| ``--rungui``        | Include GUI tests                                         |
| ``--runslow``       | Include slow tests                                        |
| ``--postgresql``    | Use PostgreSQL in the backend instead of a mock in memory |
| ``-n 4``            | Run tests in parallel (here `4` jobs)                     |

Note you can mix&match the flags, e.g.

```shell
py.test tests --runmountpoint --postgresql --runslow -n auto
```

If you want to run GUI test, it is a good idea to install ``pytest-xvfb`` in order to
hide the Qt windows when running the GUI tests

```shell
apt install xvfb
pip install pytest-xvfb
```

## Run Parsec in local

You can use the ``run_testenv`` scripts to easily create a development environment:

On linux:

```shell
. ./tests/scripts/run_testenv.sh
```

On Windows:

```cmd
.\tests\scripts\run_testenv.bat
```

This script will:

- Start a development backend server with in-memory storage
- Configure environment variables to isolate the development environment from
  your global Parsec configuration
- Create a default organization
- Create multiple users and devices for this organization

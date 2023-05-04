# Hacking Quickstart

- [Hacking Quickstart](#hacking-quickstart)
  - [Get the sources](#get-the-sources)
  - [Shared requirements](#shared-requirements)
  - [Hacking the Python code (front + back)](#hacking-the-python-code-front--back)
    - [Run the Python tests](#run-the-python-tests)
    - [Run Parsec in local](#run-parsec-in-local)
  - [Hacking the Web client (front)](#hacking-the-web-client-front)
    - [Start working on the web client](#start-working-on-the-web-client)
  - [Hacking the Rust code](#hacking-the-rust-code)
  - [If you are working on Mac](#if-you-are-working-on-mac)

## Get the sources

Source code is available on [github](https://github.com/Scille/parsec-cloud).

## Shared requirements

To start hacking, follow the basic steps detailed below:

1. Your text editor need to be configured with [`editorconfig`](https://editorconfig.org/)
2. You need to have the following tools available:
    1. [`git`](https://git-scm.com/)

       Once you have installed `git` you need to clone the repository (it will be required to correctly configure the virtual environement used by `poetry` in the bellow steps).

       ```shell
       git clone git@github.com:Scille/parsec-cloud.git
       cd parsec-cloud
       ```

    2. [`Rust v1.68.0`](https://www.rust-lang.org/fr/learn/get-started)

       > We use a `rust-toolchain.toml` file, so you can just install `rustup` and `cargo`
       > The required toolchain will be install automatically.

       ```shell
       curl --proto '=https' --tlsv1.2 -sSL https://sh.rustup.rs | sh -s -- --default-toolchain none # You can replace `none` with `1.68.0`
       ```

       > You then need to add the installation path to your `PATH` variable (or equivalent).

       You can verify that you have correctly installed `rust` and configured your `PATH` variable with:

       ```shell
       rustup --version
       ```

    3. **For Windows only** [Strawberry Perl](https://strawberryperl.com/)

       On windows, you'll need to have strawberry perl installed to be able to compile `openssl` from the source.

       Once installed, you will need to set the env variable

       ```txt
       OPENSSL_SRC_PERL=C:/Strawberry/perl/bin/perl
       ```

       > Replace `C:/Strawberry/perl/bin/perl` with the correct installation path.

    4. [`python v3.9`](https://www.python.org/)

       To install the correct python version, we use `pyenv` instead of relaying on a system package:

       1. Install `pyenv`:

          ```shell
          # Install Pyenv
          curl --proto '=https' --tlsv1.2 -sSL https://pyenv.run | bash
          ```

          > For more installation methods for `pyenv`, see <https://github.com/pyenv/pyenv#installation>

       2. Check your `pyenv` installation:

          ```shell
          pyenv --version
          ```

          If the previous command worked, you can skip directly to step 3.

          If this command failed with `command not found` (or alike) this means your OS can't find `pyenv` inside your `PATH` variable.

          To correct that you need to update your `PATH` variable. If you've installed `pyenv` via the step above you need to add `$HOME/.pyenv/bin` to that variable.

          > `$HOME` is you home path.

       3. Install the correct python version using `pyenv`

          ```shell
          # Install a specific Python version
          pyenv install 3.9.5
          ```

    5. [`poetry >=1.3.2`](https://python-poetry.org/docs/#installation)

       Install Poetry (Does not require a specific version of Python)

       ```shell
       curl --proto '=https' --tlsv1.2 -sSL https://install.python-poetry.org/ | python - --version=1.3.2
       ```

       To verify the installation of `poetry` run

       ```shell
       poetry --version
       ```

       You then need to indicate which `python` interpreter to use for `poetry`:

       If you have installed `python` using `pyenv` from above:

       ```shell
       # Create the project virtual with the correct version of Python
       poetry env use $(pyenv prefix 3.9.5)/bin/python
       ```

       > If you don't have installed `python` with `pyenv`, you need to replace `$(pyenv prefix 3.9.5)/bin/python` with the path where the python you want to use is located.

## Hacking the Python code (front + back)

Once you have installed the [basic requirements](#shared-requirements), you can procede to setup to python virtual env with `poetry`.

1. Install [`Qt5`](https://www.qt.io/).

   The `python` application uses `PyQt5`, you need to have `qt5` installed for it to work (it won't even build).

2. Install FUSE (Linux/MacOS) or WinFsp (Windows)

   Depending on your platform:

     - On Windows: you need to install [`winfsp`](https://github.com/winfsp/winfsp) a the version `1.11.22176`

       You can use `choco` for that

       ```cmd
       choco install -y --limit-output winfsp --version=1.11.22176
       ```

       Or you can use the installer <https://winfsp.dev/rel/>

       > We haven't tried this method so if you can provide feedback if you use it.

     - On MacOS: you need to install `macfuse`

       ```shell
       brew install --cask macfuse
       ```

     - On Linux: you need to have installed `libfuse2`

3. Initialize a poetry env

   ```shell
   poetry install --extra 'core backend'
   ```

4. Start a shell with the initialized virtual env

    ```shell
    poetry shell
    ```

5. To run parsec do

    ```shell
    poetry run parsec
    ```

Happy Hacking 🐍

### Run the Python tests

Run the tests with `pytest`:

```shell
poetry run py.test tests
```

In addition, the following options are available:

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

### Run Parsec in local

You can use the ``run_testenv`` scripts to easily create a development environment:

On Linux and MacOS:

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

## Hacking the Web client (front)

In addition to the [shared requirements](#shared-requirements), you need to install [`Node 18.12.0`](https://nodejs.org/en/download/releases).

> You could use [`nvm`](https://github.com/nvm-sh/nvm) to manage multiple node version
> or [`nvm-windows`](https://github.com/coreybutler/nvm-windows) for `Windows`.

1. Install [`Qt5`](https://www.qt.io/).

   The `python` application use `PyQt5`, you need to have `qt5` installed because even if you don't need that part.
   The build system will still require it (and).

   > You can hack around that problem if your system can't have qt5 installed.
   > You'll need to remove `PyQt5` from the build-system requirement in `pyproject.toml`
   > see [PR-4403](https://github.com/Scille/parsec-cloud/pull/4403)

2. Initialize a poetry env

   ```shell
   POETRY_LIBPARSEC_BUILD_PROFILE=ci poetry install --extra 'backend'
   ```

3. Install `wasm-pack`

   Currently we use `wasm-pack@0.11.0`.

   Install it with:

   ```shell
   cargo install wasm-pack@0.11.0
   ```

   > We install it using cargo because it's the simpler way to specify which version of `wasm-pack` to use.

4. Setup the web binding

   ```shell
   pushd oxidation/bindings/web
   npm install
   npm build:dev
   popd
   ```

   > You can use `python make.py web-dev-install` instead.

5. Setup the electron binding

   ```shell
   pushd oxidation/bindings/electron
   npm install
   npm run build:dev
   popd
   ```

   > You can use `python make.py electron-dev-install` instead.

6. Move to the client dir, For the later command we will consider that the current directory is `oxidation/client`

   ```shell
   cd oxidation/client
   ```

7. Install client dependencies

   ```shell
   npm install
   ```

### Start working on the web client

Before start working on the web client you need to setup a mock server
that will provide some mocked data.

1. Make sure you have the latest change for the python testbed server

   ```shell
   POETRY_LIBPARSEC_BUILD_PROFILE=ci poetry install --extra 'backend'
   ```

2. Start the testbed server that will provide the mocked data.

   On another terminal

   ```shell
   $ python tests/scripts/run_testbed_server.py
   [..]
   2023-04-18T12:16:38.649668Z [info     ] Running on http://127.0.0.1:6770 (CTRL + C to quit)
   All set ! Don't forget to export `TESTBED_SERVER_URL` environ variable:
   export TESTBED_SERVER_URL='parsec://127.0.0.1:6770?no_ssl=true'
   ```

   > Note the last 2 line `All set ...` and `export ...`
   > You need to have the env variable `TESTBED_SERVER_URL` set to the value after `export TESTBED_SERVER_URL=` (depending on how you set it, you could remove the `'` around the value) on you primary terminal (the one that would run the dev server)

   You need to keep that terminal open with the script running otherwise, the mock server would stop working.

3. On your main terminal after setting the env var `TESTBED_SERVER_URL`, you can now start the dev server

   ```shell
   npm run web:open
   ```

## Hacking the Rust code

With `git` & `rust` installed alongside your favorite editor configured with `editorconfig` you're done.

You start by building the Rust code + its dependencies

```shell
cargo build --workspace
```

Happy hacking 🦀

## If you are working on Mac

If you work on a Mac make sure you have enabled `developer mode` (`python` should work).

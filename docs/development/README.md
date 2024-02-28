<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Hacking Quickstart

- [Get the sources](#get-the-sources)
- [Shared requirements](#shared-requirements)
- [Hacking the Python server](#hacking-the-python-server)
  - [Run the Python tests](#run-the-python-tests)
- [Hacking the clients](#hacking-the-clients)
  - [Base requirement](#base-requirement)
  - [Hacking the web client](#hacking-the-web-client)
  - [Hacking the electron client](#hacking-the-electron-client)
- [Starting the testbed server](#starting-the-testbed-server)
- [Hacking the Rust code](#hacking-the-rust-code)
- [If you are working on Mac](#if-you-are-working-on-mac)

## Get the sources

Source code is available on [github](https://github.com/Scille/parsec-cloud).

## Shared requirements

To start hacking, follow the basic steps detailed below:

1. Your text editor need to be configured with [`editorconfig`](https://editorconfig.org/)
2. You need to have the following tools available:
    1. [`git`](https://git-scm.com/)

       Once you have installed `git` you need to clone the repository (it will be required to correctly configure the virtual environment used by `poetry` in the bellow steps).

       ```shell
       git clone git@github.com:Scille/parsec-cloud.git
       cd parsec-cloud
       ```

    2. [`Rust v1.71.1`](https://www.rust-lang.org/fr/learn/get-started)

       > We use a `rust-toolchain.toml` file, so you can just install `rustup` and `cargo`
       > The required toolchain will be install automatically.

       ```shell
       curl --proto '=https' --tlsv1.2 -sSL https://sh.rustup.rs | sh -s -- --default-toolchain none # You can replace `none` with `1.71.1`
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

    4. [`python v3.12`](https://www.python.org/)

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
          pyenv install 3.12.0
          ```

    5. [`poetry >=1.5.1`](https://python-poetry.org/docs/#installation)

       Install Poetry (Does not require a specific version of Python)

       ```shell
       curl --proto '=https' --tlsv1.2 -sSL https://install.python-poetry.org/ | python - --version=1.5.1
       ```

       To verify the installation of `poetry` run

       ```shell
       poetry --version
       ```

       You then need to indicate which `python` interpreter to use for `poetry`:

       If you have installed `python` using `pyenv` from above:

       ```shell
       # Create the project virtual with the correct version of Python
       poetry env use $(pyenv prefix 3.12.0)/bin/python -C server/pyproject.toml
       ```

       > If you don't have installed `python` with `pyenv`, you need to replace `$(pyenv prefix 3.12.0)/bin/python` with the path where the python you want to use is located.

## Hacking the Python server

Once you have installed the [basic requirements](#shared-requirements), you can procede to setup to python virtual env with `poetry`.

The python code is located in the folder `server`, The following steps and instruction will consider that you're in that folder.

   <!-- markdownlint-disable-next-line no-inline-html -->
1. Initialize a poetry env <a id="init-server-python-env" />

   ```shell
   python ./make.py python-dev-install
   ```

   > The Python server is built from the `server` directory.

2. Start a shell with the initialized virtual env

    ```shell
    poetry shell
    ```

3. To run parsec do

    ```shell
    poetry run parsec
    ```

Happy Hacking 🐍

### Run the Python tests

Run the tests with `pytest`:

```shell
poetry run pytest tests
```

In addition, the following options are available:

| Option           | Description                                                                                               |
| ---------------- | --------------------------------------------------------------------------------------------------------- |
| ``--runslow``    | Include slow tests                                                                                        |
| ``--postgresql`` | Use PostgreSQL in the server instead of a mock in memory</br>**⚠️ Currently postgresql tests are broken**  |
| ``-n 4``         | Run tests in parallel (here `4` jobs)                                                                     |

Note you can mix&match the flags, e.g.

```shell
poetry run pytest tests --runslow -n auto
```

## Hacking the clients

In addition to the [shared requirements](#shared-requirements), for working with the web clients you need to:

### Base requirement

1. Install [`Node 18.12.0`](https://nodejs.org/en/download/releases).

   We recommend using [`nvm`](https://github.com/nvm-sh/nvm) to manage multiple node versions:

   > [`nvm-windows`](https://github.com/coreybutler/nvm-windows) for `Windows`

   ```shell
   nvm install 18.12.0
   ```

2. Install `wasm-pack`

   Currently we use `wasm-pack@0.11.0`.

   Install it with:

   ```shell
   cargo install wasm-pack@0.11.0
   ```

   > We install it using cargo because it's the simpler way to specify which version of `wasm-pack` to use.

### Hacking the web client

1. Follow the [base requirement](#base-requirement).

2. Setup the web binding:

   ```shell
   python ./make.py web-dev-install
   ```

3. Move to the `client` directory:

   ```shell
   cd client
   ```

4. Install client dependencies from the `client` directory:

   ```shell
   npm install
   ```

5. To start working on the web client, you need 2 things:

   1. Start the testbed server on another shell, by following the instruction on [Starting the testbed server](#starting-the-testbed-server).

   2. Start the development server for the web interface with:

      ```shell
      npm run web:open
      ```

### Hacking the electron client

<!-- TODO: Currently the web client via electron doesn't provide mountpoint so fuse isn't required
1. Install FUSE (Linux/MacOS) or WinFsp (Windows)

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

     - On Linux: you need to have installed `libfuse2` -->

1. Follow the [Base requirement](#base-requirement).

2. Apply the steps 2 to 4 of [Hacking the web client](#hacking-the-web-client).

3. Setup the electron binding

   ```shell
   python ./make.py electron-dev-install
   ```

4. To start working on the electron client, you need like the web client 2 things:

   1. Start the testbed server, the instruction can be found [here](#starting-the-testbed-server).
   2. Start electron in development mode with

      ```shell
      npm run electron:open
      ```

## Starting the testbed server

Before start working on the web client you need to setup a mock server
that will provide some mocked data.

1. Make sure you have the latest change for the python testbed server, see [Initialize a poetry env](#init-server-python-env)

2. Start the testbed server that will provide the mocked data.

   On another terminal

   ```shell
   $ python ./make.py run-testbed-server
   [..]
   2023-04-18T12:16:38.649668Z [info     ] Running on http://127.0.0.1:6770 (CTRL + C to quit)
   All set ! Don't forget to export `TESTBED_SERVER_URL` environ variable:
   export TESTBED_SERVER_URL='parsec://127.0.0.1:6770?no_ssl=true'
   ```

   > Note the last 2 line `All set ...` and `export ...`
   > You need to have the env variable `TESTBED_SERVER_URL` set to the value after `export TESTBED_SERVER_URL=` (depending on how you set it, you could remove the `'` around the value) on you primary terminal (the one that would run the dev server)

   You need to keep that terminal open with the script running otherwise, the mock server would stop working.

## Hacking the Rust code

With `git` & `rust` installed alongside your favorite editor configured with `editorconfig` you're done.

You start by building the Rust code + its dependencies

```shell
cargo build --workspace
```

Happy hacking 🦀

## If you are working on Mac

If you work on a Mac make sure you have enabled `developer mode` (`python` should work).

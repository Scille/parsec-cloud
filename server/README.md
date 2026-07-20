# Parsec Server

## Development

### Quickstart

1. Start by installing the [shared requirements](../docs/development/README.md#shared-requirements).

2. Activate your virtual environment

   ```shell
   uv venv server/.venv
   ```

3. Install dependencies (from the root directory)

   ```shell
   python ./make.py python-dev-install
   ```

4. Run parsec server

   ```shell
   uv --project server run parsec
   ```

Happy Hacking! 🐍

### Running tests

Run the tests with `pytest` (from `server` directory):

- Run all tests:

  ```shell
  uv run pytest tests
  ```

- Run tests from a specific directory:

  ```shell
  uv run pytest tests/api_v5/
  ```

- Run tests from a specific test file:

  ```shell
  uv run pytest tests/api_v5/authenticated/test_shamir_recovery_setup.py
  ```

- Run specific tests functions with `-k EXPR`, a Python expression where all names are substring-matched against test names:

  ```shell
  uv run pytest -k 'not_allowed or not_found' tests
  ```

The following options are available (run `uv run pytest --help` for many more!):

| Option                | Description                                              |
| --------------------- | -------------------------------------------------------- |
| ``-n 4``              | Run tests in parallel (here `4` jobs)                    |
| ``-x``                | Exit instantly on first error or failed test             |
| ``-v``                | Increase verbosity                                       |
| ``--log-level=LEVEL`` | Level of messages to catch/display                       |
| ``--timeout=TIMEOUT`` | Timeout in seconds before dumping the stacks.            |

#### PostgreSQL tests

By default, tests are run with an *in-memory* backend implementation.
To use *PostgreSQL* backend implementation use the `--postgresql` option.

| Option                     | Description                                              |
| -------------------------- | -------------------------------------------------------- |
| `--postgresql`             | Use PostgreSQL in the server instead of a mock in memory |
| `--run-postgresql-cluster` | Instead of running the tests, only start a PostgreSQL cluster that can be use for other tests (through `PG_URL` env var) to avoid having to create a new cluster each time. |

# Parsec CLI

This is the home of Parsec command-line interface.

- [⚒️ Build](#️-build)
- [🏃 Run](#-run)
- [✅ Test](#-test)

## ⚒️ Build

```shell
cargo build --package parsec-cli
```

## 🏃 Run

```shell
cargo run --package parsec-cli
```

Use the `help` command to get a specific command description, for example:

```shell
cargo run --package parsec-cli help workspace import
```

## ✅ Test

Parsec CLI tests run against a testbed server. Make sure you start one either with Docker or by
running `python make.py run-testbed-server` in a terminal.

Open another terminal and export the TESTBED_SERVER variable, for example:

```shell
export TESTBED_SERVER='parsec3://127.0.0.1:6770?no_ssl=true'
```

Run tests with the following command:

```shell
cargo nextest run --package parsec-cli
```

> [!TIP]
> Some tests may be flaky due to `rexpect` timeouts. If that's the case, you can disable timeouts by
> setting the `CI` variable before running the tests.
>
> ```shell
> export CI=true
> cargo nextest run --package parsec-cli
> ```
>
> or
>
> ```shell
> CI=true cargo nextest run --package parsec-cli
> ```

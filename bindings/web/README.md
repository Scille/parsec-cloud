# Libparsec web bindings

This directory contains libparsec bindings for the web application.

## Build

> [!NOTE]
> The internal build commands relies on [`wasm-pack`](https://github.com/rustwasm/wasm-pack).
> Remember to install it as described in the [base requirements`](../../docs/development/README.md#base-requirement).

Build bindings using the `make.py` scripts at the root of the repo.

First run (to install dependencies):

```shell
./make.py wi    # wi is an alias for web-dev-install
```

Subsequent runs:

```shell
./make.py wr    # wr is an alias for web-dev-rebuild
```

Basically `make.py` wraps the following commands:

```shell
# install dependencies
npm install
# (re)generate dist/libparsec.node (this is a .so that node can load)
npm run build:dev
# Or for release
npm run build:release
```

## Interactive testing

To easily play with the bindings:

```shell
python ./scripts/liveplay.py
```

This starts a server exposing a web page with the bindings, from there you
can open your browser console and start playing with `libparsec`.

## Testing

There is not much tests for now, but you can use them with:

```shell
wasm-pack test --headless --firefox
```

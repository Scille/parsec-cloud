# libparsec-electron-bindings

Remember to have [`wasm-pack`](../../docs/development/README.md#base-requirement) installed !

## Build

tl;dr: `../../make.py wr`

First run (to install dependencies)

```shell
../../make.py wi    # wi is an alias for web-dev-install
```

Subsequent runs

```shell
../../make.py wr    # wr is an alias for web-dev-rebuild
```

Basically `make.py` wraps the following commands:

```shell
npm install
# (re)generate index.node
npm run build:dev
# Or for release
npm run build:release
```

The internal build command itself relies on [`wasm-pack`](https://github.com/rustwasm/wasm-pack).

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

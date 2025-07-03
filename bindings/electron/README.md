# Libparsec Electron bindings

This directory contains libparsec bindings for Electron (desktop application).

## Build

Build bindings using the `make.py` scripts at the root of the repo.




First run (to install dependencies):

```shell
./make.py ei    # ei is an alias for electron-dev-install
```

Subsequent runs:

```shell
./make.py er    # er is an alias for electron-dev-rebuild
```

Basically `make.py` wraps the following commands:

```shell
# install dependencies
npm install
# (re)generate dist/libparsec.node
npm run build:dev
# Or for release
npm run build:release
```

> [!NOTE]
> The `/client/electron` project automatically run `npm build` and copy `libparsec.node` where needed.*

## Testing (the simple way)

Start node shell

```shell
node
```

Then import libparsec package and you're good!

```javascript
libparsec = require("./bindings/electron/dist/libparsec");
await libparsec.listAvailableDevices("/foo")
```

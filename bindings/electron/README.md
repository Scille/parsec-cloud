# libparsec-electron-bindings

## Build

tl;dr: `../../make.py er`

First run (to install dependencies)

```shell
../../make.py ei    # ei is an alias for electron-dev-install
```

Subsequent runs

```shell
../../make.py er    # er is an alias for electron-dev-rebuild
```

Basically `make.py` wraps the following commands:

```shell
npm install
# (re)generate index.node
npm run build:dev
# Or for release
npm run build:release
```

## Testing (the simple way)

Start node shell

```shell
node
```

Then import libparsec package and you're good !

```javascript
libparsec = require("./bindings/electron/dist/libparsec");
await libparsec.listAvailableDevices("/foo")
```

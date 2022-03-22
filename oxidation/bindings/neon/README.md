# libparsec-neon-bindings

```shell
npm install
# (re)generate index.node
npm run build
# Or for release (everything after -- will be passed to cargo, so --feature il also ok)
npm run build -- --release
# Finally copy the generated index.node into the client electron codebase
cp index.node ../../client/electron/src/libparsec/
```

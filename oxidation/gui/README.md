1 - Pre-requirements
```shell
npm install -g @ionic/cli native-run cordova-res
```

2 - Parsec GUI Ionic project creation
```shell
# Location must be /parsec-cloud/oxidation
ionic start gui blank --type=angular --capacitor
pushd gui
npm install @capacitor-community/electron
# Ionic project must be build at least one time before adding capacitor plugins
ionic build
# Then we add the capacitor platforms
npx cap add @capacitor-community/electron
ionic cap add android
ionic cap add ios
```

3 - Launch specific versions (For development purpose)
```shell
# web browser version
ionic serve

# electron version
npx cap open @capacitor-community/electron

# android version
npx cap open android

# ios version
npx cap open ios
```

4 - Update specific versions (For development purpose)
```shell
# web browser version
ionic build

# electron version
ionic build && npx cap copy @capacitor-community/electron

# android version
ionic build && npx cap copy android

# ios version
ionic build && npx cap copy ios
```

5 - Build and package specific versions (For production purpose)
```shell
# web browser version
# TO WRITE

# electron version
# TO WRITE

# android version
# TO WRITE

# ios version
# TO WRITE
```

5 - Specific bindings (For development purpose)
```shell
# Location must be /parsec-cloud/oxidation

# web browser version
# TO WRITE => webAssembly

# electron version
# Generate `libparsec-neon-bindings/index.node` (basically a .so that node can load)
pushd libparsec-neon-bindings
npm install && npm run build
popd
pushd gui
# Copy the node module so electron can find it for packaging
cp ../libparsec-neon-bindings/index.node ./libparsec-neon-bindings # (unix)
copy ..\libparsec-neon-bindings\index.node .\libparsec-neon-bindings # (windows)

# android version
# TO WRITE => ?

# ios version
# TO WRITE => ?
```
 
tl;dr:

```shell
npm install

# Electron dev
npm run electron:open
# *modify stuff*
npm run electron:copy

# Android dev
npm run android:open
# *modify stuff*
npm run android:copy
```


## 1 - Install requirements

```shell
npm install  # install dependecies
```

Among other things, `npm install` installs ionic/vue/capacitor CLI locally, use them with npx:
```shell
npx ionic --help
```

## 2 - Web dev

```shell
npx serve
```

Note: libparsec is not available on web platform so far :(

## 2 - Electron dev

First libparsec's neon bindings must be built:
```shell
cd ../bindings/neon
npm install
npm run build  # Generate an index.node file
```
Note the `index.node` will be automatically copied by electron on build (see
`libparsec` script defined in `client/electron/package.json`)

Then for regular build:
```shell
npx ionic build  # update the `dist` folder
npx cap copy @capacitor-community/electron # copy `dist` into `electron/app` folder
cd electron
npm run electron:dist
npm run electron:dist -- --dir  # To debug the generated payload
npm run electron:dist -- --linux snap  # To generate a subset (see `npx electron-builder build --help`)
```

Or for incremental development:
```shell
npx ionic build && npx cap copy @capacitor-community/electron # Create/refresh `electron/app`
npx cap open @capacitor-community/electron  # Open the app with a live reloader on `electron/app`
cd electron
npm electron:start  # Equivalent to above, have a look to `electron/package.json` ;-)
npm libparsec  # Copy the libparsec index.node if it has been rebuilt
```

## 3 - Android dev

For regular build:
```shell
npx ionic build  # update the `dist` folder
npx cap copy android # copy `dist` into `android/app` folder
npx cap open android  # Open Android studio
```

Note: Libparsec is automatically (re)build as needed when building the Android project.

## 4 - iOS dev

For regular build:
```shell
npx ionic build  # update the `dist` folder
npx cap copy ios # copy `dist` into `ios/app` folder
npx cap open ios  # Open XCode
```

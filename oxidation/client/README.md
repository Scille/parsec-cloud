# Parsec client with Ionic & Electron

## 1 - Install

### a. Dependencies

```bash
# In /oxidation/client
npm install
```

> Among other things, it installs `Ionic` / `Vue` / `Capacitor` CLI locally, you can use them with npx like this: `npx ionic --help`

> This command will automatically execute a `npm install` in the ELectron folder, this is normally done by `npx cap add @capacitor-community/electron`, nevertheless since we have custom configuration in this folder, we can't override it.

### b. LibParsec Bindings requirements

Follow this [bindings guide](../bindings/README.md)

## 2 - Web dev

```bash
# In /oxidation/client
npm run web:open
```

> LibParsec is not available on web platform so far :(

## 3 - Electron dev

```bash
# In /oxidation/client
# Update Electron folder after main project changes
npm run electron:copy
# ----
# Update Electron folder after main project changes and launch the desktop app
npm run electron:open
# ----
# Generate Electron dist for release
npm run electron:dist
npm run electron:dist -- -- --dir  # To debug the generated payload
npm run electron:dist -- -- --linux snap  # To generate a subset (see `npx electron-builder build --help`)
```

> The `index.node` will be automatically copied by electron on build (see
> `libparsec` script defined in `client/electron/package.json`)

## 4 - Android dev

```bash
# In /oxidation/client
# Update Android folder after main project changes
npm run android:copy
# ----
# Update Android folder after main project changes and launch an Android Studio project
npm run android:open
```

> LibParsec is automatically (re)build as needed when building the Android project.

## 5 - iOS dev

```bash
# In /oxidation/client
# Update iOS folder after main project changes
npm run ios:copy
# ----
# Update iOS folder after main project changes and launch a XCode project
npm run ios:open
```

## BONUS - How to start a blank Ionic project with Electron

```bash
# Pre-requirements
npm install -g @ionic/cli native-run cordova-res
# Location must be where you want the project folder
ionic start blank-project blank --type=vue --capacitor
pushd blank-project
npm install @capacitor-community/electron
# Ionic project must be build at least one time before adding capacitor plugins
ionic build
# Then we add the capacitor platforms
npx cap add @capacitor-community/electron
ionic cap add android
ionic cap add ios
```

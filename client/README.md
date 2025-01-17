# Parsec client

This directory contains source code for Parsec client.

For general setup, take a look at [development quickstart guide](../docs/development/README.md#Hacking-the-clients).

## Install

### Dependencies

```bash
npm install
```

Among other things, it installs `Ionic` / `Vue` / `Capacitor` CLI locally, you can use them with npx like this: `npx ionic --help`

This command will also automatically execute `npm install` in the electron directory, this is normally done by `npx cap add @capacitor-community/electron`, nevertheless since we have custom configuration in this folder, we can't override it.

### Libparsec bindings

Follow this [guide](../bindings/README.md).

## Tests

### Unit tests

```bash
npm run test:unit
```

### End-to-End tests

To run these tests, you first need to [start the testbed server locally](../docs/development/README.md#starting-the-testbed-server).

```bash
export TESTBED_SERVER="parsec3://localhost:6777?no_ssl=true"
```

#### Playwright

Install Playwright dependencies:

```bash
npx playwright install --with-deps
```

Then, run the tests with:

```bash
npm run test:e2e:headless
```

## Web dev

> Libparsec is not yet available on web platform :(

```bash
npm run web:open
```

## Electron dev

```bash
# Update electron directory after main project changes
npm run electron:copy

# Update electron directory after main project changes and launch the desktop app
npm run electron:open

# Generate electron dist for release
npm run electron:dist
npm run electron:dist -- -- --dir  # To debug the generated payload
npm run electron:dist -- -- --linux snap  # To generate a subset (see `npx electron-builder build --help`)
```

> The `libparsec.node` will be automatically copied by electron on build (see
> `libparsec` script defined in `client/electron/package.json`)

## Android dev

```bash
# Update android directory after main project changes
npm run android:copy

# Update android directory after main project changes and launch an Android Studio project
npm run android:open
```

Libparsec is automatically (re)built as needed.

<!-- TODO: iOS platform not yet available
## iOS dev

```bash
# In /client
# Update iOS folder after main project changes
npm run ios:copy
# ----
# Update iOS folder after main project changes and launch a XCode project
npm run ios:open
``` -->

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

## Variables

| Name                                   | Type                      | Description                                                                             | Remark                                                                                          |
| -------------------------------------- | ------------------------- | --------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| `PARSEC_APP_DEV_BMS_CREDENTIALS`       | `<email>:<password>`      | Used as default login credentials for the BMS                                           | Only for development purposes! Avoid using `:` in your password as it will mess up the parsing. |
| `PARSEC_APP_BMS_USE_MOCK`              | `boolean`                 | Used to mock calls to the BMS                                                           | Only for development purposes!                                                                  |
| `PARSEC_APP_BMS_MOCKED_FUNCTIONS `     | `function1;function2;...` | Comma-separated list of functions from the BMS API to mock                              | Only for development purposes!                                                                  |
| `PARSEC_APP_BMS_FAIL_FUNCTIONS `       | `function1;function2;...` | Comma-separated list of functions from the BMS API that should fail if mocked           | Only for development purposes!                                                                  |
| `PARSEC_APP_DISABLE_STRIPE`            | `boolean`                 | Disable Stripe and hide the customer area                                               |                                                                                                 |
| `PARSEC_APP_DISABLE_SENTRY`            | `boolean`                 | Disable Sentry                                                                          |                                                                                                 |
| `PARSEC_APP_BMS_MOCK_WAIT_DURATION`    | `number`                  | How much time mocked BMS functions should take, to simulate network and server slowness | Only for development purposes!                                                                  |
| `PARSEC_APP_CREATE_DEFAULT_WORKSPACES` | `boolean`                 | Create default workspaces when initializing the app                                     | Only for development purposes!                                                                  |
| `PARSEC_APP_CLEAR_CACHE`               | `boolean`                 | Clear the cache                                                                         | Only for development purposes!                                                                  |
| `PARSEC_APP_TESTBED_AUTO_LOGIN`        | `boolean`                 | Logins automatically to the first device if set and if using the testbed                | Only for development purposes                                                                   |

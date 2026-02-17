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

### Environment variables

| Name                                    | Type                      | Description                                                                             | Remark                                                                                                                  |
| --------------------------------------- | ------------------------- | --------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| `PARSEC_APP_DEV_BMS_CREDENTIALS`        | `<email>:<password>`      | Used as default login credentials for the BMS                                           | Only for development purposes! Avoid using `:` in your password as it will mess up the parsing.                         |
| `PARSEC_APP_BMS_USE_MOCK`               | `boolean`                 | Used to mock calls to the BMS                                                           | Only for development purposes!                                                                                          |
| `PARSEC_APP_BMS_MOCKED_FUNCTIONS `      | `function1;function2;...` | Comma-separated list of functions from the BMS API to mock                              | Only for development purposes!                                                                                          |
| `PARSEC_APP_BMS_FAIL_FUNCTIONS `        | `function1;function2;...` | Comma-separated list of functions from the BMS API that should fail if mocked           | Only for development purposes!                                                                                          |
| `PARSEC_APP_DISABLE_STRIPE`             | `boolean`                 | Disable Stripe and hide the customer area                                               |                                                                                                                         |
| `PARSEC_APP_DISABLE_SENTRY`             | `boolean`                 | Disable Sentry                                                                          |                                                                                                                         |
| `PARSEC_APP_BMS_MOCK_WAIT_DURATION`     | `number`                  | How much time mocked BMS functions should take, to simulate network and server slowness | Only for development purposes!                                                                                          |
| `PARSEC_APP_POPULATE_DEFAULT_WORKSPACE` | `boolean`                 | Adds files to the default workspace                                                     | Only for development purposes!                                                                                          |
| `PARSEC_APP_CLEAR_CACHE`                | `boolean`                 | Clear the cache                                                                         | Only for development purposes!                                                                                          |
| `PARSEC_APP_TESTBED_AUTO_LOGIN`         | `boolean`                 | Logins automatically to the first device if set and if using the testbed                | Only for development purposes                                                                                           |
| `PARSEC_APP_POPULATE_USERS`             | `boolean`                 | Adds users to the default organization                                                  | Only for development purposes!                                                                                          |
| `PARSEC_APP_ACCOUNT_SERVER`             | `url`                     | Parsec Account server to use                                                            |                                                                                                                         |
| `PARSEC_APP_ENABLE_ACCOUNT`             | `boolean`                 | Enable Parsec Account                                                                   |                                                                                                                         |
| `PARSEC_APP_ENABLE_ACCOUNT_AUTO_LOGIN`  | `boolean`                 | Enable Account auto login                                                               | Only for development purposes! Only works if `PARSEC_APP_ENABLE_ACCOUNT` and `PARSEC_APP_MOCK_ACCOUNT` are set to true. |
| `PARSEC_APP_ENABLE_CUSTOM_BRANDING`     | `boolean`                 | Enable the custom branding                                                              |                                                                                                                         |
| `PARSEC_APP_ENABLE_EDITICS`             | `boolean`                 | Enable Parsec Editics poc                                                               |                                                                                                                         |
| `PARSEC_APP_DEFAULT_CRYPTPAD_SERVER`    | `url`                     | Cryptpad server to use                                                                  |                                                                                                                         |

### Testing variables

Those variables are used when testing the app in Playwright. They will mostly be used to enable or disable or configure features (for example, we do not need to load Stripe for most tests). They can be toggled per test.

| Name                             | Type                                              | Description                                                                                                    |
| -------------------------------- | ------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| `TESTING`                        | `boolean`                                         | Place the app in testing mode (reduce the logs, disable Sentry, does not setup the testbed automatically, ...) |
| `TESTING_PKI`                    | `boolean`                                         | Use mocks for PKI                                                                                              |
| `TESTING_DISABLE_STRIPE`         | `boolean`                                         | Disable Stripe                                                                                                 |
| `TESTING_OPEN_BAO_SERVER`        | `url`                                             | Sets the OpenBao server to use                                                                                 |
| `TESTING_ACCOUNT_AUTO_LOGIN`     | `boolean`                                         | Automatically logs into Parsec Account                                                                         |
| `TESTING_ENABLE_ACCOUNT`         | `boolean`                                         | Enables Parsec Account                                                                                         |
| `TESTING_ACCOUNT_SERVER`         | `url`                                             | Sets the Parsec Account server                                                                                 |
| `TESTING_ENABLE_EDITICS`         | `boolean`                                         | Enables Cryptpad integration                                                                                   |
| `TESTING_CRYPTPAD_SERVER`        | `url`                                             | Cryptpad server                                                                                                |
| `TESTING_EDITICS_SAVE_TIMEOUT`   | `number`                                          | Timeout before Cryptpad auto-save                                                                              |
| `TESTING_ENABLE_CUSTOM_BRANDING` | `boolean`                                         | Enabled custom branding                                                                                        |
| `TESTING_MOCK_BROWSER`           | `Chrome\|Firefox\|Safari\|Edge\|\Brave\|Chromium` | Simulates navigating with a specific browser                                                                   |
| `TESTING_SAAS_SERVERS`           | `server1;server2;server3;...`                     | Servers recognized as Saas                                                                                     |
| `TESTING_TRIAL_SERVERS`          | `server1;server2;server3;...`                     | Servers recognized as trial                                                                                    |

# Parsec client

This directory contains source code for Parsec client.

For general setup, take a look at [development quickstart guide](../docs/development/README.md#Hacking-the-clients).

## Install

### Dependencies

```bash
npm install
```

Among other things, it installs `Vue` / `Capacitor` CLI locally, you can use them with npx like this: `npx cap --help`

`megashark-lib` is installed separately, as it's a git dependency that needs its `prepare` script (a
`vite build`) to run, which `npm install` doesn't do since scripts are disabled (see `.npmrc`). It gets
installed automatically by the `megashark:install` script, itself run as part of `dev`, `web:open`,
`web:release`, `native:build` and `native:build:dev` (see `//megasharkLibSource` in `package.json`). You
can also run it explicitly with `npm run megashark:install`.

If you plan on running Electron, you may also want to launch `npm install` inside the electron directory, or it will be run automatically when you start Electron for the first time.

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

```bash
npm run web:open

# Use `TESTBED_SERVER` to have the application connect to the testbed server and start with
# a CoolOrg testbed template (see `libparsec/crates/testbed/src/templates/coolorg.rs`).
TESTBED_SERVER='parsec3://127.0.0.1:6770?no_ssl=true' npm run web:open
```

## Electron dev

```bash
# Update electron directory after main project changes
npm run electron:copy

# Update electron directory after main project changes and launch the desktop app
npm run electron:open

# Typical development configuration:
# - `TESTBED_SERVER`: The application connects to the testbed server and start with a CoolOrg testbed
#   template (see `libparsec/crates/testbed/src/templates/coolorg.rs`).
# - `SKIP_VITE_BUILD_FOR_NATIVE`: Disable building the GUI (see `client/scripts/vite_build_for_native.cjs`). This is
#   useful when working on libparsec (since the GUI code doesn't change).
# - `PARSEC_RUST_LOG_FILE`: Send libparsec logs to a file. But default libparsec logs are written to stderr which is hard
#   to obtain on electron (partly because native modules run in a separated process, partly because the Javascript ecosystem
#   is a big half-broken mess...).
SKIP_VITE_BUILD_FOR_NATIVE= TESTBED_SERVER='parsec3://127.0.0.1:6770?no_ssl=true' PARSEC_RUST_LOG_FILE=parsec.log npm run electron:open

# Generate electron dist for release
npm run electron:dist
npm run electron:dist -- -- --dir  # To debug the generated payload
npm run electron:dist -- -- --linux snap  # To generate a subset (see `npx electron-builder build --help`)
```

> The `libparsec.node` will be automatically copied by electron on build (see
> `libparsec` script defined in `client/electron/package.json`)

## Variables

### Environment variables

| Name                                    | Type                                             | Description                                                                                                                                              | Remark                                                                                                                  |
| --------------------------------------- | ------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| `PARSEC_APP_DEV_BMS_CREDENTIALS`        | `<email>:<password>`                             | Used as default login credentials for the BMS                                                                                                            | Only for development purposes! Avoid using `:` in your password as it will mess up the parsing.                         |
| `PARSEC_APP_BMS_USE_MOCK`               | `boolean`                                        | Used to mock calls to the BMS                                                                                                                            | Only for development purposes!                                                                                          |
| `PARSEC_APP_BMS_MOCKED_FUNCTIONS `      | `function1;function2;...`                        | Comma-separated list of functions from the BMS API to mock                                                                                               | Only for development purposes!                                                                                          |
| `PARSEC_APP_BMS_FAIL_FUNCTIONS `        | `function1;function2;...`                        | Comma-separated list of functions from the BMS API that should fail if mocked                                                                            | Only for development purposes!                                                                                          |
| `PARSEC_APP_DISABLE_STRIPE`             | `boolean`                                        | Disable Stripe and hide the customer area                                                                                                                |                                                                                                                         |
| `PARSEC_APP_DISABLE_SENTRY`             | `boolean`                                        | Disable Sentry                                                                                                                                           |                                                                                                                         |
| `PARSEC_APP_BMS_MOCK_WAIT_DURATION`     | `number`                                         | How much time mocked BMS functions should take, to simulate network and server slowness                                                                  | Only for development purposes!                                                                                          |
| `PARSEC_APP_DEV_POPULATE_REAL_FILES`    | `boolean`                                        | Adds real mocked files to the default workspace                                                                                                          | Only for development purposes!                                                                                          |
| `PARSEC_APP_DEV_POPULATE_MANY_FILES`    | `MAIN_FOLDER_COUNT;SUB_FOLDER_COUNT;FILES_COUNT` | Generate a lot of empty files. Will create `MAIN_FOLDER_COUNT` folders, each containing `SUB_FOLDER_COUNT` folders, each containing `FILES_COUNT` files. | Only for development purposes!                                                                                          |
| `PARSEC_APP_DEV_POPULATE_WORKSPACES`    | `number`                                         | Generate N workspaces (if used with POPULATE\_\*\_FILES variables, all workspaces will be populated).                                                    | Only for development                                                                                                    |
| `PARSEC_APP_DEV_ADD_READONLY_WORKSPACE` | `boolean`                                        | Creates a read only workspace                                                                                                                            | Only for development                                                                                                    |
| `PARSEC_APP_CLEAR_CACHE`                | `boolean`                                        | Clear the cache                                                                                                                                          | Only for development purposes!                                                                                          |
| `PARSEC_APP_TESTBED_AUTO_LOGIN`         | `boolean`                                        | Logins automatically to the first device if set and if using the testbed                                                                                 | Only for development purposes                                                                                           |
| `PARSEC_APP_DEV_POPULATE_USERS`         | `boolean`                                        | Adds users to the default organization                                                                                                                   | Only for development purposes!                                                                                          |
| `PARSEC_APP_ACCOUNT_SERVER`             | `url`                                            | Parsec Account server to use                                                                                                                             |                                                                                                                         |
| `PARSEC_APP_ENABLE_ACCOUNT`             | `boolean`                                        | Enable Parsec Account                                                                                                                                    |                                                                                                                         |
| `PARSEC_APP_ENABLE_ACCOUNT_AUTO_LOGIN`  | `boolean`                                        | Enable Account auto login                                                                                                                                | Only for development purposes! Only works if `PARSEC_APP_ENABLE_ACCOUNT` and `PARSEC_APP_MOCK_ACCOUNT` are set to true. |
| `PARSEC_APP_ENABLE_CUSTOM_BRANDING`     | `boolean`                                        | Enable the custom branding (not needed on web, see `custom-branding` meta attribute in `client/index.html`)                                              |                                                                                                                         |
| `PARSEC_APP_ENABLE_EDITICS`             | `boolean`                                        | Enable Parsec Editics poc                                                                                                                                |                                                                                                                         |
| `PARSEC_APP_FORCE_CRYPTPAD_SERVER`      | `url`                                            | Cryptpad server to use (overwrites server config)                                                                                                        |                                                                                                                         |
| `PARSEC_APP_ENABLE_SHAMIR`              | `boolean`                                        | Enable Shamir                                                                                                                                            |                                                                                                                         |

### Testing variables

Those variables are used when testing the app in Playwright. They will mostly be used to enable or disable or configure features (for example, we do not need to load Stripe for most tests). They can be toggled per test.

| Name                             | Type                                              | Description                                                                                                    |
| -------------------------------- | ------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| `TESTING`                        | `boolean`                                         | Place the app in testing mode (reduce the logs, disable Sentry, does not setup the testbed automatically, ...) |
| `TESTING_DISABLE_STRIPE`         | `boolean`                                         | Disable Stripe                                                                                                 |
| `TESTING_ACCOUNT_AUTO_LOGIN`     | `boolean`                                         | Automatically logs into Parsec Account                                                                         |
| `TESTING_ENABLE_ACCOUNT`         | `boolean`                                         | Enables Parsec Account                                                                                         |
| `TESTING_ACCOUNT_SERVER`         | `url`                                             | Sets the Parsec Account server                                                                                 |
| `TESTING_ENABLE_EDITICS`         | `boolean`                                         | Enables Cryptpad integration                                                                                   |
| `TESTING_CRYPTPAD_SERVER`        | `url`                                             | Cryptpad server                                                                                                |
| `TESTING_EDITICS_SAVE_TIMEOUT`   | `number`                                          | Timeout before Cryptpad auto-save                                                                              |
| `TESTING_ENABLE_CUSTOM_BRANDING` | `boolean`                                         | Enables custom branding                                                                                        |
| `TESTING_MOCK_BROWSER`           | `Chrome\|Firefox\|Safari\|Edge\|\Brave\|Chromium` | Simulates navigating with a specific browser                                                                   |
| `TESTING_SAAS_SERVERS`           | `server1;server2;server3;...`                     | Servers recognized as Saas                                                                                     |
| `TESTING_TRIAL_SERVERS`          | `server1;server2;server3;...`                     | Servers recognized as trial                                                                                    |
| `TESTING_ADD_USERS`              | `userName1:profile;userName2:profile;...`         | Profile as the enum value defined in parsec (`UserProfileAdmin` for an admin)                                  |
| `TESTING_MOCKED_SCWS`            | `boolean`                                         | Enable PKI support on the testbed                                                                              |
| `TESTING_ENABLE_SHAMIR`          | `boolean`                                         | Enables Shamir                                                                                                 |

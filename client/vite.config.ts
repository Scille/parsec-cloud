// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
// eslint-disable-next-line spaced-comment
/// <reference types="vitest" />

import path from 'path';
import { defineConfig, PluginOption, UserConfigExport } from 'vite';
import vue from '@vitejs/plugin-vue';
import topLevelAwait from 'vite-plugin-top-level-await';
// eslint-disable-next-line no-relative-import-paths/no-relative-import-paths
import wasmPack from './scripts/vite_plugin_wasm_pack';
import os from 'os';

const plugins: PluginOption[] = [vue(), topLevelAwait()];
let platform: string;

// Vite only expose in `import.meta.env` the environ variables with a `VITE_` prefix,
// however testbed server url must also be configured in Cypress (you guessed it: where
// only `CYPRESS_` prefixed variables are exposed).
// So we want the user to only have to set `TESTBED_SERVER_URL` for both Vite and Cypress.
if (process.env.VITE_TESTBED_SERVER_URL || process.env.TESTBED_SERVER_URL) {
  // Why this if guard ? Guess what kiddo !
  // Setting `process.env.VITE_TESTBED_SERVER_URL = undefined` got chewed up
  // in the web page into "undefined" string...
  process.env.VITE_TESTBED_SERVER_URL = process.env.VITE_TESTBED_SERVER_URL || process.env.TESTBED_SERVER_URL;
}
if (process.env.VITE_APP_TEST_MODE || process.env.APP_TEST_MODE) {
  process.env.VITE_APP_TEST_MODE = process.env.VITE_APP_TEST_MODE || process.env.APP_TEST_MODE;
}

// 1) Detect for what platform we are building for (web or native)

if (process.env.PLATFORM !== undefined) {
  console.log(`PLATFORM environ set to \`${process.env.PLATFORM}\``);
  if (process.env.PLATFORM === 'web') {
    platform = 'web';
  } else if (process.env.PLATFORM === 'native') {
    platform = 'native';
  } else {
    throw new Error('Invalid value for PLATFORM environ variable, accepted values: `web`/`native`');
  }

} else {
  // Ain't nobody got time to set environ variable !
  console.log('PLATFORM environ variable not set, defaulting to `web`');
  platform = 'web';
}

// Store the OS in an env variable
process.env.VITE_OPERATING_SYSTEM = process.platform;
// Store the default directories to env variables
if (platform !== 'web') {
  process.env.VITE_HOME_DIR = os.homedir();
  if (process.platform === 'win32') {
    process.env.VITE_CONFIG_DIR = process.env.APPDATA;
    process.env.VITE_DATA_DIR = process.env.APPDATA;
  } else if (process.platform === 'linux') {
    process.env.VITE_CONFIG_DIR = process.env.XDG_CONFIG_HOME || path.join(process.env.HOME || '', '.config');
    process.env.VITE_DATA_DIR = process.env.XDG_DATA_HOME || path.join(process.env.HOME || '', '.local/share');
  } else {
    process.env.VITE_CONFIG_DIR = '';
    process.env.VITE_DATA_DIR = '';
  }
} else {
  process.env.VITE_CONFIG_DIR = '';
  process.env.VITE_DATA_DIR = '';
  process.env.VITE_HOME_DIR = '';
}

// 2) Add the packaging of the Wasm stuff if the platform requires it

if (platform === 'web') {
  plugins.push(wasmPack([{path: '../bindings/web/', name: 'libparsec_bindings_web'}]));
}

// 3) Finally configure Vite

// https://vitejs.dev/config/
const config: UserConfigExport = () => ({
  test: {
    globals: true,
  },
  plugins: plugins,
  build: {
    sourcemap: true,
  },
  resolve: {
    alias: {
      '@libparsec_trampoline': path.resolve(__dirname, `./src/plugins/libparsec/trampoline-${platform}.ts`),
      '@': path.resolve(__dirname, './src'),
    },
  },
  server: {
    port: 8080,
    hmr: true,
  },
});

export default defineConfig(config);

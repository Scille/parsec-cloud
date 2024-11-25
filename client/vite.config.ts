// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
// eslint-disable-next-line spaced-comment
/// <reference types="vitest" />

import { sentryVitePlugin } from '@sentry/vite-plugin';
import vue from '@vitejs/plugin-vue';
import path from 'path';
import { defineConfig, loadEnv, PluginOption, UserConfigExport } from 'vite';
import topLevelAwait from 'vite-plugin-top-level-await';
// eslint-disable-next-line no-relative-import-paths/no-relative-import-paths
import wasmPack from './scripts/vite_plugin_wasm_pack';

const plugins: PluginOption[] = [vue(), topLevelAwait()];
let platform: string;

// Vite only expose in `import.meta.env` the environ variables with a `PARSEC_APP_` prefix,
// but to make it a bit easier, we're also letting the user provide TESTBED_SERVER
if (process.env.PARSEC_APP_TESTBED_SERVER || process.env.TESTBED_SERVER) {
  // Why this if guard ? Guess what kiddo !
  // Setting `process.env.PARSEC_APP_TESTBED_SERVER = undefined` got chewed up
  // in the web page into "undefined" string...
  process.env.PARSEC_APP_TESTBED_SERVER = process.env.PARSEC_APP_TESTBED_SERVER || process.env.TESTBED_SERVER;
}
if (process.env.PARSEC_APP_TEST_MODE || process.env.APP_TEST_MODE) {
  process.env.PARSEC_APP_TEST_MODE = process.env.PARSEC_APP_TEST_MODE || process.env.APP_TEST_MODE;
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

// 2) Add the packaging of the Wasm stuff if the platform requires it

if (platform === 'web') {
  plugins.push(wasmPack([{ path: '../bindings/web/', name: 'libparsec_bindings_web' }]));
}

if (process.env.PARSEC_APP_SENTRY_AUTH_TOKEN) {
  const sentryPlugin = sentryVitePlugin({
    org: 'scille',
    project: 'parsec3-frontend',
    authToken: process.env.PARSEC_APP_SENTRY_AUTH_TOKEN,
  });
  plugins.push(sentryPlugin);
} else {
  console.log('PARSEC_APP_SENTRY_AUTH_TOKEN is not set');
}

// 3) Finally configure Vite

// https://vitejs.dev/config/
const config: UserConfigExport = () => ({
  test: {
    include: ['tests/unit/specs/*.spec.ts'],
    setupFiles: [path.resolve(__dirname, './tests/component/support/setup.ts')],
    server: {
      deps: {
        inline: ['megashark-lib'],
      },
    },
    env: loadEnv('development', process.cwd(), ''),
    globals: true,
    alias: {
      '@libparsec_trampoline': path.resolve(__dirname, `./src/plugins/libparsec/trampoline-${platform}.ts`),
      '@': path.resolve(__dirname, './src'),
      '@tests': path.resolve(__dirname, './tests'),
    },
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
  define: {
    // Defined values are replaced has is in the code, here we need the version to contain quotes in the value
    // to be a string once replaced in the code.
    __APP_VERSION__: JSON.stringify(process.env.npm_package_version),
  },
  envPrefix: 'PARSEC_APP_',
  server: {
    port: 8080,
    hmr: true,
  },
  worker: {
    format: 'es',
  },
});

export default defineConfig(config);

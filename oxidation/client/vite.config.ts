import path from 'path';
import fs from 'fs';
import { defineConfig, PluginOption, UserConfigExport } from 'vite';
import vue from '@vitejs/plugin-vue';
import topLevelAwait from 'vite-plugin-top-level-await';
import wasmPack from './scripts/vite_plugin_wasm_pack';

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

// 2) Now that we know the platform, we can create `trampoline.ts` accordingly

const libparsecPluginPath = path.join(__dirname, 'src/plugins/libparsec/');
const src = path.join(libparsecPluginPath, `trampoline-${platform}.ts.in`);
const dst = path.join(libparsecPluginPath, 'trampoline.ts');

try {
  // `readFileSync` will throw an error if `dst` doesn't exist
  if (Buffer.compare(fs.readFileSync(src), fs.readFileSync(dst))) {
    throw new Error('Outdated `trampoline.ts`');
  }
} catch (error) {
  // `trampoline.ts` doesn't exist or is outdated, overwrite it
  console.log(`Copy ${src} -> ${dst}`);
  fs.copyFileSync(src, dst);
}

// 3) Add the packaging of the Wasm stuff if the platform requires it

if (platform === 'web') {
  plugins.push(wasmPack([{path: '../bindings/web/', name: 'libparsec_bindings_web'}]));
}

// 4) Finally configure Vite

// https://vitejs.dev/config/
const config: UserConfigExport = {
  plugins: plugins,
  build: {
    sourcemap: true
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src')
    }
  }
};

export default defineConfig(config);

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
// eslint-disable-next-line spaced-comment
/// <reference types="vitest" />

import { sentryVitePlugin } from '@sentry/vite-plugin';
import vue from '@vitejs/plugin-vue';
import fs from 'fs';
import path from 'path';
import { promisify } from 'util';
import { ConfigEnv, defineConfig, loadEnv, PluginOption, UserConfigFnObject } from 'vite';
import { viteStaticCopy } from 'vite-plugin-static-copy';
import { brotliCompress } from 'zlib';
// eslint-disable-next-line no-relative-import-paths/no-relative-import-paths
import wasmPack from './scripts/vite_plugin_wasm_pack';

// Vue hoists static template parts into module-level constants, created once at import time
// regardless of whether the component ever mounts. That's invisible in normal use, but it means
// V8 coverage would count a component's template as "covered" just by being imported. Disable it
// for coverage runs so template code only counts as covered once the component actually renders.
const vueOptions = process.env.COLLECT_COVERAGE === 'true' ? { template: { compilerOptions: { hoistStatic: false } } } : {};
const plugins: PluginOption[] = [vue(vueOptions)];
// Web workers are packaged separately and rely on their own set of plugins.
//
// This is because each worker makes additional calls to callbacks that would otherwise
// be expected to be used only once (e.g. the copy of the .wasm file during build).
//
// And for a similar reason (not messing up with the plugin's internal state&cache),
// we must instantiate new instances of the plugins for each worker.
//
// see:
// - https://vite.dev/config/worker-options.html
// - https://github.com/vitejs/vite/pull/6243#issuecomment-1001244758
const workerPluginsFactories: (() => PluginOption)[] = [
  (): PluginOption => {
    return vue();
  },
];
let platform: string;

// Vite only expose in `import.meta.env` the environment variables with a `PARSEC_APP_` prefix,
// but to make it a bit easier, we're also letting the user provide TESTBED_SERVER
if (process.env.PARSEC_APP_TESTBED_SERVER || process.env.TESTBED_SERVER) {
  // Why this if guard? Guess what kiddo!
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
  // Ain't nobody got time to set environment variable !
  console.log('PLATFORM environ variable not set, defaulting to `web`');
  platform = 'web';
}

// 2) Add the packaging of the Wasm stuff if the platform requires it

if (platform === 'web') {
  // In release mode, main plugin is responsible for copying the Wasm file to the dist folder
  plugins.push(wasmPack([{ path: '../bindings/web/', name: 'libparsec_bindings_web' }], true));
  workerPluginsFactories.push((): PluginOption => {
    return wasmPack([{ path: '../bindings/web/', name: 'libparsec_bindings_web' }], false);
  });
}

if (process.env.PARSEC_APP_SENTRY_AUTH_TOKEN) {
  const sentryPluginFactory = (): PluginOption => {
    return sentryVitePlugin({
      org: 'scille',
      project: 'parsec3-frontend',
      authToken: process.env.PARSEC_APP_SENTRY_AUTH_TOKEN,
    });
  };
  plugins.push(sentryPluginFactory());
  workerPluginsFactories.push(sentryPluginFactory);
} else {
  console.log('PARSEC_APP_SENTRY_AUTH_TOKEN is not set');
}

// 3) Add dev specific plugins
if (process.env.NODE_ENV === 'development' && !process.env.CI) {
  // const { default: basicSsl } = await import('@vitejs/plugin-basic-ssl');
  // plugins.push(basicSsl());
}

plugins.push(
  viteStaticCopy({
    targets: [
      {
        src: 'node_modules/pdfjs-dist/wasm/*',
        dest: 'pdfjs',
      },
    ],
  }),
);

// 4) Pre-compress built assets with Brotli

const brotliCompressAsync = promisify(brotliCompress);
const BROTLI_COMPRESSIBLE = /\.(js|css|html|svg|wasm)$/;

if (platform === 'web') {
  plugins.push({
    name: 'brotli-compress',
    apply: 'build',
    async closeBundle() {
      async function compressDir(dir: string): Promise<void> {
        const entries = await fs.promises.readdir(dir, { withFileTypes: true });
        await Promise.all(
          entries.map(async (entry) => {
            const fullPath = path.join(dir, entry.name);
            if (entry.isDirectory()) {
              await compressDir(fullPath);
            } else if (BROTLI_COMPRESSIBLE.test(entry.name)) {
              const content = await fs.promises.readFile(fullPath);
              const compressed = await brotliCompressAsync(content);
              await fs.promises.writeFile(`${fullPath}.br`, compressed);
            }
          }),
        );
      }
      await compressDir(path.resolve(__dirname, 'dist'));
    },
  });
}

// 5) Finally configure Vite

// scss additionalData is used to inject the theme variables in SCSS files imported in main.ts & .vue files
// for SCSS files imported with sass @use / @forward methods, you need to add manually the theme import
const additionalData = '@use "megashark-lib/theme" as ms;';

// https://vitejs.dev/config/
const config: UserConfigFnObject = (_env: ConfigEnv) => ({
  css: {
    preprocessorOptions: {
      scss: {
        additionalData: additionalData,
      },
    },
  },
  // Since we do not know in advance how the webapp will be hosted we use a relative base href.
  // That way the app can be at the root or in a sub-folder and limit the need to specific build.
  base: process.env.BASE_URL || './',
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
    fakeTimers: {
      toFake: ['setTimeout', 'clearTimeout', 'Date'],
    },
  },
  plugins: [
    ...plugins,
    {
      // Serve `dist/custom/*` at `/custom/*` in dev mode.
      // This folder is optional and may not be present.
      name: 'serve-dist-custom',
      configureServer(server) {
        server.middlewares.use('/custom', async (req, res, _next) => {
          const filePath = path.join(__dirname, 'dist/custom', req.url ?? '/');
          try {
            res.end(await fs.promises.readFile(filePath));
          } catch {
            res.statusCode = 404;
            res.end();
          }
        });
      },
    },
  ],
  build: {
    sourcemap: true,
    // For native, Electron 41 ships Chrome 146 (see https://www.electronjs.org/blog/electron-41-0).
    // For web, we pin these minimums so the build target is explicit and stable across Vite versions.
    // See https://oxc.rs/docs/guide/usage/transformer/lowering#target for allowed target values.
    target: platform === 'native' ? 'chrome146' : ['chrome107', 'edge107', 'firefox104', 'safari16'],
    minify: 'esbuild',
    cssCodeSplit: false,
    rolldownOptions: {
      output: {
        codeSplitting: false,
      },
    },
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
    plugins: (): PluginOption[] => {
      return workerPluginsFactories.map((factory) => factory());
    },
  },
});

export default defineConfig(config);

/// <reference types="vitest" />

import { fileURLToPath, URL } from 'url'
import path from 'path'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'

// https://vitejs.dev/config/
let config: UserConfigExport = {
  plugins: [
    vue(),
    wasm(),
    topLevelAwait()
  ],
  test: {
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src")
    },
  },
  // Fixed import problem on Windows, see https://github.com/Menci/vite-plugin-wasm/issues/26 and remove when this has been patched
  optimizeDeps: {
    disabled: true
  }
};

if (process.env.VITEST) {
  config.resolve.alias['@ionic/vue'] = fileURLToPath(new URL("./node_modules/@ionic/vue/dist/index.esm", import.meta.url));
}

export default defineConfig(config);

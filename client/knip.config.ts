// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import type { KnipConfig } from 'knip';

// Knip configuration
const config = {
  tags: ['-lintignore'],
  // For knip, a workspace is essentially a directory containing a package.json.
  // Knip reads them from the "workspaces" array in package.json but since we
  // currently do not specify it there, we must do it here in the knip config.
  // See: https://knip.dev/features/monorepos-and-workspaces#workspaces
  workspaces: {
    // Refers to client/package.json
    '.': {
      // Add entry files not added by knip's default entry file patterns
      // See: https://knip.dev/explanations/entry-files#default-entry-file-patterns
      entry: [
        'merge-playwright.ts',
        'src/parsec/types.ts',
        'src/theme/components/index.scss',
        // Editics host page is not part of the app's import graph since it is loaded in an iframe.
        'editics/offline.ts',
      ],
    },
    // Refers to client/electron/package.json
    electron: {
      // Add entry files not added by knip's default entry file patterns
      // See: https://knip.dev/explanations/entry-files#default-entry-file-patterns
      entry: ['assets/electron-publisher-custom.js', 'scripts/before-pack.cjs', 'src/preload.ts', 'src/setup.ts', 'src/winRegistry.ts'],
    },
  },
  // Exclude the following checks from the report
  // Too many "unused [exports|exported types|exported enum members]" issues
  // See: https://knip.dev/reference/issue-types
  exclude: ['enumMembers', 'exports', 'types'],
  // Exclude files reported as unused
  ignoreFiles: [
    // To create empty ODS files (currently unused)
    'src/parsec/file_templates/ods_template.ts',
    // To create empty ODT files (currently unused)
    'src/parsec/file_templates/odt_template.ts',
    // Native builds use it
    'src/plugins/libparsec/trampoline-native.ts',
    // Not used but it's a generic thing, we can keep it
    'src/services/screenshot.ts',
    // Only used when we need to measure some things
    'src/services/performanceMonitor.ts',
    // Worker, fetched dynamically at startup
    'public/file-service-worker.js',
    // loaded by the file service worker with `importScripts()`
    'public/vendor/zip-native.min.js',
  ],
  // Exclude dependencies reported as unused
  ignoreDependencies: [
    // Referenced through a vite plugin, not imported directly
    'onlyoffice-x2t',
    // used during signature of electron artifact for macOS
    '@electron/notarize',
    // an electron-builder utility, only used in partial imports for typing
    'app-builder-lib',
    // only used in electron/assets
    'electron-publish',
    // imported dynamically and only on Windows
    'regedit',
    // pin version to avoid a bug on snap
    'vue-router',
  ],
} satisfies KnipConfig;

export default config;

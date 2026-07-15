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
      entry: ['merge-playwright.ts', 'src/parsec/types.ts', 'src/theme/components/index.scss'],
    },
    // Refers to client/electron/package.json
    electron: {
      // Add entry files not added by knip's default entry file patterns
      // See: https://knip.dev/explanations/entry-files#default-entry-file-patterns
      entry: [
        'assets/electron-publisher-custom.js',
        'scripts/before-pack.cjs',
        'src/index.ts',
        'src/preload.ts',
        'src/setup.ts',
        'src/winRegistry.ts',
      ],
    },
  },
  // Exclude the following checks from the report
  // Too many "unused [exports|exported types|exported enum members]" issues
  // See: https://knip.dev/reference/issue-types
  exclude: ['enumMembers', 'exports', 'types'],
  // Exclude files reported as unused
  // TODO: check if still needed
  ignoreFiles: [
    'src/common/mocks.ts',
    'src/components/files/FileOpenFallbackChoice.vue',
    'src/components/organizations/ChooseServer.vue',
    'src/parsec/file_templates/ods_template.ts',
    'src/parsec/file_templates/odt_template.ts',
    'src/parsec/mock_generator.ts',
    'src/plugins/libparsec/trampoline-native.ts',
    'src/services/screenshot.ts',
    'src/services/smallDisplayWarning.ts',
    'src/views/about/ChangesModal.vue',
    'src/views/home/SummaryStep.vue',
    'src/services/performanceMonitor.ts',
    'public/streaming-worker.js',
  ],
  // Exclude dependencies reported as unused
  ignoreDependencies: [
    // installed by the `megashark:install` script instead of being listed under
    // `dependencies`, see `//megasharkLibSource` in `package.json`
    'megashark-lib',
    // used during signature of electron artifact for macOS
    '@electron/notarize',
    // an electron-builder utility, only used in partial imports for typing
    'app-builder-lib',
    // only used in electron/assets
    'electron-publish',
    // imported dynamically and only on Windows
    'regedit',
    // commented most of the time, but can be useful in dev
    '@vitejs/plugin-basic-ssl',
  ],
  ignoreBinaries: [
    // TODO: Investigate why is reported as unlisted binary in the CI
    'electron',
  ],
} satisfies KnipConfig;

export default config;

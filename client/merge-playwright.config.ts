// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import type { ReporterDescription } from '@playwright/test';

const CI_REPORTERS: ReporterDescription[] = [['github'], ['json', { outputFile: 'test-results.json' }], ['html', { open: 'never' }]];

export default {
  testDir: './tests/e2e',
  reporter: CI_REPORTERS,
};

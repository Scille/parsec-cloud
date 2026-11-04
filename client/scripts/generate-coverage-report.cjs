// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Run after a `COLLECT_COVERAGE=true` Playwright run to turn the raw per-worker
// coverage data accumulated under `coverage/.cache` into an HTML report.
const MCR = require('monocart-coverage-reports');
const coverageOptions = require('../mcr.config.cjs');

(async () => {
  const mcr = MCR(coverageOptions);
  await mcr.generate();
})();

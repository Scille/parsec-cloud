// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Shared by both the Playwright fixture (which calls `.add()` per test, once per worker)
// and `scripts/generate-coverage-report.cjs` (which calls `.generate()` once all workers are done).
// Both need the exact same `outputDir` so the report generator picks up every worker's cached data.
module.exports = {
  name: 'Parsec web e2e coverage',
  outputDir: './coverage',
  reports: ['v8', 'console-summary'],
  filter: {
    // Entry scripts served straight from node_modules (e.g. Vite's dep pre-bundles), and any
    // source files that sourcemaps unpack into node_modules.
    '**/node_modules/**': false,
    // Vue's style-injection module for a component's <style> block. It has no setup()/render()
    // gate: it runs unconditionally the moment the component is imported, regardless of whether
    // the component ever mounts, so it can never carry a "was this tested" signal.
    '**/*type=style*': false,
    '**/**': true,
  },
  // Vue components always compile to a top-level module scope plus a separate `_sfc_render`
  // function (the compiled <template>). Import side effects (style injection, render-helper
  // imports) make the module scope show as "covered" just by being imported, even if the
  // component itself was never mounted. When `_sfc_render` was never called, the component
  // wasn't actually used, so we zero out the module-scope range too rather than let an
  // imprecise sourcemap position (Vue templates aren't JS, so mapping is approximate) report
  // stray template lines as covered.
  onEntry: (entry) => {
    const render = entry.functions?.find((fn) => fn.functionName === '_sfc_render');
    const topLevel = entry.functions?.find((fn) => !fn.functionName);
    if (render && topLevel && render.ranges[0]?.count === 0) {
      for (const range of topLevel.ranges) {
        range.count = 0;
      }
    }
  },
};

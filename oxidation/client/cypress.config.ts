import { defineConfig } from 'cypress';

export default defineConfig({
  e2e: {
    specPattern: 'tests/e2e/specs/**.js',
    fixturesFolder: 'tests/e2e/fixtures',
    screenshotsFolder: 'tests/e2e/screenshots',
    videosFolder: 'tests/e2e/videos',
    supportFile: 'tests/e2e/support/index.js'
  }
});

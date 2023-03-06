import { defineConfig } from 'cypress';
import vitePreprocessor from 'cypress-vite';

// Cypress only expose in `Cypress.env()` the environ variables with a `CYPRESS_` prefix,
// however testbed server url must also be configured in Vite (you guessed it: where
// only `VITE_` prefixed variables are exposed).
// So we want the user to only have to set `TESTBED_SERVER_URL` for both Vite and Cypress.
process.env.CYPRESS_TESTBED_SERVER_URL = process.env.CYPRESS_TESTBED_SERVER_URL || process.env.TESTBED_SERVER_URL;

// To properly run, cypress needs two informations:
// - CYPRESS_BASE_URL: the address of the web server serving our application
// - TESTBED_SERVER_URL: the address of the testbed server for mocking Parsec server

const DEFAULT_BASE_URL = 'http://localhost:5173/';  // Vite's dev server
if (!process.env.CYPRESS_BASE_URL) {
  console.log(`\`CYPRESS_BASE_URL\` not set, defaulting to \`${DEFAULT_BASE_URL}\``);
  process.env.CYPRESS_BASE_URL = DEFAULT_BASE_URL;
}
const BASE_URL = process.env.CYPRESS_BASE_URL;
process.env.CYPRESS_BASE_URL = undefined;  // Clear variable to prevent Cypress from doing voodoo on it

const DEFAULT_TESTBED_SERVER_URL = 'http://localhost:6770/';  // Default port in run_testbed_server.py script
if (!process.env.CYPRESS_TESTBED_SERVER_URL) {
  console.log(`\`TESTBED_SERVER_URL\` not set, defaulting to \`${DEFAULT_TESTBED_SERVER_URL}\``);
  process.env.CYPRESS_TESTBED_SERVER_URL = DEFAULT_TESTBED_SERVER_URL;
}
const TESTBED_SERVER_URL = process.env.CYPRESS_TESTBED_SERVER_URL;
process.env.CYPRESS_TESTBED_SERVER_URL = undefined;  // Clear variable to prevent Cypress from doing voodoo on it

export default defineConfig({

  e2e: {
    setupNodeEvents(on) {
      on('file:preprocessor', vitePreprocessor('./vite.config.ts'));
    },
    // Note `CYPRESS_BASE_URL` is automatically used by Cypress to configure `baseUrl`,
    // so here we are just making things explicit to help the reader.
    baseUrl: BASE_URL,
    // Similarly, any env variable with `CYPRESS_` prefix got dumped in `env`, so we could
    // use `CYPRESS_TESTBED_SERVER_URL` instead of `TESTBED_SERVER_URL`. But, again, Js
    // ecosystem already contains more magic than Hogwarts.
    env: {
      TESTBED_SERVER_URL: TESTBED_SERVER_URL
    },
    screenshotsFolder: 'tests/e2e/screenshots',
    videosFolder: 'tests/e2e/videos',
    screenshotOnRunFailure: false,
    video: false,
    supportFile: 'tests/e2e/support/index.ts',
    specPattern: 'tests/e2e/specs/**.ts',
    fixturesFolder: 'tests/e2e/fixtures'
  },

  component: {
    screenshotsFolder: 'tests/component/screenshots',
    videosFolder: 'tests/component/videos',
    screenshotOnRunFailure: false,
    video: false,
    supportFile: 'tests/component/support/index.ts',
    specPattern: 'tests/component/specs/**.ts',
    indexHtmlFile: 'tests/component/support/component-index.html',
    devServer: {
      framework: 'vue',
      bundler: 'vite'
    }
  }

});

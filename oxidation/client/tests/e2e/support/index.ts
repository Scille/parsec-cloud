// ***********************************************************
// This example support/index.js is processed and
// loaded automatically before your test files.
//
// This is a great place to put global configuration and
// behavior that modifies Cypress.
//
// You can change the location of this file or turn off
// automatically serving support files with the
// 'supportFile' configuration option.
//
// You can read more here:
// https://on.cypress.io/configuration
// ***********************************************************

// /!\ Don't import application stuff to access singletons /!\
// (e.g. `import { libparsec } from '@/plugins/libparsec'`)
// instead you should use the `window` object. This is because the imports you do here
// got inlined when Cypress compile the test spec, hence making it separate from
// the actual application code (and hence singletons would be defined twice).

// Without this line the typing fail. Go on, remove it, make my day ;-)
// The issue is that Typescript consider a file to be either a script or a module,
// and only module are allowed to augment the global scope with additional typing.
// In the long "implicit clusterfuck is better than explicit" Js tradition,
// Typescript determines the file is a module if it contains import/export,
// but here have nothing to import/export... hence this dummy export.
export {};

declare global {
// eslint-disable-next-line @typescript-eslint/no-namespace
  namespace Cypress {
    interface Chainable {
      visitApp(template: 'coolorg' | 'empty'): Chainable<string>
      dropTestbed(): Chainable<null>
    }
  }
}

Cypress.Commands.add('visitApp', (template) => {
  const TESTBED_SERVER_URL = Cypress.env('TESTBED_SERVER_URL');
  assert.isDefined(TESTBED_SERVER_URL, 'Environ variable `TESTBED_SERVER_URL` must be defined to use testbed');

  cy.visit('/', {
    onBeforeLoad(win) {
      // Intercept console logs
      cy.stub(win.console, 'log').as('consoleLog');
      cy.stub(win.console, 'error').as('consoleError');
    }
  })
    .as('window')
    // Wait App to be in the first stage of the init
    .get('#app[app-state="waiting-for-config-path"]')
    .get('@window')
    .then(async (windowElem) => {
      // Type cast because Cypress expects `get` to only return JQuery
      const window = windowElem as unknown as Window;
      const [libparsec, nextStage] = window.nextStageHook();
      const configPath = await libparsec.testNewTestbed(template, TESTBED_SERVER_URL);
      assert.isDefined(configPath);
      await nextStage(configPath);
      return configPath;
    })
    .as('configPath')
    // Return the Window object (type cast because Cypress expects `get` to only return JQuery)
    .get('@window') as unknown as Cypress.Chainable<Window>;
});

Cypress.Commands.add('dropTestbed', () => {
  cy.window().then((window) => {
    cy.get('@configPath').then(async (configPath) => {
      // Type cast because Cypress expects `get` to only return JQuery
      await window.libparsec.testDropTestbed(configPath as unknown as string);
    });
  });
});

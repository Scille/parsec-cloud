// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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

import 'cypress-file-upload';
import 'cypress-real-events/support';

declare global {
  // eslint-disable-next-line @typescript-eslint/no-namespace
  namespace Cypress {
    interface Chainable {
      visitApp(template?: 'coolorg' | 'empty'): Chainable<string>;
      dropTestbed(): Chainable<null>;
      login(userName: string, password: string): Chainable<null>;
      checkToastMessage(level: 'error' | 'warning' | 'info' | 'success', message: string | RegExp): Chainable<null>;
    }
  }
}

Cypress.Commands.add('visitApp', (template = 'coolorg') => {
  const TESTBED_SERVER_URL = Cypress.env('TESTBED_SERVER_URL');
  assert.isDefined(TESTBED_SERVER_URL, 'Environ variable `TESTBED_SERVER_URL` must be defined to use testbed');
  // If the variable is not defined, Cypress gets it as the string "undefined" instead of the value. So we check that also.
  assert.notStrictEqual(TESTBED_SERVER_URL, 'undefined', 'Environ variable `TESTBED_SERVER_URL` must be defined to use testbed');

  cy
    .visit('/', {
      onBeforeLoad(win) {
        // Intercept console logs
        cy.stub(win.console, 'log').as('consoleLog');
        cy.stub(win.console, 'error').as('consoleError');
      },
    })
    .as('window')
    // Wait App to be in the first stage of the init
    .get('#app[app-state="initializing"]', { timeout: 10000 })
    .get('@window')
    .then(async (windowElem) => {
      // Type cast because Cypress expects `get` to only return JQuery
      const window = windowElem as unknown as Window;

      // Cypress can clear local storage but not indexedDB
      window.indexedDB.deleteDatabase('_ionicstorage');

      const [libparsec, nextStage] = window.nextStageHook();
      const configResult = await libparsec.testNewTestbed(template, TESTBED_SERVER_URL);
      if (!configResult.ok) {
        throw new Error('Failed to init testbed');
      }
      const configPath = configResult.value;
      assert.isDefined(configPath);
      // Force locale to en-US
      await nextStage(configPath, 'en-US');
      return configPath;
    })
    .as('configPath')
    // Return the Window object (type cast because Cypress expects `get` to only return JQuery)
    .get('@window') as unknown as Cypress.Chainable<Window>;
});

Cypress.Commands.add('login', (userName, password) => {
  cy.get('.organization-list').contains(userName).click();
  cy.get('#ms-password-input').find('input').type(password, { delay: 0 });
  cy.get('.login-button').click();
  cy.wait(200);
  cy.url().should('include', '/workspaces');
  cy.get('.topbar-left').contains('My workspaces');
});

Cypress.Commands.add('dropTestbed', () => {
  cy.window().then(async (window) => {
    const configPath = window.getConfigDir();
    await window.libparsec.testDropTestbed(configPath as unknown as string);
  });
});

Cypress.Commands.add('checkToastMessage', (level, message: string | RegExp) => {
  cy.get('.notification-toast').should('have.class', `ms-${level}`);
  cy.get('.notification-toast').shadow().find('.toast-message').contains(message);
  cy.get('.notification-toast').shadow().find('.toast-button-confirm').first().click();
});

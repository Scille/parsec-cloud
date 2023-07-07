// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check organization list', () => {

  beforeEach(() => {
    cy.visitApp();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Visit the app root url', () => {
    cy.contains('Your organizations');
    cy.get('.organization-list-row__col').should('have.length', 5);
  });

  it('Go to login page and back to organizations', () => {
    cy.get('.organization-list-row__col').should('have.length', 5);
    cy.get('.login-button-container').should('not.exist');
    cy.contains('Boby McBobFace').click();
    cy.get('.login-card').contains('Org');
    cy.get('.login-button-container').should('exist');
    cy.contains('Return to organizations').click();
    cy.contains('Your organizations');
    cy.get('.organization-list-row__col').should('have.length', 5);
  });

  it('Go to login page and enter password', () => {
    cy.contains('Boby McBobFace').click();
    cy.get('.login-button-container > ion-button').should('have.class', 'button-disabled');
    cy.get('#password-input').find('input').invoke('attr', 'type').should('eq', 'password');
    cy.get('#password-input').find('input').type('P@ssw0rd');
    cy.get('.login-button-container > ion-button').should('not.have.class', 'button-disabled');
    cy.get('.login-button-container > ion-button').click();
    cy.get('@configPath').then(($elem) => {
      const orgId = ($elem as unknown as string).split('/').slice(-1)[0];
      cy.get('@consoleLog').should('have.been.calledWith', `Log in to ${orgId} with password "P@ssw0rd"`);
    });
    cy.contains('My workspaces');
  });

  it('Go to login page and sort and filter orgs', () => {
    cy.get('.organization-list-row__col').as('orgList').should('have.length', 5);
    // Sorted by org name asc by default
    cy.get('@orgList').first().contains('Alicey McAliceFace');
    cy.get('@orgList').last().contains('mallory');
    cy.get('#search-input').find('input').type('alice');
    cy.get('@orgList').should('have.length', 2);
    // Only 2 devices shown
    cy.get('@orgList').first().contains('Alicey McAliceFace');
    cy.get('@orgList').last().contains('Alicey McAliceFace');
    cy.get('#search-input').find('input').clear();
    cy.get('@orgList').should('have.length', 5);
    // Change sort order
    cy.get('#organization-filter-select').contains('Organization').click();
    cy.get('.option').should('have.length', 4);
    cy.get('.option').eq(2).contains('User Name').click();
    cy.get('#organization-filter-select').contains('User Name').click();
    cy.get('.option').should('have.length', 4);
    cy.get('.option').first().contains('Ascending').click();
    // Now sorted by user name desc
    cy.get('@orgList').first().contains('mallory');
    cy.get('@orgList').last().contains('Alicey McAliceFace');
  });

  it('Open create organization dialog', () => {
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').contains('I want to create an organization');
  });

  it('Open join organization dialog', () => {
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').contains('I received a Parsec Cloud invitation');
  });

  it('Log into organization with command', () => {
    // Uses Cypress command to simplify the log in part
    cy.login('Boby', 'P@ssw0rd');
    cy.contains('My workspaces');
  });

  it('Log out', () => {
    cy.login('Boby', 'P@ssw0rd');
    cy.contains('My workspaces');
    cy.get('#profile-button').click();
    cy.get('.popover-viewport').contains('Log out').click();
    cy.contains('Your organizations');
  });
});

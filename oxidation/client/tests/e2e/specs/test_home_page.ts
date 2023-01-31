// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// No idea why this is required. Without it, Cypress
// stops with an error saying that there's an error in
// the code, without any more details.

// eslint-disable-next-line @typescript-eslint/no-unused-vars
Cypress.on('uncaught:exception', (err, runnable) => {
  return false;
});

describe('Check organization list', () => {

  it('Visit the app root url', () => {
    cy.visit('/');
    cy.contains('List of your organizations');
    cy.get('.organization-card-container').should('have.length', 4);
  });

  it('Go to login page and back to organizations', () => {
    cy.visit('/');
    cy.get('#login-button-container').should('not.exist');
    cy.contains('Black Mesa').click();
    cy.get('.organization-card').contains('Black Mesa');
    cy.get('#login-button-container').should('exist');
    cy.contains('Return to organizations').click();
    cy.contains('List of your organizations');
  });

  it('Go to login page and enter password', () => {
    cy.visit('/');
    cy.contains('Black Mesa').click();
    cy.get('#login-button-container > ion-button').should('have.class', 'button-disabled');
    cy.get('ion-input > input').invoke('attr', 'type').should('eq', 'password');
    cy.get('ion-input > input').type('P@ssw0rd');
    cy.get('#login-button-container > ion-button').should('not.have.class', 'button-disabled');
    cy.get('#login-button-container > ion-button').click();
  });

  it('Go to login page and sort and filter orgs', () => {
    cy.visit('/');
    cy.get('.organization-card-container').should('have.length', 4);
    // Sorted by org name asc by default
    cy.get('.organization-card-container').first().contains('Black Mesa');
    cy.get('.organization-card-container').last().contains('PPTH');
    cy.get('#search-input > input').type('la');
    cy.get('.organization-card-container').should('have.length', 2);
    // Only two orgs shown
    cy.get('.organization-card-container').first().contains('Black Mesa');
    cy.get('.organization-card-container').last().contains('Planet Express');
    cy.get('#search-input > input').clear();
    cy.get('.organization-card-container').should('have.length', 4);
    // Change sort order
    cy.get('#filter-select').contains('Organization').click();
    cy.get('.option').should('have.length', 4);
    cy.get('.option').first().contains('Ascending order').click();
    // Now sorted by org name desc
    cy.get('.organization-card-container').first().contains('PPTH');
    cy.get('.organization-card-container').last().contains('Black Mesa');
    // Sort by user name
    cy.get('#filter-select').contains('Organization').click();
    cy.get('.option').should('have.length', 4);
    cy.get('.option').eq(2).contains('User Name').click();
    // Now sorted by user name desc
    cy.get('#filter-select').contains('User Name').click();
    cy.get('.organization-card-container').first().contains('Octavius');
    cy.get('.organization-card-container').last().contains('Freeman');
  });

  it('Open create organization dialog', () => {
    cy.visit('/');
    cy.contains('Create an organization').click();
    cy.contains('Use the main PARSEC server');
  });

  it('Open join organization dialog', () => {
    cy.visit('/');
    cy.contains('Join an organization').click();
    cy.contains('Please enter the organization\'s URL');
  });
});

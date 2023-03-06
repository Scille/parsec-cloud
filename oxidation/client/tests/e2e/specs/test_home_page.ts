// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

describe('Check organization list', () => {

  beforeEach(() => {
    cy.visitApp('coolorg');
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Visit the app root url', () => {
    cy.contains('List of your organizations');
    cy.get('.organization-card-container').should('have.length', 3);
  });

  it('Go to login page and back to organizations', () => {
    cy.get('.organization-card-container').should('have.length', 3);
    cy.get('#login-button-container').should('not.exist');
    cy.contains('Boby McBobFace').click();
    cy.get('.organization-card').contains('Org');
    cy.get('#login-button-container').should('exist');
    cy.contains('Return to organizations').click();
    cy.contains('List of your organizations');
    cy.get('.organization-card-container').should('have.length', 3);
  });

  it('Go to login page and enter password', () => {
    cy.contains('Boby McBobFace').click();
    cy.get('#login-button-container > ion-button').should('have.class', 'button-disabled');
    cy.get('ion-input > input').invoke('attr', 'type').should('eq', 'password');
    cy.get('ion-input > input').type('P@ssw0rd');
    cy.get('#login-button-container > ion-button').should('not.have.class', 'button-disabled');
    cy.get('#login-button-container > ion-button').click();
    cy.get('@configPath').then(($elem) => {
      const orgId = ($elem as unknown as string).split('/').slice(-1)[0];
      cy.get('@consoleLog').should('have.been.calledWith', `Log in to ${orgId} with password "P@ssw0rd"`);
    });
  });

  it('Go to login page and sort and filter orgs', () => {
    cy.get('.organization-card-container').should('have.length', 3);
    // Sorted by org name asc by default
    cy.get('.organization-card-container').first().contains('Alicey McAliceFace');
    cy.get('.organization-card-container').last().contains('Boby McBobFace');
    cy.get('#search-input > input').type('alice');
    cy.get('.organization-card-container').should('have.length', 2);
    // Only 2 devices shown
    cy.get('.organization-card-container').first().contains('Alicey McAliceFace');
    cy.get('.organization-card-container').last().contains('Alicey McAliceFace');
    cy.get('#search-input > input').clear();
    cy.get('.organization-card-container').should('have.length', 3);
    // Change sort order
    cy.get('#filter-select').contains('Organization').click();
    cy.get('.option').should('have.length', 4);
    cy.get('.option').eq(2).contains('User Name').click();
    cy.get('#filter-select').contains('User Name').click();
    cy.get('.option').should('have.length', 4);
    cy.get('.option').first().contains('Ascending order').click();
    // Now sorted by user name desc
    cy.get('.organization-card-container').first().contains('Boby McBobFace');
    cy.get('.organization-card-container').last().contains('Alicey McAliceFace');
  });

  it('Open create organization dialog', () => {
    cy.contains('Create an organization').click();
    cy.contains('Use the main PARSEC server');
  });

  it('Open join organization dialog', () => {
    cy.contains('Join an organization').click();
    cy.contains('Please enter the organization\'s URL');
  });
});

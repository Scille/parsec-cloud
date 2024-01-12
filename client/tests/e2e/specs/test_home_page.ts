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
    cy.get('.login-button').should('not.exist');
    cy.contains('Boby McBobFace').click();
    cy.get('.login-card').contains('Org');
    cy.get('.login-button').should('exist');
    cy.contains('Return to organizations').click();
    cy.contains('Your organizations');
    cy.get('.organization-list-row__col').should('have.length', 5);
  });

  it('Go to login page and enter wrong password', () => {
    cy.contains('Boby McBobFace').click();
    cy.get('.login-button').should('have.class', 'button-disabled');
    cy.get('#ms-password-input').find('input').invoke('attr', 'type').should('eq', 'password');
    cy.get('#ms-password-input').find('input').type('Wr0ngP@ssw0rd.');
    cy.get('.login-button').should('not.have.class', 'button-disabled');
    cy.get('.login-button').click();
    cy.get('.notification-toast').as('notificationToast').should('exist').should('have.class', 'ms-error');
    cy.get('@notificationToast').shadow().find('.toast-header').should('contain.text', 'Could not login');
    cy.get('@notificationToast').shadow().find('.toast-message').should('contain.text', 'The password is incorrect!');
  });

  it('Go to login page and enter password', () => {
    cy.contains('Boby McBobFace').click();
    cy.get('.login-button').should('have.class', 'button-disabled');
    cy.get('#ms-password-input').find('input').invoke('attr', 'type').should('eq', 'password');
    cy.get('#ms-password-input').find('input').type('P@ssw0rd.');
    cy.get('.login-button').should('not.have.class', 'button-disabled');
    cy.get('.login-button').click();
    cy.contains('My workspaces');
  });

  it('Go to login page and sort and filter orgs', () => {
    cy.get('.organization-list-row__col').as('orgList').should('have.length', 5);
    // Sorted by org name asc by default
    cy.get('@orgList').first().contains('Alicey McAliceFace');
    cy.get('@orgList').last().contains('Malloryy McMalloryFace');
    cy.get('#ms-search-input').find('input').type('alice');
    cy.get('@orgList').should('have.length', 2);
    // Only 2 devices shown
    cy.get('@orgList').first().contains('Alicey McAliceFace');
    cy.get('@orgList').last().contains('Alicey McAliceFace');
    cy.get('#ms-search-input').find('input').clear();
    cy.get('@orgList').should('have.length', 5);
    // Change sort order
    cy.get('#organization-filter-select').contains('Organization').click();
    cy.get('.option').should('have.length', 4);
    cy.get('.option').eq(2).contains('User Name').click();
    cy.get('#organization-filter-select').contains('User Name').click();
    cy.get('.option').should('have.length', 4);
    cy.get('.option').first().contains('Ascending').click();
    // Now sorted by user name desc
    cy.get('@orgList').first().contains('Malloryy McMalloryFace');
    cy.get('@orgList').last().contains('Alicey McAliceFace');
  });

  it('Open create organization dialog', () => {
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').contains('I want to create an organization');
  });

  it('Open join organization dialog', () => {
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').contains('I received an invitation to join an organization');
  });

  it('Test join org link validation', () => {
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').find('ion-item').eq(1).click();
    cy.get('.text-input-modal').find('.input-item').as('input').should('have.class', 'input-default');
    cy.get('.text-input-modal').find('#next-button').as('okButton').should('have.class', 'button-disabled');
    cy.get('.text-input-modal').find('.form-error').as('error').should('not.be.visible');

    cy.get('@input').find('input').clear();
    cy.get('@input').find('input').type('http://parsec.cloud/Test?action=claim_user&token=47265123969c4d6584c2bc15960cf212', { delay: 0 });
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('@error').should('be.visible');
    cy.get('@error').contains("Link should start with 'parsec://'");

    cy.get('@input').find('input').clear();
    cy.get('@input').find('input').type('parsec://parsec.cloud/Test?token=47265123969c4d6584c2bc15960cf212', { delay: 0 });
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('@error').should('be.visible');
    cy.get('@error').contains("Link doesn't include an action");

    cy.get('@input').find('input').clear();
    cy.get('@input')
      .find('input')
      .type('parsec://parsec.cloud/Test?action=bootstrap_organization&token=47265123969c4d6584c2bc15960cf212', { delay: 0 });
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('@error').should('be.visible');
    cy.get('@error').contains('Link contains an invalid action');

    cy.get('@input').find('input').clear();
    cy.get('@input').find('input').type('parsec://parsec.cloud/Test?action=claim_user', { delay: 0 });
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('@error').should('be.visible');
    cy.get('@error').contains("Link doesn't include a token");

    cy.get('@input').find('input').clear();
    cy.get('@input').find('input').type('parsec://parsec.cloud/Test?action=claim_user&token=abcde', { delay: 0 });
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('@error').should('be.visible');
    cy.get('@error').contains('Link contains an invalid token');

    cy.get('@input').find('input').clear();
    cy.get('@input')
      .find('input')
      .type('parsec://parsec.cloud/Test?action=claim_user&token=47265123969c4d6584c2bc15960cf212', { delay: 0 });
    cy.get('@okButton').should('not.have.class', 'button-disabled');
    cy.get('@error').should('not.be.visible');
  });

  it('Log into organization with command', () => {
    // Uses Cypress command to simplify the log in part
    cy.login('Boby', 'P@ssw0rd.');
    cy.contains('Log in');
  });

  it('Log out', () => {
    cy.login('Boby', 'P@ssw0rd.');
    cy.contains('My workspaces');
    cy.get('#profile-button').click();
    cy.get('.popover-viewport').contains('Log out').click();
    cy.contains('Your organizations');
  });
});

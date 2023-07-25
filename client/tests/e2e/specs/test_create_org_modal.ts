// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

describe('Create a new organization', () => {

  beforeEach(() => {
    cy.visit('/');
  });

  it('Open org creation modal', async () => {
    cy.get('.join-organization-modal').should('not.exist');

    // join-by-link-modal'
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').get('ion-item').should('have.length', 2);
    cy.get('.popover-viewport').get('ion-item').first().contains('Create');
    cy.get('.popover-viewport').get('ion-item').first().click();
    cy.get('.create-organization-modal').should('exist');
    cy.get('.modal-header__title').should('equal', 'Create an organization');
  });

  it('Go through the org creation process', async() => {
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').get('ion-item').first().click();
    cy.get('.modal-header__title').should('equal', 'Create an organization');

    // First page, org name
    cy.get('.modal-header__text').should('equal', '1');
    cy.get('#button-next').should('be.disabled');
    cy.get('#button-previous').should('not.be.visible');
    cy.get('#org-name-input').find('input').type('MyOrg');
    cy.get('#button-next').should('be.enabled');
    cy.get('#button-next').click();

    // Second page, user info
    cy.get('.modal-header__text').should('equal', '2');
    cy.get('#button-next').should('be.disabled');
    cy.get('#button-previous').should('be.visible');
    cy.get('ion-input').eq(0).find('input').type('Banjo');
    cy.get('#button-next').should('not.be.enabled');
    cy.get('ion-input').eq(1).find('input').type('banjo@rare.com');
    cy.get('#button-next').should('not.be.enabled');
    cy.get('ion-input').eq(2).find('input').type('SpiralMountain');
    cy.get('#button-next').should('be.enabled');
    cy.get('#button-next').click();

    // Third page, server
    cy.get('.modal-header__text').should('equal', '3');
    cy.get('#button-next').should('be.enabled');
    cy.get('#button-previous').should('be.visible');
    cy.get('ion-input').should('not.be.visible');
    cy.get('ion-radio').should('have.length', 2);
    cy.get('ion-radio').first().should('have.class', 'radio-checked');
    cy.get('ion-radio').last().should('not.have.class', 'radio-checked');
    cy.get('ion-radio').last().click();
    cy.get('ion-radio').first().should('not.have.class', 'radio-checked');
    cy.get('ion-radio').last().should('have.class', 'radio-checked');
    cy.get('#button-next').should('not.be.enabled');
    cy.get('ion-input').should('be.visible');
    cy.get('ion-input').find('input').type('my-parsec-server');
    cy.get('#button-next').should('be.enabled');
    cy.get('#button-next').click();

    // Fourth page, password
    cy.get('.modal-header__text').should('equal', '4');
    cy.get('#button-next').should('not.be.enabled');
    cy.get('#button-previous').should('be.visible');
    cy.get('.password-level').should('not.be.visible');
    cy.get('.error-message').should('not.be.visible');
    cy.get('ion-input').first().find('input').type('A');
    cy.get('.container-password-level').should('be.visible');
    cy.get('.error-message').should('be.visible');
    cy.get('.error-message').contains('Do not match');
    cy.get('.password-level__text').should('equal', 'Low');
    cy.get('ion-input').first().find('input').type('LongP@s');
    cy.get('.password-level__text').should('equal', 'Medium');
    cy.get('ion-input').first().find('input').type('sw0rd');
    cy.get('.password-level__text').should('equal', 'Strong');
    cy.get('ion-input').last().find('input').type('ALongP@ssw0rd');
    cy.get('.error-message').should('not.be.visible');
    cy.get('#button-next').should('be.enabled');
    cy.get('.closeBtn').should('be.visible');
    cy.get('#button-next').click();

    // Fifth page, spinner, X button should be hidden
    cy.get('.modal-header__text').should('equal', '5');
    cy.get('.closeBtn').should('not.be.visible');
    cy.get('#button-next').should('not.be.visible');
    cy.get('#button-previous').should('not.be.visible');

    // Spinner gets replaced after 2s
    // Sixth page, end
    cy.get('.modal-header__text').should('equal', '6');
    cy.get('#button-next').should('equal', 'Done');
    cy.get('#button-next').should('be.enabled');
    cy.get('.closeBtn').should('not.be.visible');
    cy.get('@consoleLog').should(
      'have.been.calledWith',
      // eslint-disable-next-line quotes
      `Creating org MyOrg, user Banjo <banjo@rare.com>, device SpiralMountain, password ALongP@ssw0rd, backend my-parsec-server`,
    );
    cy.get('#button-next').click();
    cy.get('.modal-header__title').should('not.exist');

    // Should be logged in on workspace page
    cy.get('#button-new-workspace').contains('New workspace');
    cy.get('.card').should('have.length', 5);
  });

  it('Close with X button', async() => {
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').get('ion-item').first().click();
    cy.get('.modal-header__title').should('equal', 'Create an organization');
    cy.get('.closeBtn').should('be.visible');
    cy.get('.closeBtn').click();
    cy.get('ion-alert').contains('Are you sure?');

    // Cancel
    cy.get('ion-alert').get('.alert-button-role-cancel').click();
    cy.get('ion-alert').should('not.exist');
    cy.get('.create-organization-modal').should('exist');

    cy.get('.closeBtn').click();
    cy.get('ion-alert').contains('Are you sure?');

    // Confirm
    cy.get('.alert-button-role-confirm').click();
    cy.get('ion-alert').should('not.exist');
    cy.get('.create-organization-modal').should('not.exist');
  });

  it('Can go to the previous page', async() => {
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').get('ion-item').first().click();
    cy.get('.modal-header__title').should('equal', 'Create an organization');
    cy.get('.modal-header__text').should('equal', '1');

    cy.get('#button-previous').should('not.be.visible');
    cy.get('#org-name-input').find('input').type('MyOrg');
    cy.get('#button-next').click();

    cy.get('.modal-header__text').should('equal', '2');
    cy.get('#button-previous').should('be.visible');
    cy.get('#button-previous').click();
    cy.get('.modal-header__text').should('equal', '1');
    cy.get('#button-next').click();
    cy.get('.modal-header__text').should('equal', '2');

    cy.get('ion-input').eq(0).find('input').type('Banjo');
    cy.get('ion-input').eq(1).find('input').type('banjo@rare.com');
    cy.get('ion-input').eq(2).find('input').type('SpiralMountain');
    cy.get('#button-next').click();
    cy.get('.modal-header__text').should('equal', '3');
    cy.get('#button-previous').should('be.visible');
    cy.get('#button-previous').click();
    cy.get('.modal-header__text').should('equal', '2');
    cy.get('#button-next').click();
    cy.get('.modal-header__text').should('equal', '3');
    cy.get('#button-next').click();
    cy.get('.modal-header__text').should('equal', '4');
    cy.get('#button-previous').should('be.visible');
    cy.get('#button-previous').click();
    cy.get('.modal-header__text').should('equal', '3');
    cy.get('#button-next').click();
    cy.get('.modal-header__text').should('equal', '4');
    cy.get('ion-input').first().find('input').type('ALongP@ssw0rd');
    cy.get('ion-input').last().find('input').type('ALongP@ssw0rd');
    cy.get('#button-next').click();
    cy.get('#button-previous').should('not.be.visible');
  });
});

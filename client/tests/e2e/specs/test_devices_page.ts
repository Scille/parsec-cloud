// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Display client devices', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('#profile-button').click();
    cy.get('.popover-viewport').contains('My devices').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Check initial state', () => {
    cy.get('.title-h2').contains('Devices');
    cy.get('.devices-container').find('h2').contains('Your devices');
    cy.get('.devices-container').find('ion-button').contains('Add');
    cy.get('.devices-container').find('ion-item').as('devices').should('have.length', 2);
    cy.get('@devices').first().find('ion-text').contains('My First Device');
    cy.get('@devices').first().find('ion-text').contains('Current');
  });

  // check if restore-password section is displayed
  it('Check if restore-password section is displayed', () => {
    cy.get('.restore-password').find('h3').contains('Password recovery files');
    cy.get('.restore-password').find('ion-label').should('have.class', 'danger');
    cy.get('.restore-password').find('.restore-password-button').find('ion-button').contains('Create recovery files');
    cy.get('.restore-password').find('.restore-password-button').find('ion-button').click();
    cy.get('.restore-password').find('ion-label').should('have.class', 'done');
    cy.get('.restore-password').find('.restore-password-button').find('ion-button').contains('Downloading files again');
  });
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Display client devices', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('#profile-button').click();
    cy.get('.popover-viewport').find('ion-item').eq(1).click();
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
});

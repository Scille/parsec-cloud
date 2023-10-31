// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check about modal', () => {
  beforeEach(() => {
    cy.visitApp('coolorg');
    cy.contains('Your organizations');
    cy.get('.sidebar-footer__version').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Opens the settings dialog', () => {
    cy.get('.about-modal').should('exist');
    cy.get('.about-modal').get('.title-h2').contains('About');
    cy.get('.about-modal').find('.info-list').find('.app-info-key').as('keys').should('have.length', 4);
    cy.get('.about-modal').find('.info-list').find('.app-info-value').as('values').should('have.length', 4);
    cy.get('@keys').eq(0).contains('Version');
    cy.get('@keys').eq(1).contains('Developer');
    cy.get('@keys').eq(2).contains('License');
    cy.get('@keys').eq(3).contains('Project');
    cy.get('@values')
      .eq(0)
      .contains(/v[\da-z.-]+/);
    cy.get('@values').eq(1).contains('Parsec Cloud');
    cy.get('@values').eq(2).contains('BUSL-1.1');
    cy.get('@values').eq(3).contains('GitHub');
  });

  it('Close with X', () => {
    cy.get('.about-modal').should('exist');
    cy.get('.closeBtn').click();
    cy.get('.about-modal').should('not.exist');
  });
});

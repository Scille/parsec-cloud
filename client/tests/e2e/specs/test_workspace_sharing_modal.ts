// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check workspace sharing modal', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd');
    cy.get('.card').eq(1).find('.shared-group').click();
    cy.get('.workspace-sharing-modal').find('ion-title').contains('Share this workspace');
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Checks initial status', () => {
    cy.get('ion-list').find('.content').should('have.length', 11);
    // cspell:disable-next-line
    cy.get('ion-list').find('.person-name').first().contains('Cernd');
    cy.get('ion-list').find('.content').first().find('.filter-button').contains('Not shared');
    // cspell:disable-next-line
    cy.get('ion-list').find('.person-name').last().contains('Yoshimo');
    cy.get('ion-list').find('.content').last().find('.filter-button').contains('Not shared');
    cy.get('ion-list').find('.content').eq(2).find('.filter-button').contains('Owner');

    cy.get('ion-list').find('.content').first().find('.filter-button').click();
    cy.get('.popover-viewport').find('ion-item').eq(0).contains('Reader');
    cy.get('.popover-viewport').find('ion-item').eq(1).contains('Contributor');
    cy.get('.popover-viewport').find('ion-item').eq(2).contains('Manager');
    cy.get('.popover-viewport').find('ion-item').eq(3).contains('Owner');
    cy.get('.popover-viewport').find('ion-item').eq(4).contains('Not shared');
    cy.get('.popover-viewport').find('ion-item').eq(4).find('ion-icon').should('have.class', 'checked');
    cy.get('.popover-viewport').find('ion-item').eq(4).find('ion-icon').should('have.class', 'selected');

  });

  it('Change user role', () => {
    cy.get('ion-list').find('.content').first().find('.filter-button').contains('Not shared');
    cy.get('ion-list').find('.content').first().find('.filter-button').click();
    cy.get('.popover-viewport').find('ion-item').eq(0).contains('Reader');
    cy.get('.popover-viewport').find('ion-item').eq(0).click();
    cy.get('ion-list').find('.content').first().find('.filter-button').contains('Reader');
    // cspell:disable-next-line
    cy.get('@consoleLog').should('have.been.calledWith', 'Update user Cernd role to reader');
  });

  it('Filter users', () => {
    cy.get('ion-modal').find('ion-list').find('.content').should('have.length', 11);
    cy.get('ion-modal').find('ion-input').as('searchInput');
    cy.get('@searchInput').find('input').type('a');
    cy.get('ion-modal').find('ion-list').find('.content').should('have.length', 7);
    // Check upper-case too
    cy.get('@searchInput').find('input').type('L');
    cy.get('ion-modal').find('ion-list').find('.content').should('have.length', 2);
    // cspell:disable-next-line
    cy.get('ion-list').find('.person-name').first().contains('Valygar Corthala');
    // cspell:disable-next-line
    cy.get('ion-list').find('.person-name').last().contains('Nalia De\'Arnise');
  });

  it('Close modal', () => {
    cy.get('ion-modal').should('exist');
    cy.get('ion-modal').find('.closeBtn').click();
    cy.get('ion-modal').should('not.exist');
  });
});

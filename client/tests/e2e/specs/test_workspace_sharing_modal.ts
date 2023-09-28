// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check workspace sharing modal', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('.card').eq(1).find('.shared-group').click();
    cy.get('.workspace-sharing-modal').find('ion-title').contains('Share this workspace');
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Checks initial status', () => {
    cy.get('ion-list').find('.content').should('have.length', 4);
    cy.get('ion-list').find('.person-name').first().contains('Me');
    cy.get('ion-list').find('.filter-button').first().should('have.class', 'button-disabled');
    // cspell:disable-next-line
    cy.get('ion-list').find('.person-name').eq(1).contains('Korgan Bloodaxe');
    cy.get('ion-list').find('.content').eq(1).find('.filter-button').contains('Reader');
    // cspell:disable-next-line
    cy.get('ion-list').find('.person-name').last().contains('Jaheira');
    cy.get('ion-list').find('.content').last().find('.filter-button').contains('Not shared');
    cy.get('ion-list').find('.content').eq(2).find('.filter-button').contains('Contributor');

    cy.get('ion-list').find('.content').eq(1).find('.filter-button').click();
    cy.get('.popover-viewport').find('ion-item').eq(0).contains('Reader');
    cy.get('.popover-viewport').find('ion-item').eq(1).contains('Contributor');
    cy.get('.popover-viewport').find('ion-item').eq(2).contains('Manager');
    cy.get('.popover-viewport').find('ion-item').eq(3).contains('Owner');
    cy.get('.popover-viewport').find('ion-item').eq(4).contains('Not shared');
    cy.get('.popover-viewport').find('ion-item').eq(0).find('ion-icon').should('have.class', 'checked');
    cy.get('.popover-viewport').find('ion-item').eq(0).find('ion-icon').should('have.class', 'selected');
  });

  it('Change user role', () => {
    cy.get('ion-list').find('.content').eq(3).find('.filter-button').contains('Not shared').click();
    cy.get('.popover-viewport').find('ion-item').eq(0).contains('Reader').click();
    cy.get('ion-list').find('.content').eq(3).find('.filter-button').contains('Reader');
    // cspell:disable-next-line
    cy.checkToastMessage('Jaheira\'s role has been updated to Reader.');
    cy.get('ion-list').find('.content').eq(1).find('.filter-button').contains('Reader').click();
    cy.get('.popover-viewport').find('ion-item').eq(4).contains('Not shared').click();
    // cspell:disable-next-line
    cy.checkToastMessage('The workspace is no longer shared with Korgan Bloodaxe.');
  });

  it('Filter users', () => {
    cy.get('ion-modal').find('ion-list').find('.content').should('have.length', 4);
    cy.get('ion-modal').find('ion-input').as('searchInput');
    cy.get('@searchInput').find('input').type('a');
    cy.get('ion-modal').find('ion-list').find('.content').should('have.length', 3);
    // Check upper-case too
    cy.get('@searchInput').find('input').type('H');
    cy.get('ion-modal').find('ion-list').find('.content').should('have.length', 2);
    // cspell:disable-next-line
    cy.get('ion-list').find('.person-name').first().contains('Me');
    // cspell:disable-next-line
    cy.get('ion-list').find('.person-name').last().contains('Jaheira');
  });

  it('Close modal', () => {
    cy.get('ion-modal').should('exist');
    cy.get('ion-modal').find('.closeBtn').click();
    cy.get('ion-modal').should('not.exist');
  });
});

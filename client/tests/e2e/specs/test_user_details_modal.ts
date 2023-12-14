// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check user details modal', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('.organization-card__manageBtn').click();
    cy.get('.users-container').find('.user-list-item').eq(2).find('.options-button').invoke('show').click();
    cy.get('.user-context-menu').find('.menu-list').find('ion-item').as('menuItems');
    cy.get('@menuItems').eq(3).contains('View details').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Tests user details modal', () => {
    cy.get('.user-details-modal').as('modal').find('ion-header').contains('User details');
    cy.get('@modal').find('.ms-modal-content').as('modal-content').find('ion-text').eq(0).contains('Name');
    // cspell:disable-next-line
    cy.get('@modal-content').find('ion-text').eq(1).contains('Jaheira');
    cy.get('@modal-content').find('ion-text').eq(2).contains('Joined');
    cy.get('@modal-content').find('ion-text').eq(3).contains('one second ago');
    cy.get('@modal-content').find('ion-text').eq(4).contains('Shared workspaces');
    cy.get('@modal-content').find('ion-list').find('ion-card').as('workspace-cards').should('have.length', 2);
    cy.get('@workspace-cards').eq(0).find('ion-text').contains('Workspace1');
    cy.get('@workspace-cards').eq(0).find('ion-icon').should('exist');
    cy.get('@workspace-cards').eq(0).find('ion-label').contains('Owner');
    cy.get('@workspace-cards').eq(1).find('ion-text').contains('Workspace2');
    cy.get('@workspace-cards').eq(1).find('ion-icon').should('exist');
    cy.get('@workspace-cards').eq(1).find('ion-label').contains('Contributor');
    cy.get('@modal-content').find('ion-button').contains('Close').click();
    cy.get('@modal').should('not.exist');
  });
});

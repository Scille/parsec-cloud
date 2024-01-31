// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check user details modal', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('.organization-card-manageBtn').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Tests user details modal', () => {
    cy.get('.users-container').find('.user-list-item').eq(2).find('.options-button').invoke('show').click();
    cy.get('.user-context-menu').find('.menu-list').find('ion-item').as('menuItems');
    cy.get('@menuItems').eq(3).contains('View details').click();
    cy.get('.user-details-modal').as('modal').find('ion-header').contains('User details');
    cy.get('@modal').find('.ms-modal').as('modal-content').find('ion-text').eq(0).contains('Name');
    // cspell:disable-next-line
    cy.get('@modal-content').find('ion-text').eq(1).contains('Jaheira');
    cy.get('@modal-content').find('ion-text').eq(2).contains('Joined');
    cy.get('@modal-content')
      .find('ion-text')
      .eq(3)
      .contains(/^\w+ (seconds?|minute?s) ago$/);
    cy.get('@modal-content').find('ion-text').eq(4).contains('Shared workspaces');
    cy.get('@modal-content').find('.workspace-empty').should('not.be.visible');
    cy.get('@modal-content').find('ion-list').find('ion-card').as('workspace-cards').should('have.length', 3);
    cy.get('@workspace-cards').eq(0).find('ion-text').contains('Trademeet');
    cy.get('@workspace-cards').eq(0).find('ion-icon').should('exist');
    cy.get('@workspace-cards').eq(0).find('ion-label').contains('Contributor');
    cy.get('@workspace-cards').eq(1).find('ion-text').contains('The Copper Coronet');
    cy.get('@workspace-cards').eq(1).find('ion-icon').should('exist');
    cy.get('@workspace-cards').eq(1).find('ion-label').contains('Contributor');
    cy.get('@modal-content').find('ion-button').contains('Close').click();
    cy.get('@modal').should('not.exist');
  });

  it('Tests no common workspaces', () => {
    cy.get('.user-menu__item').eq(1).contains('Revoked').click();
    cy.get('.revoked-users-container').find('.user-list-item').should('have.length', 1);
    cy.get('.revoked-users-container').find('.user-list-item').eq(0).find('.options-button').invoke('show').click();
    cy.get('.user-context-menu').find('.menu-list').find('ion-item').as('menuItems');
    cy.get('@menuItems').eq(1).contains('View details').click();
    cy.get('.user-details-modal').as('modal').find('ion-header').contains('User details');
    cy.get('@modal').find('.ms-modal').as('modal-content').find('ion-text').eq(0).contains('Name');
    // cspell:disable-next-line
    cy.get('@modal-content').find('ion-text').eq(1).contains('Valygar Corthala');
    cy.get('@modal-content').find('ion-text').eq(2).contains('Joined');
    cy.get('@modal-content')
      .find('ion-text')
      .eq(3)
      .contains(/^\w+ (seconds?|minute?s) ago$/);
    cy.get('@modal-content').find('ion-text').eq(4).contains('Shared workspaces');
    cy.get('@modal-content').find('ion-list').find('ion-card').should('have.length', 0);
    cy.get('@modal-content').find('.workspace-empty').contains('You have no workspaces in common with this user.');
    cy.get('@modal-content').find('.workspace-empty').should('be.visible');

    cy.get('@modal-content').find('ion-button').contains('Close').click();
    cy.get('@modal').should('not.exist');
  });
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

describe('Check workspaces page', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd');
    cy.get('.organization-card__manageBtn').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Checks initial status', () => {
    cy.get('.manage-organization').find('ion-label').contains('Manage my organization');
    cy.get('.organization-card__header').should('not.be.visible');
    cy.get('.organization-card__manageBtn').should('not.be.visible');
    cy.get('.sidebar').find('.list-workspaces').should('not.be.visible');
    cy.get('.sidebar').find('.list-users').contains('Users').should('have.class', 'category-selected');
    cy.get('.sidebar').find('.list-users').find('ion-item').as('usersItem').should('have.length', 3);
    cy.get('@usersItem').eq(0).contains('Active');
    cy.get('@usersItem').eq(0).should('have.class', 'user-menu-selected');
    cy.get('@usersItem').eq(1).contains('Revoked');
    cy.get('@usersItem').eq(1).should('not.have.class', 'user-menu-selected');
    cy.get('@usersItem').eq(2).contains('Invitations');
    cy.get('@usersItem').eq(2).should('not.have.class', 'user-menu-selected');
    cy.get('.sidebar').find('.storage').contains('Storage').should('not.have.class', 'category-selected');
    cy.get('.sidebar').find('.organization').contains('Information').should('not.have.class', 'category-selected');
    cy.get('.topbar-left').find('.title-h2').contains('Active users');
    cy.get('.toolbar').find('.button-option').contains('Invite a user');
  });

  it('Switch page', () => {
    function checkUserMenuSelected(index: number): void {
      cy.get('@usersItem').each((elem, i) => {
        if (i === index) {
          expect(elem).to.have.class('user-menu-selected');
        } else {
          expect(elem).to.not.have.class('user-menu-selected');
        }
      });
    }

    cy.get('.sidebar').find('.list-users__header').should('have.class', 'category-selected');
    cy.get('.sidebar').find('.storage__header').should('not.have.class', 'category-selected');
    cy.get('.sidebar').find('.organization__header').should('not.have.class', 'category-selected');
    cy.get('.topbar-left').find('.title-h2').as('title').contains('Active users');
    cy.get('.sidebar').find('.list-users').find('ion-item').as('usersItem');
    checkUserMenuSelected(0);
    cy.get('@usersItem').eq(1).click();
    checkUserMenuSelected(1);
    cy.get('.topbar-left').find('.title-h2').contains('Revoked users');
    cy.get('@usersItem').eq(2).click();
    checkUserMenuSelected(2);
    cy.get('.topbar-left').find('.title-h2').contains('Invitations');

    cy.get('.sidebar').find('.storage__header').click();
    cy.get('.sidebar').find('.list-users__header').should('not.have.class', 'category-selected');
    cy.get('.sidebar').find('.storage__header').should('have.class', 'category-selected');
    cy.get('.sidebar').find('.organization__header').should('not.have.class', 'category-selected');
    cy.get('@title').contains('Storage');

    cy.get('.sidebar').find('.organization__header').click();
    cy.get('.sidebar').find('.list-users__header').should('not.have.class', 'category-selected');
    cy.get('.sidebar').find('.storage__header').should('not.have.class', 'category-selected');
    cy.get('.sidebar').find('.organization__header').should('have.class', 'category-selected');
    cy.get('@title').contains('Information');
  });

  it('Back to workspaces', () => {
    cy.get('.manage-organization').find('ion-button').click();
    cy.get('.manage-organization').should('not.be.visible');
    cy.get('.topbar-left').contains('My workspaces');
  });
});

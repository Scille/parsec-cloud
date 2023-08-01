// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check manage org page', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd');
    cy.get('.organization-card__manageBtn').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Checks initial status', () => {
    cy.get('.back-organization').find('ion-label').contains('Manage my organization');
    cy.get('.organization-card__header').should('not.be.visible');
    cy.get('.organization-card__manageBtn').should('not.be.visible');
    cy.get('.sidebar').find('.list-workspaces').should('not.be.visible');
    cy.get('.sidebar').find('.users').find('ion-item').first().contains('Users');
    cy.get('.sidebar').find('.users').find('ion-item').first().should('have.class', 'item-selected');
    cy.get('.sidebar').find('.user-menu').find('ion-item').as('userItems').should('have.length', 3);
    cy.get('@userItems').eq(0).contains('Active');
    cy.get('@userItems').eq(0).should('have.class', 'user-menu-selected');
    cy.get('@userItems').eq(1).contains('Revoked');
    cy.get('@userItems').eq(1).should('not.have.class', 'user-menu-selected');
    cy.get('@userItems').eq(2).contains('Invitations');
    cy.get('@userItems').eq(2).should('not.have.class', 'user-menu-selected');
    cy.get('.sidebar').find('.storage').find('ion-item').first().contains('Storage');
    cy.get('.sidebar').find('.storage').find('ion-item').first().should('have.class', 'item-not-selected');
    cy.get('.sidebar').find('.organization').find('ion-item').first().contains('Information');
    cy.get('.sidebar').find('.organization').find('ion-item').first().should('have.class', 'item-not-selected');
    cy.get('.topbar-left').find('.title-h2').contains('Active users');
    cy.get('.toolbar').find('.ms-action-bar-button').contains('Invite a user');
  });

  it('Switch page', () => {
    function checkUserMenuSelected(index: number): void {
      cy.get('@userItems').each((elem, i) => {
        if (i === index) {
          expect(elem).to.have.class('user-menu-selected');
        } else {
          expect(elem).to.have.class('user-menu-not-selected');
        }
      });
    }

    cy.get('.sidebar').find('.users').find('ion-item').first().as('usersEl').should('have.class', 'item-selected');
    cy.get('.sidebar').find('.storage').find('ion-item').first().as('storageEl').should('have.class', 'item-not-selected');
    cy.get('.sidebar').find('.organization').find('ion-item').first().as('orgEl').should('have.class', 'item-not-selected');
    cy.get('.topbar-left').find('.title-h2').as('title').contains('Active users');
    cy.get('.sidebar').find('.user-menu').find('ion-item').as('userItems');
    checkUserMenuSelected(0);
    cy.get('@userItems').eq(1).click();
    cy.wait(200);
    cy.get('.topbar-left').find('.title-h2').contains('Revoked users');
    checkUserMenuSelected(1);
    cy.get('@userItems').eq(2).click();
    cy.wait(200);
    checkUserMenuSelected(2);
    cy.get('.topbar-left').find('.title-h2').contains('Invitations');

    cy.get('@storageEl').click();
    cy.get('@usersEl').should('have.class', 'item-not-selected');
    cy.get('@storageEl').should('have.class', 'item-selected');
    cy.get('@orgEl').should('have.class', 'item-not-selected');
    cy.get('@title').contains('Storage');

    cy.get('@orgEl').click();
    cy.get('@usersEl').should('have.class', 'item-not-selected');
    cy.get('@storageEl').should('have.class', 'item-not-selected');
    cy.get('@orgEl').should('have.class', 'item-selected');
    cy.get('@title').contains('Information');
  });

  it('Back to workspaces', () => {
    cy.get('.back-organization').find('ion-button').click();
    cy.get('.back-organization').should('not.be.visible');
    cy.get('.topbar-left').contains('My workspaces');
  });
});

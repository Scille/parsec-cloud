// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check manage org page', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('.organization-card-manageBtn').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Checks initial status', () => {
    cy.get('.back-organization').find('ion-label').contains('Manage my organization');
    cy.get('.organization-card-header').should('not.be.visible');
    cy.get('.organization-card-manageBtn').should('not.be.visible');
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

  it('Check org info page', () => {
    cy.get('.sidebar').find('.organization').find('ion-item').first().contains('Information').click();
    cy.get('.topbar-left').find('.title-h2').contains('Information');
    cy.get('.org-info-container').find('h1').contains('Information on MyOrg');
    cy.get('.org-info-container').find('ion-item').as('items').should('have.length', 10);
    // Config outsider
    cy.get('@items').eq(0).find('ion-label').contains('Outsider profile');
    cy.get('@items').eq(0).find('ion-chip').contains('Allowed');
    // User limit
    cy.get('@items').eq(1).find('ion-label').contains('Users limit');
    cy.get('@items').eq(1).find('ion-chip').contains('Unlimited');
    // Backend addr
    cy.get('@items').eq(2).find('ion-label').eq(0).contains('Server address');
    cy.get('@items').eq(2).find('ion-label').eq(1).contains('parsec://example.com/MyOrg');
    // Global data
    cy.get('@items').eq(3).find('ion-label').eq(0).contains('Global data');
    cy.get('@items').eq(3).find('ion-label').eq(1).contains('4.09 TB');
    // Meta data
    cy.get('@items').eq(4).find('ion-label').eq(0).contains('Meta-data');
    cy.get('@items').eq(4).find('ion-label').eq(1).contains('40.5 MB');
    // Active users
    cy.get('@items').eq(5).find('ion-chip').contains('Active');
    cy.get('@items').eq(5).find('ion-label').contains('10');
    // Administrators
    cy.get('@items').eq(6).find('ion-chip').contains('Administrators');
    cy.get('@items').eq(6).find('ion-label').contains('2');
    // Standards
    cy.get('@items').eq(7).find('ion-chip').contains('Standards');
    cy.get('@items').eq(7).find('ion-label').contains('5');
    // Outsiders
    cy.get('@items').eq(8).find('ion-chip').contains('Outsiders');
    cy.get('@items').eq(8).find('ion-label').contains('3');
    // Revoked
    cy.get('@items').eq(9).find('ion-chip').contains('Revoked');
    cy.get('@items').eq(9).find('ion-label').contains('2');
  });
});

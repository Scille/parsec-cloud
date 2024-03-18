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
    cy.get('.back-organization').contains('Back');
    cy.get('.organization-card-header').should('not.be.visible');
    cy.get('.organization-card-manageBtn').should('not.be.visible');
    cy.get('.sidebar').find('.list-workspaces').should('not.be.visible');
    cy.get('.sidebar').find('.users').find('ion-item').first().contains('Users');
    cy.get('.sidebar').find('.users').find('ion-item').first().should('have.class', 'item-selected');
    // cy.get('.sidebar').find('.storage').find('ion-item').first().contains('Storage');
    // cy.get('.sidebar').find('.storage').find('ion-item').first().should('have.class', 'item-not-selected');
    cy.get('.sidebar').find('.organization').find('ion-item').first().contains('Information');
    cy.get('.sidebar').find('.organization').find('ion-item').first().should('have.class', 'item-not-selected');
    cy.get('.topbar-left').find('.title-h2').contains('Users');
    cy.get('.toolbar').find('.ms-action-bar-button').contains('Invite a user');
  });

  it('Switch page', () => {
    cy.get('.sidebar').find('.users').find('ion-item').first().as('usersEl').should('have.class', 'item-selected');
    // cy.get('.sidebar').find('.storage').find('ion-item').first().as('storageEl').should('have.class', 'item-not-selected');
    cy.get('.sidebar').find('.organization').find('ion-item').first().as('orgEl').should('have.class', 'item-not-selected');
    cy.get('.topbar-left').find('.title-h2').as('title').contains('Users');

    // cy.get('@storageEl').click();
    // cy.get('@usersEl').should('have.class', 'item-not-selected');
    // cy.get('@storageEl').should('have.class', 'item-selected');
    // cy.get('@orgEl').should('have.class', 'item-not-selected');
    // cy.get('@title').contains('Storage');

    cy.get('@orgEl').click();
    cy.get('@usersEl').should('have.class', 'item-not-selected');
    // cy.get('@storageEl').should('have.class', 'item-not-selected');
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

    // Config outsider
    cy.get('.org-config').find('.org-info-title').contains('Configuration');
    cy.get('.org-config').find('.org-info-item-title').eq(0).contains('Outsider profile');
    cy.get('.org-config').find('.org-config-list-item__value').eq(0).contains('Enabled');
    cy.get('.org-config').find('.org-info-item-title').eq(1).contains('User limit');
    cy.get('.org-config').find('.org-config-list-item__value').eq(1).contains('Unlimited');
    cy.get('.org-config').find('.org-info-item-title').eq(2).contains('Server address');
    cy.get('.org-config').find('.server-address-value__text').contains('parsec3://example.com/MyOrg');
    cy.get('.org-config').find('#copy-link-btn').click();
    cy.get('.org-config').find('.server-address-value__copied').contains('Copied');

    // Storage
    // cy.get('.org-storage').find('.org-info-title').contains('Storage');
    // cy.get('.org-storage').find('.org-info-item-title').eq(0).contains('Global data');
    // cy.get('.org-storage').find('.org-storage-list-item__value').eq(0).contains(/^[\d.]+ [K|M|G|T]?B$/);
    // cy.get('.org-storage').find('.org-info-item-title').eq(1).contains('Meta-data');
    // cy.get('.org-storage').find('.org-storage-list-item__value').eq(1).contains(/^[\d.]+ [K|M|G|T]?B$/);

    // Active users
    cy.get('.org-user').find('.org-info-title').contains('Users');
    cy.get('.org-user').find('.user-active-header__title').contains('Active');
    cy.get('.org-user').find('.user-active-header span').contains('5');

    // Administrators
    cy.get('.org-user').find('ion-chip').eq(0).contains('Administrator');
    cy.get('.org-user').find('.user-active-list-item__value').eq(0).contains('3');

    // Standard
    cy.get('.org-user').find('ion-chip').eq(1).contains('Standard');
    cy.get('.org-user').find('.user-active-list-item__value').eq(1).contains('3');

    // Outsider
    cy.get('.org-user').find('ion-chip').eq(2).contains('Outsider');
    cy.get('.org-user').find('.user-active-list-item__value').eq(2).contains('2');

    // Revoked users
    cy.get('.org-user').find('.user-revoked-header__title').contains('Revoked');
    cy.get('.org-user').find('.user-revoked-header span').contains('3');
  });
});

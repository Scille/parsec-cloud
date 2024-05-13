// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check active users page', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('.organization-card-manageBtn').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Tests selection grid item', () => {
    cy.get('#activate-users-ms-action-bar').find('#grid-view').as('gridButton').click();

    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').contains('Invite a user');
    cy.get('.users-container-grid').find('.user-card-item').as('userItems').should('have.length', 8);

    // checking "you" mark for the current user
    cy.get('@userItems').eq(0).find('.name-you').should('be.visible');

    cy.get('@userItems').eq(1).realHover().find('.checkbox').should('be.visible');

    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').should('exist');

    cy.get('@userItems').eq(3).realHover().find('.checkbox').click();
    cy.get('#activate-users-ms-action-bar').find('#button-revoke-user').contains('Revoke this user');
    cy.get('#activate-users-ms-action-bar').find('#button-common-workspaces').contains('View details');
  });

  it('Tests sort users', () => {
    cy.get('#select-popover-button').click();
    cy.get('ion-popover').find('#sort-order-button').as('sortOrderButton').contains('Ascending').click();
    cy.wait(500);

    cy.get('#select-popover-button').click();
    cy.get('ion-popover').find('#sort-item-list').find('ion-item').as('itemButtons').should('have.length', 4);
    cy.get('@itemButtons').eq(0).contains('Name');
    cy.get('@itemButtons').eq(1).contains('Date joined');
    cy.get('@itemButtons').eq(2).contains('Profile');
    cy.get('@itemButtons').eq(2).find('.checked');
    cy.get('@itemButtons').eq(3).contains('Status');

    // Sort by name descending
    cy.get('@itemButtons').eq(0).click();
    cy.wait(500);
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 8);
    // cspell:disable-next-line
    cy.get('@userItems').eq(1).find('.person-name').contains('Valygar Corthala');
    cy.get('@userItems').eq(2).find('.person-name').contains('Patches');
    // cspell:disable-next-line
    cy.get('@userItems').eq(3).find('.person-name').contains('Karl Hungus');
    // cspell:disable-next-line
    cy.get('@userItems').eq(4).find('.person-name').contains('Jaheira');
    cy.get('@userItems').eq(5).find('.person-name').contains('Gaia');
    // cspell:disable-next-line
    cy.get('@userItems').eq(6).find('.person-name').contains('Cernd');
    // cspell:disable-next-line
    cy.get('@userItems').eq(7).find('.person-name').contains('Arthas Menethil');

    // Sort by profile descending
    cy.get('#select-popover-button').click();
    cy.get('@itemButtons').eq(0).find('.checked');
    cy.get('@itemButtons').eq(2).click();
    cy.wait(500);
    cy.get('@userItems').eq(1).find('.label-profile').contains('Outsider');
    cy.get('@userItems').eq(7).find('.label-profile').contains('Admin');

    // Sort by status ascending
    cy.get('#select-popover-button').click();
    cy.get('@sortOrderButton').contains('Descending').click();
    cy.wait(500);
    cy.get('#select-popover-button').click();
    cy.get('@itemButtons').eq(2).find('.checked');
    cy.get('@itemButtons').eq(3).click();
    cy.wait(500);
    cy.get('@userItems').eq(1).find('.label-status').contains('Active');
    cy.get('@userItems').eq(7).find('.label-status').contains('Revoked');

    // Sort by date ascending, only checking users with set join dates
    cy.get('#select-popover-button').click();
    cy.get('@itemButtons').eq(3).find('.checked');
    cy.get('@itemButtons').eq(1).click();
    cy.wait(500);
    cy.get('@userItems').eq(4).find('.person-name').contains('Gaia');
    cy.get('@userItems').eq(5).find('.person-name').contains('Patches');
    // cspell:disable-next-line
    cy.get('@userItems').eq(6).find('.person-name').contains('Arthas Menethil');
    // cspell:disable-next-line
    cy.get('@userItems').eq(7).find('.person-name').contains('Karl Hungus');
  });

  it('Tests filter users', () => {
    cy.get('.counter').contains('8 users');
    cy.get('#users-page-user-list').find('ion-item').as('userList').should('have.length', 8);
    // Select active users to check that hiding them unchecks it
    cy.get('.user-list-header').find('ion-checkbox').click();
    cy.get('#select-filter-popover-button').click();
    cy.get('ion-popover').find('#user-filter-list').as('filterList');
    cy.get('@filterList').find('#filter-title-status').contains('Status');
    cy.get('@filterList').find('#filter-title-role').contains('Role');
    cy.get('.counter').contains('4 users selected');
    cy.get('@filterList').find('#filter-check-revoked').find('ion-checkbox').click();
    cy.get('.counter').contains('4 users selected');
    cy.get('@userList').should('have.length', 5);
    cy.get('@filterList').find('#filter-check-admin').find('ion-checkbox').click();
    cy.get('.counter').contains('3 users selected');
    cy.get('@userList').should('have.length', 4);
    cy.get('.users-container').find('.user-list-item').as('userItems').eq(1).find('.label-profile').contains('Standard');
    cy.get('@userItems').eq(1).find('.label-status').contains('Active');
    cy.get('@userItems').eq(2).find('.label-profile').contains('Standard');
    cy.get('@userItems').eq(2).find('.label-status').contains('Active');
    cy.get('@userItems').eq(3).find('.label-profile').contains('Outsider');
    cy.get('@userItems').eq(3).find('.label-status').contains('Active');
    cy.get('@filterList').find('#filter-check-standard').find('ion-checkbox').click();
    cy.get('.counter').contains('One user selected');
    cy.get('@filterList').find('#filter-check-active').find('ion-checkbox').click();
    cy.get('.counter').contains('One user');
    cy.get('@filterList').find('#filter-check-revoked').find('ion-checkbox').click();
    cy.get('@userItems').eq(1).find('.label-profile').contains('Outsider');
    cy.get('@userItems').eq(1).find('.label-status').contains('Revoked');
    cy.get('@filterList').find('#filter-check-admin').find('ion-checkbox').click();
    cy.get('@filterList').find('#filter-check-active').find('ion-checkbox').click();
    cy.get('.counter').contains('5 users');
    cy.get('@userList').should('have.length', 5);
    cy.get('@filterList').find('#filter-check-standard').find('ion-checkbox').click();
    cy.get('.counter').contains('8 users');
    cy.get('@userList').should('have.length', 8);
    cy.get('.user-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
  });
});

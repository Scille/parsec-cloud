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

  it('Checks initial status', () => {
    cy.get('.back-organization').contains('Back');
    cy.get('.sidebar').find('.users').find('ion-item').first().contains('Users');
    cy.get('.sidebar').find('.users').find('ion-item').first().should('have.class', 'item-selected');
    cy.get('.topbar-left__title').find('.title-h2').contains('Users');
    cy.get('#activate-users-ms-action-bar').find('#grid-view').should('not.have.attr', 'disabled');
    cy.get('#activate-users-ms-action-bar').find('#list-view').should('have.attr', 'disabled');
    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').contains('Invite a user');
    cy.get('.user-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 8);
    cy.get('@userItems').eq(0).should('have.class', 'item-disabled');

    cy.get('@userItems').eq(1).find('ion-checkbox').should('not.be.visible');
    // cspell:disable-next-line
    cy.get('@userItems').eq(1).find('.person-name').contains('Jaheira');
    // cspell:disable-next-line
    cy.get('@userItems').eq(1).find('.user-email__label').contains('jaheira@gmail.com');
    cy.get('@userItems').eq(1).find('.label-profile').contains('Admin');
    cy.get('@userItems').eq(1).find('.label-status').contains('Active');

    // cspell:disable-next-line
    cy.get('@userItems').eq(2).find('.person-name').contains('Arthas Menethil');
    // cspell:disable-next-line
    cy.get('@userItems').eq(2).find('.user-email__label').contains('arthasmenethil@gmail.com');
    cy.get('@userItems').eq(2).find('.label-profile').contains('Admin');
    cy.get('@userItems').eq(2).find('.label-status').contains('Revoked');

    // cspell:disable-next-line
    cy.get('@userItems').eq(3).find('.person-name').contains('Cernd');
    // cspell:disable-next-line
    cy.get('@userItems').eq(3).find('.user-email__label').contains('cernd@gmail.com');
    cy.get('@userItems').eq(3).find('.label-profile').contains('Standard');
    cy.get('@userItems').eq(3).find('.label-status').contains('Active');

    // cspell:disable-next-line
    cy.get('@userItems').eq(4).find('.person-name').contains('Patches');
    // cspell:disable-next-line
    cy.get('@userItems').eq(4).find('.user-email__label').contains('patches@yahoo.fr');
    cy.get('@userItems').eq(4).find('.label-profile').contains('Standard');
    cy.get('@userItems').eq(4).find('.label-status').contains('Active');

    // cspell:disable-next-line
    cy.get('@userItems').eq(5).find('.person-name').contains('Valygar Corthala');
    // cspell:disable-next-line
    cy.get('@userItems').eq(5).find('.user-email__label').contains('val@gmail.com');
    cy.get('@userItems').eq(5).find('.label-profile').contains('Standard');
    cy.get('@userItems').eq(5).find('.label-status').contains('Revoked');

    // cspell:disable-next-line
    cy.get('@userItems').eq(6).find('.person-name').contains('Karl Hungus');
    // cspell:disable-next-line
    cy.get('@userItems').eq(6).find('.user-email__label').contains('karlhungus@gmail.com');
    cy.get('@userItems').eq(6).find('.label-profile').contains('Outsider');
    cy.get('@userItems').eq(6).find('.label-status').contains('Active');

    // cspell:disable-next-line
    cy.get('@userItems').eq(7).find('.person-name').contains('Gaia');
    // cspell:disable-next-line
    cy.get('@userItems').eq(7).find('.user-email__label').contains('gaia@gmail.com');
    cy.get('@userItems').eq(7).find('.label-profile').contains('Outsider');
    cy.get('@userItems').eq(7).find('.label-status').contains('Revoked');
  });

  it('Tests selection list item', () => {
    function checkChecked(checked: boolean): void {
      for (let i = 1; i < 6 && i !== 2 && i !== 5; i++) {
        cy.get('@userItems')
          .eq(i)
          .find('ion-checkbox')
          .should(checked ? 'be.visible' : 'not.be.visible');
        cy.get('@userItems')
          .eq(i)
          .find('ion-checkbox')
          .should(checked ? 'have.class' : 'not.have.class', 'checkbox-checked');
      }
    }

    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').contains('Invite a user');
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 8);
    // All unchecked by default
    checkChecked(false);

    // Options button should not be visible
    cy.get('@userItems').eq(1).find('.options-button').should('not.be.visible');
    cy.get('@userItems').eq(2).find('.options-button').should('not.be.visible');

    cy.get('.counter').contains('8 users');

    // Select all
    cy.get('.user-list-header').find('ion-checkbox').click();
    cy.get('.user-list-header').find('ion-checkbox').should('have.class', 'checkbox-checked');
    checkChecked(true);

    cy.get('.counter').contains('4 users selected');
    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').should('not.be.visible');
    cy.get('#activate-users-ms-action-bar').find('#button-revoke-user').contains('Revoke these users');

    // Deselect [3] and [4] and [6], only [1] is selected
    cy.get('@userItems').eq(3).find('ion-checkbox').click();
    cy.get('@userItems').eq(4).find('ion-checkbox').click();
    cy.get('@userItems').eq(6).find('ion-checkbox').click();
    cy.get('#activate-users-ms-action-bar').find('#button-revoke-user').contains('Revoke this user');
    cy.get('#activate-users-ms-action-bar').find('#button-common-workspaces').contains('View details');

    // Check all should be indeterminate
    cy.get('.user-list-header').find('ion-checkbox').should('have.class', 'checkbox-indeterminate');

    // Re-select all
    cy.get('.user-list-header').find('ion-checkbox').click();
    checkChecked(false);
    cy.get('.user-list-header').find('ion-checkbox').click();
    checkChecked(true);
    cy.get('.counter').contains('4 users selected');

    // Deselect all
    cy.get('.user-list-header').find('ion-checkbox').click();
    checkChecked(false);
    cy.get('.counter').contains('8 users');
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

  it('Tests context menu', () => {
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 8);
    cy.get('.user-context-menu').should('not.exist');
    cy.get('@userItems').eq(1).find('.options-button').invoke('show').click();
    cy.get('.user-context-menu').should('exist');
    cy.get('.user-context-menu').find('.menu-list').find('ion-item').as('menuItems').should('have.length', 4);
    // 0 is title, 1 is revoke button
    cy.get('@menuItems').eq(0).contains('Deletion');
    cy.get('@menuItems').eq(1).contains('Revoke this user');
    cy.get('@menuItems').eq(2).contains('User details');
    cy.get('@menuItems').eq(3).contains('View details');
  });

  it('Tests revoke one user', () => {
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 8);
    cy.get('@userItems').eq(1).find('.options-button').invoke('show').click();
    cy.get('.user-context-menu').find('.menu-list').find('ion-item').eq(1).contains('Revoke this user').click();
    cy.get('.question-modal').find('.ms-modal-header__title').contains('Revoke this user?');
    cy.get('.question-modal').find('#next-button').click();
    // cspell:disable-next-line
    cy.checkToastMessage('success', 'Jaheira has been revoked. They can no longer access this organization.');
  });

  it('Tests revoke one selected user', () => {
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 8);
    cy.get('@userItems').eq(1).realHover().find('.checkbox').click({ force: true });
    cy.get('@userItems').eq(1).find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.counter').contains('One user selected');
    cy.get('.contextual-menu').find('#button-revoke-user').contains('Revoke this user').click();
    cy.get('.question-modal').find('.ms-modal-header__title').contains('Revoke this user?');
    cy.get('.question-modal').find('#next-button').click();
    // cspell:disable-next-line
    cy.checkToastMessage('success', 'Jaheira has been revoked. They can no longer access this organization.');
    // Ultimately, they probably should not appear anymore
    cy.get('@userItems').eq(1).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
  });

  it('Tests revoke multiple', () => {
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 8);
    cy.get('.user-list-header').find('ion-checkbox').click();
    cy.get('@userItems').eq(1).find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('@userItems').eq(3).find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('@userItems').eq(4).find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('@userItems').eq(6).find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.contextual-menu').find('#button-revoke-user').contains('Revoke these users').click();
    cy.get('.question-modal').find('.ms-modal-header__title').contains('Revoke these users?');
    cy.get('.question-modal').find('#next-button').click();
    // cspell:disable-next-line
    cy.checkToastMessage('success', '4 users have been revoked, they can no longer access this organization.');
    // Ultimately, they probably should not appear anymore
    cy.get('@userItems').eq(1).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('@userItems').eq(3).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('@userItems').eq(4).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('@userItems').eq(6).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
  });

  it('Tests sort users', () => {
    cy.get('#select-popover-button').click();
    cy.get('ion-popover').find('#sort-order-button').as('sortOrderButton').contains('Ascending').click();
    cy.wait(500);

    cy.get('#select-popover-button').click();
    cy.get('ion-popover').find('#sort-item-list').find('ion-item').as('itemButtons').should('have.length', 4);
    cy.get('@itemButtons').eq(0).contains('By name');
    cy.get('@itemButtons').eq(1).contains('By date joined');
    cy.get('@itemButtons').eq(2).contains('By profile');
    cy.get('@itemButtons').eq(2).find('.checked');
    cy.get('@itemButtons').eq(3).contains('By status');

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

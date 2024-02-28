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
    cy.get('.back-organization').find('ion-label').contains('Manage my organization');
    cy.get('.sidebar').find('.users').find('ion-item').first().contains('Users');
    cy.get('.sidebar').find('.users').find('ion-item').first().should('have.class', 'item-selected');
    cy.get('.topbar-left__title').find('.title-h2').contains('Users');
    cy.get('#activate-users-ms-action-bar').find('#grid-view').should('not.have.attr', 'disabled');
    cy.get('#activate-users-ms-action-bar').find('#list-view').should('have.attr', 'disabled');
    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').contains('Invite a user');
    cy.get('.user-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 4);
    cy.get('@userItems').eq(0).should('have.class', 'item-disabled');

    cy.get('@userItems').eq(1).find('ion-checkbox').should('not.be.visible');
    // cspell:disable-next-line
    cy.get('@userItems').eq(1).find('.person-name').contains('Cernd');
    // cspell:disable-next-line
    cy.get('@userItems').eq(1).find('.user-email__label').contains('cernd@gmail.com');
    cy.get('@userItems').eq(1).find('.label-profile').contains('Standard');
    cy.get('@userItems').eq(1).find('.label-status').contains('Active');

    // cspell:disable-next-line
    cy.get('@userItems').eq(2).find('.person-name').contains('Jaheira');
    // cspell:disable-next-line
    cy.get('@userItems').eq(2).find('.user-email__label').contains('jaheira@gmail.com');
    cy.get('@userItems').eq(2).find('.label-profile').contains('Admin');
    cy.get('@userItems').eq(2).find('.label-status').contains('Active');

    // cspell:disable-next-line
    cy.get('@userItems').eq(3).find('.person-name').contains('Valygar Corthala');
    // cspell:disable-next-line
    cy.get('@userItems').eq(3).find('.user-email__label').contains('val@gmail.com');
    cy.get('@userItems').eq(3).find('.label-profile').contains('Standard');
    cy.get('@userItems').eq(3).find('.label-status').contains('Revoked');
  });

  it('Tests selection list item', () => {
    function checkChecked(checked: boolean): void {
      for (let i = 1; i < 3; i++) {
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
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 4);
    // All unchecked by default
    checkChecked(false);

    // Options button should not be visible
    cy.get('@userItems').eq(1).find('.options-button').should('not.be.visible');
    cy.get('@userItems').eq(2).find('.options-button').should('not.be.visible');
    cy.get('@userItems').eq(3).find('.options-button').should('not.be.visible');

    cy.get('.counter').contains('4 users');

    // Select all
    cy.get('.user-list-header').find('ion-checkbox').click();
    cy.get('.user-list-header').find('ion-checkbox').should('have.class', 'checkbox-checked');
    checkChecked(true);

    cy.get('.counter').contains('3 users selected');
    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').should('not.be.visible');
    cy.get('#activate-users-ms-action-bar').find('#button-revoke-user').contains('Revoke these users');

    // Deselect [2] and [3], only [1] is selected
    cy.get('@userItems').eq(2).find('ion-checkbox').click();
    cy.get('@userItems').eq(3).find('ion-checkbox').click();
    cy.get('#activate-users-ms-action-bar').find('#button-revoke-user').contains('Revoke this user');
    cy.get('#activate-users-ms-action-bar').find('#button-common-workspaces').contains('View details');

    // Check all should be indeterminate
    cy.get('.user-list-header').find('ion-checkbox').should('have.class', 'checkbox-indeterminate');

    // Re-select all
    cy.get('.user-list-header').find('ion-checkbox').click();
    checkChecked(true);
    cy.get('.counter').contains('3 users selected');

    // Deselect all
    cy.get('.user-list-header').find('ion-checkbox').click();
    checkChecked(false);
    cy.get('.counter').contains('4 users');
  });

  it('Tests selection grid item', () => {
    cy.get('#activate-users-ms-action-bar').find('#grid-view').as('gridButton').click();

    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').contains('Invite a user');
    cy.get('.users-container-grid').find('.user-card-item').as('userItems').should('have.length', 4);

    cy.get('@userItems').eq(1).realHover().find('.checkbox').should('be.visible');

    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').should('exist');

    cy.get('@userItems').eq(2).realHover().find('.checkbox').click();
    cy.get('#activate-users-ms-action-bar').find('#button-revoke-user').contains('Revoke this user');
    cy.get('#activate-users-ms-action-bar').find('#button-common-workspaces').contains('View details');
  });

  it('Tests context menu', () => {
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 4);
    cy.get('.user-context-menu').should('not.exist');
    cy.get('@userItems').eq(2).find('.options-button').invoke('show').click();
    cy.get('.user-context-menu').should('exist');
    cy.get('.user-context-menu').find('.menu-list').find('ion-item').as('menuItems').should('have.length', 4);
    // 0 is title, 1 is revoke button
    cy.get('@menuItems').eq(0).contains('Deletion');
    cy.get('@menuItems').eq(1).contains('Revoke this user');
    cy.get('@menuItems').eq(2).contains('User details');
    cy.get('@menuItems').eq(3).contains('View details');
  });

  it('Tests revoke one user', () => {
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 4);
    cy.get('@userItems').eq(1).find('.options-button').invoke('show').click();
    cy.get('.user-context-menu').find('.menu-list').find('ion-item').eq(1).contains('Revoke this user').click();
    cy.get('.question-modal').find('.ms-modal-header__title').contains('Revoke this user?');
    cy.get('.question-modal').find('#next-button').click();
    // cspell:disable-next-line
    cy.checkToastMessage('success', 'Cernd has been revoked. They can no longer access this organization.');
  });

  it('Tests revoke one selected user', () => {
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 4);
    // Since the checkbox appears only on hover, it's easier to select all and deselect the one we don't want
    cy.get('.user-list-header').find('ion-checkbox').click();
    cy.get('@userItems').eq(2).find('ion-checkbox').click();
    cy.get('@userItems').eq(3).find('ion-checkbox').click();
    cy.get('@userItems').eq(1).find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('@userItems').eq(2).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.counter').contains('One user selected');
    cy.get('.contextual-menu').find('#button-revoke-user').contains('Revoke this user').click();
    cy.get('.question-modal').find('.ms-modal-header__title').contains('Revoke this user?');
    cy.get('.question-modal').find('#next-button').click();
    // cspell:disable-next-line
    cy.checkToastMessage('success', 'Cernd has been revoked. They can no longer access this organization.');
    // Ultimately, they probably should not appear anymore
    cy.get('@userItems').eq(1).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
  });

  it('Tests revoke multiple', () => {
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 4);
    cy.get('.user-list-header').find('ion-checkbox').click();
    cy.get('@userItems').eq(3).find('ion-checkbox').click();
    cy.get('@userItems').eq(1).find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('@userItems').eq(2).find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.contextual-menu').find('#button-revoke-user').contains('Revoke these users').click();
    cy.get('.question-modal').find('.ms-modal-header__title').contains('Revoke these users?');
    cy.get('.question-modal').find('#next-button').click();
    // cspell:disable-next-line
    cy.checkToastMessage('success', '2 users have been revoked, they can no longer access this organization.');
    // Ultimately, they probably should not appear anymore
    cy.get('@userItems').eq(1).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('@userItems').eq(2).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
  });
});

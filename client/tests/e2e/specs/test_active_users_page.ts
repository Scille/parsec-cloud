// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check active users page', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('.organization-card__manageBtn').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Checks initial status', () => {
    cy.get('.back-organization').find('ion-label').contains('Manage my organization');
    cy.get('.sidebar').find('.users').find('ion-item').first().contains('Users');
    cy.get('.sidebar').find('.users').find('ion-item').first().should('have.class', 'item-selected');
    cy.get('.sidebar').find('.user-menu').find('ion-item').as('userItems').should('have.length', 3);
    cy.get('@userItems').eq(0).contains('Active');
    cy.get('@userItems').eq(0).should('have.class', 'user-menu-selected');
    cy.get('.topbar-left__title').find('.title-h2').contains('Active users');
    cy.get('#activate-users-ms-action-bar').find('#grid-view').should('not.have.attr', 'disabled');
    cy.get('#activate-users-ms-action-bar').find('#list-view').should('have.attr', 'disabled');
    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').contains('Invite a user');
    cy.get('.user-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 3);
    cy.get('@userItems').eq(0).should('have.class', 'item-disabled');
    cy.get('@userItems').eq(1).find('ion-checkbox').should('not.be.visible');
    // cspell:disable-next-line
    cy.get('@userItems').eq(1).find('.person-name').contains('Cernd');
    // cspell:disable-next-line
    cy.get('@userItems').eq(1).find('.user-email__label').contains('cernd@gmail.com');
    cy.get('@userItems').eq(1).find('.label-profile').contains('Standard');
    // cspell:disable-next-line
    cy.get('@userItems').eq(2).find('.person-name').contains('Jaheira');
    // cspell:disable-next-line
    cy.get('@userItems').eq(2).find('.user-email__label').contains('jaheira@gmail.com');
    cy.get('@userItems').eq(2).find('.label-profile').contains('Admin');
  });

  it('Tests selection', () => {
    function checkChecked(checked: boolean): void {
      for (let i = 1; i < 3; i++) {
        cy.get('@userItems').eq(i).find('ion-checkbox').should(checked ? 'be.visible' : 'not.be.visible');
        cy.get('@userItems').eq(i).find('ion-checkbox').should(checked ? 'have.class' : 'not.have.class', 'checkbox-checked');
      }
    }

    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').contains('Invite a user');
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 3);
    // All unchecked by default
    checkChecked(false);

    cy.get('.user-footer__container').contains('3 users');
    cy.get('.user-footer__container').find('#button-revoke-user').should('not.exist');

    // Select all
    cy.get('.user-list-header').find('ion-checkbox').click();
    cy.get('.user-list-header').find('ion-checkbox').should('have.class', 'checkbox-checked');
    checkChecked(true);
    cy.get('.user-footer__container').contains('2 users selected');
    cy.get('.user-footer__container').find('#button-revoke-user').should('exist');
    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').should('not.exist');
    cy.get('#activate-users-ms-action-bar').find('#button-revoke-user').contains('Revoke these users');

    // Deselect [2], only [1] is selected
    cy.get('@userItems').eq(2).find('ion-checkbox').click();
    cy.get('#activate-users-ms-action-bar').find('#button-revoke-user').contains('Revoke this user');
    cy.get('#activate-users-ms-action-bar').find('#button-common-workspaces').contains('See common workspaces');
    cy.get('.user-footer__container').find('#button-revoke-user').should('exist');
    cy.get('.user-footer__container').find('#button-common-workspaces').should('exist');

    // Check all should be indeterminate
    cy.get('.user-list-header').find('ion-checkbox').should('have.class', 'checkbox-indeterminate');

    // Re-select all
    cy.get('.user-list-header').find('ion-checkbox').click();
    checkChecked(true);
    cy.get('.user-footer__container').contains('2 users selected');

    // Deselect all
    cy.get('.user-list-header').find('ion-checkbox').click();
    checkChecked(false);
    cy.get('.user-footer__container').contains('3 users');
  });

  it('Tests context menu', () => {
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 3);
    cy.get('.user-context-menu').should('not.exist');
    cy.get('@userItems').eq(2).find('.user-options').find('ion-button').click();
    cy.get('.user-context-menu').should('exist');
    cy.get('.user-context-menu').find('.menu-list').find('ion-item').as('menuItems').should('have.length', 4);
    // 0 is title, 1 is revoke button
    cy.get('@menuItems').eq(1).contains('Revoke');
    cy.get('@menuItems').eq(1).click();
    // cspell:disable-next-line
    cy.get('@consoleLog').should('have.been.calledWith', 'Revoke user Jaheira');
  });

  it('Invite from active user page', () => {
    cy.get('.topbar-left__title').find('.title-h2').contains('Active users');
    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').contains('Invite a user').click();
    cy.wait(500);
    cy.get('.topbar-left__title').find('.title-h2').contains('Invitations');
    cy.get('.create-user-invitation-modal').find('#next-button').as('inviteButton').should('have.attr', 'disabled');
    cy.get('.create-user-invitation-modal').find('ion-input').find('input').type('gordon.freeman@blackmesa.nm');
    cy.get('@inviteButton').should('not.have.attr', 'disabled');
    cy.get('@inviteButton').click();
    cy.checkToastMessage('An invitation to join the organization has been sent to gordon.freeman@blackmesa.nm.');
  });
});

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
    cy.get('#activate-users-ms-action-bar').find('#grid-view').should('not.have.attr', 'disabled');
    cy.get('#activate-users-ms-action-bar').find('#list-view').should('have.attr', 'disabled');
    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').contains('Invite a user');
    cy.get('.user-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 4);
    cy.get('@userItems').eq(0).find('ion-checkbox').should('not.be.visible');
    // cspell:disable-next-line
    cy.get('@userItems').eq(0).find('.person-name').contains('Cernd');
    // cspell:disable-next-line
    cy.get('@userItems').eq(0).find('.user-email__label').contains('cernd@gmail.com');
    cy.get('@userItems').eq(0).find('.label-profile').contains('Standard');
    // cspell:disable-next-line
    cy.get('@userItems').eq(3).find('.person-name').contains('Coloia Hoji');
    cy.get('.user-footer__container').contains('4 users');
  });

  it('Tests selection', () => {
    function checkChecked(checked: boolean): void {
      for (let i = 0; i < 4; i++) {
        cy.get('@userItems').eq(i).find('ion-checkbox').should(checked ? 'be.visible' : 'not.be.visible');
        cy.get('@userItems').eq(i).find('ion-checkbox').should(checked ? 'have.class' : 'not.have.class', 'checkbox-checked');
      }
    }

    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').contains('Invite a user');
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 4);
    // All unchecked by default
    checkChecked(false);

    cy.get('.user-footer__container').contains('4 users');
    cy.get('.user-footer__container').find('#button-revoke-user').should('not.exist');

    // Select all
    cy.get('.user-list-header').find('ion-checkbox').click();
    cy.get('.user-list-header').find('ion-checkbox').should('have.class', 'checkbox-checked');
    checkChecked(true);
    cy.get('.user-footer__container').contains('4 users selected');
    cy.get('.user-footer__container').find('#button-revoke-user').should('exist');
    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').should('not.exist');
    cy.get('#activate-users-ms-action-bar').find('#button-revoke-user').contains('Revoke these users');

    // Deselect [2]
    cy.get('@userItems').eq(2).find('ion-checkbox').click();
    cy.get('@userItems').eq(2).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.user-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.user-footer__container').contains('3 users selected');

    cy.get('#activate-users-ms-action-bar').find('#button-revoke-user').click();
    // cspell:disable-next-line
    cy.get('@consoleLog').should('have.been.calledWith', 'Revoke user Cernd');
    // cspell:disable-next-line
    cy.get('@consoleLog').should('have.been.calledWith', 'Revoke user Valygar Corthala');
    // cspell:disable-next-line
    cy.get('@consoleLog').should('have.been.calledWith', 'Revoke user Coloia Hoji');

    // Deselect [0] and [1], only [3] is selected
    cy.get('@userItems').eq(0).find('ion-checkbox').click();
    cy.get('@userItems').eq(1).find('ion-checkbox').click();
    cy.get('#activate-users-ms-action-bar').find('#button-revoke-user').contains('Revoke this user');
    cy.get('#activate-users-ms-action-bar').find('#button-common-workspaces').contains('See common workspaces');
    cy.get('.user-footer__container').find('#button-revoke-user').should('exist');
    cy.get('.user-footer__container').find('#button-common-workspaces').should('exist');

    // Check all should be indeterminate
    cy.get('.user-list-header').find('ion-checkbox').should('have.class', 'checkbox-indeterminate');

    // Re-select all
    cy.get('.user-list-header').find('ion-checkbox').click();
    checkChecked(true);
    cy.get('.user-footer__container').contains('4 users selected');

    // Deselect all
    cy.get('.user-list-header').find('ion-checkbox').click();
    checkChecked(false);
    cy.get('.user-footer__container').contains('4 users');
  });

  it('Tests context menu', () => {
    cy.get('.users-container').find('.user-list-item').as('userItems').should('have.length', 4);
    cy.get('.user-context-menu').should('not.exist');
    cy.get('@userItems').eq(2).find('.user-options').find('ion-button').click();
    cy.get('.user-context-menu').should('exist');
    cy.get('.user-context-menu').find('.menu-list').find('ion-item').as('menuItems').should('have.length', 4);
    // 0 is title, 1 is revoke button
    cy.get('@menuItems').eq(1).contains('Revoke');
    cy.get('@menuItems').eq(1).click();
    // cspell:disable-next-line
    cy.get('@consoleLog').should('have.been.calledWith', 'Revoke user Drizzt Do\'Urden');
  });
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check invitations page', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('.organization-card__manageBtn').click();
    cy.get('.user-menu__item').eq(2).click();
  });

  it('Check the initial state', () => {
    cy.get('.topbar-left__title').find('.title-h2').contains('Invitations');
    cy.get('.invitation-list').find('.invitation-list-item').as('invitations').should('have.length', 2);
    // cspell:disable-next-line
    cy.get('@invitations').eq(0).find('.invitation-email').contains('shadowheart@swordcoast.faerun');
    cy.get('@invitations').eq(0).find('.invitation-status').contains('Waiting');
  });

  it('Create new invitation', () => {
    cy.get('.topbar-left__title').find('.title-h2').contains('Invitations');
    cy.get('#activate-users-ms-action-bar').find('#button-invite-user').click({force: true});
    cy.get('.create-user-invitation-modal').find('#next-button').as('inviteButton').should('have.attr', 'disabled');
    cy.get('.create-user-invitation-modal').find('ion-input').find('input').type('gordon.freeman@blackmesa.nm');
    cy.get('@inviteButton').should('not.have.attr', 'disabled');
    cy.get('@inviteButton').click();
    cy.checkToastMessage('An invitation to join the organization has been sent to gordon.freeman@blackmesa.nm.');
  });
});

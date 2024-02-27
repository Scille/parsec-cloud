// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check invitations page', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('#invitations-button').contains('2 invitations').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Check the initial state', () => {
    cy.get('.invitation-list-item-container').find('.invitation-list-item').as('invitations').should('have.length', 2);

    cy.get('.invitations-list-header').find('.invitations-list-header__title').contains('Invitations');

    // cspell:disable-next-line
    cy.get('@invitations').first().find('.invitation-email').contains('shadowheart@swordcoast.faerun');
  });

  it('Create new invitation', () => {
    cy.get('.invitations-list-popover').find('.invitations-list-header__button').contains('Invite a new user').click();
    cy.get('.invitations-list-popover').should('not.exist');
    cy.get('.ms-modal-footer-buttons').find('ion-button').eq(1).as('inviteButton').contains('Invite');
    cy.get('@inviteButton').should('have.class', 'button-disabled');
    cy.get('.text-input-modal').find('ion-input').find('input').type('gordon.freeman@blackmesa.nm');
    cy.get('@inviteButton').should('not.have.class', 'button-disabled');
    cy.get('@inviteButton').click();
    cy.checkToastMessage('success', 'An invitation to join the organization has been sent to gordon.freeman@blackmesa.nm.');
  });

  it('Check copy link button', () => {
    cy.get('.invitation-list-item-container').find('.invitation-list-item').as('invitations').should('have.length', 2);
    cy.get('@invitations').first().realHover().find('.copy-link').click();
    cy.checkToastMessage('info', 'The link has been copied to the clipboard.');
    cy.window().then((win) => {
      win.navigator.clipboard.readText().then((text) => {
        expect(text).to.eq('parsec://parsec.example.com/MyOrg?action=claim_device&token=12346565645645654645645645645645');
      });
    });
  });

  it('Reject invitation', () => {
    cy.get('.invitation-list-item-container').find('.invitation-list-item').as('invitations').should('have.length', 2);
    cy.get('@invitations').first().realHover().find('.invitation-actions-buttons').first().contains('Cancel').click();
    cy.get('.question-modal').find('#next-button').contains('Cancel the invitation').click();
    cy.checkToastMessage('success', 'The invitation has been cancelled.');
  });
});

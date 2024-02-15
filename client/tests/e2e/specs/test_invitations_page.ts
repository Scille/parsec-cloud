// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check invitations page', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('.organization-card-manageBtn').click();
    cy.get('.user-menu__item').eq(2).click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Check the initial state', () => {
    cy.get('.topbar-left__title').find('.title-h2').contains('Invitations');
    cy.get('.invitation-list').find('.invitation-list-item').as('invitations').should('have.length', 2);
    // cspell:disable-next-line
    cy.get('@invitations').eq(0).find('.invitation-email').contains('shadowheart@swordcoast.faerun');
    cy.get('@invitations').eq(0).find('.invitation-status').contains('Ready');
  });

  it('Create new invitation', () => {
    // Fails sometimes
    // cy.get('.topbar-left__title').find('.title-h2').contains('Invitations');
    // cy.get('#activate-users-ms-action-bar').find('#button-invite-user').click({force: true});
    // cy.get('.ms-modal-footer-buttons').find('ion-button').eq(1).as('inviteButton').contains('Invite');
    // cy.get('@inviteButton').should('have.class', 'button-disabled');
    // cy.get('.text-input-modal').find('ion-input').find('input').type('gordon.freeman@blackmesa.nm');
    // cy.get('@inviteButton').should('not.have.class', 'button-disabled');
    // cy.get('@inviteButton').click();
    // cy.checkToastMessage('An invitation to join the organization has been sent to gordon.freeman@blackmesa.nm.');
  });

  it('Check copy link button', () => {
    cy.get('.invitation-list').find('.invitation-list-item').as('invitations').should('have.length', 2);
    cy.get('.counter').contains('2 pending invitations');
    cy.get('@invitations').eq(0).find('.invitation-email').contains('shadowheart@swordcoast.faerun');
    cy.get('@invitations').eq(0).find('ion-button').eq(0).click();
    cy.checkToastMessage('info', 'Link copied', 'The link has been copied to the clipboard.');
    cy.window().then((win) => {
      win.navigator.clipboard.readText().then((text) => {
        expect(text).to.eq('parsec://parsec.example.com/MyOrg?action=claim_device&token=12346565645645654645645645645645');
      });
    });
  });
});

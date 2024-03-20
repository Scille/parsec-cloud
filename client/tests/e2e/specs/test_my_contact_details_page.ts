// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Display client info', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('#profile-button').click();
    cy.get('.popover-viewport').contains('My contact details').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Check initial state', () => {
    cy.get('.page').find('.title-h2').contains('Gordon Freeman');
    cy.get('.page').find('.label-profile').contains('Administrator');
  });

  it('Open change password modal', () => {
    cy.get('.user-info').find('ion-input').eq(0).find('input').should('have.attr', 'disabled');
    cy.get('.user-info').find('#change-password-button').contains('Update').click();
    cy.get('.change-password-modal').should('exist');
    cy.get('.change-password-modal').find('.modal-header__title').contains('Enter your current password');
  });

  it('Changes the password', () => {
    cy.get('.page').find('.user-info').as('user-info');
    cy.get('@user-info').find('ion-input').eq(1).find('input').should('have.attr', 'disabled');
    cy.get('@user-info').find('#change-password-button').contains('Update').click();

    cy.get('.change-password-modal').as('changePasswordModal').find('.modal-header__title').contains('Enter your current password');
    cy.get('@changePasswordModal').find('ion-input').eq(0).find('input').type('P@ssw0rd.');
    cy.get('@changePasswordModal').find('#next-button').click();

    cy.get('@changePasswordModal').find('.modal-header__title').contains('Choose a new password');
    cy.get('@changePasswordModal').find('#next-button').should('have.class', 'button-disabled');
    cy.get('@changePasswordModal').find('ion-input').eq(1).find('input').type('New-P@ssw0rd.6786?6786');
    cy.get('@changePasswordModal').find('ion-input').eq(2).find('input').type('New-P@ssw0rd.6786?6786');
    cy.get('@changePasswordModal').find('#next-button').click();
    cy.checkToastMessageWithSidebar('success', 'You can log in with your new password.');
  });
});

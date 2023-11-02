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
    cy.get('.page').find('h2').contains('Gordon Freeman');
    cy.get('.page').find('.label-profile').contains('Administrator');
    cy.get('.page').find('ion-text').contains('user@host.com');
  });

  it('Changes the password', () => {
    cy.get('.page').find('.password-change').as('passwordChange').find('h3').contains('Change your password');
    cy.get('@passwordChange').find('.password-change-button').as('changeButton').should('have.class', 'button-disabled');
    cy.get('@passwordChange').find('ion-input').eq(0).find('input').type('P@ssw0rd.');
    cy.get('@changeButton').should('have.class', 'button-disabled');
    cy.get('@passwordChange').find('ion-input').eq(1).find('input').type('N3w-P@ssw0rd.1337');
    cy.get('@changeButton').should('have.class', 'button-disabled');
    cy.get('@passwordChange').find('ion-input').eq(2).find('input').type('N3w-P@ssw0rd.1337');
    cy.get('@changeButton').should('not.have.class', 'button-disabled');
    cy.get('@changeButton').click();
    cy.checkToastMessage('Your password has been updated. Make sure you remember it!');
    // Clear inputs on success
    cy.get('@passwordChange').find('ion-input').eq(0).find('input').should('have.value', '');
    cy.get('@passwordChange').find('ion-input').eq(1).find('input').should('have.value', '');
    cy.get('@passwordChange').find('ion-input').eq(2).find('input').should('have.value', '');
  });

  it('Changes the password, invalid password', () => {
    cy.get('.page').find('.password-change').as('passwordChange').find('h3').contains('Change your password');
    cy.get('@passwordChange').find('.password-change-button').as('changeButton').should('have.class', 'button-disabled');
    // cspell:disable-next-line
    cy.get('@passwordChange').find('ion-input').eq(0).find('input').type('1nval1dP@ssw0rd.');
    cy.get('@changeButton').should('have.class', 'button-disabled');
    cy.get('@passwordChange').find('ion-input').eq(1).find('input').type('N3w-P@ssw0rd.1337');
    cy.get('@changeButton').should('have.class', 'button-disabled');
    cy.get('@passwordChange').find('ion-input').eq(2).find('input').type('N3w-P@ssw0rd.1337');
    cy.get('@changeButton').should('not.have.class', 'button-disabled');
    cy.get('@changeButton').click();
    cy.checkToastMessage('Wrong password!');
    // Don't clear inputs on failure
    // cspell:disable-next-line
    cy.get('@passwordChange').find('ion-input').eq(0).find('input').should('have.value', '1nval1dP@ssw0rd.');
    cy.get('@passwordChange').find('ion-input').eq(1).find('input').should('have.value', 'N3w-P@ssw0rd.1337');
    cy.get('@passwordChange').find('ion-input').eq(2).find('input').should('have.value', 'N3w-P@ssw0rd.1337');
  });
});

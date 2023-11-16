// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Display export recovery device page', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('#profile-button').click();
    cy.get('.popover-viewport').find('ion-item').eq(2).click();
    cy.get('.restore-password-button').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Check export recovery device', () => {
    cy.get('.topbar-left__title').find('.title-h2').contains('Recovery files');
    cy.get('.recovery-container').find('.block').as('blocks').should('have.length', 2);
    cy.get('@blocks').eq(0).contains('Recovery File');
    cy.get('@blocks').eq(1).contains('Secret Key');
    cy.get('.password-input-modal').should('not.exist');
    cy.get('.recovery-container').find('#exportDevice').contains('I understand').click();
    cy.get('.password-input-modal').should('exist');
    cy.get('.password-input-modal').find('.ms-modal-header__title').contains('Password needed');
    cy.get('.password-input-modal').find('.footer-md').find('#next-button').as('okButton').contains('Validate');
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('.password-input-modal').find('#ms-password-input').find('input').type('wr0ng.');
    cy.get('@okButton').should('not.have.class', 'button-disabled').click();
    cy.checkToastMessage('Invalid password.');
    cy.get('.recovery-container').find('#exportDevice').contains('I understand').click();
    cy.get('.password-input-modal').find('#ms-password-input').find('input').type('P@ssw0rd.');
    cy.get('@okButton').should('not.have.class', 'button-disabled').click();
    cy.get('.recovery-container').find('#back-to-workspaces-button').should('not.be.visible');
    cy.get('.recovery-container').find('.block').as('blocks').should('have.length', 2);
    cy.get('@blocks').eq(0).as('thirdBlock').contains('Recovery File');
    cy.get('@blocks').eq(1).as('fourthBlock').contains('Secret Key');
    cy.get('@thirdBlock').find('#downloadButton').as('fileDownloadButton').should('not.have.class', 'button-disabled');
    cy.get('@fourthBlock').find('#downloadButton').as('keyDownloadButton').should('not.have.class', 'button-disabled');
    cy.get('@thirdBlock').contains('File downloaded').should('not.be.visible');
    cy.get('@fourthBlock').contains('File downloaded').should('not.be.visible');
    cy.get('@fileDownloadButton').click();
    cy.checkToastMessage('The recovery file was successfully downloaded');
    cy.get('@thirdBlock').find('#downloadButton').should('not.be.visible');
    cy.get('@thirdBlock').contains('File downloaded').should('be.visible');
    cy.get('@keyDownloadButton').click();
    cy.checkToastMessage('The secret key was successfully downloaded');
    cy.get('@fourthBlock').find('#downloadButton').should('not.be.visible');
    cy.get('.recovery-container').find('#back-to-workspaces-button').as('returnButton').should('be.visible');
    cy.get('@fourthBlock').contains('File downloaded').should('be.visible');
    cy.get('@returnButton').should('not.have.class', 'button-disabled').click();
  });
});

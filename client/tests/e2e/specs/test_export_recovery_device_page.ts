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
    cy.get('.recovery-container').find('.file-item').as('file-item').should('have.length', 2);
    cy.get('@file-item').eq(0).contains('Recovery File');
    cy.get('@file-item').eq(1).contains('Secret Key');
    cy.get('.password-input-modal').should('not.exist');
    cy.get('.recovery-container').find('#exportDevice').contains('I understand').click();
    cy.get('.password-input-modal').should('exist');
    cy.get('.password-input-modal').find('.ms-modal-header__title').contains('Password needed');
    cy.get('.password-input-modal').find('.footer-md').find('#next-button').as('okButton').contains('Validate');
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('.password-input-modal').find('#ms-password-input').find('input').type('wr0ng.');
    cy.get('@okButton').should('not.have.class', 'button-disabled').click();
    cy.checkToastMessage('error', 'Invalid password.');
    cy.get('.recovery-container').find('#exportDevice').contains('I understand').click();
    cy.get('.password-input-modal').find('#ms-password-input').find('input').type('P@ssw0rd.');
    cy.get('@okButton').should('not.have.class', 'button-disabled').click();
    cy.get('.download').find('#back-to-devices-button').should('not.be.visible');
    cy.get('.download').find('.file-item').as('file-item').should('have.length', 2);
    cy.get('@file-item').eq(0).as('thirdItem').contains('Recovery File');
    cy.get('@file-item').eq(1).as('fourthItem').contains('Secret Key');
    cy.get('@thirdItem').find('#downloadButton').as('fileDownloadButton').should('not.have.class', 'button-disabled');
    cy.get('@fourthItem').find('#downloadButton').as('keyDownloadButton').should('not.have.class', 'button-disabled');
    cy.get('@thirdItem').contains('File downloaded').should('not.be.visible');
    cy.get('@fourthItem').contains('File downloaded').should('not.be.visible');
    cy.get('@fileDownloadButton').click();
    cy.wait(500);
    cy.checkToastMessage('success', 'The recovery file was successfully downloaded');
    cy.get('@thirdItem').find('#downloadButton').should('be.visible');
    cy.get('@thirdItem').find('#downloadButton').contains('Download again');
    cy.get('@thirdItem').contains('File downloaded').should('be.visible');
    cy.get('@keyDownloadButton').click();
    cy.wait(500);
    cy.checkToastMessage('success', 'The secret key was successfully downloaded');
    cy.get('@fourthItem').find('#downloadButton').should('be.visible');
    cy.get('@fourthItem').find('#downloadButton').contains('Download again');
    cy.get('.recovery-container').find('#back-to-devices-button').as('returnButton').should('be.visible');
    cy.get('@fourthItem').contains('File downloaded').should('be.visible');
    cy.get('@returnButton').should('not.have.class', 'button-disabled').click();
  });
});

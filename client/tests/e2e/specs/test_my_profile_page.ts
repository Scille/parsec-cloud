// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Display client info', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('#profile-button').click();
    cy.get('.popover-viewport').contains('My profile').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Check device list', () => {
    cy.get('.devices-container').find('ion-button').contains('Add');
    cy.get('.devices-container').find('ion-item').as('devices').should('have.length', 3);
    cy.get('@devices').first().find('ion-text').contains('Web');
    cy.get('@devices').first().find('ion-text').contains('Current');
  });

  // check if restore-password section is displayed
  it('Check if restore-password section is displayed', () => {
    cy.get('.restore-password').find('h3').contains('Password recovery files');
    cy.get('.restore-password').find('ion-label').should('not.have.class', 'danger');
    cy.get('.restore-password').find('.restore-password-button').find('ion-button').contains('Re-download recovery files');
    // TODO check section has changed after downloading when correctly implemented
    // Currently defaults to 're-download files' page
  });

  it('Open authentication section', () => {
    cy.get('ion-radio').should('have.length', 2).eq(1).click();
    cy.get('.user-info').find('ion-input').eq(0).find('input').should('have.attr', 'disabled');
    cy.get('.user-info').find('#change-password-button').contains('Update').click();
    cy.get('.change-password-modal').should('exist');
    cy.get('.change-password-modal').find('.modal-header__title').contains('Enter your current password');
  });

  it('Change password', () => {
    cy.get('ion-radio').should('have.length', 2).eq(1).click();
    cy.get('.page').find('.user-info').as('user-info');
    cy.get('@user-info').find('ion-input').find('input').should('have.attr', 'disabled');
    cy.get('@user-info').find('#change-password-button').contains('Update').click();

    cy.get('.change-password-modal').as('changePasswordModal').find('.modal-header__title').contains('Enter your current password');
    cy.get('@changePasswordModal').find('ion-input').eq(0).find('input').type('P@ssw0rd.');
    cy.get('@changePasswordModal').find('#next-button').click();

    cy.get('@changePasswordModal').find('.modal-header__title').contains('Choose a new password');
    cy.get('@changePasswordModal').find('#next-button').should('have.class', 'button-disabled');
    cy.get('@changePasswordModal').find('ion-input').eq(1).find('input').type('New-P@ssw0rd.6786?6786');
    cy.get('@changePasswordModal').find('ion-input').eq(2).find('input').type('New-P@ssw0rd.6786?6786');
    cy.get('@changePasswordModal').find('#next-button').click();
    cy.checkToastMessage('success', 'You can log in with your new password.');
  });
});

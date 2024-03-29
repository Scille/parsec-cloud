// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Display import recovery device page', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.contains('Alicey McAliceFace').click();
    cy.get('#forgotten-password-button').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  //   it('Check import recovery device', () => {
  //     // First state: file upload and secret key input
  //     cy.get('.recovery-content').as('r-content').find('.recovery-header__title').contains('Forgotten password');
  //     cy.get('@r-content').find('#warning-text').contains('You must have created recovery files in order to reset your password.');
  //     cy.get('@r-content').find('#to-password-change-btn').should('have.class', 'button-disabled');
  //     cy.get('@r-content').find('.recovery-list-item').as('file-item').should('have.length', 2);
  //     cy.get('@file-item').eq(0).as('file').contains('Recovery file');
  //     cy.get('@file').contains('No file selected');
  //     // Upload invalid file
  //     cy.wait(200);
  //     cy.get('@file').find('#browse-button').click();
  //     cy.get('input[type="file"]').attachFile('splash.png');
  //     cy.get('@file').contains('splash.png');
  //     // Input incomplete secret key
  //     cy.get('@file-item').eq(1).as('key').contains('Secret key');
  //     cy.get('@key').find('#secret-key-input').as('key-input').type('ABCD', { delay: 0 });
  //     cy.get('@key').find('#checkmark-icon').should('not.be.visible');
  //     cy.get('@r-content').find('#to-password-change-btn').should('have.class', 'button-disabled');
  //     // Input 64-char secret key with one invalid character
  //     // cspell:disable-next-line
  //     cy.get('@key-input').type('-EFGH-IJKL-MNOP-QRST-UVWX-YZ12-3456-7890-ABCD-EFGH-IJKL-MNOp', { delay: 0 });
  //     cy.get('@key').find('#checkmark-icon').should('not.be.visible');
  //     // Input valid key but not the expected one
  //     cy.get('@key-input').type('{backspace}O');
  //     cy.get('@key').find('#checkmark-icon').should('be.visible');
  //     cy.get('@r-content').find('#to-password-change-btn').as('toPassword').should('not.have.class', 'button-disabled').click();
  //     cy.checkToastMessage('error', 'The secret key does not match the recovery file.');
  //     // Input valid and expected secret key
  //     cy.get('@key-input').type('{backspace}P');
  //     cy.get('@toPassword').click();
  //     cy.checkToastMessage('error', 'Invalid recovery file.');
  //     // Replace uploaded file with valid one
  //     cy.get('@file').find('#browse-button').click();
  //     cy.get('input[type="file"]').attachFile('recoveryfile.psrk');
  //     cy.get('@file').contains('recoveryfile.psrk');
  //     cy.get('@toPassword').click();
  //     // Second state: password input
  //     cy.get('.info__text').contains(
  //       'Your password must be secure and you must remember it. \
  // We advise you to create several devices if possible and to share workspaces.',
  //     );
  //     cy.get('.password-criteria').contains(
  //       'Make sure it corresponds to a strong level: avoid repetitions, overly common words and sequences.',
  //     );
  //     cy.get('.choose-password').find('.inputs-container-item').as('password-containers').should('have.length', 2);
  //     cy.get('@password-containers').eq(0).should('contain', 'Choose a new password').find('ion-input').as('first-password-input');
  //     // Input password first characters to test weak password recognition
  //     cy.get('@first-password-input').find('input').type('A583n.');
  //     cy.get('@password-containers').eq(1).should('contain', 'Confirm password').find('ion-input').find('input').type('A583n.x@f3.');
  //     cy.get('@password-containers').eq(1).contains('Do not match');
  //     cy.get('.password-level').find('.password-level-container').should('have.class', 'password-level-low');
  //     cy.get('#validate-password-btn').should('have.class', 'button-disabled');
  //     // Have same strong password in both input fields
  //     cy.get('@first-password-input').find('input').type('x@f3.');
  //     cy.get('@password-containers').eq(1).should('not.contain', 'Do not match');
  //     cy.get('.password-level').find('.password-level-container').should('have.class', 'password-level-high');
  //     cy.get('#validate-password-btn').should('not.have.class', 'button-disabled').click();
  //     // Third state: validation
  //     cy.get('#success-step')
  //       .should('contain', 'Password was successfully changed!')
  //       .and('contain', 'You can now login with your new password.');
  //     cy.get('#success-step').find('ion-button').contains('Go back to login').click();
  //     cy.get('#forgotten-password-button').should('exist');
  //   });
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check settings modal', () => {
  beforeEach(() => {
    cy.visitApp('coolorg');
    cy.contains('Your organizations');
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Open the settings modal from homepage', () => {
    cy.get('#trigger-settings-button').click();
    // Cypress bug without waiting
    cy.wait(200);
    cy.get('.modal-default').should('exist');
    cy.get('ion-modal').get('ion-title').contains('Settings');
    cy.wait(200);
  });

  it('Open the settings modal after login', () => {
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('#profile-button').click();
    cy.get('.popover-viewport').contains('Settings').click();
    cy.get('ion-modal').get('ion-title').contains('Settings');
  });

  it('Check General tab', () => {
    cy.get('#trigger-settings-button').click();
    cy.get('ion-radio-group').invoke('attr', 'modelvalue').should('eq', 'General');
    cy.get('ion-radio').first().should('have.class', 'radio-checked');
    cy.get('.settings-option').first().as('language');
    cy.get('@language').find('ion-text.title').contains('Language');
    cy.get('@language').find('ion-text.description').contains('Choose application language');
    cy.get('@language').find('#dropdown-popover-button').contains('English');
    cy.get('.settings-option').eq(1).as('theme');
    cy.get('@theme').find('ion-text.title').contains('Theme');
    cy.get('@theme').find('ion-text.description').contains('Choose application theme');
    cy.get('@theme')
      .find('#dropdown-popover-button')
      .contains(/Light|Dark|System/g);
  });

  // it('Check Advanced tab', () => {
  // cy.get('#trigger-settings-button').click();
  //   cy.get('ion-radio').eq(1).click();
  //   cy.get('ion-radio-group').invoke('attr', 'modelvalue').should('eq', 'Advanced');
  //   cy.get('ion-radio').eq(1).should('have.class', 'radio-checked');

  //   cy.get('.settings-option').first().as('telemetry');
  //   cy.get('@telemetry').find('ion-toggle').should('have.class', 'toggle-checked');
  //   cy.get('@telemetry').find('ion-toggle').click();
  //   cy.get('@telemetry').find('ion-toggle').should('not.have.class', 'toggle-checked');
  //   cy.get('@telemetry').find('ion-toggle').click();
  //   cy.get('@telemetry').find('ion-toggle').should('have.class', 'toggle-checked');

  //   cy.get('.settings-option').eq(1).as('sync');
  //   cy.get('@sync').find('ion-toggle').should('not.have.class', 'toggle-checked');
  //   cy.get('@sync').find('ion-toggle').click();
  //   cy.get('@sync').find('ion-toggle').should('have.class', 'toggle-checked');
  //   cy.get('@sync').find('ion-toggle').click();
  //   cy.get('@sync').find('ion-toggle').should('not.have.class', 'toggle-checked');
  // });

  it('Close with X', () => {
    cy.get('#trigger-settings-button').click();
    cy.get('.modal-default').should('exist');
    cy.get('.closeBtn').click();
    cy.get('.modal-default').should('not.exist');
  });
});

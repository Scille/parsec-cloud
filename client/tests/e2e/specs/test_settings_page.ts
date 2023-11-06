// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check settings page', () => {
  beforeEach(() => {
    cy.visitApp('coolorg');
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('#profile-button').click();
    cy.get('.popover-viewport').contains('Settings').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Opens the settings page', () => {
    cy.get('.title-h2').contains('Settings');
  });

  it('Check General tab', () => {
    cy.get('ion-radio-group').invoke('attr', 'modelvalue').should('eq', 'General');
    cy.get('ion-radio').first().should('have.class', 'radio-checked');
    cy.get('.settings-option').first().as('language');
    cy.get('@language').find('ion-text.title').contains('Language');
    cy.get('@language').find('ion-text.description').contains('Choose the application language');
    cy.get('@language').find('#dropdown-popover-button').contains('English');
    cy.get('.settings-option').eq(1).as('theme');
    cy.get('@theme').find('ion-text.title').contains('Theme');
    cy.get('@theme').find('ion-text.description').contains('Choose the application appearance');
    cy.get('@theme').find('#dropdown-popover-button').contains(/Light|Dark|System/g);
  });

  it('Check Advanced tab', () => {
    cy.get('ion-radio').eq(1).click();
    cy.get('ion-radio-group').invoke('attr', 'modelvalue').should('eq', 'Advanced');
    cy.get('ion-radio').eq(1).should('have.class', 'radio-checked');

    cy.get('.settings-option').first().as('telemetry');
    cy.get('@telemetry').find('ion-toggle').should('have.class', 'toggle-checked');
    cy.get('@telemetry').find('ion-toggle').click();
    cy.get('@telemetry').find('ion-toggle').should('not.have.class', 'toggle-checked');
    cy.get('@telemetry').find('ion-toggle').click();
    cy.get('@telemetry').find('ion-toggle').should('have.class', 'toggle-checked');

    cy.get('.settings-option').eq(1).as('sync');
    cy.get('@sync').find('ion-toggle').should('not.have.class', 'toggle-checked');
    cy.get('@sync').find('ion-toggle').click();
    cy.get('@sync').find('ion-toggle').should('have.class', 'toggle-checked');
    cy.get('@sync').find('ion-toggle').click();
    cy.get('@sync').find('ion-toggle').should('not.have.class', 'toggle-checked');
  });

  it('Go back to workspaces', () => {
    cy.get('.topbar-left__breadcrumb').should('not.contain', 'My workspaces');
    cy.get('.topbar-left').find('.back-button').click();
    cy.get('.topbar-left__breadcrumb').should('contain', 'My workspaces');
  });
});

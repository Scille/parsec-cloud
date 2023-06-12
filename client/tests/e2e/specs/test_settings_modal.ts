// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

describe('Check settings modal', () => {
  beforeEach(() => {
    cy.visitApp('coolorg');
    cy.contains('Your organizations');
    cy.get('#trigger-settings-button').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Opens the settings dialog', () => {
    cy.get('.modal-default').should('exist');
    cy.get('ion-modal').get('ion-title').contains('Settings');
  });

  it('Check General tab', () => {
    cy.get('ion-radio-group').invoke('attr', 'modelvalue').should('eq', 'General');
    cy.get('ion-radio').first().should('have.class', 'radio-checked');
    cy.get('ion-select').first().shadow().as('langShadow');
    cy.get('@langShadow').find('.label-text').contains('Language');
    cy.get('@langShadow').find('.select-text').contains('English');
    cy.get('ion-select').eq(1).shadow().as('themeShadow');
    cy.get('@themeShadow').find('.label-text').contains('Theme');
    cy.get('@themeShadow').find('.select-text').contains(/Light|Dark|System/g);
  });

  it('Check Advanced tab', () => {
    cy.get('ion-radio').eq(1).click();
    cy.get('ion-radio-group').invoke('attr', 'modelvalue').should('eq', 'Advanced');
    cy.get('ion-radio').eq(1).should('have.class', 'radio-checked');

    cy.get('.toggle-settings').first().as('telemetry');
    cy.get('@telemetry').find('ion-toggle').should('have.class', 'toggle-checked');
    cy.get('@telemetry').find('ion-toggle').click();
    cy.get('@telemetry').find('ion-toggle').should('not.have.class', 'toggle-checked');
    cy.get('@telemetry').find('ion-toggle').click();
    cy.get('@telemetry').find('ion-toggle').should('have.class', 'toggle-checked');

    cy.get('.toggle-settings').eq(1).as('sync');
    cy.get('@sync').find('ion-toggle').should('have.class', 'toggle-checked');
    cy.get('@sync').find('ion-toggle').click();
    cy.get('@sync').find('ion-toggle').should('not.have.class', 'toggle-checked');
    cy.get('@sync').find('ion-toggle').click();
    cy.get('@sync').find('ion-toggle').should('have.class', 'toggle-checked');
  });

  it('Close with X', () => {
    cy.get('.modal-default').should('exist');
    cy.get('.closeBtn').click();
    cy.get('.modal-default').should('not.exist');
  });
});

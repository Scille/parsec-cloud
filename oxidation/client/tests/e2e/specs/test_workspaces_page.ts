// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

describe('Check workspaces page', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd');
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Checks initial status', () => {
    cy.get('#button-new-workspace').contains('New workspace');
    cy.get('#filter-select').contains('Name');
    cy.get('#grid-view').should('have.attr', 'disabled');
    cy.get('#list-view').should('not.have.attr', 'disabled');
    cy.get('.card').should('have.length', 5);
    cy.get('.workspace-list-item').should('have.length', 0);
    cy.get('.card').first().contains('Druid Grove');
  });

  it('Sort workspaces in grid view', () => {
    cy.get('.card').first().contains('Druid Grove');
    cy.get('.card').last().contains('Trademeet');
    cy.get('#filter-select').click();
    cy.get('.popover-viewport').contains('Size').click();
    cy.get('.card').first().contains('Druid Grove');
    cy.get('.card').last().contains('The Copper Coronet');
    cy.get('#filter-select').click();
    cy.get('.popover-viewport').contains('Last update').click();
    cy.get('#filter-select').contains('Last update');
    cy.get('.card').first().contains('Trademeet');
    cy.get('.card').last().contains('The Asylum');
  });

  it('Switch views', () => {
    cy.get('.card').should('have.length', 5);
    cy.get('#list-view').click();
    cy.get('#grid-view').should('not.have.attr', 'disabled');
    cy.get('#list-view').should('have.attr', 'disabled');
    cy.get('.card').should('have.length', 0);
    cy.get('.workspace-list-item').should('have.length', 5);
  });

  it('Sort workspaces in list view', () => {
    cy.get('#list-view').click();
    cy.get('.workspace-list-item').should('have.length', 5);
    cy.get('.workspace-list-item').first().contains('Druid Grove');
    cy.get('.workspace-list-item').last().contains('Trademeet');
    cy.get('#filter-select').click();
    cy.get('.popover-viewport').contains('Size').click();
    cy.get('.workspace-list-item').first().contains('Druid Grove');
    cy.get('.workspace-list-item').last().contains('The Copper Coronet');
    cy.get('#filter-select').click();
    cy.get('.popover-viewport').contains('Last update').click();
    cy.get('#filter-select').contains('Last update');
    cy.get('.workspace-list-item').first().contains('Trademeet');
    cy.get('.workspace-list-item').last().contains('The Asylum');
  });

  it('Navigate into a workspace', () => {
    cy.contains('Trademeet').click();
    cy.get('h2').last().should('have.text', 'Path is /how/boring/and/small');
    cy.get('ion-button.back-button').click();
    cy.get('.card').should('have.length', 5);
  });

  it('Open workspace menu in grid view', () => {
    cy.get('.card-option').first().click();
    cy.get('.popover-viewport').get('.group-item').should('have.length', 7);
    cy.get('.popover-viewport').get('.group-title').should('have.length', 3);
  });

  it('Open workspace menu in list view', () => {
    cy.get('#list-view').click();
    cy.get('.workspace-options > ion-button').first().click();
    cy.get('.popover-viewport').get('.group-item').should('have.length', 7);
    cy.get('.popover-viewport').get('.group-title').should('have.length', 3);
  });
});

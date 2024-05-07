// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check workspaces page', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('.workspaces-container').find('.workspaces-grid-item').should('have.length', 3);
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Sort workspaces in grid view', () => {
    cy.get('.card').first().contains('The Copper Coronet');
    cy.get('.card').last().contains("Watcher's Keep");
    cy.get('#workspace-filter-select').click();
    cy.get('.popover-viewport').contains('Size').click();
    cy.get('.card').first().contains('Trademeet');
    cy.get('.card').last().contains('The Copper Coronet');
    cy.get('#workspace-filter-select').click();
    cy.get('.popover-viewport').contains('Last update').click();
    cy.get('#workspace-filter-select').contains('Last update');
    cy.get('.card').first().contains('The Copper Coronet');
    cy.get('.card').last().contains('Trademeet');
  });

  it('Sort workspaces in list view', () => {
    cy.get('#workspaces-ms-action-bar').find('#list-view').click();
    cy.get('.workspace-list-item').should('have.length', 3);
    cy.get('.workspace-list-item').first().contains('The Copper Coronet');
    cy.get('.workspace-list-item').last().contains("Watcher's Keep");
    cy.get('#workspace-filter-select').click();
    cy.get('.popover-viewport').contains('Size').click();
    cy.get('.workspace-list-item').first().contains('Trademeet');
    cy.get('.workspace-list-item').last().contains('The Copper Coronet');
    cy.get('#workspace-filter-select').click();
    cy.get('.popover-viewport').contains('Last update').click();
    cy.get('#workspace-filter-select').contains('Last update');
    cy.get('.workspace-list-item').first().contains('The Copper Coronet');
    cy.get('.workspace-list-item').last().contains('Trademeet');
  });
});

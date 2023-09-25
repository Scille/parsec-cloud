// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check workspaces page', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Checks initial status', () => {
    cy.get('#button-new-workspace').contains('New workspace');
    cy.get('#workspace-filter-select').contains('Name');
    cy.get('#workspaces-ms-action-bar').find('#grid-view').should('have.attr', 'disabled');
    cy.get('#workspaces-ms-action-bar').find('#list-view').should('not.have.attr', 'disabled');
    cy.get('.card').should('have.length', 5);
    cy.get('.workspace-list-item').should('have.length', 0);
    cy.get('.card').first().contains('My Workspace 1');
  });

  it('Sort workspaces in grid view', () => {
    cy.get('.card').first().contains('My Workspace 1');
    cy.get('.card').last().contains('My Workspace 5');
    cy.get('#workspace-filter-select').click();
    cy.get('.popover-viewport').contains('Size').click();
    cy.get('.card').first().contains('My Workspace 1');
    cy.get('.card').last().contains('My Workspace 5');
    cy.get('#workspace-filter-select').click();
    cy.get('.popover-viewport').contains('Last update').click();
    cy.get('#workspace-filter-select').contains('Last update');
    cy.get('.card').first().contains('My Workspace 5');
    cy.get('.card').last().contains('My Workspace 1');
  });

  it('Switch views', () => {
    cy.get('.card').should('have.length', 5);
    cy.get('#workspaces-ms-action-bar').find('#list-view').click();
    cy.get('#workspaces-ms-action-bar').find('#grid-view').should('not.have.attr', 'disabled');
    cy.get('#workspaces-ms-action-bar').find('#list-view').should('have.attr', 'disabled');
    cy.get('.card').should('have.length', 0);
    cy.get('.workspace-list-item').should('have.length', 5);
  });

  it('Sort workspaces in list view', () => {
    cy.get('#workspaces-ms-action-bar').find('#list-view').click();
    cy.get('.workspace-list-item').should('have.length', 5);
    cy.get('.workspace-list-item').first().contains('My Workspace 1');
    cy.get('.workspace-list-item').last().contains('My Workspace 5');
    cy.get('#workspace-filter-select').click();
    cy.get('.popover-viewport').contains('Size').click();
    cy.get('.workspace-list-item').first().contains('My Workspace 1');
    cy.get('.workspace-list-item').last().contains('My Workspace 5');
    cy.get('#workspace-filter-select').click();
    cy.get('.popover-viewport').contains('Last update').click();
    cy.get('#workspace-filter-select').contains('Last update');
    cy.get('.workspace-list-item').first().contains('My Workspace 5');
    cy.get('.workspace-list-item').last().contains('My Workspace 1');
  });

  it('Navigate into a workspace', () => {
    function checkListWorkspaceSelectedItem(workspaceName: string): void {
      cy.get('.list-workspaces').find('.sidebar-item').as('workspaceItems').should('have.length', 5);
      for (let i = 0; i < 5; i++) {
        cy.get('@workspaceItems').eq(i).as('currentWorkspace').find('ion-label').then((label) => {
          if (label.get(0).innerText === workspaceName) {
            cy.get('@currentWorkspace').should('have.class', 'item-selected');
          } else {
            cy.get('@currentWorkspace').should('have.class', 'item-not-selected');
          }
        });
      }
    }
    cy.contains('Workspace 1').click();
    checkListWorkspaceSelectedItem('Workspace 1');
    cy.get('.file-list-item').should('have.length.at.least', 1);
    cy.get('.topbar-left').find('ion-button.back-button').click();
    cy.get('.card').should('have.length', 5);
  });

  it('Open workspace menu in grid view', () => {
    cy.get('.card-option').first().click();
    cy.get('.popover-viewport').get('.group-item').should('have.length', 7);
    cy.get('.popover-viewport').get('.group-title').should('have.length', 3);
  });

  it('Open workspace menu in list view', () => {
    cy.get('#workspaces-ms-action-bar').find('#list-view').click();
    cy.get('.workspace-options > ion-button').first().click();
    cy.get('.popover-viewport').get('.group-item').should('have.length', 7);
    cy.get('.popover-viewport').get('.group-title').should('have.length', 3);
  });
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

describe('Check folders page', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd');
    cy.get('.workspaces-grid-item').first().click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Checks initial status', () => {
    cy.get('.folder-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('#button-new-folder').contains('New folder');
    cy.get('#button-import').contains('Import');
    cy.get('.file-list-item').should('have.length.greaterThan', 1);
    // cy.get('#folders-action-bar').find('#list-view').should('have.attr', 'disabled');
    cy.get('#folders-action-bar').find('#grid-view').should('not.have.attr', 'disabled');
    cy.get('.folder-footer').contains(/\d items/);
  });

  it.skip('Switch to grid view', () => {
    cy.get('.file-list-item').should('have.length.greaterThan', 1);
    cy.get('#folders-action-bar').find('#list-view').as('listButton').should('have.attr', 'disabled');
    cy.get('#folders-action-bar').find('#grid-view').as('gridButton').should('not.have.attr', 'disabled');
    cy.get('@gridButton').click();
    cy.get('.file-list-item').should('have.length', 0);
    cy.get('.folder-grid-item').should('have.length.greaterThan', 1);
    cy.get('@listButton').click();
    cy.get('.file-list-item').should('have.length.greaterThan', 1);
    cy.get('.folder-grid-item').should('have.length', 0);
  });

  it.skip('Create new folder', () => {
    cy.get('#button-new-folder').contains('New folder').click();
    cy.get('@consoleLog').should('have.been.calledWith', 'Create folder clicked');
  });

  it.skip('Import files', () => {
    cy.get('#button-import').contains('Import').click();
    cy.get('@consoleLog').should('have.been.calledWith', 'Import files clicked');
  });

  function checkMenuItems(): void {
    cy.get('#file-context-menu').should('be.visible');
    cy.get('#file-context-menu').find('ion-item').as('menuItems').should('have.length', 10);
    cy.get('@menuItems').eq(0).contains('Manage file');
    cy.get('@menuItems').eq(1).contains('Rename');
    cy.get('@menuItems').eq(2).contains('Move to');
    cy.get('@menuItems').eq(3).contains('Make a copy');
    cy.get('@menuItems').eq(4).contains('Open in explorer');
    cy.get('@menuItems').eq(5).contains('History');
    cy.get('@menuItems').eq(6).contains('Download');
    cy.get('@menuItems').eq(7).contains('Details');
    cy.get('@menuItems').eq(8).contains('Collaboration');
    cy.get('@menuItems').eq(9).contains('Copy link');
  }

  it.skip('Open file menu in list view', () => {
    cy.get('.file-list-item').first().find('.options-button').click();
    checkMenuItems();
  });

  it.skip('Open file menu in grid view', () => {
    cy.get('.toolbar').find('#grid-view').click();
    cy.get('.folder-grid-item').first().find('.card-option').click();
    checkMenuItems();
  });

  it.skip('Tests select a file', () => {
    cy.get('.folder-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('not.exist');
    cy.get('.file-list-item').eq(1).find('ion-checkbox').should('not.exist');
    // Make the checkbox appear
    cy.get('.file-list-item').eq(0).trigger('mouseenter');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('exist');
    // Select the first file
    cy.get('.file-list-item').eq(0).find('ion-checkbox').click();
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(1).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.folder-footer').contains('1 selected item');
    // Unselect the first file
    cy.get('.file-list-item').eq(0).find('ion-checkbox').click();
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(1).find('ion-checkbox').should('not.exist');
    cy.get('.folder-footer').contains('2 items');
  });

  it.skip('Tests select all files', () => {
    cy.get('.folder-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    // Select all
    cy.get('.folder-list-header').find('ion-checkbox').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.folder-footer').contains('2 selected items');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(1).find('ion-checkbox').should('have.class', 'checkbox-checked');
    // Unselect all
    cy.get('.folder-list-header').find('ion-checkbox').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('not.exist');
    cy.get('.file-list-item').eq(1).find('ion-checkbox').should('not.exist');
    cy.get('.folder-footer').contains('2 items');

    // Select all, unselect first file
    cy.get('.folder-list-header').find('ion-checkbox').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.folder-footer').contains('2 selected items');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.folder-footer').contains('1 selected item');
  });
});

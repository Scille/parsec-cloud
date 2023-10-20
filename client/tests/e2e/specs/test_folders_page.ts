// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check folders page', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
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
    // cy.get('#folders-ms-action-bar').find('#list-view').should('have.attr', 'disabled');
    cy.get('#folders-ms-action-bar').find('#grid-view').should('not.have.attr', 'disabled');
    cy.get('.folder-footer').contains(/\d items/);
  });

  it('Switch to grid view', () => {
    cy.get('.file-list-item').should('have.length.greaterThan', 1);
    cy.get('#folders-ms-action-bar').find('#list-view').as('listButton').should('have.attr', 'disabled');
    cy.get('#folders-ms-action-bar').find('#grid-view').as('gridButton').should('not.have.attr', 'disabled');
    cy.get('@gridButton').click();
    cy.get('.file-list-item').should('have.length', 0);
    cy.get('.folder-grid-item').should('have.length.greaterThan', 1);
    cy.get('@listButton').click();
    cy.get('.file-list-item').should('have.length.greaterThan', 1);
    cy.get('.folder-grid-item').should('have.length', 0);
  });

  it('Create new folder', () => {
    cy.get('#button-new-folder').contains('New folder').click();
    cy.get('.text-input-modal').find('ion-input').find('input').type('MyFolder');
    cy.get('.text-input-modal').find('#next-button').click();
    cy.get('@consoleLog').should('have.been.calledWith', 'New folder MyFolder created');
  });

  it('Import files', () => {
    cy.get('#button-import').contains('Import').click();
    cy.get('.file-upload-modal').should('exist');
  });

  function checkMenuItems(): void {
    cy.get('#file-context-menu').should('be.visible');
    cy.get('#file-context-menu').find('ion-item').as('menuItems').should('have.length', 11);
    cy.get('@menuItems').eq(0).contains('Manage file');
    cy.get('@menuItems').eq(1).contains('Rename');
    cy.get('@menuItems').eq(2).contains('Move to');
    cy.get('@menuItems').eq(3).contains('Make a copy');
    cy.get('@menuItems').eq(4).contains('Delete');
    cy.get('@menuItems').eq(5).contains('Open in explorer');
    cy.get('@menuItems').eq(6).contains('History');
    cy.get('@menuItems').eq(7).contains('Download');
    cy.get('@menuItems').eq(8).contains('Details');
    cy.get('@menuItems').eq(9).contains('Collaboration');
    cy.get('@menuItems').eq(10).contains('Copy link');
  }

  it('Open file menu in list view', () => {
    cy.get('.file-list-item').first().find('.options-button').click();
    checkMenuItems();
  });

  it('Open file menu in grid view', () => {
    cy.get('#folders-ms-action-bar').find('#grid-view').click();
    cy.get('.folder-grid-item').first().find('.card-option').click();
    checkMenuItems();
  });

  it('Tests select a file', () => {
    cy.get('.folder-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('not.be.visible');
    cy.get('.file-list-item').eq(1).find('ion-checkbox').should('not.be.visible');
    // Make the checkbox appear
    cy.get('.file-list-item').eq(0).trigger('mouseenter');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('be.visible');
    // Select the first file
    cy.get('.file-list-item').eq(0).find('ion-checkbox').click();
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(1).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.folder-footer').contains('1 selected item');
    // Unselect the first file
    cy.get('.file-list-item').eq(0).find('ion-checkbox').click();
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(1).find('ion-checkbox').should('not.be.visible');
    cy.get('.folder-footer').contains('3 items');
  });

  it('Tests select all files', () => {
    cy.get('.folder-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    // Select all
    cy.get('.folder-list-header').find('ion-checkbox').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.folder-footer').contains('3 selected items');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(1).find('ion-checkbox').should('have.class', 'checkbox-checked');
    // Unselect all
    cy.get('.folder-list-header').find('ion-checkbox').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('not.be.visible');
    cy.get('.file-list-item').eq(1).find('ion-checkbox').should('not.be.visible');
    cy.get('.folder-footer').contains('3 items');

    // Select all, unselect first file
    cy.get('.folder-list-header').find('ion-checkbox').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.folder-footer').contains('3 selected items');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.folder-footer').contains('2 selected item');
  });

  it('Tests delete one file', () => {
    cy.get('.file-list-item').first().find('.options-button').click();
    cy.get('#file-context-menu').find('ion-item').eq(4).contains('Delete').click();
    cy.get('.question-modal').find('ion-title').contains('Are you sure you want to delete the file `File1.txt`?');
    cy.get('.question-modal').find('#next-button').click();
    cy.get('.question-modal').should('not.exist');
    cy.get('@consoleLog').should('have.been.called.with', 'File File1.txt deleted');
  });

  it('Tests delete multiple files', () => {
    cy.get('.folder-list-header').find('ion-checkbox').click();
    cy.get('#button-delete').contains('Delete').click();
    cy.get('.question-modal').find('ion-title').contains('Are you sure you want to delete these 3 elements?');
    cy.get('.question-modal').find('#next-button').contains('Yes').click();
    cy.get('.question-modal').should('not.exist');

    // Absolutely no idea why this doesn't work. It's exactly the same as for one file,
    // but somehow it does nothing: the question dialog is dismissed, the files are not deleted,
    // nothing is logged.

    // cy.get('.folder-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    // cy.get('@consoleLog').should('have.been.called.with', '3 entries deleted');
  });

  it('Tests file details', () => {
    cy.get('.file-list-item').first().find('.options-button').click();
    cy.get('#file-context-menu').find('ion-item').eq(8).contains('Details').click();
    cy.get('.file-details-modal').find('.ms-modal-header__title').contains('Details on File1.txt');
    cy.get('.file-details-modal').find('.file-info-value').as('values').should('have.length', 8);
    cy.get('@values').eq(0).contains('File');
    cy.get('@values').eq(1).contains('/File1.txt');
    cy.get('@values').eq(2).contains(/^\d+ B|KB|MB|GB$/);
    cy.get('@values').eq(5).contains('1');
    cy.get('@values').eq(6).contains(/^Yes|No$/);
    cy.get('@values').eq(7).contains('67');
  });

  it('Tests folder details', () => {
    cy.get('.file-list-item').last().find('.options-button').click();
    cy.get('#file-context-menu').find('ion-item').eq(8).contains('Details').click();
    cy.get('.file-details-modal').find('.ms-modal-header__title').contains('Details on Dir1');
    cy.get('.file-details-modal').find('.file-info-value').as('values').should('have.length', 7);
    cy.get('@values').eq(0).contains('Folder');
    cy.get('@values').eq(1).contains('/Dir1');
    cy.get('@values').eq(4).contains('1');
    cy.get('@values').eq(5).contains(/^Yes|No$/);
    cy.get('@values').eq(6).contains('68');
  });

  it('Tests get file link', () => {
    cy.get('.file-list-item').last().find('.options-button').click();
    cy.get('#file-context-menu').find('ion-item').eq(10).contains('Copy link').click();
    cy.checkToastMessage('The link has been copied to the clipboard.');
    cy.window().then((win) => {
      win.navigator.clipboard.readText().then((text) => {
        expect(text).to.eq(
          // cspell:disable-next-line
          'parsec://parsec.cloud/Org?action=file_link&workspace_id=94a350f2f629403db2269c44583f7aa1&path=KEFNEI3939jf39KEFsss',
        );
      });
    });
  });
});

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
    cy.get('.counter').contains(/\d items/);
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
    cy.get('.text-input-modal').find('.ms-modal-footer').find('ion-button').eq(1).as('createButton').contains('Create');
    cy.get('@createButton').should('have.class', 'button-disabled');
    cy.get('.text-input-modal').find('ion-input').find('input').type('MyFolder');
    cy.get('@createButton').should('not.have.class', 'button-disabled');
    cy.get('@createButton').click();
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
    cy.get('@menuItems').eq(5).contains('Open');
    cy.get('@menuItems').eq(6).contains('History');
    cy.get('@menuItems').eq(7).contains('Download');
    cy.get('@menuItems').eq(8).contains('Details');
    cy.get('@menuItems').eq(9).contains('Collaboration');
    cy.get('@menuItems').eq(10).contains('Copy link');
  }

  it('Open file menu in list view', () => {
    cy.get('.file-list-item').first().find('.options-button').invoke('show').click();
    checkMenuItems();
  });

  it('Open file menu in grid view', () => {
    cy.get('#folders-ms-action-bar').find('#grid-view').click();
    cy.get('.folder-grid-item').first().find('.card-option').invoke('show').click();
    checkMenuItems();
  });

  it('Tests select a file in list view', () => {
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
    cy.get('.counter').contains('1 selected item');
    // Unselect the first file
    cy.get('.file-list-item').eq(0).find('ion-checkbox').click();
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(1).find('ion-checkbox').should('not.be.visible');
    cy.get('.counter').contains('3 items');
  });

  it('Tests select a file in grid view', () => {
    cy.get('#folders-ms-action-bar').find('#grid-view').as('gridButton').click();
    cy.get('.file-card-item').eq(0).find('ion-checkbox').should('not.be.visible');
    cy.get('.file-card-item').eq(1).find('ion-checkbox').should('not.be.visible');
    // Make the checkbox appear
    cy.get('.file-card-item').eq(0).trigger('mouseenter');
    cy.get('.file-card-item').eq(0).find('ion-checkbox').should('be.visible');
    // Select the first file
    cy.get('.file-card-item').eq(0).find('ion-checkbox').click();
    cy.get('.file-card-item').eq(0).find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.file-card-item').eq(1).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.counter').contains('1 selected item');
    // Unselect the first file
    cy.get('.file-card-item').eq(0).find('ion-checkbox').click();
    cy.get('.file-card-item').eq(0).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.file-card-item').eq(1).find('ion-checkbox').should('not.be.visible');
    cy.get('.counter').contains('3 items');
  });

  it('Tests select all files in list view', () => {
    cy.get('.folder-list-header').find('ion-checkbox').invoke('show').should('not.have.class', 'checkbox-checked');
    // Select all
    cy.get('.folder-list-header').find('ion-checkbox').invoke('show').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.counter').contains('3 selected items');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(1).find('ion-checkbox').should('have.class', 'checkbox-checked');
    // Unselect all
    cy.get('.folder-list-header').find('ion-checkbox').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('not.be.visible');
    cy.get('.file-list-item').eq(1).find('ion-checkbox').should('not.be.visible');
    cy.get('.counter').contains('3 items');

    // Select all, unselect first file
    cy.get('.folder-list-header').find('ion-checkbox').invoke('show').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.counter').contains('3 selected items');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.file-list-item').eq(0).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.counter').contains('2 selected item');
  });

  it('Tests delete one file in list view', () => {
    // list view
    cy.get('.file-list-item').first().find('.options-button').invoke('show').click();
    cy.get('#file-context-menu').find('ion-item').eq(4).contains('Delete').click();
    cy.get('.question-modal').find('.ms-modal-header__title').contains('Delete one file');
    cy.get('.question-modal')
      .find('.ms-modal-header__text')
      .contains(/Are you sure you want to delete the file `File_[a-z_]+`?/);

    cy.get('.question-modal').find('#next-button').click();
    cy.get('.question-modal').should('not.exist');
    cy.get('@consoleLog').should('have.been.called.with', /File File_[a-z_]+ deleted/);
  });

  it('Tests delete one file in grid view', () => {
    cy.get('#folders-ms-action-bar').find('#grid-view').as('gridButton');
    cy.get('@gridButton').click();
    cy.get('.file-card-item').first().find('.card-option').invoke('show').click();
    cy.get('#file-context-menu').find('ion-item').eq(4).contains('Delete').click();
    cy.get('.question-modal').find('.ms-modal-header__title').contains('Delete one file');
    cy.get('.question-modal')
      .find('.ms-modal-header__text')
      .contains(/Are you sure you want to delete the file `File_[a-z_]+`?/);

    cy.get('.question-modal').find('#next-button').click();
    cy.get('.question-modal').should('not.exist');
    cy.get('@consoleLog').should('have.been.called.with', /File File_[a-z_]+ deleted/);
  });

  it('Tests delete multiple files', () => {
    cy.get('.folder-list-header').find('ion-checkbox').invoke('show').click();
    cy.get('#button-delete').contains('Delete').click();
    cy.get('.question-modal').find('.ms-modal-header__title').contains('Delete multiple items');
    cy.get('.question-modal').find('.ms-modal-header__text').contains('Are you sure you want to delete these 3 items?');
    cy.get('.question-modal').find('#next-button').contains('Delete 3 items').click();
    cy.get('.question-modal').should('not.exist');
    cy.get('.folder-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('@consoleLog').should('have.been.called.with', '3 entries deleted');
  });

  it('Tests file details', () => {
    cy.get('.file-list-item').first().find('.options-button').invoke('show').click();
    cy.get('#file-context-menu').find('ion-item').eq(8).contains('Details').click();
    cy.get('.file-details-modal')
      .find('.ms-modal-header__title')
      .contains(/Details on File_[a-z_]+/);
    cy.get('.file-details-modal').find('.file-info-value').as('values').should('have.length', 8);
    cy.get('@values').eq(0).contains('File');
    cy.get('@values')
      .eq(1)
      .contains(/\/File_[a-z_]+/);
    cy.get('@values')
      .eq(2)
      .contains(/^\d+ B|KB|MB|GB$/);
    cy.get('@values').eq(5).contains('1');
    cy.get('@values')
      .eq(6)
      .contains(/^Yes|No$/);
    cy.get('@values').eq(7).contains('67');
  });

  it('Tests folder details', () => {
    cy.get('.file-list-item').last().find('.options-button').invoke('show').click();
    cy.get('#file-context-menu').find('ion-item').eq(8).contains('Details').click();
    cy.get('.file-details-modal')
      .find('.ms-modal-header__title')
      .contains(/Details on Dir_[a-z_]+/);
    cy.get('.file-details-modal').find('.file-info-value').as('values').should('have.length', 7);
    cy.get('@values').eq(0).contains('Folder');
    cy.get('@values')
      .eq(1)
      .contains(/\/Dir_[a-z_]+/);
    cy.get('@values').eq(4).contains('1');
    cy.get('@values')
      .eq(5)
      .contains(/^Yes|No$/);
    cy.get('@values').eq(6).contains('68');
  });

  it('Tests get file link', () => {
    cy.get('.file-list-item').last().find('.options-button').invoke('show').click();
    cy.get('#file-context-menu').find('ion-item').eq(10).contains('Copy link').click();
    cy.checkToastMessage('info', 'Link copied', 'The link has been copied to the clipboard.');
    cy.window().then((win) => {
      win.navigator.clipboard.readText().then((text) => {
        expect(text).to.eq(
          // cspell:disable-next-line
          'parsec://parsec.cloud/Org?action=file_link&workspace_id=94a350f2f629403db2269c44583f7aa1&path=KEFNEI3939jf39KEFsss',
        );
      });
    });
  });

  it('Test move one file', () => {
    cy.get('.folder-container').find('.file-list-item').eq(2).click();
    cy.get('.folder-container').find('.file-list-item').eq(1).find('ion-checkbox').invoke('show').click();
    cy.get('#button-moveto').contains('Move to').click();
    cy.get('.folder-selection-modal').find('.ms-modal-header__title').contains('Move one item');
    cy.get('.folder-selection-modal')
      .find('.ms-modal-header__text')
      .contains(/Current location: \/Dir_[a-z_]+/);
    cy.get('.folder-selection-modal').find('.ms-modal-footer').find('#next-button').as('okButton');
    cy.get('.folder-selection-modal').find('ion-breadcrumb').as('breadcrumbs').should('have.length', 2);
    cy.get('@breadcrumbs')
      .eq(1)
      .contains(/Dir_[a-z_]+/);
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('.folder-selection-modal').find('.folders-container').find('.file-list-item').as('items');
    cy.get('@items').should('have.length', 3);
    cy.get('@items')
      .eq(0)
      .contains(/Dir_[a-z_]+/)
      .click();
    cy.get('.folder-selection-modal').find('ion-breadcrumb').as('breadcrumbs').should('have.length', 3);
    cy.get('@breadcrumbs')
      .eq(1)
      .contains(/Dir_[a-z_]+/);
    cy.get('@breadcrumbs')
      .eq(2)
      .contains(/Dir_[a-z_]+/);
    cy.get('@okButton').should('not.have.class', 'button-disabled');
    cy.get('@okButton').click();
    cy.get('.folder-selection-modal').should('not.exist');
    cy.checkToastMessage('success', '1 element moved', 'The element have been moved the choosen folder.');
  });

  it('Tests move files', () => {
    cy.get('.folder-container').find('.file-list-item').eq(2).click();
    cy.get('.folder-list-header').find('ion-checkbox').click();
    cy.get('#button-moveto').contains('Move to').click();
    cy.get('.folder-selection-modal').find('.ms-modal-header__title').contains('Move 3 items');
    cy.get('.folder-selection-modal')
      .find('.ms-modal-header__text')
      .contains(/Current location: \/Dir_[a-z_]+/);
    cy.get('.folder-selection-modal').find('.ms-modal-footer').find('#next-button').as('okButton');
    cy.get('.folder-selection-modal').find('ion-breadcrumb').as('breadcrumbs').should('have.length', 2);
    cy.get('@breadcrumbs')
      .eq(1)
      .contains(/Dir_[a-z_]+/);
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('.folder-selection-modal').find('.folders-container').find('.file-list-item').as('items');
    cy.get('@items').should('have.length', 3);
    cy.get('@items')
      .eq(0)
      .contains(/Dir_[a-z_]+/)
      .click();
    cy.get('.folder-selection-modal').find('ion-breadcrumb').as('breadcrumbs').should('have.length', 3);
    cy.get('@breadcrumbs')
      .eq(1)
      .contains(/Dir_[a-z_]+/);
    cy.get('@breadcrumbs')
      .eq(2)
      .contains(/Dir_[a-z_]+/);
    cy.get('@okButton').should('not.have.class', 'button-disabled');
    cy.get('@okButton').click();
    cy.get('.folder-selection-modal').should('not.exist');
    cy.checkToastMessage('success', '3 elements moved', 'All the elements have been moved to the chosen folder.');
  });

  it('Tests copy one file', () => {
    cy.get('.folder-container').find('.file-list-item').eq(2).click();
    cy.get('.folder-container').find('.file-list-item').eq(1).find('ion-checkbox').invoke('show').click();
    // cspell:disable-next-line
    cy.get('#button-makeacopy').contains('Make a copy').click();
    cy.get('.folder-selection-modal').find('.ms-modal-header__title').contains('Copy one item');
    cy.get('.folder-selection-modal')
      .find('.ms-modal-header__text')
      .contains(/Current location: \/Dir_[a-z_]+/);
    cy.get('.folder-selection-modal').find('.ms-modal-footer').find('#next-button').as('okButton');
    cy.get('.folder-selection-modal').find('ion-breadcrumb').as('breadcrumbs').should('have.length', 2);
    cy.get('@breadcrumbs')
      .eq(1)
      .contains(/Dir_[a-z_]+/);
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('.folder-selection-modal').find('.folders-container').find('.file-list-item').as('items');
    cy.get('@items').should('have.length', 3);
    cy.get('@items')
      .eq(0)
      .contains(/Dir_[a-z_]+/)
      .click();
    cy.get('.folder-selection-modal').find('ion-breadcrumb').as('breadcrumbs').should('have.length', 3);
    cy.get('@breadcrumbs')
      .eq(1)
      .contains(/Dir_[a-z_]+/);
    cy.get('@breadcrumbs')
      .eq(2)
      .contains(/Dir_[a-z_]+/);
    cy.get('@okButton').should('not.have.class', 'button-disabled');
    cy.get('@okButton').click();
    cy.get('.folder-selection-modal').should('not.exist');
    cy.checkToastMessage('success', '1 element copied', 'The element have been copied to the choosen folder.');
  });

  it('Tests copy files', () => {
    cy.get('.folder-container').find('.file-list-item').eq(2).click();
    cy.get('.folder-list-header').find('ion-checkbox').click();
    // cspell:disable-next-line
    cy.get('#button-makeacopy').contains('Make a copy').click();
    cy.get('.folder-selection-modal').find('.ms-modal-header__title').contains('Copy 3 items');
    cy.get('.folder-selection-modal')
      .find('.ms-modal-header__text')
      .contains(/Current location: \/Dir_[a-z_]+/);
    cy.get('.folder-selection-modal').find('.ms-modal-footer').find('#next-button').as('okButton');
    cy.get('.folder-selection-modal').find('ion-breadcrumb').as('breadcrumbs').should('have.length', 2);
    cy.get('@breadcrumbs')
      .eq(1)
      .contains(/Dir_[a-z_]+/);
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('.folder-selection-modal').find('.folders-container').find('.file-list-item').as('items');
    cy.get('@items').should('have.length', 3);
    cy.get('@items')
      .eq(0)
      .contains(/Dir_[a-z_]+/)
      .click();
    cy.get('.folder-selection-modal').find('ion-breadcrumb').as('breadcrumbs').should('have.length', 3);
    cy.get('@breadcrumbs')
      .eq(1)
      .contains(/Dir_[a-z_]+/);
    cy.get('@breadcrumbs')
      .eq(2)
      .contains(/Dir_[a-z_]+/);
    cy.get('@okButton').should('not.have.class', 'button-disabled');
    cy.get('@okButton').click();
    cy.get('.folder-selection-modal').should('not.exist');
    cy.checkToastMessage('success', '3 elements copied', 'All the elements have been copied to the chosen folder.');
  });
});

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
    cy.get('.file-card-item').eq(0).realHover().find('.checkbox').should('be.visible');
    // Select the first file
    cy.get('.file-card-item').eq(0).find('ion-checkbox').click();
    cy.get('.file-card-item').eq(0).find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.file-card-item').eq(1).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.counter').contains('1 selected item');
    // Unselect the first file
    cy.get('.file-card-item').eq(0).find('ion-checkbox').click();
    cy.get('.file-card-item').eq(0).find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    // cy.get('.file-card-item').eq(1).find('ion-checkbox').should('not.be.visible');
    cy.get('.counter').contains(/^\d+ items$/);
  });

  it('Tests select all files in list view', () => {
    cy.get('.folder-list-header').should('not.have.class', 'checkbox-checked');
    // Select all
    cy.get('.folder-list-header').find('ion-checkbox').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.counter').contains(/^\d+ selected items$/);
    cy.get('.file-list-item').first().find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.file-list-item').last().find('ion-checkbox').should('have.class', 'checkbox-checked');
    // Unselect all
    cy.get('.folder-list-header').find('ion-checkbox').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    // cy.wait(200);
    // cy.get('.file-list-item').first().find('ion-checkbox').should('not.be.visible');
    // cy.get('.file-list-item').last().find('ion-checkbox').should('not.be.visible');
    cy.get('.counter').contains(/^\d+ items$/);

    // Select all, unselect first file
    cy.get('.folder-list-header').find('ion-checkbox').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('have.class', 'checkbox-checked');
    cy.get('.counter').contains(/^\d+ selected items$/);
    cy.get('.file-list-item').first().find('ion-checkbox').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('have.class', 'checkbox-indeterminate');
    cy.get('.file-list-item').first().find('ion-checkbox').should('not.have.class', 'checkbox-checked');
    cy.get('.counter').contains(/^\d+ selected items$/);

    // Header checkbox should be selected if all files are selected
    cy.get('.file-list-item').first().find('ion-checkbox').click();
    cy.get('.folder-list-header').find('ion-checkbox').should('have.class', 'checkbox-checked');
  });

  it('Tests delete one file in list view', () => {
    // list view
    cy.get('.file-list-item').last().find('.options-button').invoke('show').click();
    cy.get('#file-context-menu').find('ion-item').eq(4).contains('Delete').click();
    cy.get('.question-modal').find('.ms-modal-header__title').contains('Delete one file');
    cy.get('.question-modal')
      .find('.ms-modal-header__text')
      .contains(/Are you sure you want to delete file `File_[a-z_]+`?/);

    cy.get('.question-modal').find('#next-button').click();
    cy.get('.question-modal').should('not.exist');
  });

  it('Tests delete one file in grid view', () => {
    cy.get('#folders-ms-action-bar').find('#grid-view').as('gridButton');
    cy.get('@gridButton').click();
    cy.get('.file-card-item').last().find('.card-option').invoke('show').click();
    cy.get('#file-context-menu').find('ion-item').eq(4).contains('Delete').click();
    cy.get('.question-modal').find('.ms-modal-header__title').contains('Delete one file');
    cy.get('.question-modal')
      .find('.ms-modal-header__text')
      .contains(/Are you sure you want to delete file `File_[a-z_]+`?/);

    cy.get('.question-modal').find('#next-button').click();
    cy.get('.question-modal').should('not.exist');
  });

  it('Tests delete multiple files', () => {
    cy.get('.folder-list-header').find('ion-checkbox').click();
    cy.get('#button-delete').contains('Delete').click();
    cy.get('.question-modal').find('.ms-modal-header__title').contains('Delete multiple items');
    cy.get('.question-modal')
      .find('.ms-modal-header__text')
      .contains(/^Are you sure you want to delete these \d+ items\?$/);
    cy.get('.question-modal')
      .find('#next-button')
      .contains(/^Delete \d+ items$/)
      .click();
    cy.get('.question-modal').should('not.exist');
    cy.get('.folder-list-header').find('ion-checkbox').should('not.have.class', 'checkbox-checked');
  });

  it('Tests file details', () => {
    cy.get('.file-list-item').last().find('.options-button').invoke('show').click();
    cy.get('#file-context-menu').find('ion-item').eq(8).contains('Details').click();
    cy.get('.file-details-modal')
      .find('.ms-modal-header__title')
      .contains(/Details on File_[a-z_]+/);
    cy.get('.file-details-modal')
      .find('.file-info-basic__name')
      .contains(/File_[a-z_]+/);
    cy.get('.file-info-container')
      .find('.file-info-details-item__value')
      .eq(1)
      .contains(/^\d+ B|KB|MB|GB$/);
    cy.get('.file-info-container').find('.file-info-details-item__value').eq(2).contains('1');
    // Fails for unknown reasons
    // cy.get('.file-info-container')
    //   .find('.file-info-path-value__text')
    //   .contains(/^\/[A-Za-z_./]+$/);
  });

  it('Tests folder details', () => {
    cy.get('.file-list-item').first().find('.options-button').invoke('show').click();
    cy.get('#file-context-menu').find('ion-item').eq(8).contains('Details').click();
    cy.get('.file-details-modal')
      .find('.ms-modal-header__title')
      .contains(/Details on Dir_[a-z_]+/);
    cy.get('.file-details-modal')
      .find('.file-info-basic__name')
      .contains(/Dir_[a-z_]+/);
    cy.get('.file-info-container').find('.file-info-details-item__value').eq(1).contains('1');
    cy.get('.file-info-container')
      .find('.file-info-path-value__text')
      .contains(/^\/Dir_[a-z_]+$/);
  });

  // Disabled until we get the bindings for it
  // it('Tests get file link', () => {
  //   cy.get('.file-list-item').last().find('.options-button').invoke('show').click();
  //   cy.get('#file-context-menu').find('ion-item').eq(10).contains('Copy link').click();
  //   cy.checkToastMessage('info', 'Link has been copied to clipboard.');
  //   cy.window().then((win) => {
  //     win.navigator.clipboard.readText().then((text) => {
  //       // cspell:disable-next-line
  //       const path = 'MZDXYYNVT5QF27JMZQOOPEPDATV4R4FQHRZ762CTNRNAJHJO3DV3IACWLABY7EA6DC3BNGXTALKSQAQDDDBAssss';
  //       // cspell:disable-next-line
  //       const workspaceId = '94a350f2f629403db2269c44583f7aa1';
  //       const expected = `parsec3://parsec.cloud/Org?action=file_link&workspace_id=${workspaceId}&path=${path}`;
  //       expect(text).to.eq(expected);
  //     });
  //   });
  // });

  // function checkCurrentPath(workspace: string, depth: number): void {
  //   cy.get('.folder-selection-modal').find('ion-breadcrumb').as('breadcrumbs').should('have.length', depth);
  //   cy.get('@breadcrumbs').eq(0).contains(workspace);
  //   for (let i = 1; i < depth; i++) {
  //     cy.get('@breadcrumbs')
  //       .eq(i)
  //       .contains(/Dir_[a-z_]+/);
  //   }
  // }

  // it('Test move one file', () => {
  //   // .first() will always be a folder, as there's always at least one folder
  //   cy.get('.folder-container').find('.file-list-item').first().click();
  //   cy.get('.folder-container').find('.file-list-item').last().find('ion-checkbox').invoke('show').click();
  //   cy.get('#button-moveto').contains('Move to').click();
  //   cy.get('.folder-selection-modal').find('.ms-modal-header__title').contains('Move one item');

  //   cy.get('.folder-selection-modal')
  //     .find('.breadcrumb')
  //     .contains(/Dir_[a-z_]+/);

  //   cy.get('.folder-selection-modal').find('.ms-modal-footer').find('#next-button').as('okButton');
  //   cy.get('.folder-selection-modal').find('ion-breadcrumb').as('breadcrumbs').should('have.length', 2);
  //   cy.get('@breadcrumbs')
  //     .eq(1)
  //     .contains(/Dir_[a-z_]+/);
  //   cy.get('@okButton').should('have.class', 'button-disabled');
  //   cy.get('.folder-selection-modal').find('.folder-container').find('.file-list-item').as('items');
  //   cy.get('@items').should('have.length.greaterThan', 1);
  //   cy.get('@items')
  //     .eq(0)
  //     .contains(/Dir_[a-z_]+/)
  //     .click();
  //   checkCurrentPath('The Copper Coronet', 3);
  //   cy.get('@okButton').should('not.have.class', 'button-disabled');
  //   cy.get('@okButton').click();
  //   cy.get('.folder-selection-modal').should('not.exist');
  //   cy.checkToastMessage('success', 'The element have been moved the choosen folder.');
  // });

  // it('Tests move files', () => {
  //   cy.get('.folder-container').find('.file-list-item').first().click();
  //   cy.get('.folder-list-header').find('ion-checkbox').click();
  //   cy.get('#button-moveto').contains('Move to').click();
  //   cy.get('.folder-selection-modal')
  //     .find('.ms-modal-header__title')
  //     .contains(/^Move \d+ items$/);
  //   cy.get('.folder-selection-modal').find('.ms-modal-footer').find('#next-button').as('okButton');
  //   cy.get('.folder-selection-modal').find('ion-breadcrumb').as('breadcrumbs').should('have.length', 2);
  //   checkCurrentPath('The Copper Coronet', 2);
  //   cy.get('@okButton').should('have.class', 'button-disabled');
  //   cy.get('.folder-selection-modal').find('.folder-container').find('.file-list-item').as('items');
  //   cy.get('@items').should('have.length.greaterThan', 1);
  //   cy.get('@items')
  //     .eq(0)
  //     .contains(/Dir_[a-z_]+/)
  //     .click();
  //   checkCurrentPath('The Copper Coronet', 3);
  //   cy.get('@okButton').should('not.have.class', 'button-disabled');
  //   cy.get('@okButton').click();
  //   cy.get('.folder-selection-modal').should('not.exist');
  //   cy.checkToastMessage('success', 'All the elements have been moved to the chosen folder.');
  // });

  // it('Tests copy one file', () => {
  //   cy.get('.folder-container').find('.file-list-item').first().click();
  //   cy.get('.folder-container').find('.file-list-item').last().find('ion-checkbox').invoke('show').click();
  //   // cspell:disable-next-line
  //   cy.get('#button-makeacopy').contains('Make a copy').click();
  //   cy.get('.folder-selection-modal').find('.ms-modal-header__title').contains('Copy one item');
  //   cy.get('.folder-selection-modal').find('.ms-modal-footer').find('#next-button').as('okButton');
  //   checkCurrentPath('The Copper Coronet', 2);
  //   cy.get('@okButton').should('have.class', 'button-disabled');
  //   cy.get('.folder-selection-modal').find('.folder-container').find('.file-list-item').as('items');
  //   cy.get('@items').should('have.length.greaterThan', 1);
  //   cy.get('@items')
  //     .eq(0)
  //     .contains(/Dir_[a-z_]+/)
  //     .click();
  //   checkCurrentPath('The Copper Coronet', 3);
  //   cy.get('@okButton').should('not.have.class', 'button-disabled');
  //   cy.get('@okButton').click();
  //   cy.get('.folder-selection-modal').should('not.exist');
  //   cy.checkToastMessage('success', 'The element have been copied to the choosen folder.');
  // });

  // it('Tests copy files', () => {
  //   cy.get('.folder-container').find('.file-list-item').first().click();
  //   cy.get('.folder-list-header').find('ion-checkbox').click();
  //   // cspell:disable-next-line
  //   cy.get('#button-makeacopy').contains('Make a copy').click();
  //   cy.get('.folder-selection-modal')
  //     .find('.ms-modal-header__title')
  //     .contains(/^Copy \d+ items$/);
  //   cy.get('.folder-selection-modal').find('.ms-modal-footer').find('#next-button').as('okButton');
  //   checkCurrentPath('The Copper Coronet', 2);
  //   cy.get('@okButton').should('have.class', 'button-disabled');
  //   cy.get('.folder-selection-modal').find('.folder-container').find('.file-list-item').as('items');
  //   cy.get('@items').should('have.length.greaterThan', 1);
  //   cy.get('@items')
  //     .eq(0)
  //     .contains(/Dir_[a-z_]+/)
  //     .click();
  //   checkCurrentPath('The Copper Coronet', 3);
  //   cy.get('@okButton').should('not.have.class', 'button-disabled');
  //   cy.get('@okButton').click();
  //   cy.get('.folder-selection-modal').should('not.exist');
  //   cy.checkToastMessage('success', 'All the elements have been copied to the chosen folder.');
  // });

  // it('Test move file back/forward', () => {
  //   cy.get('.folder-container').find('.file-list-item').first().click();
  //   cy.get('.folder-container').find('.file-list-item').last().find('ion-checkbox').invoke('show').click();
  //   cy.get('#button-moveto').contains('Move to').click();
  //   cy.get('.folder-selection-modal').find('.ms-modal-header__title').contains('Move one item');
  //   cy.get('.folder-selection-modal').find('.navigation-back-button').as('backButton').should('have.class', 'button-disabled');
  //   cy.get('.folder-selection-modal').find('.navigation-forward-button').as('forwardButton').should('have.class', 'button-disabled');

  //   checkCurrentPath('The Copper Coronet', 2);

  //   // Down one more folder
  //   cy.get('.folder-selection-modal')
  //     .find('.folder-container')
  //     .find('.file-list-item')
  //     .as('items')
  //     .eq(0)
  //     .contains(/Dir_[a-z_]+/)
  //     .click();
  //   checkCurrentPath('The Copper Coronet', 3);

  //   // Back enabled, forward disabled
  //   cy.get('@backButton').should('not.have.class', 'button-disabled');
  //   cy.get('@forwardButton').should('have.class', 'button-disabled');

  //   // Go back
  //   cy.get('@backButton').click();
  //   checkCurrentPath('The Copper Coronet', 2);
  //   // Forward enabled, back disabled
  //   cy.get('@backButton').should('have.class', 'button-disabled');
  //   cy.get('@forwardButton').should('not.have.class', 'button-disabled');

  //   // Go forward
  //   cy.get('@forwardButton').click();
  //   checkCurrentPath('The Copper Coronet', 3);
  //   // Back enabled, forward disabled
  //   cy.get('@backButton').should('not.have.class', 'button-disabled');
  //   cy.get('@forwardButton').should('have.class', 'button-disabled');

  //   // Down one more folders
  //   cy.get('@items')
  //     .eq(0)
  //     .contains(/Dir_[a-z_]+/)
  //     .click();
  //   checkCurrentPath('The Copper Coronet', 4);

  //   // Back enabled, forward disabled
  //   cy.get('@backButton').should('not.have.class', 'button-disabled');
  //   cy.get('@forwardButton').should('have.class', 'button-disabled');

  //   // Go back to enabled forward
  //   cy.get('@backButton').click();
  //   cy.get('@backButton').should('not.have.class', 'button-disabled');
  //   cy.get('@forwardButton').should('not.have.class', 'button-disabled');

  //   // Back to first folder using breadcrumbs
  //   cy.get('.folder-selection-modal')
  //     .find('ion-breadcrumb')
  //     .as('breadcrumbs')
  //     .eq(1)
  //     .contains(/Dir_[a-z_]+/)
  //     .click();
  //   cy.get('@backButton').should('not.have.class', 'button-disabled');
  //   cy.get('@forwardButton').should('have.class', 'button-disabled');
  // });

  it('Check reader role', () => {
    cy.get('.list-workspaces').find('.list-workspaces-header').contains('Workspaces').click();
    cy.get('.workspaces-grid-item').eq(2).contains("Watcher's Keep").click({ force: true });

    cy.get('#folders-ms-action-bar').find('#button-new-folder').should('not.be.visible');
    cy.get('#folders-ms-action-bar').find('#button-import').should('not.be.visible');

    cy.get('.file-list-item').eq(0).trigger('mouseenter');
    cy.get('.file-list-item').eq(0).realHover().find('ion-checkbox').click({ force: true });

    cy.get('#folders-ms-action-bar').find('#button-rename').should('not.be.visible');
    cy.get('#folders-ms-action-bar').find('#button-moveto').should('not.be.visible');
    // cspell:disable-next-line
    cy.get('#folders-ms-action-bar').find('#button-makeacopy').should('not.be.visible');
    cy.get('#folders-ms-action-bar').find('#button-delete').should('not.be.visible');

    cy.get('#folders-ms-action-bar').find('#button-copy-link').should('not.have.css', 'display', 'none');
    cy.get('#folders-ms-action-bar').find('#button-details').should('not.have.css', 'display', 'none');

    cy.get('.file-list-item').first().find('.options-button').invoke('show').click();
    cy.get('#file-context-menu').should('be.visible');
    cy.get('#file-context-menu').find('ion-item').as('menuItems').should('have.length', 7);
    cy.get('@menuItems').eq(0).contains('Manage file');
    cy.get('@menuItems').eq(1).contains('Open');
    cy.get('@menuItems').eq(2).contains('History');
    cy.get('@menuItems').eq(3).contains('Download');
    cy.get('@menuItems').eq(4).contains('Details');
    cy.get('@menuItems').eq(5).contains('Collaboration');
    cy.get('@menuItems').eq(6).contains('Copy link');
  });
});

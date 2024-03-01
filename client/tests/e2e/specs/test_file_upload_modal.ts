// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Upload files', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('.workspaces-grid-item').first().click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Open import modal', () => {
    cy.get('#button-import').click();
    cy.get('.file-upload-modal').should('exist');
    cy.get('.ion-page').find('.ms-modal-header__title').contains('Upload your files');
    cy.get('.ion-page').find('.drop-zone').should('exist');
    cy.get('.ion-page').find('.import-button').find('ion-button').should('exist');
    cy.get('.ion-page').find('.closeBtn').should('exist');
  });

  it('Close modal with X', () => {
    cy.get('#button-import').click();
    cy.get('.file-upload-modal').should('exist');
    cy.get('.ion-page').find('.closeBtn').should('exist');
    cy.get('.ion-page').find('.closeBtn').click();
    cy.get('.file-upload-modal').should('not.exist');
  });

  it('Import file with drag&drop', () => {
    cy.get('#button-import').click();
    cy.get('.ion-page').find('.ms-modal-header__title').contains('Upload your files');
    cy.get('.ion-page').find('.drop-zone').attachFile('splash.png', { subjectType: 'drag-n-drop' });
    // Sadly, the drop does react but when calling webkitGetAsEntry() on the data provided,
    // it returns null, preventing us from going further (for the time being).
  });

  it('Import file with button', () => {
    cy.get('#button-import').click();
    cy.get('.ion-page').find('.ms-modal-header__title').contains('Upload your files');
    cy.get('.ion-page').find('.import-button').find('ion-button').click();
    cy.get('.ion-page').find('.import-button input').attachFile('splash.png');
    cy.get('.upload-menu-list').find('.element-container .element-details__name').should('contain', 'splash.png');
    cy.get('.upload-menu-list').find('.element-container').should('have.class', 'progress');
    cy.wait(3000);
    cy.get('.upload-menu-tabs').find('.upload-menu-tabs__item ion-text').eq(1).contains('Done');
    cy.get('.upload-menu-tabs').find('.upload-menu-tabs__item').eq(1).click();
    cy.get('.upload-menu-list').find('.element-container').should('have.class', 'done');
  });

  it('Import multiple files with button', () => {
    cy.get('#button-import').click();
    cy.get('.ion-page').find('.ms-modal-header__title').contains('Upload your files');
    cy.get('.ion-page').find('.import-button').find('ion-button').click();
    cy.get('.ion-page').find('.import-button input').attachFile(['splash.png', 'splash.png']);
    cy.get('.upload-menu-list').should('have.length', 1);
    cy.get('.upload-menu-list').find('.element-container .element-details__name').should('contain', 'splash.png');
    cy.wait(3000);
    cy.get('.upload-menu-tabs').find('.upload-menu-tabs__item').eq(1).click();
    cy.get('.upload-menu-list').find('.element-container').should('have.class', 'done');
  });

  // Import is instant, can't cancel it
  // it('Import file with button and cancel', () => {
  //   cy.get('#button-import').click();
  //   cy.get('.ion-page').find('.import-button').find('ion-button').click();
  //   cy.get('.ion-page').find('.import-button input').attachFile('splash.png');
  //   cy.get('.upload-menu-list').find('.element-container').find('.element-details__name').should('contain', 'splash.png');
  //   cy.get('.upload-menu-list').find('.element-container').should('have.class', 'progress');
  //   cy.get('.upload-menu-list').find('.element-container').find('.cancel-button').click();
  //   cy.get('.upload-menu-tabs').find('.upload-menu-tabs__item').eq(1).click();
  //   cy.get('.upload-menu-list').find('.element-container').should('have.class', 'cancelled');
  // });
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Upload files', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd');
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
});

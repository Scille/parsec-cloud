// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check create workspace modal', () => {
  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd');
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Open create workspace modal', () => {
    cy.get('#button-new-workspace').click();
    cy.get('.create-workspace-modal').should('exist');
    cy.get('.ms-modal-header__title').contains('Create a new workspace');
    cy.get('.ion-page').find('.closeBtn').should('exist');
    cy.get('ion-input').should('be.visible');
    cy.get('#cancel-button').should('be.visible');
    cy.get('#next-button').should('be.visible');
    cy.get('.ion-page').find('#next-button').should('exist');
  });

  it('Create workspace', () => {
    cy.get('#button-new-workspace').click();
    cy.get('.create-workspace-modal').should('exist');
    cy.get('.input').eq(0).find('input').type('MyWorkspace');
    cy.get('#next-button').should('not.be.disabled');
    cy.get('#next-button').click();
    cy.get('.create-workspace-modal').should('not.exist');
  });

  it('Close modal', () => {
    cy.get('#button-new-workspace').click();
    cy.get('ion-modal').should('exist');
    cy.get('ion-modal').find('.closeBtn').click();
    cy.get('ion-modal').should('not.exist');
  });
});

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

  it('Checks initial status', () => {
    cy.get('#button-new-workspace').contains('New workspace');
    cy.get('#workspace-filter-select').contains('Name');
    cy.get('#workspaces-ms-action-bar').find('#grid-view').should('have.attr', 'disabled');
    cy.get('#workspaces-ms-action-bar').find('#list-view').should('not.have.attr', 'disabled');
    cy.get('.card').should('have.length', 3);
    cy.get('.workspace-list-item').should('have.length', 0);
    cy.get('.card').first().contains('The Copper Coronet');
    cy.get('.card').last().contains("Watcher's Keep");
    cy.get('.card').first().find('.shared-group').find('.person-avatar').as('avatars').should('have.length', 2);
    cy.get('@avatars').first().contains('Ko');
    cy.get('@avatars').eq(1).contains('Ce');
    cy.get('.card').first().find('.workspace-info').find('.not-shared-label').should('not.be.visible');
    // Test sometimes fails because reasons without that wait
    cy.wait(300);
  });

  it('Checks submenu', () => {
    cy.get('.card').eq(0).find('.card-option').click();
    cy.get('.popover-viewport').find('.menu-list').find('ion-item').as('items').should('have.length', 12);
    cy.get('@items').eq(0).contains('Offline');
    cy.get('@items').eq(1).contains('Enable offline availability');
    cy.get('@items').eq(2).contains('Manage workspace');
    cy.get('@items').eq(3).contains('Rename');
    cy.get('@items').eq(4).contains('Open in explorer');
    cy.get('@items').eq(5).contains('History');
    cy.get('@items').eq(6).contains('Details');
    cy.get('@items').eq(7).contains('Collaboration');
    cy.get('@items').eq(8).contains('Copy link');
    cy.get('@items').eq(9).contains('Sharing and roles');
    cy.get('@items').eq(10).contains('Miscellaneous');
    cy.get('@items').eq(11).contains('Add to favorites');
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

  it('Switch views', () => {
    cy.get('.card').should('have.length', 3);
    cy.get('#workspaces-ms-action-bar').find('#list-view').click();
    cy.get('#workspaces-ms-action-bar').find('#grid-view').should('not.have.attr', 'disabled');
    cy.get('#workspaces-ms-action-bar').find('#list-view').should('have.attr', 'disabled');
    cy.get('.card').should('have.length', 0);
    cy.get('.workspace-list-item').should('have.length', 3);
    cy.get('.workspace-list-item').first().find('.shared-group').find('.person-avatar').as('avatars').should('have.length', 2);
    cy.get('@avatars').first().contains('Ko');
    cy.get('@avatars').eq(1).contains('Ce');
    cy.get('.workspace-list-item').first().find('.workspace-users').find('.not-shared-label').should('not.be.visible');
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

  it('Test favorites in grid view', () => {
    cy.get('.card').eq(1).contains('Trademeet');
    cy.get('.card').eq(1).find('.card-option').click();
    cy.get('.popover-viewport').find('.menu-list').find('ion-item').eq(11).contains('Add to favorites').click();
    cy.get('.card').eq(0).contains('Trademeet');
    cy.get('.card').eq(0).find('.card-favorite-on').click();
    cy.get('.card').eq(1).contains('Trademeet');
    cy.get('.card').eq(1).find('.card-favorite-off');
  });

  it('Test favorites in list view', () => {
    cy.get('#workspaces-ms-action-bar').find('#list-view').click();
    cy.get('.workspace-list-item').eq(1).contains('Trademeet');
    cy.get('.workspace-list-item').eq(1).find('.card-favorite-off').click();
    cy.get('.workspace-list-item').eq(0).contains('Trademeet');
    cy.get('.workspace-list-item').eq(0).find('.workspace-options').click();
    cy.get('.popover-viewport').find('.menu-list').find('ion-item').eq(11).contains('Remove from favorites').click();
    cy.get('.workspace-list-item').eq(1).contains('Trademeet');
    cy.get('.workspace-list-item').eq(1).find('.card-favorite-off');
  });

  it('Create workspace', () => {
    cy.get('#button-new-workspace').click();
    cy.get('.text-input-modal').should('exist');
    cy.get('.ms-modal-header__title').contains('Create a new workspace');
    cy.get('.ion-page').find('.closeBtn').should('exist');
    cy.get('ion-input').should('be.visible');
    cy.get('.text-input-modal').find('.ms-modal-footer-buttons').find('ion-button').eq(1).as('createButton').contains('Create');
    cy.get('@createButton').should('have.class', 'button-disabled');
    cy.get('.text-input-modal').find('.input').find('input').type('MyWorkspace');
    cy.get('@createButton').should('not.have.class', 'button-disabled');
    cy.get('@createButton').click();
    cy.get('.text-input-modal').should('not.exist');
    cy.checkToastMessage('success', "The workspace 'MyWorkspace' has been created!");
    cy.get('#button-new-workspace').click();
    cy.get('ion-modal').should('exist');
    cy.get('ion-modal').find('.closeBtn').click();
    cy.get('ion-modal').should('not.exist');
  });

  it('Rename workspace', () => {
    cy.get('.card').eq(1).find('.card-option').click();
    cy.get('#workspace-context-menu').find('.menu-list').find('ion-item').eq(3).contains('Rename').click();
    cy.get('.text-input-modal').should('exist');
    cy.get('.ms-modal-header__title').contains('Rename workspace');
    cy.get('.ion-page').find('.closeBtn').should('exist');
    cy.get('ion-input').should('be.visible');
    cy.get('#cancel-button').should('be.visible');
    cy.get('.text-input-modal').find('.ms-modal-footer-buttons').find('ion-button').eq(1).as('renameButton').contains('Rename');
    cy.get('.text-input-modal').find('.input').find('input').type('New workspace name');
    cy.get('@renameButton').click();
    cy.get('.text-input-modal').should('not.exist');
    cy.checkToastMessage('success', 'Workspace has been successfully renamed to New workspace name');
  });

  it('Navigate into a workspace', () => {
    function checkListWorkspaceSelectedItem(workspaceName: string): void {
      cy.get('.list-workspaces').find('.sidebar-item').as('workspaceItems').should('have.length', 3);
      for (let i = 0; i < 3; i++) {
        cy.get('@workspaceItems')
          .eq(i)
          .as('currentWorkspace')
          .find('ion-label')
          .then((label) => {
            if (label.get(0).innerText === workspaceName) {
              cy.get('@currentWorkspace').should('have.class', 'item-selected');
            } else {
              cy.get('@currentWorkspace').should('have.class', 'item-not-selected');
            }
          });
      }
    }
    cy.contains('Trademeet').click();
    checkListWorkspaceSelectedItem('Trademeet');
    cy.get('.file-list-item').should('have.length.at.least', 1);
    cy.get('.topbar-left').find('ion-button.back-button').click();
    cy.get('.card').should('have.length', 3);
  });

  it('Open workspace menu in grid view', () => {
    cy.get('.card-option').first().click();
    cy.get('.popover-viewport').get('.list-group-item').should('have.length', 8);
    cy.get('.popover-viewport').get('.list-group-title').should('have.length', 4);
  });

  it('Open workspace menu in list view', () => {
    cy.get('#workspaces-ms-action-bar').find('#list-view').click();
    cy.get('.workspace-options > ion-button').first().click();
    cy.get('.popover-viewport').get('.list-group-item').should('have.length', 8);
    cy.get('.popover-viewport').get('.list-group-title').should('have.length', 4);
  });

  it('Check workspace sharing', () => {
    cy.get('.card').first().find('.shared-group').find('ion-avatar').as('avatars').should('have.length', 2);
    cy.get('@avatars').eq(0).contains('Ko');
    cy.get('@avatars').eq(1).contains('Ce');
  });

  it('Get link to the workspace', () => {
    cy.get('.card').eq(0).find('.card-option').click();
    cy.get('#workspace-context-menu').find('.menu-list').find('ion-item').eq(8).contains('Copy link').click();
    cy.checkToastMessage('info', 'Workspace link has been copied to clipboard.');
    cy.window().then((win) => {
      win.navigator.clipboard.readText().then((text) => {
        // cspell:disable-next-line
        const payload = 'k8QY94a350f2f629403db2269c44583f7aa1AcQ0Zkd8YbWfYF19LMwc55HjBOvI8LA8c_9oU2xaBJ0u2Ou0AFZYA4-QHhi2FprzAtUoAgMYwg';
        expect(text).to.eq(`parsec3://parsec.cloud/Org?a=path&p=${payload}`);
      });
    });
  });

  it('Check context menu from sidebar', () => {
    cy.get('.list-workspaces').find('.sidebar-item').as('workspaceItems').should('have.length', 3);
    cy.get('@workspaceItems').eq(1).find('.workspace-option').click();
    cy.get('#workspace-context-menu').find('.menu-list').find('ion-item').as('menuItem').should('have.length', 12);
    cy.get('@menuItem').eq(2).contains('Manage workspace');
    cy.get('@menuItem').eq(3).contains('Rename').should('not.be.visible');
    cy.get('@menuItem').eq(7).contains('Collaboration');
    cy.get('@menuItem').eq(8).contains('Copy link').click();
    cy.checkToastMessage('info', 'Workspace link has been copied to clipboard.');
    cy.window().then((win) => {
      win.navigator.clipboard.readText().then((text) => {
        // cspell:disable-next-line
        const payload = 'k8QY94a350f2f629403db2269c44583f7aa1AcQ0Zkd8YbWfYF19LMwc55HjBOvI8LA8c_9oU2xaBJ0u2Ou0AFZYA4-QHhi2FprzAtUoAgMYwg';
        expect(text).to.eq(`parsec3://parsec.cloud/Org?a=path&p=${payload}`);
      });
    });
    cy.get('@workspaceItems').eq(1).find('.workspace-option').click();
    cy.get('@menuItem').eq(9).contains('Sharing and roles').click();
    cy.get('.workspace-sharing-modal').find('.closeBtn').click();
    cy.get('@workspaceItems').eq(0).find('.workspace-option').click();
    cy.get('@menuItem').eq(3).contains('Rename').should('be.visible').click();
    cy.get('ion-modal').find('ion-title').contains('Rename workspace');
    cy.get('ion-modal').find('.closeBtn').click();
    cy.get('@workspaceItems').eq(1).find('.workspace-option').click();
    cy.get('@menuItem').eq(11).contains('Add to favorites').click();
    cy.get('@workspaceItems').eq(0).contains('The Copper Coronet');
    cy.get('.favorites').eq(0).find('.sidebar-item').should('be.visible');
  });
});

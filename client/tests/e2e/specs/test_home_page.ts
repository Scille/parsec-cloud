// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Check organization list', () => {
  beforeEach(() => {
    cy.visitApp();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Visit the app root url', () => {
    cy.contains('Your organizations');
    cy.get('.organization-list-row__col').should('have.length', 5);
  });

  it('Go to login page and back to organizations', () => {
    cy.get('.organization-list-row__col').should('have.length', 5);
    cy.get('.login-button').should('not.exist');
    cy.get('.organization-list').contains('Boby McBobFace').click();
    cy.wait(200);
    cy.get('.login-card-header').contains('Boby McBobFace');
    cy.get('.login-button').should('exist');
    cy.contains('Return to organizations').click();
    cy.contains('Your organizations');
    cy.get('.organization-list-row__col').should('have.length', 5);
  });

  it('Go to login page and enter wrong password', () => {
    cy.contains('Boby McBobFace').click();
    cy.get('.login-button').should('have.class', 'button-disabled');
    cy.get('#password-input').find('.input-item').as('inputItem').should('not.have.class', 'input-invalid');
    cy.get('#password-input').find('input').as('input').invoke('attr', 'type').should('eq', 'password');
    cy.get('@input').type('Wr0ngP@ssw0rd.');
    cy.get('.login-button').should('not.have.class', 'button-disabled');
    cy.get('.login-button').click();
    cy.get('#password-input').find('.form-error').as('formError').should('contain.text', 'Incorrect password.');
    cy.get('@inputItem').should('have.class', 'input-invalid');

    cy.get('@input').type('{backspace}');
    cy.get('@formError').should('not.contain.text', 'Incorrect password.');
    cy.get('@inputItem').should('have.class', 'input-invalid');

    cy.get('@input').clear();
    cy.get('@formError').should('not.contain.text', 'Incorrect password.');
    cy.get('@inputItem').should('not.have.class', 'input-invalid');
  });

  it('Go to login page and enter password', () => {
    cy.get('.organization-list-row__col').should('have.length', 5);
    cy.contains('Boby McBobFace').click();
    cy.get('.login-button').should('have.class', 'button-disabled');
    cy.get('#ms-password-input').find('input').invoke('attr', 'type').should('eq', 'password');
    cy.get('#ms-password-input').find('input').type('P@ssw0rd.');
    cy.get('.login-button').should('not.have.class', 'button-disabled');
    cy.get('.login-button').click();
    cy.get('.topbar-left__breadcrumb').contains('My workspaces');
    cy.wait(200);
  });

  it('Go to login page and sort and filter orgs', () => {
    cy.get('.organization-list-row__col').as('orgList').should('have.length', 5);
    // Sorted by org name asc by default
    cy.get('@orgList').first().contains('Alicey McAliceFace');
    cy.get('@orgList').last().contains('Malloryy McMalloryFace');
    cy.get('#ms-search-input').find('input').type('alice');
    cy.get('@orgList').should('have.length', 2);
    // Only 2 devices shown
    cy.get('@orgList').first().contains('Alicey McAliceFace');
    cy.get('@orgList').last().contains('Alicey McAliceFace');
    cy.get('#ms-search-input').find('input').clear();
    cy.get('@orgList').should('have.length', 5);
    // Change sort order
    cy.get('#organization-filter-select').contains('Organization').click();
    cy.get('.sorter-container').find('ion-item').as('item').should('have.length', 4);
    cy.get('@item').should('have.length', 4);
    cy.get('@item').eq(2).contains('User Name').click();
    cy.get('#organization-filter-select').contains('User Name').click();
    cy.get('@item').should('have.length', 4);
    cy.get('@item').first().contains('Ascending').click();
    // Now sorted by user name desc
    cy.get('@orgList').first().contains('Malloryy McMalloryFace');
    cy.get('@orgList').last().contains('Alicey McAliceFace');
  });

  it('Open create organization dialog', () => {
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').contains('I want to create an organization');
  });

  it('Open join organization dialog', () => {
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').contains('I received an invitation to join an organization');
  });

  it('Test join org link validation', () => {
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').find('ion-item').eq(1).click();
    cy.get('.text-input-modal').find('.input-item').as('input').should('have.class', 'input-default');
    cy.get('.text-input-modal').find('#next-button').as('okButton').should('have.class', 'button-disabled');
    cy.get('.text-input-modal').find('.form-error').as('error').should('not.be.visible');

    cy.get('@input').find('input').clear();
    // cspell:disable-next-line
    cy.get('@input').find('input').type('http://parsec.cloud/Test?a=claim_user&p=xBBHJlEjlpxNZYTCvBWWDPIS', { delay: 0 }).blur();
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('@error').should('be.visible');
    cy.get('@error').contains("Link should start with 'parsec3://'");

    cy.get('@input').find('input').clear();
    // cspell:disable-next-line
    cy.get('@input').find('input').type('parsec3://parsec.cloud/Test?p=xBBHJlEjlpxNZYTCvBWWDPIS', { delay: 0 }).blur();
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('@error').should('be.visible');
    cy.get('@error').contains('Link does not include an action');

    cy.get('@input').find('input').clear();
    cy.get('@input')
      .find('input')
      // cspell:disable-next-line
      .type('parsec3://parsec.cloud/Test?a=bootstrap_organization&p=xBBHJlEjlpxNZYTCvBWWDPIS', { delay: 0 })
      .blur();
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('@error').should('be.visible');
    cy.get('@error').contains('Link contains an invalid action');

    cy.get('@input').find('input').clear();
    cy.get('@input').find('input').type('parsec3://parsec.cloud/Test?a=claim_user', { delay: 0 }).blur();
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('@error').should('be.visible');
    cy.get('@error').contains('Link does not include a token');

    cy.get('@input').find('input').clear();
    cy.get('@input').find('input').type('parsec3://parsec.cloud/Test?a=claim_user&p=abcde', { delay: 0 }).blur();
    cy.get('@okButton').should('have.class', 'button-disabled');
    cy.get('@error').should('be.visible');
    cy.get('@error').contains('Link contains an invalid token');

    cy.get('@input').find('input').clear();
    cy.get('@input')
      .find('input')
      // cspell:disable-next-line
      .type('parsec3://parsec.cloud/Test?a=claim_user&p=xBBHJlEjlpxNZYTCvBWWDPIS', { delay: 0 })
      .blur();
    cy.get('@okButton').should('not.have.class', 'button-disabled');
    cy.get('@error').should('not.be.visible');
  });

  it('Log into organization with command and log out', () => {
    // Uses Cypress command to simplify the log in part
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('.topbar-left__breadcrumb').contains('My workspaces');
    cy.get('#profile-button').click();
    cy.get('.popover-viewport').contains('Log out').click();
    cy.get('.ion-page').find('.ms-modal').find('ion-buttons').contains('Log out').click();
    cy.get('.organization-title').contains('Your organizations');
    cy.wait(300);
  });
});

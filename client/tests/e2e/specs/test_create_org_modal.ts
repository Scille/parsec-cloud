// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Create a new organization', () => {

  beforeEach(() => {
    cy.visitApp('coolorg');
    cy.contains('Your organizations');
  });

  it('Open org creation modal', () => {
    cy.get('.create-organization-modal').should('not.exist');
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').find('ion-item').should('have.length', 2);
    cy.get('.popover-viewport').find('ion-item').first().contains('Create');
    cy.get('.popover-viewport').find('ion-item').first().click();
    cy.get('.create-organization-modal').should('exist');
    cy.get('.create-organization-modal').find('.modal-header__title').contains('Create an organization');
  });

  it('Go through the org creation process', () => {
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').find('ion-item').first().click();
    cy.wait(200);
    cy.get('.modal-header__title').as('title').contains('Create an organization');
    cy.get('.modal-header__text').as('subtitle').contains('Choose the name of your organization');

    // First page, org name
    cy.get('#next-button').should('have.class', 'button-disabled');
    cy.get('#previous-button').should('not.be.visible');
    cy.get('.org-name').find('ion-input').find('input').type('MyOrg');
    cy.get('#next-button').should('not.have.class', 'button-disabled');
    cy.get('#next-button').click();

    // Second page, user info
    cy.get('@title').contains('Enter your personal information');
    cy.get('@subtitle').contains('State your name, your email and your device name');
    cy.get('#next-button').should('have.class', 'button-disabled');
    cy.get('#previous-button').should('be.visible');
    cy.get('.user-info').find('ion-input').eq(0).find('input').type('Banjo');
    cy.get('#next-button').should('have.class', 'button-disabled');
    cy.get('.user-info').find('ion-input').eq(1).find('input').type('banjo@rare.com');
    // Device name is provided by default
    cy.get('#next-button').should('not.have.class', 'button-disabled');
    cy.get('#next-button').click();

    // Third page, server
    cy.get('@title').contains('Choose the server you need');
    cy.get('@subtitle').contains('Choose the type of server you need');
    cy.get('#next-button').should('not.have.class', 'button-disabled');
    cy.get('#previous-button').should('be.visible');

    cy.get('.org-server').find('ion-input').should('not.be.visible');
    cy.get('.org-server').find('ion-radio').should('have.length', 2);
    cy.get('.org-server').find('ion-radio').first().should('have.class', 'radio-checked');
    cy.get('.org-server').find('ion-radio').last().should('not.have.class', 'radio-checked');
    cy.get('.org-server').find('ion-radio').last().click();
    cy.get('.org-server').find('ion-radio').first().should('not.have.class', 'radio-checked');
    cy.get('.org-server').find('ion-radio').last().should('have.class', 'radio-checked');
    cy.get('#next-button').should('have.class', 'button-disabled');
    cy.get('.org-server').find('ion-input').should('be.visible');

    // Type 'parsec' as server addr, it's invalid
    cy.get('.org-server').find('ion-input').find('input').type('parsec');
    cy.get('#next-button').should('have.class', 'button-disabled');
    // Adds the last part of the server addr
    cy.get('.org-server').find('ion-input').find('input').type('://localhost?no_ssl=true');
    cy.get('#next-button').should('not.have.class', 'button-disabled');
    cy.get('#next-button').click();

    // Fourth page, password
    cy.get('@title').contains('Create your password');
    cy.get('@subtitle').contains('Finally, create your password');

    cy.get('#next-button').should('have.class', 'button-disabled');
    cy.get('#previous-button').should('be.visible');

    cy.get('.org-password').find('ion-input').first().find('input').type('AVery');
    cy.get('.org-password').find('.form-helperText').contains('Do not match');
    cy.get('.org-password').find('.password-level').should('have.class', 'password-level-low');
    cy.get('.org-password').find('.password-level__text').contains('Low');
    cy.get('#next-button').should('have.class', 'button-disabled');

    cy.get('.org-password').find('ion-input').first().find('input').type('L0ng');
    cy.get('.org-password').find('.password-level__text').contains('Moderate');
    cy.get('.org-password').find('.password-level').should('have.class', 'password-level-medium');

    cy.get('.org-password').find('ion-input').first().find('input').type('P@ssw0rd');
    cy.get('.org-password').find('.password-level__text').contains('Strong');
    cy.get('.org-password').find('.password-level').should('have.class', 'password-level-high');

    cy.get('.org-password').find('ion-input').last().find('input').type('AVeryL0ngP@ssw0rd');
    cy.get('.org-password').find('.form-helperText').should('not.exist');
    cy.get('#next-button').should('not.have.class', 'button-disabled');
    cy.get('#next-button').click();

    // Fifth page, summary
    cy.get('@title').contains('Overview of your organization');
    cy.get('@subtitle').contains('Check that the information is correct');
    cy.get('.org-summary').find('.summary-item').as('summaryItems').should('have.length', 5);
    cy.get('@summaryItems').eq(0).find('.summary-item__text').contains('MyOrg');
    cy.get('@summaryItems').eq(1).find('.summary-item__text').contains('Banjo');
    cy.get('@summaryItems').eq(2).find('.summary-item__text').contains('banjo@rare.com');
    cy.get('@summaryItems').eq(3).find('.summary-item__text').contains('my_device');
    cy.get('@summaryItems').eq(4).find('.summary-item__text').contains('parsec://localhost?no_ssl=true');
    cy.get('#next-button').contains('Create organization').click();

    // Sixth page, spinner, X button should be hidden
    cy.get('.closeBtn').should('not.be.visible');
    cy.get('#next-button').should('not.be.visible');
    cy.get('#previous-button').should('not.be.visible');

    cy.wait(200);

    // Spinner gets replaced after 2s
    // Seventh page, end
    cy.get('@title').contains('Your organization has been created!');
    cy.get('#next-button').contains('Let\'s go!');
    cy.get('#next-button').should('not.have.class', 'button-disabled');
    cy.get('.closeBtn').should('not.be.visible');
    cy.get('#next-button').click();
    cy.get('.modal-header__title').should('not.exist');

    // Should be logged in on workspace page
    cy.get('#button-new-workspace').contains('New workspace');
    cy.get('.card').should('have.length', 2);
  });

  it('Close with X button', () => {
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').find('ion-item').first().click();
    cy.get('.create-organization-modal').find('.modal-header__title').contains('Create an organization');
    cy.get('.closeBtn').should('be.visible');
    cy.get('.closeBtn').click();
    cy.get('ion-alert').contains('Are you sure?');

    // Cancel
    cy.get('ion-alert').get('.alert-button-role-cancel').click();
    cy.get('ion-alert').should('not.exist');
    cy.get('.create-organization-modal').should('exist');

    cy.get('.closeBtn').click();
    cy.get('ion-alert').contains('Are you sure?');

    // Confirm
    cy.get('.alert-button-role-confirm').click();
    cy.get('ion-alert').should('not.exist');
    cy.get('.create-organization-modal').should('not.exist');
  });

  it('Can go to the previous page', () => {
    function goToPage(page: number): void {
      for (let i = 1; i < page; i++) {
        cy.get('#next-button').click();
      }
    }

    function goBackToStartFrom(page: number): void {
      for (let i = page; i > 1; i--) {
        cy.get('#previous-button').click();
      }
      cy.get('#previous-button').should('not.be.visible');
      cy.get('@title').contains('Create an organization');
    }

    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').find('ion-item').first().click();
    cy.wait(200);
    cy.get('.modal-header__title').as('title').contains('Create an organization');

    // Page 1
    cy.get('.org-name').find('ion-input').find('input').type('MyOrg');

    // Go to 2
    cy.get('#next-button').click();
    cy.get('@title').contains('Enter your personal information');

    // Back to 1
    goBackToStartFrom(2);
    // Back to 2
    goToPage(2);

    cy.get('.user-info').find('ion-input').eq(0).find('input').type('Banjo');
    cy.get('.user-info').find('ion-input').eq(1).find('input').type('banjo@rare.com');

    // Go to 3
    cy.get('#next-button').click();
    // Back to 1
    goBackToStartFrom(3);
    // Back to 3
    goToPage(3);

    // 3 is just the server choice, to straight to 4
    cy.get('#next-button').click();
    // Back to 1
    goBackToStartFrom(4);
    // Back to 4
    goToPage(4);

    cy.get('.org-password').find('ion-input').first().find('input').type('AVeryL0ngP@ssw0rd');
    cy.get('.org-password').find('ion-input').last().find('input').type('AVeryL0ngP@ssw0rd');
    cy.get('#next-button').click();

    goBackToStartFrom(5);
    goToPage(5);

    cy.get('#next-button').click();
    // From now on, shouldn't be able to go back
    cy.get('#previous-button').should('not.be.visible');
  });

  it('Can edit from summary page', () => {
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').find('ion-item').first().click();

    cy.wait(200);

    cy.get('.modal-header__title').as('title').contains('Create an organization');

    cy.get('.org-name').find('ion-input').find('input').type('MyOrg');
    cy.get('#next-button').click();
    cy.get('.user-info').find('ion-input').eq(0).find('input').type('Banjo');
    cy.get('.user-info').find('ion-input').eq(1).find('input').type('banjo@rare.com');
    cy.get('#next-button').click();
    cy.get('#next-button').click();
    cy.get('.org-password').find('ion-input').first().find('input').type('AVeryL0ngP@ssw0rd');
    cy.get('.org-password').find('ion-input').last().find('input').type('AVeryL0ngP@ssw0rd');
    cy.get('#next-button').click();

    cy.get('.org-summary').find('.summary-item').as('summaryItems').should('have.length', 5);
    // Org name, should take us back to the first page
    cy.get('@summaryItems').eq(0).find('.summary-item__button').click();
    cy.get('@title').contains('Create an organization');

    // Back to summary
    cy.get('#next-button').click();
    cy.get('#next-button').click();
    cy.get('#next-button').click();
    cy.get('#next-button').click();

    // User name, should take us back to the second page
    cy.get('@summaryItems').eq(1).find('.summary-item__button').click();
    cy.get('@title').contains('Enter your personal information');

    // Back to summary
    cy.get('#next-button').click();
    cy.get('#next-button').click();
    cy.get('#next-button').click();

    // User email, should take us back to the second page
    cy.get('@summaryItems').eq(2).find('.summary-item__button').click();
    cy.get('@title').contains('Enter your personal information');

    // Back to summary
    cy.get('#next-button').click();
    cy.get('#next-button').click();
    cy.get('#next-button').click();

    // Device name, should take us back to the second page
    cy.get('@summaryItems').eq(3).find('.summary-item__button').click();
    cy.get('@title').contains('Enter your personal information');

    // Back to summary
    cy.get('#next-button').click();
    cy.get('#next-button').click();
    cy.get('#next-button').click();

    // Server, should take us back to the third page
    cy.get('@summaryItems').eq(4).find('.summary-item__button').click();
    cy.get('@title').contains('Choose the server you need');

    // Back to summary
    cy.get('#next-button').click();
    cy.get('#next-button').click();
  });
});

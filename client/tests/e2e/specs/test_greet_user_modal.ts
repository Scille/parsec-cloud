// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Greet user into an organization', () => {
  const WAIT_TIME = 500;

  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('#invitations-button').contains('2 invitations').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Open greet user modal', () => {
    cy.get('.invitations-list-popover').should('exist');
    cy.get('.greet-organization-modal').should('not.exist');
    cy.get('.invitation-list-item-container').find('.invitation-list-item').as('invitations').should('have.length', 2);
    cy.get('@invitations').first().realHover().find('.invitation-actions-buttons').last().contains('Greet').click();
    cy.get('.invitations-list-popover').should('not.exist');
    cy.get('.greet-organization-modal').should('exist');
    cy.get('.greet-organization-modal').find('.modal-header__title').contains('Onboard a new user');
    cy.get('.greet-organization-modal').find('#next-button').should('have.attr', 'disabled');
    cy.wait(WAIT_TIME);
    cy.get('.greet-organization-modal').find('#next-button').should('not.have.attr', 'disabled');
    cy.get('.greet-organization-modal').find('.closeBtn').should('exist');
    cy.get('.greet-organization-modal').find('.closeBtn').should('be.visible');
  });

  it('Go through the greet process', () => {
    function checkStepper(activeIndex: number): void {
      cy.get('@stepper').find('.ms-wizard-stepper-step').as('steps').should('have.length', 3);
      for (let i = 0; i < 3; i++) {
        if (i < activeIndex) {
          cy.get('@steps').eq(i).find('.circle').find('.inner-circle-done').should('exist');
        } else if (i === activeIndex) {
          cy.get('@steps').eq(i).find('.circle').find('.inner-circle-active').should('exist');
        } else {
          cy.get('@steps').eq(i).find('.circle').find('div').should('have.length', 0);
        }
      }
    }

    cy.get('.invitation-list-item-container').find('.invitation-list-item').as('invitations').should('have.length', 2);
    cy.get('@invitations').first().realHover().find('.invitation-actions-buttons').last().contains('Greet').click();
    cy.get('.greet-organization-modal').should('exist');
    cy.wait(WAIT_TIME);
    cy.get('.greet-organization-modal').find('#next-button').as('nextButton').click();
    cy.get('.greet-organization-modal').find('.modal-footer').as('footer');
    cy.get('.greet-organization-modal').find('.modal-header').find('.modal-header__title').as('title');
    cy.get('.greet-organization-modal').find('.ms-wizard-stepper').as('stepper');
    checkStepper(0);
    cy.get('@title').contains('Share your code');
    cy.get('.greet-organization-modal').find('.caption-code').contains('2DEF');
    cy.get('@nextButton').should('not.be.visible');
    cy.wait(WAIT_TIME);
    cy.get('@title').contains('Get guest code');
    cy.get('.greet-organization-modal').find('ion-grid').find('.caption-code').should('have.length', 4);
    checkStepper(1);
    cy.get('.greet-organization-modal').find('ion-grid').find('.caption-code').eq(1).click();
    cy.get('@title').contains('Contact details');
    cy.get('@footer').find('ion-spinner').should('be.visible');
    cy.get('@nextButton').should('not.be.visible');
    cy.wait(WAIT_TIME);
    cy.get('@title').contains('Contact details');
    checkStepper(2);
    cy.get('@footer').find('ion-spinner').should('not.be.visible');
    cy.get('.greet-organization-modal').find('.user-info-page').find('ion-input').as('inputs').should('have.length', 2);
    cy.get('@inputs').eq(1).should('have.class', 'input-disabled');
    cy.get('.user-info-page').find('#dropdown-popover-button').as('select');
    cy.get('@select').should('not.have.value');
    cy.get('@nextButton').contains('Approve');
    cy.get('@nextButton').should('have.attr', 'disabled');
    cy.get('@select').click();
    cy.get('@select').find('ion-icon').should('have.class', 'popover-is-open');
    cy.get('.popover-viewport').should('exist').find('ion-list').find('ion-item').as('roles-popover').should('have.length', 3);
    cy.get('@roles-popover').find('.option-text__label').contains('Administrator');
    cy.get('@roles-popover').find('.option-text__description').contains('Can manage the organization');
    cy.get('@roles-popover').find('.option-text__label').contains('Standard');
    cy.get('@roles-popover').find('.option-text__description').contains('Can create workspaces.');
    cy.get('@roles-popover').find('.option-text__label').contains('Outsider');
    cy.get('@roles-popover').find('.option-text__description').contains('Can only access shared workspaces.');
    cy.get('@roles-popover').eq(1).click();
    cy.get('@nextButton').should('not.have.attr', 'disabled');
    cy.get('@nextButton').click();
    cy.get('@title').contains('User has been added successfully!');
    cy.get('.greet-organization-modal').find('.user-info__email').find('.cell').contains('gordon.freeman@blackmesa.nm');
    cy.get('.greet-organization-modal').find('.user-info__role').find('.label-profile').contains('Standard');
    cy.get('@stepper').should('not.be.visible');
    cy.get('@nextButton').click();
    cy.get('.greet-organization-modal').should('not.exist');
  });

  it('Select wrong code', () => {
    cy.get('.invitation-list-item-container').find('.invitation-list-item').as('invitations').should('have.length', 2);
    cy.get('@invitations').first().realHover().find('.invitation-actions-buttons').last().contains('Greet').click();
    cy.get('.greet-organization-modal').should('exist');
    cy.wait(WAIT_TIME);
    cy.get('.greet-organization-modal').find('#next-button').as('nextButton').click();
    cy.get('.greet-organization-modal').find('.modal-footer').as('footer');
    cy.get('.greet-organization-modal').find('.modal-header').find('.modal-header__title').as('title');
    cy.get('@title').contains('Share your code');
    cy.wait(WAIT_TIME);
    cy.get('@title').contains('Get guest code');
    cy.get('.greet-organization-modal').find('ion-grid').find('.caption-code').should('have.length', 4);
    cy.get('.greet-organization-modal').find('ion-grid').find('.caption-code').eq(0).click();
    cy.checkToastMessage('error', "You didn't select the correct code. Please restart the process.");
    cy.wait(WAIT_TIME);
    cy.get('@title').contains('Onboard a new user');
    cy.get('@nextButton').contains('Start');
    cy.get('@nextButton').should('not.have.attr', 'disabled');
  });

  it('Select none code', () => {
    cy.get('.invitation-list-item-container').find('.invitation-list-item').as('invitations').should('have.length', 2);
    cy.get('@invitations').first().realHover().find('.invitation-actions-buttons').last().contains('Greet').click();
    cy.get('.greet-organization-modal').should('exist');
    cy.wait(WAIT_TIME);
    cy.get('.greet-organization-modal').find('#next-button').as('nextButton').click();
    cy.get('.greet-organization-modal').find('.modal-footer').as('footer');
    cy.get('.greet-organization-modal').find('.modal-header').find('.modal-header__title').as('title');
    cy.get('@title').contains('Share your code');
    cy.wait(WAIT_TIME);
    cy.get('@title').contains('Get guest code');
    cy.get('.greet-organization-modal').find('ion-grid').find('.button-clear').contains('None shown').click();
    cy.checkToastMessage('error', "If you didn't see the correct code, it could be a security concern.");
    cy.wait(WAIT_TIME);
    cy.get('@title').contains('Onboard a new user');
    cy.get('@nextButton').contains('Start');
    cy.get('@nextButton').should('not.have.attr', 'disabled');
  });

  it('Close with X button', () => {
    cy.get('.invitation-list-item-container').find('.invitation-list-item').as('invitations').should('have.length', 2);
    cy.get('@invitations').first().realHover().find('.invitation-actions-buttons').last().contains('Greet').click();
    cy.get('.greet-organization-modal').should('exist');
    cy.get('.greet-organization-modal').find('#next-button').click();
    cy.get('.greet-organization-modal').find('.closeBtn').should('be.visible');
    cy.get('.greet-organization-modal').find('.closeBtn').click();

    cy.get('.question-modal').find('.ms-modal-header__title').contains('Cancel the onboarding');
    cy.get('.question-modal')
      .find('.ms-modal-header__text')
      .contains('Are you sure you want to cancel the process? Information will not be saved, you will have to restart.');
    cy.get('.question-modal').find('#next-button').contains('Cancel the process');
    cy.get('.question-modal').find('#cancel-button').contains('Resume').click();
    cy.get('.question-modal').should('not.exist');

    // Can't get the modal to dismiss
    // cy.get('.greet-organization-modal').find('.closeBtn').click();
    // cy.get('.question-modal').find('#next-button').click();
    // cy.get('.question-modal').should('not.exist');
    // cy.get('.greet-organization-modal').should('not.exist');
  });
});

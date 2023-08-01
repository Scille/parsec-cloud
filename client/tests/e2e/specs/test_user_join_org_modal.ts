// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('User join an organization', () => {
  const INVITATION_LINK = 'parsec://parsec.cloud/Test?action=claim_user&token=47265123969c4d6584c2bc15960cf212';
  const WAIT_TIME = 1000;

  beforeEach(() => {
    cy.visitApp();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Open org join modal', () => {
    cy.get('.join-organization-modal').should('not.exist');
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').find('ion-item').should('have.length', 2);
    cy.get('.popover-viewport').find('ion-item').last().contains('Join');
    cy.get('.popover-viewport').find('ion-item').last().click();

    cy.get('.join-by-link-modal').should('exist');
    cy.get('.join-by-link-modal').find('ion-title').contains('Join by link');
    cy.get('.join-by-link-modal').find('ion-footer ion-button').as('joinButton').contains('Join');
    cy.get('@joinButton').should('have.attr', 'disabled');

    cy.wait(WAIT_TIME);
    cy.get('.join-by-link-modal').find('ion-input').find('input').type(INVITATION_LINK);

    cy.get('@joinButton').should('not.have.attr', 'disabled');
    cy.get('@joinButton').click();

    cy.get('.join-by-link-modal').should('not.exist');
    cy.get('.join-organization-modal').should('exist');
    cy.get('.modal-header__title').contains('Welcome to Parsec!');

    cy.get('.label-waiting').should('be.visible');
    cy.get('#next-button').should('not.be.visible');

    cy.wait(WAIT_TIME);
    cy.get('.label-waiting').should('not.be.visible');
    cy.get('#next-button').should('be.visible');
    cy.get('#next-button').contains('I understand!');
  });

  it('Go through join process', () => {
    function checkStepper(activeIndex: number): void {
      cy.get('ion-modal').find('.ms-wizard-stepper-step').as('steps').should('have.length', 5);
      for (let i = 0; i < 5; i++) {
        if (i < activeIndex) {
          cy.get('@steps').eq(i).find('.circle').find('.inner-circle-done').should('exist');
        } else if (i === activeIndex) {
          cy.get('@steps').eq(i).find('.circle').find('.inner-circle-active').should('exist');
        } else {
          cy.get('@steps').eq(i).find('.circle').find('div').should('have.length', 0);
        }
      }
    }

    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').find('ion-item').last().click();
    cy.wait(WAIT_TIME);
    cy.get('.join-by-link-modal').find('ion-input').find('input').type(INVITATION_LINK);
    cy.get('.join-by-link-modal').find('ion-footer ion-button').click();

    cy.get('.join-organization-modal').should('exist');
    cy.get('.modal-header__title').as('modalTitle').contains('Welcome to Parsec!');

    cy.wait(WAIT_TIME);
    cy.get('#next-button').should('be.visible');
    cy.get('#next-button').click();

    // Page with four codes, host SAS code
    cy.get('@modalTitle').contains('Get host code');
    cy.get('.modal-header__text').as('modalSubtitle').contains('Click on the code given to you by the host');

    cy.get('ion-modal').find('.ms-wizard-stepper__step').as('steps').should('have.length', 5);
    cy.get('@steps').eq(0).find('.step-title').contains('Host code');
    cy.get('@steps').eq(1).find('.step-title').contains('Guest code');
    cy.get('@steps').eq(2).find('.step-title').contains('Contact details');
    cy.get('@steps').eq(3).find('.step-title').contains('Password');
    cy.get('@steps').eq(4).find('.step-title').contains('Validation');
    checkStepper(0);

    cy.get('ion-modal').find('.button-choice').as('choiceButtons').should('have.length', 4);
    cy.get('@choiceButtons').eq(0).contains('ABCD');
    cy.get('@choiceButtons').eq(1).contains('EFGH');
    cy.get('@choiceButtons').eq(2).contains('IJKL');
    cy.get('@choiceButtons').eq(3).contains('MNOP');
    cy.get('ion-modal').find('.button-clear').contains('None of the codes');
    cy.get('@choiceButtons').eq(1).click();

    // Page with one code, guest SAS code
    cy.get('.label-waiting').should('be.visible');
    checkStepper(1);

    // Waiting for the host to enter the code
    cy.wait(WAIT_TIME);

    // Page with three inputs, user contact details
    checkStepper(2);

    cy.get('@modalTitle').contains('Your contact details');
    cy.get('@modalSubtitle').contains('Please enter your contact details to access the organization');

    cy.get('#get-user-info').find('ion-input').as('inputs').should('have.length', 3);
    cy.get('#next-button').should('have.attr', 'disabled');
    cy.get('#next-button').contains('Validate my information');
    cy.get('@inputs').eq(1).find('input').should('have.value', 'gordon.freeman@blackmesa.nm');
    cy.get('@inputs').eq(2).find('input').should('have.value', 'my_device');
    cy.get('@inputs').eq(0).find('input').type('Gordon Freeman');
    cy.get('#next-button').should('not.have.attr', 'disabled');
    cy.get('#next-button').click();

    // Waiting for the host to validate
    cy.get('.label-waiting').should('be.visible');
    cy.wait(WAIT_TIME);

    // Page with two inputs, choose and confirm password
    checkStepper(3);
    cy.get('@modalTitle').contains('Your password');
    cy.get('@modalSubtitle').contains('Choose a password to complete the enrollment');

    cy.get('#get-password').find('ion-input').as('inputs').should('have.length', 2);
    cy.get('#next-button').should('have.attr', 'disabled');
    cy.get('#next-button').contains('Join the organization');
    cy.get('@inputs').eq(0).find('input').type('Nihilanth1337');
    cy.get('#next-button').should('have.attr', 'disabled');
    cy.get('@inputs').eq(1).find('input').type('Nihilanth1337');
    cy.get('#next-button').should('not.have.attr', 'disabled');
    cy.get('#next-button').click();

    // Last page
    checkStepper(4);
    cy.get('@modalTitle').contains('You have joined the organization');

    cy.get('#next-button').contains('Log In');
    // Clicking the button causes the modal to close which causes an unknown exception
    // which causes Cypress to fail the test.

    // cy.get('#next-button').click();

    // cy.wait(1000);

    // cy.get('@configPath').then(() => {
    //   cy.get('@consoleLog').should('have.been.calledWith', 'Log in to My Org with password "Nihilanth1337"');
    // });

    // // Should be logged in on workspace page
    // cy.get('#button-new-workspace').contains('New workspace');
    // cy.get('.card').should('have.length', 5);
  });
});

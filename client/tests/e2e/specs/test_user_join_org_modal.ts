// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('User join an organization', () => {
  // cspell:disable-next-line
  const INVITATION_LINK = 'parsec3://parsec.cloud/Test?a=claim_user&p=xBBHJlEjlpxNZYTCvBWWDPIS';
  const WAIT_TIME = 500;

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

    cy.get('.text-input-modal').should('exist');
    cy.get('.text-input-modal').find('ion-title').contains('Join by link');
    cy.get('.text-input-modal').find('#next-button').as('joinButton').contains('Join');
    cy.get('@joinButton').should('have.attr', 'disabled');

    cy.wait(WAIT_TIME);
    cy.get('.text-input-modal').find('ion-input').find('input').type(INVITATION_LINK, { delay: 0 });

    cy.get('@joinButton').should('not.have.attr', 'disabled');
    cy.get('@joinButton').click();

    cy.get('.text-input-modal').should('not.exist');
    cy.get('.join-organization-modal').should('exist');
    cy.get('.modal-header__title').contains('Welcome to Parsec!');

    cy.get('.spinner-container').should('be.visible');
    cy.get('#next-button').should('not.be.visible');

    cy.wait(WAIT_TIME);
    cy.get('.spinner-container').should('not.be.visible');
    cy.get('#next-button').should('be.visible');
    cy.get('#next-button').contains('I understand!');
  });

  it('Go through join process', () => {
    function checkStepper(activeIndex: number): void {
      cy.get('ion-modal').find('.ms-wizard-stepper-step').as('steps').should('have.length', 4);
      for (let i = 0; i < 4; i++) {
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
    cy.get('.text-input-modal').find('ion-input').find('input').type(INVITATION_LINK, { delay: 0 });
    cy.get('.text-input-modal').find('#next-button').click();

    cy.get('.join-organization-modal').should('exist');
    cy.get('.modal-header__title').as('modalTitle').contains('Welcome to Parsec!');

    cy.wait(WAIT_TIME);
    cy.get('#next-button').should('be.visible');
    cy.get('#next-button').click();

    // Page with four codes, host SAS code
    cy.get('@modalTitle').contains('Get host code');
    cy.get('.modal-header__text').as('modalSubtitle').contains('Click on the code given to you by the host');

    cy.get('ion-modal').find('.ms-wizard-stepper__step').as('steps').should('have.length', 4);
    cy.get('@steps').eq(0).find('.step-title').contains('Host code');
    cy.get('@steps').eq(1).find('.step-title').contains('Guest code');
    cy.get('@steps').eq(2).find('.step-title').contains('Contact details');
    cy.get('@steps').eq(3).find('.step-title').contains('Authentication');
    checkStepper(0);

    cy.get('ion-modal').find('.button-choice').as('choiceButtons').should('have.length', 4);
    cy.get('@choiceButtons').eq(0).contains('1ABC');
    cy.get('@choiceButtons').eq(1).contains('2DEF');
    cy.get('@choiceButtons').eq(2).contains('3GHI');
    cy.get('@choiceButtons').eq(3).contains('4JKL');
    cy.get('ion-modal').find('.button-clear').contains('None shown');
    cy.get('@choiceButtons').eq(1).click();

    // Page with one code, guest SAS code
    cy.get('.caption-code').contains('1337');
    cy.get('.spinner-container').should('be.visible');
    checkStepper(1);

    // Waiting for the host to enter the code
    cy.wait(WAIT_TIME);

    // Page with three inputs, user contact details
    checkStepper(2);

    cy.get('@modalTitle').contains('Your contact details');
    cy.get('@modalSubtitle').contains('Please enter your contact details to access the organization');

    cy.get('#get-user-info').find('ion-input').as('inputs').should('have.length', 2);
    cy.get('#next-button').should('have.attr', 'disabled');
    cy.get('#next-button').contains('Validate my information');
    cy.get('@inputs').eq(1).find('input').should('have.value', 'shadowheart@swordcoast.faerun');
    // cspell:disable-next-line
    cy.get('@inputs').eq(0).find('input').type('Shadowheart');
    cy.get('#next-button').should('not.have.attr', 'disabled');
    cy.get('#next-button').click();

    // Waiting for the host to validate
    cy.get('.spinner-container').should('be.visible');
    cy.wait(WAIT_TIME);

    // Page with two inputs, choose and confirm password
    checkStepper(3);

    cy.get('@modalTitle').contains('Authentication');
    cy.get('@modalSubtitle').contains('Choose an authentication method to complete the onboarding.');
    cy.get('.choose-auth-page').find('ion-radio').first().contains('Unavailable on web');
    cy.get('.choose-auth-page').find('ion-radio').first().should('have.class', 'radio-disabled');

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
  });

  it('Close with X button', () => {
    cy.get('#create-organization-button').click();
    cy.get('.popover-viewport').find('ion-item').last().click();
    cy.wait(WAIT_TIME);
    cy.get('.text-input-modal').find('ion-input').find('input').type(INVITATION_LINK), { delay: 0 };
    cy.get('.text-input-modal').find('#next-button').click();

    cy.get('.join-organization-modal').should('exist');
    cy.get('.join-organization-modal').find('.closeBtn').should('be.visible');
    cy.get('.join-organization-modal').find('.closeBtn').click();

    cy.get('.question-modal').find('.ms-modal-header__title').contains('Cancel the onboarding');
    cy.get('.question-modal')
      .find('.ms-modal-header__text')
      .contains('Are you sure you want to cancel the onboarding process? Information will not be saved, you will have to restart.');
    cy.get('.question-modal').find('#next-button').contains('Cancel process');
    cy.get('.question-modal').find('#cancel-button').contains('Resume').click();
    cy.get('.question-modal').should('not.exist');

    // Can't get the modal to dismiss
    // cy.get('.join-organization-modal').find('.closeBtn').click();
    // cy.get('.question-modal').find('#next-button').click();
    // cy.get('.question-modal').should('not.exist');
    // cy.get('.join-organization-modal').should('not.exist');
  });
});

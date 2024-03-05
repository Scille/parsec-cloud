// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Claim new device', () => {
  const INVITATION_LINK = 'parsec3://parsec.cloud/Test?action=claim_device&token=47265123969c4d6584c2bc15960cf212';
  const WAIT_TIME = 1000;

  beforeEach(() => {
    cy.visitApp();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Open org join modal', () => {
    cy.get('.text-input-modal').should('not.exist');
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
    cy.get('.modal-header__title').contains('Add a new device');

    cy.wait(WAIT_TIME);
    cy.get('.label-waiting').should('not.be.visible');
    cy.get('#next-button').should('be.visible');
    cy.get('#next-button').contains('I understand!');
  });

  it('Go through join process', () => {
    function checkStepper(activeIndex: number): void {
      cy.get('ion-modal').find('.ms-wizard-stepper-step').as('steps').should('have.length', 3);
      for (let i = 0; i < 2; i++) {
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
    cy.get('.modal-header__title').as('modalTitle').contains('Add a new device');

    cy.wait(WAIT_TIME);
    cy.get('#next-button').should('be.visible');
    cy.get('#next-button').click();

    // Page with four codes, host SAS code
    cy.get('@modalTitle').contains('Get host code');
    cy.get('.modal-header__text').as('modalSubtitle').contains('Click on the code you see on the main device.');

    cy.get('ion-modal').find('.ms-wizard-stepper__step').as('steps').should('have.length', 3);
    cy.get('@steps').eq(0).find('.step-title').contains('Host code');
    cy.get('@steps').eq(1).find('.step-title').contains('Guest code');
    cy.get('@steps').eq(2).find('.step-title').contains('Password');
    checkStepper(0);

    cy.get('ion-modal').find('.button-choice').as('choiceButtons').should('have.length', 4);
    cy.get('@choiceButtons').eq(0).contains('5MNO');
    cy.get('@choiceButtons').eq(1).contains('6PQR');
    cy.get('@choiceButtons').eq(2).contains('7STU');
    cy.get('@choiceButtons').eq(3).contains('8VWX');
    cy.get('ion-modal').find('.button-clear').contains('None shown');
    cy.get('@choiceButtons').eq(2).click();

    // Page with one code, guest SAS code
    cy.get('.label-waiting').should('be.visible');
    checkStepper(1);

    // Waiting for the host to enter the code
    cy.wait(WAIT_TIME);

    // Page with 3 inputs (password, confirmPassword and deviceName)
    checkStepper(2);

    cy.get('@modalTitle').contains('Create a password');
    cy.get('@modalSubtitle').contains('Finally, create a password for your new device.');

    cy.get('#get-password').find('ion-input').as('inputs').should('have.length', 2);
    cy.get('#next-button').should('have.attr', 'disabled');
    cy.get('#next-button').contains('Confirm password');
    cy.get('@inputs').eq(0).find('input').type('Password23;-$aze');
    cy.get('#next-button').should('have.attr', 'disabled');
    cy.get('@inputs').eq(1).find('input').type('Password23;-$aze');
    cy.get('#next-button').should('not.have.attr', 'disabled');
    cy.get('#next-button').click();

    // Last page
    cy.get('@modalTitle').contains('The device has been added!');

    cy.get('#next-button').contains('Log in');
  });
});

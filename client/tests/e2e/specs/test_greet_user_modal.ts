// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Greet user into an organization', () => {

  const WAIT_TIME = 1000;

  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd');
    cy.get('.organization-card__manageBtn').click();
    cy.get('.user-menu__item').eq(2).click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Open greet user modal', () => {
    cy.get('.greet-organization-modal').should('not.exist');
    cy.get('.invitation-list').find('.invitation-list-item').should('have.length', 2);
    cy.get('.invitation-list').find('.invitation-list-item').find('.button-default').eq(0).click();
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

    cy.get('.invitation-list').find('.invitation-list-item').find('.button-default').eq(0).click();
    cy.get('.greet-organization-modal').should('exist');
    cy.wait(WAIT_TIME);
    cy.get('.greet-organization-modal').find('#next-button').as('nextButton').click();
    cy.get('.greet-organization-modal').find('.modal-footer').as('footer');
    cy.get('.greet-organization-modal').find('.modal-header').find('.modal-header__title').as('title');
    cy.get('.greet-organization-modal').find('.ms-wizard-stepper').as('stepper');
    checkStepper(0);
    cy.get('@title').contains('Share your code');
    cy.get('.greet-organization-modal').find('.caption-code').contains('ABCD');
    cy.get('@footer').find('ion-spinner').should('exist');
    cy.get('@footer').find('ion-spinner').should('be.visible');
    cy.get('@nextButton').should('not.be.visible');
    cy.wait(WAIT_TIME);
    cy.get('@title').contains('Get guest code');
    cy.get('.greet-organization-modal').find('ion-grid').find('.caption-code').should('have.length', 4);
    checkStepper(1);
    cy.get('.greet-organization-modal').find('ion-grid').find('.caption-code').eq(0).click();
    cy.get('@title').contains('Contact details');
    cy.get('@footer').find('ion-spinner').should('be.visible');
    cy.get('@nextButton').should('not.be.visible');
    cy.wait(WAIT_TIME);
    cy.get('@title').contains('Contact details');
    checkStepper(2);
    cy.get('@footer').find('ion-spinner').should('not.be.visible');
    cy.get('.greet-organization-modal').find('.user-info-page').find('ion-input').as('inputs').should('have.length', 3);
    cy.get('@inputs').eq(1).should('have.class', 'input-disabled');
    cy.get('.user-info-page').find('ion-select').as('select').find('input').should('not.have.value');
    cy.get('@nextButton').contains('Approve');
    cy.get('@nextButton').should('have.attr', 'disabled');
    cy.get('@select').click();
    cy.get('ion-alert').find('.select-interface-option').should('have.length', 3);
    cy.get('ion-alert').find('.select-interface-option').eq(1).click();
    cy.get('ion-alert').find('.alert-button-group').find('button').eq(1).click();
    cy.get('@select').find('input').should('have.value', 'standard');
    cy.get('@nextButton').should('not.have.attr', 'disabled');
    cy.get('@nextButton').click();
    cy.get('@title').contains('User has been added successfully!');
    cy.get('.greet-organization-modal').find('.user-profile').contains('Standard');
    cy.get('@stepper').should('not.be.visible');
    cy.get('@nextButton').click();
    cy.get('.greet-organization-modal').should('not.exist');
  });
});

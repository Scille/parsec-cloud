// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

describe('Greet a new device', () => {
  const WAIT_TIME = 500;

  beforeEach(() => {
    cy.visitApp();
    cy.login('Boby', 'P@ssw0rd.');
    cy.get('#profile-button').click();
    cy.get('.popover-viewport').contains('My devices').click();
  });

  afterEach(() => {
    cy.dropTestbed();
  });

  it('Open greet device modal', () => {
    cy.get('.greet-organization-modal').should('not.exist');
    cy.get('.devices-container').find('ion-button').contains('Add').click();
    cy.get('.greet-organization-modal').should('exist');
    cy.get('.greet-organization-modal').find('.modal-header__title').contains('Create a new device');
    cy.get('.greet-organization-modal').find('#next-button').should('have.attr', 'disabled');
    cy.wait(WAIT_TIME);
    cy.get('.greet-organization-modal').find('#next-button').should('not.have.attr', 'disabled');
    cy.get('.greet-organization-modal').find('.closeBtn').should('exist');
    cy.get('.greet-organization-modal').find('.closeBtn').should('be.visible');
  });

  it('Copy invitation link', () => {
    cy.get('.devices-container').find('ion-button').contains('Add').click();
    cy.get('.greet-organization-modal')
      .find('#copy-link-btn')
      .click()
      .then(() => {
        cy.window().then((win) => {
          win.navigator.clipboard.readText().then((text) => {
            // cspell:disable-next-line
            expect(text).to.eq('parsec3://example.parsec.cloud/Org?action=claim_device&token=9ae715f49bc0468eac211e1028f15529');
          });
        });
      });
    cy.checkToastMessageWithSidebar('info', 'Invitation link has been copied to clipboard.');
  });

  it('Go through the greet process', () => {
    function checkStepper(activeIndex: number): void {
      cy.get('@stepper').find('.ms-wizard-stepper-step').as('steps').should('have.length', 2);
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

    cy.get('.devices-container').find('ion-button').contains('Add').click();
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
    cy.get('@title').contains('Waiting for device information');
    cy.get('@footer').find('ion-spinner').should('be.visible');
    cy.get('@nextButton').should('not.be.visible');
    cy.wait(WAIT_TIME);
    cy.get('@title').contains('New device added');
    cy.get('.greet-organization-modal').find('.modal-header__text').contains('The device My Device has been added to your profile.');
    cy.get('@stepper').should('not.be.visible');
    cy.get('@nextButton').click();
    cy.get('.greet-organization-modal').should('not.exist');
  });

  it('Select wrong code', () => {
    cy.get('.devices-container').find('ion-button').contains('Add').click();
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
    cy.checkToastMessageWithSidebar('error', 'You did not select the correct code. Please restart the onboarding process.');
    cy.wait(WAIT_TIME);
    cy.get('@title').contains('Create a new device');
    cy.get('@nextButton').contains('Start');
    cy.get('@nextButton').should('not.have.attr', 'disabled');
  });

  it('Select none code', () => {
    cy.get('.devices-container').find('ion-button').contains('Add').click();
    cy.get('.greet-organization-modal').should('exist');
    cy.wait(WAIT_TIME);
    cy.get('.greet-organization-modal').find('#next-button').as('nextButton').click();
    cy.get('.greet-organization-modal').find('.modal-footer').as('footer');
    cy.get('.greet-organization-modal').find('.modal-header').find('.modal-header__title').as('title');
    cy.get('@title').contains('Share your code');
    cy.wait(WAIT_TIME);
    cy.get('@title').contains('Get guest code');
    cy.get('.greet-organization-modal').find('ion-grid').find('.button-clear').contains('None shown').click();
    cy.checkToastMessageWithSidebar(
      'error',
      'If you did not see the correct code, this could be a sign of a security issue during the onboarding. Please restart the process.',
    );
    cy.wait(WAIT_TIME);
    cy.get('@title').contains('Create a new device');
    cy.get('@nextButton').contains('Start');
    cy.get('@nextButton').should('not.have.attr', 'disabled');
  });
});

// https://docs.cypress.io/api/introduction/api.html

describe('Check organization list', () => {

  it('Visit the app root url', () => {
    cy.visit('/');
    cy.contains('List of your organizations');
  });

  it('Go to login page', () => {
    cy.visit('/');
    cy.storeDefaultDevice();
    cy.contains('List of your organizations');
    cy.contains('Wraeclast').click();
    cy.contains('Password');
  });

  it('Go to login page and back to organizations', () => {
    cy.visit('/');
    cy.storeDefaultDevice();
    cy.contains('Wraeclast').click();
    cy.contains('Return to organizations').click();
    cy.contains('List of your organizations');
  });

  it('Go to login page and enter password', () => {
    cy.visit('/');
    cy.storeDefaultDevice();
    cy.contains('Wraeclast').click();
    cy.get('#login-button-container > ion-button').should('have.class', 'button-disabled');
    cy.get('input').type('P@ssw0rd');
    cy.get('input').invoke('attr', 'type').should('eq', 'password');
    cy.get('.button-has-icon-only').click();
    cy.get('input').invoke('attr', 'type').should('eq', 'text');
    cy.get('input').should('have.value', 'P@ssw0rd');
    cy.get('#login-button-container > ion-button').should('not.have.class', 'button-disabled');
    cy.get('#login-button-container > ion-button').click();
  });

  it('Open create organization dialog', () => {
    cy.visit('/');
    cy.storeDefaultDevice();
    cy.contains('Create an organization').click();
    cy.contains('Use the main PARSEC server');
  });

  it('Open join organization dialog', () => {
    cy.visit('/');
    cy.storeDefaultDevice();
    cy.contains('Join an organization').click();
    cy.contains('Please enter the organization\'s URL');
  });

});

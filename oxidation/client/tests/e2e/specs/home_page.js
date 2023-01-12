// https://docs.cypress.io/api/introduction/api.html

describe('Check organization list', () => {
  it('Visits the app root url', () => {
    cy.visit('/');
    cy.contains('List of your organizations');
  });

  it('Go to login page', () => {
    cy.visit('/');
    cy.contains('List of your organizations');
    cy.contains('MegaShark').click();
    cy.contains('Password');
  });

  it('Go to login page and back to organizations', () => {
    cy.visit('/');
    cy.contains('MegaShark').click();
    cy.contains('Return to organizations').click();
    cy.contains('List of your organizations');
  });

  it('Open create organization dialog', () => {
    cy.visit('/');
    cy.contains('Create an organization').click();
    cy.contains('Use the main PARSEC server');
  });

  it('Open join organization dialog', () => {
    cy.visit('/');
    cy.contains('Join an organization').click();
    cy.contains('Please enter the organization\'s URL');
  });

});

// https://docs.cypress.io/api/introduction/api.html

describe('My First Test', () => {
  it('Visits the app root url', () => {
    cy.visit('/');
    cy.contains('Welcome. Please add an organization to start using Parsec.');
  });

  it('Visits the app root url', () => {
    cy.visit('/');
    cy.get('#workspaces-link').click();
    cy.url().should('include', '/documents/workspaces');
  });
});

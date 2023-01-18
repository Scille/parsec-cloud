// https://docs.cypress.io/api/introduction/api.html

import { libparsec } from '../../../src/plugins/libparsec';

describe('Check organization list', () => {

  it('Visit the app root url', () => {
    cy.visit('/', {
      /*
      onBeforeLoad(win) {
        cy.stub(win.libparsec, "listAvailableDevices").returns([{
          keyFilePath: '/path',
          organizationId: 'Planet Express',
          deviceId: 'Trashcan',
          humanHandle: 'John A. Zoidberg',
          deviceLabel: 'label',
          slug: 'brain_slug',
          ty: {
            type: 'Password'
          }
        }]);
      }
      */
    });
    cy.contains('List of your organizations');
    //cy.contains('Dr. John A. Zoidberg');
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

  it('Go to login page and enter password', () => {
    cy.visit('/');
    cy.contains('MegaShark').click();
    cy.get('#login-button-container > ion-button').should('have.class', 'button-disabled');
    cy.get('input').type('P@ssw0rd');
    cy.get('input').invoke('attr', 'type').should('eq', 'password');
    cy.get('#login-button-container > ion-button').should('not.have.class', 'button-disabled');
    cy.get('#login-button-container > ion-button').click();
  })

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

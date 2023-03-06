// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import PasswordInput from '@/components/PasswordInput.vue';

beforeEach(() => {
  const enterSpy = cy.spy().as('enterSpy');
  const changeSpy = cy.spy().as('changeSpy');

  cy.mount(PasswordInput, {
    props: {
      label: 'The Label'
    },
    listeners: {
      change: changeSpy,
      enter: enterSpy
    }
  }).as('passwordInput');
});

it('test password input', () => {
  cy.get('ion-label').should('contains.text', 'The Label');
});

it('should toggle password visibility button icon and password input type on password visibility button click', () => {
  // Input type should be `password` by default
  cy.get('ion-input input').as('input').invoke('attr', 'type').should('eq', 'password');
  // Type in and check that the value is updated
  cy.get('ion-input').type('P@ssw0rd').should('have.value', 'P@ssw0rd');
  // Click on button to reveal the password and check that the type changes
  cy.get('ion-button').click();
  cy.get('@input').invoke('attr', 'type').should('eq', 'text');
  // Click again and check that the type changes back to password
  cy.get('ion-button').click();
  cy.get('@input').invoke('attr', 'type').should('eq', 'password');
});

it('should emit a signal when input changes', () => {
  // cy.get('ion-input input').type('P@ssw0rd{enter}').then(() => {
  //   cy.wrap(Cypress.vueWrapper.emitted()).should('have.property', 'change');
  //   cy.wrap(Cypress.vueWrapper.emitted()).should('have.property', 'enter');
  // });
});

it('should not emit enter when input is empty', () => {
  // cy.get('ion-input input').type('{enter}').then(() => {
  //   cy.wrap(Cypress.vueWrapper.emitted()).should('not.have.property', 'enter');
  // });
});

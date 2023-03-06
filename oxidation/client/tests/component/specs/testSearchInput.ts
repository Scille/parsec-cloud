// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import SearchInput from '@/components/SearchInput.vue';

beforeEach(() => {
  const changeSpy = cy.spy().as('changeSpy');

  cy.mount(SearchInput, {
    props: {
      label: 'The Label'
    },
    listeners: {
      change: changeSpy
    }
  }).as('searchInput');
});

it('test search input', () => {
  cy.get('ion-label').should('contains.text', 'The Label');
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

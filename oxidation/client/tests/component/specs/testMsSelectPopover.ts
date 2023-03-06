// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import MsSelectPopover from '@/components/MsSelectPopover.vue';
import { MsSelectOption, MsSelectSortByLabels } from '@/components/MsSelectOption';

// MsSelectPopover will try to call popoverController to close itself
// No idea how to stub this with Cypress so we'll just ignore this
// exception.
Cypress.on('uncaught:exception', (err, _) => {
  if (err.message.includes('overlay does not exist')) {
    return false;
  }
});

const defaultOptions: MsSelectOption[] = [
  { label: 'Label A', key: '1' },
  { label: 'Label B', key: '2' },
  { label: 'Label C', key: '3' }
];

const defaultSortLabels: MsSelectSortByLabels = {
  asc: 'Asc',
  desc: 'Desc'
};

beforeEach(() => {
  const enterSpy = cy.spy().as('enterSpy');
  const changeSpy = cy.spy().as('changeSpy');

  cy.mount(MsSelectPopover, {
    props: {
      options: defaultOptions,
      sortByLabels: defaultSortLabels,
      sortByAsc: true
    }
  }).as('selectPopover');
});

it('renders select popover', () => {
  cy.get('#sort-order-button').should('have.text', 'Asc ');
  cy.get('ion-list > ion-item').each(($el, index, _) => {
    if (index > 0) {
      cy.wrap($el).invoke('attr', 'class').should('not.contain', 'selected');
    }
  });
});

it('changes sort order when clicked', () => {
  cy.get('#sort-order-button').as('sortButton').should('have.text', 'Asc ');
  cy.get('@sortButton').click().should('have.text', 'Desc ');
});

it('changes selected when clicked', () => {
  // 0 is sort button, 1 is Label A
  cy.get('ion-list > ion-item').eq(2).as('labelB').should('have.text', 'Label B');
  cy.get('@labelB').invoke('attr', 'class').should('not.contain', 'selected');
  cy.get('@labelB').click();
  cy.get('ion-list > ion-item').each(($el, index, _) => {
    if (index > 0) {
      if (index === 2) {
        cy.wrap($el).invoke('attr', 'class').should('contain', 'selected');
      } else {
        cy.wrap($el).invoke('attr', 'class').should('not.contain', 'selected');
      }
    }
  });
});

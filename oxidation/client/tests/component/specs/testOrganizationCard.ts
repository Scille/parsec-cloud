// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import OrganizationCard from '@/components/OrganizationCard.vue';
import { AvailableDevice } from '@/plugins/libparsec/definitions';

const DEVICE: AvailableDevice = {
  keyFilePath: '/path',
  organizationId: 'Black Mesa',
  deviceId: 'device_id',
  humanHandle: 'Gordon Freeman',
  deviceLabel: 'hev',
  slug: '1',
  ty: {tag: 'Password'}
};

it('display the organization', () => {
  cy.mount(OrganizationCard, {
    props: {
      device: DEVICE
    }
  }).as('orgCard');

  cy.get('@orgCard').get('ion-avatar').should('have.text', 'Bl');
  cy.get('@orgCard').get('.organization-info > p').eq(0).as('orgId').should('have.text', 'Black Mesa');
  cy.get('@orgCard').get('.organization-info > p').eq(1).as('humanHandle').should('have.text', 'Gordon Freeman');
});

it('handles empty human handles', () => {
  DEVICE.humanHandle = null;

  cy.mount(OrganizationCard, {
    props: {
      device: DEVICE
    }
  }).as('orgCard');

  cy.get('@orgCard').get('ion-avatar').should('have.text', 'Bl');
  cy.get('@orgCard').get('.organization-info > p').eq(0).as('orgId').should('have.text', 'Black Mesa');
  cy.get('@orgCard').get('.organization-info > p').eq(1).as('humanHandle').should('have.text', '');

});

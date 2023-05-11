// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

// cSpell:disable

/*
import WorkspaceCard from '@/components/WorkspaceCard.vue';
import { MockWorkspace } from '@/common/mocks';
import { InjectionKey } from 'vue';

import { DateTime } from 'luxon';

import { Formatters } from '../../../src/main';

function mockTimeSince(_: DateTime | undefined, __: string, ___: string): string {
  return 'One minute ago'
}

function mockFileSize(_: number): string {
  return '1MB'
}

const WORKSPACE: MockWorkspace = {
  id: 'id1',
  name: 'Waukeen\'s Promenade',
  sharedWith: ['Aerie', 'Cernd'],
  size: 60_817_408,
  role: 'Reader',
  availableOffline: true,
  lastUpdate: DateTime.fromISO('2023-05-08T12:00:00')
};

it('display the workspace card', () => {
  cy.mount(WorkspaceCard, {
    props: {
      workspace: WORKSPACE
    },
    provide: {
      formattersKey: {
        'timeSince': mockTimeSince,
        'fileSize': mockFileSize
      }
    }
  }).as('workspaceCard');

  cy.get('@workspaceCard').get('.workspace-label').should('have.text', WORKSPACE.name);
  cy.get('@workspaceCard').get('.workspace-info').find('ion-label').eq(0).should('have.text', '1MB');
  cy.get('@workspaceCard').get('.workspace-info').find('ion-label').eq(1).should('have.text', '2 people');
});
*/

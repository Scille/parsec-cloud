// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import WorkspaceCard from '@/components/WorkspaceCard.vue';
import { MockWorkspace, WorkspaceRole } from '@/common/mocks';
import { mount, VueWrapper } from '@vue/test-utils';
import { DateTime } from 'luxon';
import { getDefaultMockConfig, getDefaultProvideConfig } from '../support/mocks';
import { IonAvatar } from '@ionic/vue';

describe('Workspace Card', () => {

  let wrapper: VueWrapper;

  const WORKSPACE: MockWorkspace = {
    id: 'id1',
    name: 'My Workspace',
    sharedWith: ['AUser', 'BUser', 'CUser', 'DUser'],
    size: 60_817_408,
    role: WorkspaceRole.Reader,
    availableOffline: true,
    lastUpdate: DateTime.fromISO('2023-05-08T12:00:00')
  };

  beforeEach(() => {
    wrapper = mount(WorkspaceCard, {
      props: {
        workspace: WORKSPACE
      },
      global: {
        provide: getDefaultProvideConfig(),
        mocks: getDefaultMockConfig()
      }
    });
  });

  it('Display the workspace card', () => {
    expect(wrapper.get('.card-content__title').text()).to.equal('My Workspace');
    expect(wrapper.get('.label-file-size').text()).to.equal('1MB');
    expect(wrapper.get('.card-content-last-update').text()).to.equal('WorkspacesPage.Workspace.lastUpdateOne minute ago');
    const avatars = wrapper.get('.shared-group').findAllComponents(IonAvatar);
    expect(avatars.length).to.equal(3);
    expect(avatars.at(0).text()).to.equal('AU');
    expect(avatars.at(1).text()).to.equal('BU');
    expect(avatars.at(2).text()).to.equal('+ 2');
  });
});

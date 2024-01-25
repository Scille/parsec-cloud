// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import WorkspaceCard from '@/components/workspaces/WorkspaceCard.vue';
import { UserProfile, WorkspaceInfo, WorkspaceRole } from '@/parsec';
import { IonAvatar } from '@ionic/vue';
import { getDefaultProvideConfig } from '@tests/component/support/mocks';
import { VueWrapper, mount } from '@vue/test-utils';
import { DateTime } from 'luxon';

describe('Workspace Card', () => {
  let wrapper: VueWrapper;

  const WORKSPACE: WorkspaceInfo = {
    id: 'id1',
    name: 'My Workspace',
    sharing: [
      [
        {
          // cspell:disable-next-line
          id: 'auser',
          // cspell:disable-next-line
          humanHandle: { label: 'AUser', email: 'usera@gmail.com' },
          profile: UserProfile.Standard,
        },
        WorkspaceRole.Contributor,
      ],
      [
        {
          // cspell:disable-next-line
          id: 'buser',
          // cspell:disable-next-line
          humanHandle: { label: 'BUser', email: 'userb@gmail.com' },
          profile: UserProfile.Standard,
        },
        WorkspaceRole.Reader,
      ],
      [
        {
          // cspell:disable-next-line
          id: 'cuser',
          // cspell:disable-next-line
          humanHandle: { label: 'CUser', email: 'userc@gmail.com' },
          profile: UserProfile.Standard,
        },
        WorkspaceRole.Owner,
      ],
      [
        {
          // cspell:disable-next-line
          id: 'duser',
          // cspell:disable-next-line
          humanHandle: { label: 'DUser', email: 'userd@gmail.com' },
          profile: UserProfile.Standard,
        },
        WorkspaceRole.Manager,
      ],
    ],
    size: 60_817_408,
    selfCurrentRole: WorkspaceRole.Reader,
    availableOffline: true,
    lastUpdated: DateTime.fromISO('2023-05-08T12:00:00'),
    isStarted: false,
  };

  beforeEach(() => {
    wrapper = mount(WorkspaceCard, {
      props: {
        workspace: WORKSPACE,
        clientProfile: UserProfile.Admin,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });
  });

  it('Display the workspace card', () => {
    expect(wrapper.get('.card-content__title').text()).to.equal('My Workspace');
    expect(wrapper.get('.label-file-size').text()).to.equal('58.0 MB');
    expect(wrapper.get('.card-content-last-update').text()).to.equal('Last updated:May 8, 2023');
    const avatars = wrapper.get('.shared-group').findAllComponents(IonAvatar);
    expect(avatars.length).to.equal(3);
    expect(avatars.at(0).text()).to.equal('AU');
    expect(avatars.at(1).text()).to.equal('BU');
    expect(avatars.at(2).text()).to.equal('+ 2');
  });

  it('Emit click', async () => {
    wrapper.trigger('click');
    expect(wrapper.emitted('click')?.length).to.equal(1);
    const workspace = wrapper.emitted('click')?.at(0)?.at(1);
    expect(workspace).to.deep.equal(WORKSPACE);
  });

  it('Emit menuClick', async () => {
    wrapper.get('.card-option').trigger('click');
    expect(wrapper.emitted('menuClick')?.length).to.equal(1);
    const workspace = wrapper.emitted('menuClick')?.at(0)?.at(1);
    expect(workspace).to.deep.equal(WORKSPACE);
  });

  it('Emit shareClick', async () => {
    wrapper.get('.shared-group').trigger('click');
    expect(wrapper.emitted('shareClick')?.length).to.equal(1);
    const workspace = wrapper.emitted('shareClick')?.at(0)?.at(1);
    expect(workspace).to.deep.equal(WORKSPACE);
  });
});

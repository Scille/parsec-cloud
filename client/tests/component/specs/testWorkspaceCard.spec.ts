// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { mount, VueWrapper } from '@vue/test-utils';
import { DateTime } from 'luxon';
import { mockI18n, getDefaultProvideConfig } from 'tests/component/support/mocks';
import { IonAvatar } from '@ionic/vue';
import WorkspaceCard from '@/components/workspaces/WorkspaceCard.vue';
import { WorkspaceRole, WorkspaceInfo } from '@/parsec';

mockI18n();

describe('Workspace Card', () => {

  let wrapper: VueWrapper;

  const WORKSPACE: WorkspaceInfo = {
    id: 'id1',
    name: 'My Workspace',
    sharing: [
      // cspell:disable-next-line
      [{id: 'auser', humanHandle: {label: 'AUser', email: 'usera@gmail.com'}}, WorkspaceRole.Contributor],
      // cspell:disable-next-line
      [{id: 'buser', humanHandle: {label: 'BUser', email: 'userb@gmail.com'}}, WorkspaceRole.Reader],
      // cspell:disable-next-line
      [{id: 'cuser', humanHandle: {label: 'CUser', email: 'userc@gmail.com'}}, WorkspaceRole.Owner],
      // cspell:disable-next-line
      [{id: 'duser', humanHandle: {label: 'DUser', email: 'userd@gmail.com'}}, WorkspaceRole.Manager],
    ],
    size: 60_817_408,
    selfRole: WorkspaceRole.Reader,
    availableOffline: true,
    lastUpdated: DateTime.fromISO('2023-05-08T12:00:00'),
  };

  beforeEach(() => {
    wrapper = mount(WorkspaceCard, {
      props: {
        workspace: WORKSPACE,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
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

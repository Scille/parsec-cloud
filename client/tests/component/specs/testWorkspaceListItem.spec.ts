// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MockWorkspace, WorkspaceRole } from '@/common/mocks';
import { mount, VueWrapper } from '@vue/test-utils';
import { DateTime } from 'luxon';
import { mockI18n, getDefaultProvideConfig } from 'tests/component/support/mocks';
import { IonAvatar } from '@ionic/vue';
import WorkspaceListItem from '@/components/workspaces/WorkspaceListItem.vue';

mockI18n();

describe('Workspace List Item', () => {

  let wrapper: VueWrapper;

  const WORKSPACE: MockWorkspace = {
    id: 'id1',
    name: 'My Workspace',
    sharingInfo: new Map<string, WorkspaceRole | null>([
      ['AUser', WorkspaceRole.Contributor],
      ['BUser', WorkspaceRole.Reader],
      ['CUser', WorkspaceRole.Owner],
      ['DUSer', WorkspaceRole.Manager],
    ]),
    size: 60_817_408,
    role: WorkspaceRole.Reader,
    availableOffline: true,
    lastUpdate: DateTime.fromISO('2023-05-08T12:00:00'),
  };

  beforeEach(() => {
    wrapper = mount(WorkspaceListItem, {
      props: {
        workspace: WORKSPACE,
      },
      global: {
        provide: getDefaultProvideConfig(),
        stubs: ['TagWorkspaceRole'],
      },
      stubs: ['TagWorkspaceRole'],
    });
  });

  it('Display the workspace card', () => {
    expect(wrapper.get('.workspace-name__label').text()).to.equal('My Workspace');
    expect(wrapper.get('.label-size').text()).to.equal('1MB');
    expect(wrapper.get('.label-last-update').text()).to.equal('One minute ago');
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
    wrapper.get('.options-button').trigger('click');
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

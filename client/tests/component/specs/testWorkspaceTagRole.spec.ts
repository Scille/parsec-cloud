// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getDefaultProvideConfig } from '@tests/component/support/mocks';

import WorkspaceTagRole from '@/components/workspaces/WorkspaceTagRole.vue';
import { WorkspaceRole } from '@/parsec';
import { mount } from '@vue/test-utils';

describe('User Avatar', () => {
  it('Display avatar for user', () => {
    const wrapper = mount(WorkspaceTagRole, {
      props: {
        role: WorkspaceRole.Manager,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.get('.tag').text()).to.equal('Manager');
  });
});

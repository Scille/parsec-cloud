// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import UserAvatarName from '@/components/users/UserAvatarName.vue';
import WorkspaceUserRole from '@/components/workspaces/WorkspaceUserRole.vue';
import { UserProfile, WorkspaceRole } from '@/parsec';
import { getDefaultProvideConfig } from '@tests/component/support/mocks';
import { mount } from '@vue/test-utils';
import { MsDropdown } from 'megashark-lib';
import { describe, expect, it } from 'vitest';

describe('Workspace user role selector', () => {
  it('Display workspace user role selector', () => {
    const wrapper = mount(WorkspaceUserRole, {
      props: {
        user: {
          id: 'id',
          humanHandle: { label: 'A User', email: 'user@host' },
          profile: UserProfile.Admin,
        },
        role: WorkspaceRole.Contributor,
        disabled: false,
        clientProfile: UserProfile.Admin,
        clientRole: WorkspaceRole.Owner,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.findComponent(UserAvatarName).text()).to.equal('A A User');
    expect(wrapper.findComponent(MsDropdown).text()).to.equal('Contributor');
  });
});

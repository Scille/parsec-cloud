// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { it } from 'vitest';
import { mockI18n, mockRouter, getDefaultProvideConfig } from '@tests/component/support/mocks';

// Mock before importing anything else
mockI18n();
mockRouter();

import WorkspaceUserRole from '@/components/workspaces/WorkspaceUserRole.vue';
import MsDropdown from '@/components/core/ms-dropdown/MsDropdown.vue';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { WorkspaceRole, UserProfile } from '@/parsec';
import { mount } from '@vue/test-utils';

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
    expect(wrapper.findComponent(MsDropdown).text()).to.equal('workspaceRoles.contributor');
  });
});

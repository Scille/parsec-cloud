// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { mockI18n, getDefaultProvideConfig } from '@tests/component/support/mocks';

// Before other imports
mockI18n();

import TagProfile from '@/components/users/TagProfile.vue';
import { UserProfile } from '@/parsec';
import { mount } from '@vue/test-utils';

describe('User Avatar', () => {

  it('Display avatar for user', () => {
    const wrapper = mount(TagProfile, {
      props: {
        profile: UserProfile.Outsider,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.get('.tag').text()).to.equal('UsersPage.profile.outsider');
  });
});

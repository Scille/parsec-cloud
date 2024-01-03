// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getDefaultProvideConfig } from '@tests/component/support/mocks';

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

    expect(wrapper.get('.tag').text()).to.equal('Outsider');
  });
});

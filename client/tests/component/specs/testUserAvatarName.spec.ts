// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getDefaultProvideConfig } from '@tests/component/support/mocks';

import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { mount } from '@vue/test-utils';
import { describe, expect, it } from 'vitest';

describe('User Avatar', () => {
  it('Display avatar for user', () => {
    const wrapper = mount(UserAvatarName, {
      props: {
        userAvatar: 'Avatar',
        userName: 'User Name',
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.get('.avatar').text()).to.equal('Av');
    expect(wrapper.get('.person-name').text()).to.equal('User Name');
  });

  it('Optional user name', () => {
    const wrapper = mount(UserAvatarName, {
      props: {
        userAvatar: 'A',
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.get('.avatar').text()).to.equal('A');
    expect(wrapper.find('.person-name').exists()).to.be.false;
  });
});

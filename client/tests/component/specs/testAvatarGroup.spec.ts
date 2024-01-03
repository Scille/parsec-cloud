// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import AvatarGroup from '@/components/workspaces/AvatarGroup.vue';
import { getDefaultProvideConfig } from '@tests/component/support/mocks';
import { mount } from '@vue/test-utils';

describe('User Avatar', () => {
  it('Display avatar for user', () => {
    const PEOPLE = ['First', 'Second', 'Third', 'Fourth', 'Fifth'];

    const wrapper = mount(AvatarGroup, {
      props: {
        people: PEOPLE,
        maxDisplay: 3,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    const avatars = wrapper.findAll('.person-avatar');
    expect(avatars.length).to.equal(3);
    expect(avatars.at(0)?.text()).to.equal('Fi');
    expect(avatars.at(1)?.text()).to.equal('Se');
    expect(avatars.at(2)?.text()).to.equal('Th');
    expect(wrapper.find('.extra-avatar').exists()).to.be.true;
    expect(wrapper.get('.extra-avatar').text()).to.equal('+ 2');
  });

  it('Display all', () => {
    const PEOPLE = ['First', 'Second'];

    const wrapper = mount(AvatarGroup, {
      props: {
        people: PEOPLE,
        maxDisplay: 3,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.findAll('.person-avatar').length).to.equal(2);
    expect(wrapper.find('.extra-avatar').exists()).to.be.false;
  });
});

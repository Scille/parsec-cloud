// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import UserCard from '@/components/users/UserCard.vue';
import { MockUser, Profile } from '@/common/mocks';
import { mount } from '@vue/test-utils';
import { DateTime } from 'luxon';
import { mockI18n, getDefaultProvideConfig } from 'tests/component/support/mocks';

mockI18n();

describe('User Card', () => {
  it('Display item for user', () => {
    const USER: MockUser = {
      id: '0',
      name: 'John Smith',
      email: 'john.smith@gmail.com',
      avatar: 'unused',
      joined: DateTime.now(),
      profile: Profile.Standard,
      revoked: false,
    };

    const wrapper = mount(UserCard, {
      props: {
        user: USER,
        showCheckbox: true,
      },
      global: {
        provide: getDefaultProvideConfig(),
        stubs: {
          TagProfile: {
            template: '{{ profile }}',
            props: {
              profile: Profile,
            },
          },
        },
      },
    });

    expect((wrapper.vm as any).isSelected).to.be.false;
    // "JoJohn Smith" because the user is displayed with an avatar before their name,
    // currently using the first two letters for the avatar, (Jo) John Smith
    expect(wrapper.get('.card-content-avatar').text()).to.equal('Jo');
    expect(wrapper.get('.user-name').text()).to.equal('John Smith');
    expect(wrapper.get('.user-profile').text()).to.equal('standard');
    wrapper.trigger('click');
    expect(wrapper.emitted('click')?.length).to.equal(1);
    expect(wrapper.emitted('click')?.at(0)?.at(1)).to.deep.equal(USER);
    wrapper.get('.card-option').trigger('click');
    expect(wrapper.emitted('menuClick')?.length).to.equal(1);
    expect(wrapper.emitted('menuClick')?.at(0)?.at(1)).to.deep.equal(USER);
  });
});

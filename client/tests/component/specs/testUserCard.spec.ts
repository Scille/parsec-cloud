// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { UserModel } from '@/components/users';
import UserCard from '@/components/users/UserCard.vue';
import { UserProfile } from '@/parsec';
import { getDefaultProvideConfig } from '@tests/component/support/mocks';
import { mount } from '@vue/test-utils';
import { DateTime } from 'luxon';

describe('User Card', () => {
  it('Display item for user', () => {
    const USER: UserModel = {
      id: '0',
      humanHandle: { label: 'John Smith', email: 'john.smith@gmail.com' },
      currentProfile: UserProfile.Standard,
      createdOn: DateTime.now(),
      createdBy: 'device',
      revokedOn: null,
      revokedBy: null,
      isRevoked: () => false,
      isSelected: false,
    };

    const wrapper = mount(UserCard, {
      props: {
        user: USER,
        showCheckbox: true,
        showOptions: true,
      },
      global: {
        provide: getDefaultProvideConfig(),
        stubs: {
          TagProfile: {
            template: '{{ profile }}',
            props: {
              profile: UserProfile,
            },
          },
        },
      },
    });

    // "JoJohn Smith" because the user is displayed with an avatar before their name,
    // currently using the first two letters for the avatar, (Jo) John Smith
    expect(wrapper.get('.user-card-avatar').text()).to.equal('Jo');
    expect(wrapper.get('.user-card-info__name').text()).to.equal('John Smith');
    expect(wrapper.get('.user-card-profile').text()).to.equal('UserProfileStandard');
    wrapper.trigger('click');
    expect(wrapper.emitted('click')?.length).to.equal(1);
    expect(wrapper.emitted('click')?.at(0)?.at(1)).to.deep.equal(USER);
    wrapper.get('.user-card-option').trigger('click');
    expect(wrapper.emitted('menuClick')?.length).to.equal(1);
    expect(wrapper.emitted('menuClick')?.at(0)?.at(1)).to.deep.equal(USER);
  });
});

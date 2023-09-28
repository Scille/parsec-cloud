// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import UserListItem from '@/components/users/UserListItem.vue';
import { mount } from '@vue/test-utils';
import { DateTime } from 'luxon';
import { mockI18n, getDefaultProvideConfig } from 'tests/component/support/mocks';
import { UserProfile, UserInfo } from '@/parsec';

mockI18n();

describe('User List Item', () => {
  it('Display item for user', () => {
    const USER: UserInfo = {
      id: '0',
      humanHandle: {label: 'John Smith', email: 'john.smith@gmail.com'},
      currentProfile: UserProfile.Standard,
      createdOn: DateTime.now(),
      createdBy: 'device',
      revokedOn: null,
      revokedBy: null,
      isRevoked: () => false,
    };

    const wrapper = mount(UserListItem, {
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
              profile: UserProfile,
            },
          },
        },
      },
    });

    expect((wrapper.vm as any).isSelected).to.be.false;
    // "JoJohn Smith" because the user is displayed with an avatar before their name,
    // currently using the first two letters for the avatar, (Jo) John Smith
    expect(wrapper.get('.user-name__label').text()).to.equal('JoJohn Smith');
    expect(wrapper.get('.user-email__label').text()).to.equal('john.smith@gmail.com');
    expect(wrapper.get('.user-profile').text()).to.equal('UserProfileStandard');
    expect(wrapper.get('.user-join-label').text()).to.equal('One minute ago');
    wrapper.trigger('click');
    expect(wrapper.emitted('click')?.length).to.equal(1);
    expect(wrapper.emitted('click')?.at(0)?.at(1)).to.deep.equal(USER);
    wrapper.get('.options-button').trigger('click');
    expect(wrapper.emitted('menuClick')?.length).to.equal(1);
    expect(wrapper.emitted('menuClick')?.at(0)?.at(1)).to.deep.equal(USER);
  });
});

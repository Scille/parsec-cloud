// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { mockI18n, getDefaultProvideConfig } from '@tests/component/support/mocks';

mockI18n();

import InvitationListItem from '@/components/users/InvitationListItem.vue';
import { mount } from '@vue/test-utils';
import { DateTime } from 'luxon';
import { InvitationStatus, UserInvitation } from '@/parsec';
import { InviteListItemTag } from '@/plugins/libparsec';

describe('User Invitation List Item', () => {
  it('Display invitation', () => {
    const INVITATION: UserInvitation = {
      tag: InviteListItemTag.User,
      addr: 'parsec://parsec.example.com/MyOrg?action=claim_user&token=1234',
      token: '1234',
      createdOn: DateTime.now(),
      claimerEmail: 'dung.eater@lands-between',
      status: InvitationStatus.Ready,
    };

    const wrapper = mount(InvitationListItem, {
      props: {
        invitation: INVITATION,
      },
      global: {
        provide: getDefaultProvideConfig(),
      },
    });

    expect(wrapper.get('.invitation-status').text()).to.equal('UsersPage.invitation.status.ready');
    expect(wrapper.get('.invitation-email').text()).to.equal('dung.eater@lands-between');
    const buttons = wrapper.findAll('ion-button');
    expect(buttons.at(0)?.text()).to.equal('UsersPage.invitation.copyLink');
    expect(buttons.at(1)?.text()).to.equal('UsersPage.invitation.rejectUser');
    expect(buttons.at(2)?.text()).to.equal('UsersPage.invitation.greetUser');

    buttons.at(1)?.trigger('click');
    expect(wrapper.emitted('rejectUser')?.length).to.equal(1);
    expect(wrapper.emitted('rejectUser')?.at(0)?.at(0)).to.deep.equal(INVITATION);
    buttons.at(2)?.trigger('click');
    expect(wrapper.emitted('greetUser')?.length).to.equal(1);
    expect(wrapper.emitted('greetUser')?.at(0)?.at(0)).to.deep.equal(INVITATION);
  });
});

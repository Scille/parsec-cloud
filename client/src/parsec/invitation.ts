// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { libparsec, InviteListItem } from '@/plugins/libparsec';
import { getParsecHandle } from '@/parsec/routing';
import { DateTime } from 'luxon';
import { needsMocks } from '@/parsec/environment';
import {
  Result,
  NewInvitationInfo,
  NewUserInvitationError,
  InvitationEmailSentStatus,
  InvitationToken,
  NewDeviceInvitationError,
  UserInvitation,
  ListInvitationsError,
  InvitationStatus,
  DeleteInvitationError,
} from '@/parsec/types';

export async function inviteUser(email: string): Promise<Result<NewInvitationInfo, NewUserInvitationError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.clientNewUserInvitation(handle, email, true);
  } else {
    return {ok: true, value: {
      token: '12346565645645654645645645645645',
      emailSentStatus: InvitationEmailSentStatus.Success,
      addr: 'parsec://parsec.example.com/Org?action=claimer_user&token=12346565645645654645645645645645',
    }};
  }
}

export async function inviteDevice(sendEmail: boolean):
  Promise<Result<[InvitationToken, InvitationEmailSentStatus], NewDeviceInvitationError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    const ret = await libparsec.clientNewDeviceInvitation(handle, sendEmail);
    if (ret.ok) {
      return {ok: true, value: [ret.value.token, ret.value.emailSentStatus]};
    } else {
      return ret;
    }
  }
  return {ok: true, value: ['1234', InvitationEmailSentStatus.Success]};
}

export async function listUserInvitations(): Promise<Result<Array<UserInvitation>, ListInvitationsError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    const result = await libparsec.clientListInvitations(handle);

    if (!result.ok) {
      return result;
    }
    // No need to add device invitations
    result.value = result.value.filter((item: InviteListItem) => item.tag === 'User');
    // Convert InviteListItemUser to UserInvitation
    result.value = result.value.map((item) => {
      item.createdOn = DateTime.fromSeconds(item.createdOn as any as number);
      return item;
    });
    return result as any;
  } else {
    return new Promise<Result<Array<UserInvitation>, ListInvitationsError>>((resolve, _reject) => {
      const ret: Array<UserInvitation> = [{
        tag: 'User',
        addr: 'parsec://parsec.example.com/MyOrg?action=claim_device&token=12346565645645654645645645645645',
        token: '12346565645645654645645645645645',
        createdOn: DateTime.now(),
        claimerEmail: 'shadowheart@swordcoast.faerun',
        status: InvitationStatus.Ready,
      }, {
        tag: 'User',
        addr: 'parsec://parsec.example.com/MyOrg?action=claim_user&token=32346565645645654645645645645645',
        token: '32346565645645654645645645645645',
        createdOn: DateTime.now(),
        claimerEmail: 'gale@waterdeep.faerun',
        status: InvitationStatus.Ready,
      }];
      resolve({ok: true, value: ret});
    });
  }
}

export async function cancelInvitation(token: InvitationToken): Promise<Result<null, DeleteInvitationError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.clientDeleteInvitation(handle, token);
  } else {
    return {ok: true, value: null};
  }
}

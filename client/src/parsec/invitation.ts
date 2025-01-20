// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { getParsecHandle } from '@/parsec/routing';
import {
  ClientCancelInvitationError,
  ClientNewDeviceInvitationError,
  ClientNewUserInvitationError,
  ClientNewUserInvitationErrorTag,
  InvitationEmailSentStatus,
  InvitationStatus,
  InvitationToken,
  ListInvitationsError,
  NewInvitationInfo,
  Result,
  UserInvitation,
} from '@/parsec/types';
import { listUsers } from '@/parsec/user';
import { InviteListInvitationCreatedByTag, InviteListItem, InviteListItemTag, libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

export async function inviteUser(email: string): Promise<Result<NewInvitationInfo, ClientNewUserInvitationError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.clientNewUserInvitation(handle, email, true);
  } else {
    const usersResult = await listUsers(true);
    if (usersResult.ok) {
      if (usersResult.value.map((u) => u.humanHandle.email).includes(email)) {
        return {
          ok: false,
          error: {
            tag: ClientNewUserInvitationErrorTag.AlreadyMember,
            error: `${email} is already a member of this organization`,
          },
        };
      }
    }
    return {
      ok: true,
      value: {
        token: '12346565645645654645645645645645',
        emailSentStatus: InvitationEmailSentStatus.Success,
        // cspell:disable-next-line
        addr: 'parsec3://parsec.example.com/Org?a=claimer_user&p=xBjXbfjrnrnrjnrjnrnjrjnrjnrjnrjnrjk',
      },
    };
  }
}

export async function inviteDevice(
  sendEmail: boolean,
): Promise<Result<[InvitationToken, InvitationEmailSentStatus], ClientNewDeviceInvitationError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    const ret = await libparsec.clientNewDeviceInvitation(handle, sendEmail);
    if (ret.ok) {
      return { ok: true, value: [ret.value.token, ret.value.emailSentStatus] };
    } else {
      return ret;
    }
  }
  return { ok: true, value: ['1234', InvitationEmailSentStatus.Success] };
}

export async function listUserInvitations(options?: {
  includeCancelled?: boolean;
  includeFinished?: boolean;
}): Promise<Result<Array<UserInvitation>, ListInvitationsError>> {
  function shouldIncludeInvitation(invite: InviteListItem): boolean {
    if (invite.tag !== InviteListItemTag.User) {
      return false;
    }
    if (invite.status === InvitationStatus.Cancelled && (!options || !options.includeCancelled)) {
      return false;
    }
    if (invite.status === InvitationStatus.Finished && (!options || !options.includeFinished)) {
      return false;
    }
    return true;
  }

  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    const result = await libparsec.clientListInvitations(handle);

    if (!result.ok) {
      return result;
    }

    // No need to add device invitations
    result.value = result.value.filter((item: InviteListItem) => shouldIncludeInvitation(item));
    // Convert InviteListItemUser to UserInvitation
    result.value = result.value.map((item) => {
      item.createdOn = DateTime.fromSeconds(item.createdOn as any as number);
      return item;
    });
    return result as any;
  } else {
    return {
      ok: true,
      value: [
        {
          tag: InviteListItemTag.User,
          // cspell:disable-next-line
          addr: 'parsec3://parsec.example.com/MyOrg?a=claim_device&token=xBjXbfjrnrnrjnrjnrnjrjnrjnrjnrjnrjk',
          token: '12346565645645654645645645645645',
          createdOn: DateTime.now(),
          claimerEmail: 'shadowheart@swordcoast.faerun',
          createdBy: {
            tag: InviteListInvitationCreatedByTag.User,
            humanHandle: {
              email: 'gale@waterdeep.faerun',
              // cspell:disable-next-line
              label: 'Gale Dekarios',
            },
            userId: '1234',
          },
          status: InvitationStatus.Ready,
        },
        {
          tag: InviteListItemTag.User,
          // cspell:disable-next-line
          addr: 'parsec3://parsec.example.com/MyOrg?a=claim_user&token=xBjfbfjrnrnrjnrjnrnjrjnrjnrjnrjnrjk',
          token: '32346565645645654645645645645645',
          createdOn: DateTime.now(),
          createdBy: {
            tag: InviteListInvitationCreatedByTag.User,
            humanHandle: {
              email: 'gale@waterdeep.faerun',
              // cspell:disable-next-line
              label: 'Gale Dekarios',
            },
            userId: '1234',
          },
          claimerEmail: 'gale@waterdeep.faerun',
          status: InvitationStatus.Ready,
        },
      ],
    };
  }
}

export async function cancelInvitation(token: InvitationToken): Promise<Result<null, ClientCancelInvitationError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.clientCancelInvitation(handle, token);
  } else {
    return { ok: true, value: null };
  }
}

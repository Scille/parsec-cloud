// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getParsecHandle } from '@/parsec/routing';
import {
  ClientCancelInvitationError,
  ClientNewUserInvitationError,
  InvitationStatus,
  InvitationToken,
  ListInvitationsError,
  NewInvitationInfo,
  Result,
  UserInvitation,
} from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { InviteListItem, InviteListItemTag, libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

export async function inviteUser(email: string): Promise<Result<NewInvitationInfo, ClientNewUserInvitationError>> {
  const handle = getParsecHandle();

  if (handle !== null) {
    return await libparsec.clientNewUserInvitation(handle, email, true);
  }
  return generateNoHandleError<ClientNewUserInvitationError>();
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

  if (handle !== null) {
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
  }
  return generateNoHandleError<ListInvitationsError>();
}

export async function cancelInvitation(token: InvitationToken): Promise<Result<null, ClientCancelInvitationError>> {
  const handle = getParsecHandle();

  if (handle !== null) {
    return await libparsec.clientCancelInvitation(handle, token);
  }
  return generateNoHandleError<ClientCancelInvitationError>();
}

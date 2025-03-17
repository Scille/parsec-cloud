// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ClientAcceptTosError, ClientGetTosError, Result, Tos } from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { libparsec } from '@/plugins/libparsec';
import { getConnectionHandle } from '@/router';
import { DateTime } from 'luxon';

export async function acceptTOS(updatedOn: DateTime): Promise<Result<null, ClientAcceptTosError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    return await libparsec.clientAcceptTos(handle, updatedOn.toSeconds() as any as DateTime);
  }
  return generateNoHandleError<ClientAcceptTosError>();
}

export async function getTOS(): Promise<Result<Tos, ClientGetTosError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    const result = await libparsec.clientGetTos(handle);
    if (result.ok) {
      result.value.updatedOn = DateTime.fromSeconds(result.value.updatedOn as any as number);
    }
    return result;
  }
  return generateNoHandleError<ClientGetTosError>();
}

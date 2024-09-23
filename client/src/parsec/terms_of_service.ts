// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { getParsecHandle } from '@/parsec/routing';
import { ClientAcceptTosError, ClientGetTosError, Result, Tos } from '@/parsec/types';
import { libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

export async function acceptTOS(updatedOn: DateTime): Promise<Result<null, ClientAcceptTosError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.clientAcceptTos(handle, updatedOn.toSeconds() as any as DateTime);
  } else {
    return { ok: true, value: null };
  }
}

export async function getTOS(): Promise<Result<Tos, ClientGetTosError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    const result = await libparsec.clientGetTos(handle);
    if (result.ok) {
      result.value.updatedOn = DateTime.fromSeconds(result.value.updatedOn as any as number);
    }
    return result;
  } else {
    return {
      ok: true,
      value: {
        perLocaleUrls: new Map<string, string>([
          ['en-US', 'https://www.youtube.com/watch?v=dQw4w9WgXcQ'],
          ['fr-FR', 'https://www.youtube.com/watch?v=hEmODTcKJmE'],
        ]),
        updatedOn: DateTime.now(),
      },
    };
  }
}

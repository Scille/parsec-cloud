// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ClientGetOutboundSyncBacklogError, Result, UploadProgress } from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { libparsec } from '@/plugins/libparsec';
import { getConnectionHandle } from '@/router';

export async function getGlobalUploadProgress(): Promise<Result<UploadProgress, ClientGetOutboundSyncBacklogError>> {
  const handle = getConnectionHandle();

  if (!handle) {
    return generateNoHandleError<ClientGetOutboundSyncBacklogError>();
  }
  const result = await libparsec.clientGetOutboundSyncBacklog(handle);
  if (result.ok) {
    return {
      ok: true,
      value: {
        totalBytes: Number(result.value.totalPendingBytesForStartedWorkspaces),
        totalFiles: Number(result.value.totalPendingEntriesForStartedWorkspaces),
      },
    };
  }
  return result;
}

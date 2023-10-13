// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ClientConfig } from '@/parsec/types';
import { WorkspaceStorageCacheSizeTag } from '@/plugins/libparsec';

export const MOCK_WAITING_TIME = 500;
export const DEFAULT_HANDLE = 42;

export async function wait(delay: number): Promise<void> {
  return new Promise((res) => setTimeout(res, delay));
}

export function getClientConfig(): ClientConfig {
  return {
    configDir: window.getConfigDir(),
    dataBaseDir: window.getDataBaseDir(),
    mountpointBaseDir: window.getMountpointBaseDir(),
    workspaceStorageCacheSize: { tag: WorkspaceStorageCacheSizeTag.Default },
  };
}

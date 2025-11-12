// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getClientConfig } from '@/parsec/internals';
import { GetServerConfigError, Result, ServerConfig } from '@/parsec/types';
import { libparsec } from '@/plugins/libparsec';

export async function getServerConfig(serverAddr: string): Promise<Result<ServerConfig, GetServerConfigError>> {
  console.log(serverAddr, getClientConfig().configDir);
  return await libparsec.getServerConfig(getClientConfig().configDir, serverAddr);
}

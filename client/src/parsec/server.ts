// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getClientConfig } from '@/parsec/internals';
import { getCurrentAvailableDevice } from '@/parsec/login';
import { GetServerConfigError, Result, ServerConfig } from '@/parsec/types';
import { GetServerConfigErrorTag, libparsec, ParsecAddr, ParsedParsecAddr } from '@/plugins/libparsec';

export async function getServerConfig(serverAddr: string): Promise<Result<ServerConfig, GetServerConfigError>> {
  return await libparsec.getServerConfig(getClientConfig().configDir, serverAddr);
}

export async function getCurrentServerConfig(): Promise<Result<ServerConfig, GetServerConfigError>> {
  const result = await getCurrentServerAddr();
  if (result.ok) {
    return await getServerConfig(result.value);
  }
  return result;
}

export async function getCurrentServerAddr(): Promise<Result<ParsecAddr, GetServerConfigError>> {
  const result = await getCurrentAvailableDevice();
  if (!result.ok) {
    return { ok: false, error: { tag: GetServerConfigErrorTag.Internal, error: result.error.tag } };
  }
  return { ok: true, value: result.value.serverAddr };
}

export async function buildParsecAddr(addr: ParsedParsecAddr): Promise<string> {
  return await libparsec.buildParsecAddr(addr.hostname, addr.port, addr.useSsl);
}

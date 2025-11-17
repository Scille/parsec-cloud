// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ParsedParsecAddr } from '@/plugins/libparsec';
import { Env } from '@/services/environment';

export const TRIAL_EXPIRATION_DAYS = 15;

export enum ServerType {
  Saas = 'saas',
  Trial = 'trial',
  Custom = 'custom',
}

function matchesHost(hostList: Array<string>, host: string): boolean {
  return hostList.find((h) => h.trim().toLocaleLowerCase() === host.trim().toLocaleLowerCase()) !== undefined;
}

export function getServerTypeFromParsedParsecAddr(addr: ParsedParsecAddr): ServerType {
  // TODO
  if (matchesHost(Env.getSaasServers(), addr.hostname)) {
    return ServerType.Saas;
  } else if (matchesHost(Env.getTrialServers(), addr.hostname)) {
    return ServerType.Trial;
  }
  return ServerType.Custom;
}

export function getTrialServerAddress(): string {
  return `parsec3://${Env.getTrialServers()[0]}`;
}

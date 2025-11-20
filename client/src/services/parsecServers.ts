// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ParsedParsecAddr } from '@/plugins/libparsec';
import { Env } from '@/services/environment';

export const TRIAL_EXPIRATION_DAYS = 15;

export enum ServerType {
  Saas = 'saas',
  Trial = 'trial',
  Custom = 'custom',
}

function matchesHost(hostList: Array<string>, addr: ParsedParsecAddr): boolean {
  for (const host of hostList) {
    const aTrimmed = host.trim().toLocaleLowerCase();
    const bTrimmed = addr.hostname.trim().toLocaleLowerCase();

    if (aTrimmed === bTrimmed || `${bTrimmed}:${addr.port}` === aTrimmed) {
      return true;
    }
  }
  return false;
}

export function getServerTypeFromParsedParsecAddr(addr: ParsedParsecAddr): ServerType {
  if (matchesHost(Env.getSaasServers(), addr)) {
    return ServerType.Saas;
  } else if (matchesHost(Env.getTrialServers(), addr)) {
    return ServerType.Trial;
  }
  return ServerType.Custom;
}

export function getTrialServerAddress(): string {
  return `parsec3://${Env.getTrialServers()[0]}`;
}

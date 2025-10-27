// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Env } from '@/services/environment';

export const TRIAL_EXPIRATION_DAYS = 15;

export enum ServerType {
  Saas = 'saas',
  Trial = 'trial',
  Custom = 'custom',
}

const DEFAULT_PORT = 443;
const NO_SSL_PORT = 80;

function matchesHost(hostList: Array<string>, host: string): boolean {
  return hostList.find((h) => h.trim().toLocaleLowerCase() === host.trim().toLocaleLowerCase()) !== undefined;
}

export function getServerTypeFromAddress(addr: string): ServerType {
  let url;
  try {
    url = new URL(addr);
  } catch (_err: any) {
    return ServerType.Custom;
  }

  const host = !url.port || url.port === DEFAULT_PORT.toString() ? url.hostname : url.host;

  if (matchesHost(Env.getSaasServers(), host)) {
    return ServerType.Saas;
  } else if (matchesHost(Env.getTrialServers(), host)) {
    return ServerType.Trial;
  }
  return ServerType.Custom;
}

export function getServerTypeFromHost(host: string, port?: number, useSsl?: boolean): ServerType {
  const defaultPort = useSsl === false ? NO_SSL_PORT : DEFAULT_PORT;
  const addr = port === undefined || port === defaultPort ? host : `${host}:${port}`;

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

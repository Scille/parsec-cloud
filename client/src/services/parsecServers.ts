// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

const SAAS_SERVER_HOST = 'saas-v3.parsec.cloud';
const TRIAL_SERVER_HOST = 'trial.parsec.cloud';
const TEST_SERVER_HOST = 'saas-demo-v3-mightyfairy.parsec.cloud';

export const TRIAL_EXPIRATION_DAYS = 14;

export enum ServerType {
  Saas = 'saas',
  Trial = 'trial',
  Custom = 'custom',
}

const DEFAULT_PORT = 80;

function getSaasHost(): string {
  if (window.isDev()) {
    return TEST_SERVER_HOST;
  }
  return SAAS_SERVER_HOST;
}

export function getServerTypeFromAddress(addr: string): ServerType {
  addr = addr.toLocaleLowerCase();
  let url;
  try {
    url = new URL(addr);
  } catch (e: any) {
    return ServerType.Custom;
  }

  if (url.hostname === getSaasHost() && (!url.port || url.port === DEFAULT_PORT.toString())) {
    return ServerType.Saas;
  } else if (url.hostname === TRIAL_SERVER_HOST && (!url.port || url.port === DEFAULT_PORT.toString())) {
    return ServerType.Trial;
  }
  return ServerType.Custom;
}

export function getServerTypeFromHost(host: string, port?: number): ServerType {
  const addr = port === undefined || port === DEFAULT_PORT ? host.toLocaleLowerCase() : `${host.toLocaleLowerCase()}:${port}`;

  if (addr === getSaasHost()) {
    return ServerType.Saas;
  } else if (addr === TRIAL_SERVER_HOST) {
    return ServerType.Trial;
  }
  return ServerType.Custom;
}

export function getServerAddress(type: ServerType): string | undefined {
  if (type === ServerType.Saas) {
    return `parsec3://${getSaasHost()}`;
  } else if (type === ServerType.Trial) {
    return `parsec3://${TRIAL_SERVER_HOST}`;
  }
}

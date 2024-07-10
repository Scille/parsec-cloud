// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

const SAAS_SERVER_HOST = 'saas-v3.parsec.cloud';
const TRIAL_SERVER_HOST = 'trial.parsec.cloud';
// const TEST_SERVER_HOST = 'saas-demo-v3-mightyfairy.parsec.cloud';

export enum ServerType {
  Saas,
  Trial,
  Custom,
}

export function getServerTypeFromAddr(addr: string): ServerType {
  addr = addr.toLocaleLowerCase();
  if (`parsec3://${SAAS_SERVER_HOST}/`.startsWith(addr)) {
    return ServerType.Saas;
  } else if (`parsec3://${TRIAL_SERVER_HOST}/`.startsWith(addr)) {
    return ServerType.Trial;
  }
  return ServerType.Custom;
}

export function getServerTypeFromHost(host: string, port?: number): ServerType {
  const addr = port !== undefined ? `${host.toLocaleLowerCase()}:${port}` : host.toLocaleLowerCase();

  if (addr === SAAS_SERVER_HOST) {
    return ServerType.Saas;
  } else if (addr === TRIAL_SERVER_HOST) {
    return ServerType.Trial;
  }
  return ServerType.Custom;
}

export function getServerAddress(type: ServerType): string | undefined {
  if (type === ServerType.Saas) {
    return `parsec3://${SAAS_SERVER_HOST}/`;
  } else if (type === ServerType.Trial) {
    return `parsec3://${TRIAL_SERVER_HOST}/`;
  }
}

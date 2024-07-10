// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ServerType, getServerAddress, getServerTypeFromAddress, getServerTypeFromHost } from '@/services/parsecServers';
import { beforeAll, it } from 'vitest';

describe('Parsec Server detection', () => {
  beforeAll(() => {
    window.isDev = (): boolean => false;
  });

  it.each([
    ['saas-v3.parsec.cloud', undefined, ServerType.Saas],
    ['Saas-v3.Parsec.Cloud', undefined, ServerType.Saas],
    ['trial.parsec.cloud', undefined, ServerType.Trial],
    ['Trial.Parsec.Cloud', undefined, ServerType.Trial],
    ['saas-v3.parsec.cloud', 80, ServerType.Saas],
    ['TRIAL.PARSEC.CLOUD', 80, ServerType.Trial],
    ['trial.parsec.cloud', 1337, ServerType.Custom],
    ['saas-v3.parsec.cloud', 1337, ServerType.Custom],
    ['my-server.com', undefined, ServerType.Custom],
    ['my-server.com', 80, ServerType.Custom],
  ])('Test getServerTypeFromHost %s:%s => %s', (host: string, port: number | undefined, expected: ServerType) => {
    expect(getServerTypeFromHost(host, port)).to.equal(expected);
  });

  it.each([
    ['parsec3://saas-v3.parsec.cloud:80', ServerType.Saas],
    ['parsec3://saas-v3.parsec.cloud', ServerType.Saas],
    ['parsec3://saas-v3.parsec.cloud:1337', ServerType.Custom],
    ['Parsec3://Saas-v3.Parsec.Cloud', ServerType.Saas],
    ['PARSEC3://SAAS-V3.PARSEC.CLOUD', ServerType.Saas],
    ['trial.parsec.cloud', ServerType.Custom],
    ['parsec3://trial.parsec.cloud', ServerType.Trial],
    ['parsec3://trial.parsec.cloud:80', ServerType.Trial],
    ['parsec3://my-server.com', ServerType.Custom],
    ['parsec3://my-server.com:1337', ServerType.Custom],
    ['parsec3://saas-v3.parsec.cloud?action=bootstrap_organization', ServerType.Saas],
    ['parsec3://saas-v3.parsec.cloud/myOrg?action=bootstrap_organization', ServerType.Saas],
    ['parsec3://saas-v3.parsec.cloud:80/myOrg?action=bootstrap_organization', ServerType.Saas],
    ['parsec3://saas-v3.parsec.cloud:1337/myOrg?action=bootstrap_organization', ServerType.Custom],
  ])('Test getServerTypeFromAddress %s => %s', (addr, expected: ServerType) => {
    expect(getServerTypeFromAddress(addr)).to.equal(expected);
  });
});

it.each([
  [ServerType.Saas, 'parsec3://saas-v3.parsec.cloud'],
  [ServerType.Trial, 'parsec3://trial.parsec.cloud'],
])('Test getServerAddress %s => %s', (serverType: ServerType, addr: string) => {
  expect(getServerAddress(serverType) ?? '').to.equal(addr);
});

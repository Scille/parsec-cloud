// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ServerType, getServerTypeFromAddress, getServerTypeFromHost } from '@/services/parsecServers';
import { describe, expect, it } from 'vitest';

describe('Parsec Server detection', () => {
  it.each([
    ['saas-v3.parsec.cloud', undefined, ServerType.Saas],
    ['Saas-v3.Parsec.Cloud', undefined, ServerType.Saas],
    ['trial.parsec.cloud', undefined, ServerType.Trial],
    ['Trial.Parsec.Cloud', undefined, ServerType.Trial],
    ['saas-v3.parsec.cloud', 443, ServerType.Saas],
    ['TRIAL.PARSEC.CLOUD', 443, ServerType.Trial],
    ['trial.parsec.cloud', 1337, ServerType.Custom],
    ['saas-v3.parsec.cloud', 1337, ServerType.Custom],
    ['my-server.com', undefined, ServerType.Custom],
    ['my-server.com', 443, ServerType.Custom],
    ['saas-demo-v3-mightyfairy.parsec.cloud', 443, ServerType.Saas],
    ['saas-demo-v3-mightyfairy.parsec.cloud', undefined, ServerType.Saas],
    ['Saas-Demo-v3-MightyFairy.Parsec.Cloud', 443, ServerType.Saas],
  ])('Test getServerTypeFromHost %s:%s => %s', (host: string, port: number | undefined, expected: ServerType) => {
    expect(getServerTypeFromHost(host, port)).to.equal(expected);
  });

  it.each([
    ['parsec3://saas-v3.parsec.cloud:443', ServerType.Saas],
    ['parsec3://saas-v3.parsec.cloud', ServerType.Saas],
    ['parsec3://saas-v3.parsec.cloud:1337', ServerType.Custom],
    ['Parsec3://Saas-v3.Parsec.Cloud', ServerType.Saas],
    ['PARSEC3://SAAS-V3.PARSEC.CLOUD', ServerType.Saas],
    ['trial.parsec.cloud', ServerType.Custom],
    ['parsec3://trial.parsec.cloud', ServerType.Trial],
    ['parsec3://trial.parsec.cloud:443', ServerType.Trial],
    ['parsec3://my-server.com', ServerType.Custom],
    ['parsec3://my-server.com:1337', ServerType.Custom],
    ['parsec3://saas-v3.parsec.cloud?action=bootstrap_organization', ServerType.Saas],
    ['parsec3://saas-v3.parsec.cloud/myOrg?action=bootstrap_organization', ServerType.Saas],
    ['parsec3://saas-v3.parsec.cloud:443/myOrg?action=bootstrap_organization', ServerType.Saas],
    ['parsec3://saas-v3.parsec.cloud:1337/myOrg?action=bootstrap_organization', ServerType.Custom],
    ['parsec3://saas-demo-v3-mightyfairy.parsec.cloud', ServerType.Saas],
    ['parsec3://saas-demo-v3-mightyfairy.parsec.cloud:443', ServerType.Saas],
    ['parsec3://saas-demo-v3-mightyfairy.parsec.cloud:443/org', ServerType.Saas],
    ['parsec3://saas-demo-v3-mightyfairy.parsec.cloud:443/org', ServerType.Saas],
    ['parsec://saas-demo-v3-mightyfairy.parsec.cloud:443/org', ServerType.Saas],
  ])('Test getServerTypeFromAddress %s => %s', (addr, expected: ServerType) => {
    expect(getServerTypeFromAddress(addr)).to.equal(expected);
  });
});

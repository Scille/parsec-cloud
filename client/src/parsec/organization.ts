// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BootstrapOrganizationErrorTag, DeviceSaveStrategyTag, WorkspaceStorageCacheSizeTag, libparsec } from '@/plugins/libparsec';

import {
  AvailableDevice,
  ClientConfig,
  Result,
  BootstrapOrganizationError,
  OrganizationID,
  DeviceFileType,
  ParsedBackendAddr,
  BackendAddr,
  ClientEventPing,
  ParseBackendAddrError,
} from '@/parsec/types';
import { DateTime } from 'luxon';
import { MOCK_WAITING_TIME, wait } from '@/parsec/internals';
import { needsMocks } from '@/parsec/environment';

export async function createOrganization(
  backendAddr: BackendAddr, orgName: OrganizationID, userName: string, email: string, password: string, deviceLabel: string,
): Promise<Result<AvailableDevice, BootstrapOrganizationError>> {

  function parsecEventCallback(event: ClientEventPing): void {
    console.log('On event', event);
  }

  const bootstrapAddr = await libparsec.buildBackendOrganizationBootstrapAddr(backendAddr, orgName);

  if (!needsMocks()) {
    const config: ClientConfig = {
      configDir: window.getConfigDir(),
      dataBaseDir: window.getDataBaseDir(),
      mountpointBaseDir: window.getMountpointBaseDir(),
      workspaceStorageCacheSize: { tag: WorkspaceStorageCacheSizeTag.Default },
    };
    const result = await libparsec.bootstrapOrganization(
      config,
      parsecEventCallback,
      bootstrapAddr,
      {tag: DeviceSaveStrategyTag.Password, password: password},
      {label: userName, email: email},
      deviceLabel,
      null,
    );
    if (!result.ok && result.error.tag === BootstrapOrganizationErrorTag.BadTimestamp) {
      result.error.clientTimestamp = DateTime.fromSeconds(result.error.clientTimestamp as any as number);
      result.error.serverTimestamp = DateTime.fromSeconds(result.error.serverTimestamp as any as number);
    }
    return result;
  } else {
    await wait(MOCK_WAITING_TIME);
    return {ok: true, value: {
      keyFilePath: '/path',
      organizationId: 'MyOrg',
      deviceId: 'deviceid',
      humanHandle: {
        label: 'A',
        email: 'a@b.c',
      },
      deviceLabel: 'a@b',
      slug: 'slug',
      ty: DeviceFileType.Password,
    }};
  }
}

export async function parseBackendAddr(addr: string): Promise<Result<ParsedBackendAddr, ParseBackendAddrError>> {
  return await libparsec.parseBackendAddr(addr);
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BootstrapOrganizationErrorTag, DeviceSaveStrategyTag, WorkspaceStorageCacheSizeTag, libparsec } from '@/plugins/libparsec';

import { needsMocks } from '@/parsec/environment';
import { MOCK_WAITING_TIME, wait } from '@/parsec/internals';
import { getParsecHandle } from '@/parsec/routing';
import {
  AvailableDevice,
  BootstrapOrganizationError,
  ClientConfig,
  ClientEvent,
  DeviceFileType,
  OrganizationID,
  OrganizationInfo,
  OrganizationInfoError,
  ParseBackendAddrError,
  ParsecAddr,
  ParsedParsecAddr,
  Result,
} from '@/parsec/types';
import { DateTime } from 'luxon';

export async function createOrganization(
  backendAddr: ParsecAddr,
  orgName: OrganizationID,
  userName: string,
  email: string,
  password: string,
  deviceLabel: string,
): Promise<Result<AvailableDevice, BootstrapOrganizationError>> {
  function parsecEventCallback(event: ClientEvent): void {
    console.log('On event', event);
  }

  const bootstrapAddr = await libparsec.buildBackendOrganizationBootstrapAddr(backendAddr, orgName);

  if (!needsMocks()) {
    const config: ClientConfig = {
      configDir: window.getConfigDir(),
      dataBaseDir: window.getDataBaseDir(),
      mountpointBaseDir: window.getMountpointBaseDir(),
      workspaceStorageCacheSize: { tag: WorkspaceStorageCacheSizeTag.Default },
      withMonitors: true,
    };
    const result = await libparsec.bootstrapOrganization(
      config,
      parsecEventCallback,
      bootstrapAddr,
      { tag: DeviceSaveStrategyTag.Password, password: password },
      { label: userName, email: email },
      deviceLabel,
      null,
    );
    if (!result.ok && result.error.tag === BootstrapOrganizationErrorTag.TimestampOutOfBallpark) {
      result.error.clientTimestamp = DateTime.fromSeconds(result.error.clientTimestamp as any as number);
      result.error.serverTimestamp = DateTime.fromSeconds(result.error.serverTimestamp as any as number);
    }
    return result;
  } else {
    await wait(MOCK_WAITING_TIME);
    return {
      ok: true,
      value: {
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
      },
    };
  }
}

export async function parseBackendAddr(addr: string): Promise<Result<ParsedParsecAddr, ParseBackendAddrError>> {
  return await libparsec.parseBackendAddr(addr);
}

export async function getOrganizationInfo(): Promise<Result<OrganizationInfo, OrganizationInfoError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return {
      ok: true,
      value: {
        users: {
          revoked: 2,
          total: 12,
          active: 10,
          admins: 2,
          standards: 5,
          outsiders: 3,
        },
        size: {
          metadata: 42_492_122,
          data: 4_498_394_233_231,
          total: 4_498_351_741_109,
        },
        outsidersAllowed: true,
        userLimit: -1,
      },
    };
  } else {
    return {
      ok: true,
      value: {
        users: {
          revoked: 2,
          total: 12,
          active: 10,
          admins: 2,
          standards: 5,
          outsiders: 3,
        },
        size: {
          metadata: 42_492_122,
          data: 4_498_394_233_231,
          total: 4_498_351_741_109,
        },
        outsidersAllowed: true,
        userLimit: -1,
      },
    };
  }
}

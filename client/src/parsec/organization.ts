// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  ActiveUsersLimitLimitedTo,
  ActiveUsersLimitTag,
  MountpointMountStrategyTag,
  WorkspaceStorageCacheSizeTag,
  libparsec,
} from '@/plugins/libparsec';

import { getClientInfo } from '@/parsec/login';
import {
  AvailableDevice,
  BootstrapOrganizationError,
  BootstrapOrganizationErrorTag,
  ClientConfig,
  DeviceSaveStrategy,
  OrganizationID,
  OrganizationInfo,
  OrganizationInfoError,
  OrganizationInfoErrorTag,
  ParseParsecAddrError,
  ParsecAddr,
  ParsedParsecAddr,
  Result,
  UserProfile,
} from '@/parsec/types';
import { listUsers } from '@/parsec/user';
import { DateTime } from 'luxon';

export async function createOrganization(
  serverAddr: ParsecAddr,
  orgName: OrganizationID,
  userName: string,
  email: string,
  deviceLabel: string,
  saveStrategy: DeviceSaveStrategy,
): Promise<Result<AvailableDevice, BootstrapOrganizationError>> {
  const bootstrapAddr = await libparsec.buildParsecOrganizationBootstrapAddr(serverAddr, orgName);

  const config: ClientConfig = {
    configDir: window.getConfigDir(),
    dataBaseDir: window.getDataBaseDir(),
    mountpointMountStrategy: { tag: MountpointMountStrategyTag.Disabled },
    workspaceStorageCacheSize: { tag: WorkspaceStorageCacheSizeTag.Default },
    withMonitors: false,
    preventSyncPattern: null,
    logLevel: null,
  };
  const result = await libparsec.bootstrapOrganization(
    config,
    bootstrapAddr,
    saveStrategy,
    { label: userName, email: email },
    deviceLabel,
    null,
  );
  if (!result.ok && result.error.tag === BootstrapOrganizationErrorTag.TimestampOutOfBallpark) {
    result.error.clientTimestamp = DateTime.fromSeconds(result.error.clientTimestamp as any as number);
    result.error.serverTimestamp = DateTime.fromSeconds(result.error.serverTimestamp as any as number);
  } else if (result.ok) {
    result.value.createdOn = DateTime.fromSeconds(result.value.createdOn as any as number);
    result.value.protectedOn = DateTime.fromSeconds(result.value.protectedOn as any as number);
  }
  return result;
}

export async function bootstrapOrganization(
  bootstrapAddr: string,
  userName: string,
  email: string,
  deviceLabel: string,
  saveStrategy: DeviceSaveStrategy,
): Promise<Result<AvailableDevice, BootstrapOrganizationError>> {
  const config: ClientConfig = {
    configDir: window.getConfigDir(),
    dataBaseDir: window.getDataBaseDir(),
    mountpointMountStrategy: { tag: MountpointMountStrategyTag.Disabled },
    workspaceStorageCacheSize: { tag: WorkspaceStorageCacheSizeTag.Default },
    withMonitors: false,
    preventSyncPattern: null,
    logLevel: null,
  };
  const result = await libparsec.bootstrapOrganization(
    config,
    bootstrapAddr,
    saveStrategy,
    { label: userName, email: email },
    deviceLabel,
    null,
  );
  if (!result.ok && result.error.tag === BootstrapOrganizationErrorTag.TimestampOutOfBallpark) {
    result.error.clientTimestamp = DateTime.fromSeconds(result.error.clientTimestamp as any as number);
    result.error.serverTimestamp = DateTime.fromSeconds(result.error.serverTimestamp as any as number);
  } else if (result.ok) {
    result.value.createdOn = DateTime.fromSeconds(result.value.createdOn as any as number);
    result.value.protectedOn = DateTime.fromSeconds(result.value.protectedOn as any as number);
  }
  return result;
}

export async function parseParsecAddr(addr: string): Promise<Result<ParsedParsecAddr, ParseParsecAddrError>> {
  return await libparsec.parseParsecAddr(addr);
}

export async function getOrganizationInfo(): Promise<Result<OrganizationInfo, OrganizationInfoError>> {
  const usersResult = await listUsers(false);
  const clientInfoResult = await getClientInfo();

  if (!usersResult.ok || !clientInfoResult.ok) {
    return { ok: false, error: { tag: OrganizationInfoErrorTag.Internal } };
  }

  return {
    ok: true,
    value: {
      users: {
        revoked: usersResult.value.filter((user) => user.isRevoked()).length,
        active: usersResult.value.filter((user) => !user.isRevoked()).length,
        total: usersResult.value.length,
        admins: usersResult.value.filter((user) => user.currentProfile === UserProfile.Admin && !user.isRevoked()).length,
        standards: usersResult.value.filter((user) => user.currentProfile === UserProfile.Standard && !user.isRevoked()).length,
        outsiders: usersResult.value.filter((user) => user.currentProfile === UserProfile.Outsider && !user.isRevoked()).length,
        frozen: usersResult.value.filter((user) => user.isFrozen() && !user.isRevoked()).length,
      },
      size: {
        metadata: 14_234_953,
        data: 5_348_491_032,
      },
      outsidersAllowed: clientInfoResult.value.serverConfig.userProfileOutsiderAllowed,
      userLimit:
        clientInfoResult.value.serverConfig.activeUsersLimit.tag === ActiveUsersLimitTag.LimitedTo
          ? Number((clientInfoResult.value.serverConfig.activeUsersLimit as ActiveUsersLimitLimitedTo).x1)
          : undefined,
      hasUserLimit: clientInfoResult.value.serverConfig.activeUsersLimit.tag !== ActiveUsersLimitTag.NoLimit,
      organizationAddr: clientInfoResult.value.organizationAddr,
      organizationId: clientInfoResult.value.organizationId,
    },
  };
}

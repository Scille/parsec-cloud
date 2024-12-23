// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { AvailableDevice, OrganizationID, ParsedParsecAddrTag, parseParsecAddr, Platform } from '@/parsec';
import { I18n } from 'megashark-lib';

export function getDefaultDeviceName(): string {
  switch (window.getPlatform()) {
    case Platform.Web:
      return I18n.translate('common.deviceTypes.web');
    case Platform.Android:
      return I18n.translate('common.deviceTypes.android');
    case Platform.Windows:
      return I18n.translate('common.deviceTypes.windows');
    case Platform.MacOS:
      return I18n.translate('common.deviceTypes.macos');
    case Platform.Linux:
      return I18n.translate('common.deviceTypes.linux');
    default:
      return I18n.translate('common.deviceTypes.unknown');
  }
}

export async function availableDeviceMatchesServer(
  device: AvailableDevice,
  serverInfo: { hostname: string; port?: number; organization?: OrganizationID },
): Promise<boolean> {
  const deviceAddr = device.serverUrl.replace('http://', 'parsec3://').replace('https://', 'parsec3://');
  const parseResult = await parseParsecAddr(deviceAddr);
  if (!parseResult.ok) {
    return false;
  }
  if (serverInfo.hostname !== parseResult.value.hostname) {
    return false;
  }
  if (serverInfo.port !== undefined && serverInfo.port !== parseResult.value.port) {
    return false;
  }
  if (
    serverInfo.organization &&
    parseResult.value.tag !== ParsedParsecAddrTag.Server &&
    parseResult.value.organizationId !== serverInfo.organization
  ) {
    return false;
  }
  return true;
}

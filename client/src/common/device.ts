// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { AvailableDevice, OrganizationID, ParsedParsecAddrTag, parseParsecAddr, Platform } from '@/parsec';
import { getPlatforms } from '@ionic/vue';

export enum DeviceLabel {
  MobileWeb = 'MobileWeb',
  Web = 'Web',
  Windows = 'Windows',
  MacOS = 'MacOS',
  Linux = 'Linux',
  Android = 'Android',
  Unknown = 'Unknown',
}

export function getDefaultDeviceName(): DeviceLabel {
  switch (window.getPlatform()) {
    case Platform.Web:
      const platforms = getPlatforms();
      // cspell:disable-next-line
      if (platforms.includes('mobileweb')) {
        return DeviceLabel.MobileWeb;
      }
      return DeviceLabel.Web;
    case Platform.Android:
      return DeviceLabel.Android;
    case Platform.Windows:
      return DeviceLabel.Windows;
    case Platform.MacOS:
      return DeviceLabel.MacOS;
    case Platform.Linux:
      return DeviceLabel.Linux;
    default:
      return DeviceLabel.Unknown;
  }
}

export async function availableDeviceMatchesServer(
  device: AvailableDevice,
  serverInfo: { hostname: string; port?: number; organization?: OrganizationID },
): Promise<boolean> {
  const parseResult = await parseParsecAddr(device.serverAddr);
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

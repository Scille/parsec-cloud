// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Platform } from '@/parsec';

export async function getDefaultDeviceName(): Promise<string> {
  switch (window.getPlatform()) {
    case Platform.Web:
      return 'common.deviceTypes.web';
    case Platform.Android:
      return 'common.deviceTypes.android';
    case Platform.Windows:
      return 'common.deviceTypes.windows';
    case Platform.MacOS:
      return 'common.deviceTypes.macos';
    case Platform.Linux:
      return 'common.deviceTypes.linux';
    default:
      return 'common.deviceTypes.unknown';
  }
}

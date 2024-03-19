// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Platform } from '@/parsec';
import { msTranslate } from '@/services/translation';

export async function getDefaultDeviceName(): Promise<string> {
  switch (window.getPlatform()) {
    case Platform.Web:
      return msTranslate('common.deviceTypes.web');
    case Platform.Android:
      return msTranslate('common.deviceTypes.android');
    case Platform.Windows:
      return msTranslate('common.deviceTypes.windows');
    case Platform.MacOS:
      return msTranslate('common.deviceTypes.macos');
    case Platform.Linux:
      return msTranslate('common.deviceTypes.linux');
    default:
      return 'Unknown';
  }
}

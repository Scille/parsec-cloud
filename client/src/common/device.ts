// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Platform } from '@/parsec';
import { translate } from '@/services/translation';

export async function getDefaultDeviceName(): Promise<string> {
  switch (window.getPlatform()) {
    case Platform.Web:
      return translate('common.deviceTypes.web');
    case Platform.Android:
      return translate('common.deviceTypes.android');
    case Platform.Windows:
      return translate('common.deviceTypes.windows');
    case Platform.MacOS:
      return translate('common.deviceTypes.macos');
    case Platform.Linux:
      return translate('common.deviceTypes.linux');
    default:
      return 'Unknown';
  }
}

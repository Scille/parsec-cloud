// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Platform } from '@/parsec';
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

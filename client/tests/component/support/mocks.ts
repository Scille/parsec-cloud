// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DateTime } from 'luxon';
import { FormattersKey, NotificationKey } from '@/common/injectionKeys';
import { vi } from 'vitest';
import { config } from '@vue/test-utils';

function mockTimeSince(_date: DateTime | undefined, _default: string, _format: string): string {
  return 'One minute ago';
}

function mockFileSize(_size: number): string {
  return '1MB';
}

async function mockShowToast(_notif: Notification): Promise<void> {
  // Do nothing
}

function getDefaultProvideConfig(timeSince = mockTimeSince, fileSize = mockFileSize, showToast = mockShowToast): any {
  const provide: any = {};

  provide[FormattersKey] = {
    'timeSince': timeSince,
    'fileSize': fileSize,
  };
  provide[NotificationKey] = {
    'showToast': showToast,
  };

  return provide;
}

function mockI18n(): void {
  vi.mock('vue-i18n', () => {
    return { useI18n: (): any => {
      return {t: (key: string): string => key };
    } };
  });

  config.global.mocks = {
    $t: (key: string): string => key,
  };
}

export {
  mockTimeSince,
  mockFileSize,
  mockI18n,
  getDefaultProvideConfig,
};

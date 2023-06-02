// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { DateTime } from 'luxon';
import { FormattersKey } from '@/common/injectionKeys';
import { vi } from 'vitest';
import { config } from '@vue/test-utils';
import { useI18n } from 'vue-i18n';

function mockTimeSince(_date: DateTime | undefined, _default: string, _format: string): string {
  return 'One minute ago';
}

function mockFileSize(_size: number): string {
  return '1MB';
}

function getDefaultProvideConfig(timeSince = mockTimeSince, fileSize = mockFileSize): any {
  const provide: any = {};

  provide[FormattersKey] = {
    'timeSince': timeSince,
    'fileSize': fileSize
  };

  return provide;
}

function mockI18n(): void {
  vi.mock('vue-i18n');

  (useI18n as any).mockReturnValue({
    t: (key: string): string => key
  });

  config.global.mocks = {
    $t: (key: string): string => key
  };
}

export {
  mockTimeSince,
  mockFileSize,
  mockI18n,
  getDefaultProvideConfig
};

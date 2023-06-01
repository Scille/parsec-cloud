// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { DateTime } from 'luxon';
import { FormattersKey } from '@/common/injectionKeys';

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

function getDefaultMockConfig(): any {
  return {
    $t: (key: string): string => {
      return key;
    }
  };
}

export {
  mockTimeSince,
  mockFileSize,
  getDefaultMockConfig,
  getDefaultProvideConfig
};

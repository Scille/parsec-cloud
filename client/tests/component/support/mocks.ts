// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { FormattersKey, NotificationKey } from '@/common/injectionKeys';
import { translate } from '@/services/translation';
import { config } from '@vue/test-utils';
import { DateTime } from 'luxon';
import { vi } from 'vitest';

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
    timeSince: timeSince,
    fileSize: fileSize,
  };
  provide[NotificationKey] = {
    showToast: showToast,
  };

  return provide;
}

function mockI18n(): void {
  config.global.mocks = {
    $t: (key: string, attrs?: object, count?: number): string => translate(key, attrs, count),
  };
}

interface Route {
  route: string;
  params?: object;
  query?: object;
}

const ROUTES_CALLED: Array<Route> = [];

function mockRouter(): void {
  // Mocking the following import:
  // import { useRouter } from 'vue-router';
  vi.mock('vue-router', async () => {
    const router = await vi.importActual<typeof import('vue-router')>('vue-router');
    return {
      ...router,
      useRouter: (): any => {
        return {
          push: (options: any): void => {
            ROUTES_CALLED.push({ route: options.name, params: options.params, query: options.query });
          },
        };
      },
    };
  });
}

function getRoutesCalled(): Array<Route> {
  return ROUTES_CALLED;
}

function resetRoutesCalled(): void {
  ROUTES_CALLED.splice(0);
}

export { getDefaultProvideConfig, getRoutesCalled, mockFileSize, mockI18n, mockRouter, mockTimeSince, resetRoutesCalled };

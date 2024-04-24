// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { HotkeyManagerKey } from '@/services/hotkeyManager';
import { InformationManagerKey } from '@/services/informationManager';
import { config } from '@vue/test-utils';
import { I18n, Translatable } from 'megashark-lib';
import { vi } from 'vitest';

async function mockShowToast(_notif: Notification): Promise<void> {
  // Do nothing
}

class MockHotKeys {
  add(_key: string, _modifiers: number, _platforms: number, _callback: () => Promise<void>): void {}
}

function mockNewHotKeys(_group: number): MockHotKeys {
  return new MockHotKeys();
}

function mockUnregister(_hotkeys: MockHotKeys): void {}

function mockGroupEnableDisable(_group: number): void {}

function getDefaultProvideConfig(showToast = mockShowToast): any {
  const provide: any = {};

  provide[InformationManagerKey] = {
    showToast: showToast,
  };
  provide[HotkeyManagerKey] = {
    newHotkeys: mockNewHotKeys,
    unregister: mockUnregister,
    disableGroup: mockGroupEnableDisable,
    enableGroup: mockGroupEnableDisable,
  };
  return provide;
}

function mockLibParsec(): void {
  vi.mock('@/parsec', async () => {
    const parsec = await vi.importActual<typeof import('@/parsec')>('@/parsec');
    const Path = parsec.Path;
    return {
      ...parsec,
      isValidOrganizationName: async (_value: string): Promise<boolean> => {
        return false;
      },
      parseParsecAddr: async (_value: string): Promise<any> => {
        return { ok: false, error: 'error' };
      },
      Path: {
        ...Path,
        parent: async (path: string): Promise<string> => {
          const idx = path.lastIndexOf('/');
          if (idx === -1 || idx === 0) {
            return path;
          }
          return path.slice(0, idx);
        },
        join: async (path: string, fileName: string): Promise<string> => {
          return `${path}/${fileName}`;
        },
      },
    };
  });
}

function mockI18n(): void {
  config.global.mocks = {
    $msTranslate: (tr: Translatable): string => I18n.translate(tr),
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

export { getDefaultProvideConfig, getRoutesCalled, mockI18n, mockLibParsec, mockRouter, resetRoutesCalled };

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import test, { BrowserContext, Page } from '@playwright/test';

export const OPEN_BAO_SERVER = 'https://openbao';
export const SSO_PRO_CONNECT_SERVER = 'https://proconnect';

interface MockOptions {
  timeout?: boolean;
  httpErrorCode?: number;
}

type MockOpenBaoOptions = MockOptions;
type MockProConnectOptions = MockOptions;

interface MockSsoOptions {
  OpenBao?: MockOpenBaoOptions;
  ProConnect?: MockProConnectOptions;
}

const DEFAULT_CODE = 'Th3C0d3';
const DEFAULT_STATE = 'Th3St4t3';
const DEFAULT_TOKEN = 'Th3T0k3n';
const STORED_DATA = new Map<string, any>();
const START_KEY_PATH = '/v1/parsec-keys/data';

function getKey(rawUrl: string): string {
  const parsed = new URL(rawUrl);
  const fullPath = parsed.pathname;
  fullPath.replace(START_KEY_PATH, '');
  return fullPath;
}

export async function mockOpenBao(page: Page | BrowserContext, opts?: MockOpenBaoOptions): Promise<void> {
  await page.route(`${OPEN_BAO_SERVER}/**`, async (route) => {
    if (opts?.timeout) {
      await route.abort('timedout');
    } else if (opts?.httpErrorCode) {
      await route.fulfill({ status: opts.httpErrorCode });
    } else {
      if (route.request().method() === 'POST' && route.request().url().includes('/v1/auth/oidc/oidc/auth_url')) {
        // eslint-disable-next-line camelcase
        await route.fulfill({ json: { data: { auth_url: `${SSO_PRO_CONNECT_SERVER}/login` } } });
      } else if (route.request().method() === 'GET' && route.request().url().includes('/v1/auth/oidc/oidc/callback')) {
        // eslint-disable-next-line camelcase
        await route.fulfill({ json: { auth: { client_token: DEFAULT_TOKEN } } });
      } else if (route.request().method() === 'POST' && route.request().url().includes(START_KEY_PATH)) {
        const key = getKey(route.request().url());
        STORED_DATA.set(key, route.request().postDataJSON());
        await route.fulfill({ status: 200 });
      } else if (route.request().method() === 'GET' && route.request().url().includes(START_KEY_PATH)) {
        const key = getKey(route.request().url());
        const data = STORED_DATA.get(key);
        await route.fulfill({ status: 200, json: { data: data } });
      } else {
        await route.fulfill({ status: 404 });
      }
    }
  });
}

export async function mockProConnect(page: Page | BrowserContext, opts?: MockProConnectOptions): Promise<void> {
  await page.route(`${SSO_PRO_CONNECT_SERVER}/**`, async (route) => {
    if (opts?.timeout) {
      await route.abort('timedout');
    } else if (opts?.httpErrorCode) {
      await route.fulfill({ status: opts.httpErrorCode });
    } else {
      const redirectUrl = `${test.info().project.use.baseURL}/oidc/callback?code=${DEFAULT_CODE}&state=${DEFAULT_STATE}`;
      await route.fulfill({ status: 301, headers: { Location: redirectUrl } });
    }
  });
}

export async function mockSso(page: Page | BrowserContext, opts?: MockSsoOptions): Promise<void> {
  await Promise.all([mockOpenBao(page, opts?.OpenBao), mockProConnect(page, opts?.ProConnect)]);
}

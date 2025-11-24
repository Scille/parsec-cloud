// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BrowserContext, Page } from '@playwright/test';
import { OrganizationInformation, UserData } from '@tests/e2e/helpers/data';
import { LibParsecFunction } from '@tests/e2e/helpers/libparsec';

export enum DisplaySize {
  Small = 'small',
  Large = 'large',
}

export interface SetupOptions {
  testbedPath?: string;
  skipTestbed?: boolean;
  location?: string;
  skipGoto?: boolean;
  withParsecAccount?: boolean;
  withEditics?: boolean;
  parsecAccountAutoLogin?: boolean;
  withCustomBranding?: boolean;
  displaySize?: DisplaySize;
  mockBrowser?: 'Chrome' | 'Firefox' | 'Safari' | 'Edge' | 'Brave' | 'Chromium';
  trialServers?: string;
  saasServers?: string;
  cryptpadServer?: string;
  openBaoServer?: string;
  expectTimeout?: number;
  libparsecMockFunctions?: Array<LibParsecFunction>;
  enableStripe?: boolean;
  enableUpdateEvent?: boolean;
}

export interface MsPage extends Page {
  userData: UserData;
  orgInfo: OrganizationInformation;
  release: () => Promise<void>;
  isReleased: boolean;
  skipTestbedRelease: boolean;
  getContext: () => MsContext;
  openNewTab: (opts?: SetupOptions) => Promise<MsPage>;
  isDebugEnabled: () => boolean;
  displaySize: DisplaySize;
  defaultLargeSize: [number, number];
  defaultSmallSize: [number, number];
  setDisplaySize: (displaySize: DisplaySize) => Promise<void>;
  getDisplaySize: () => Promise<DisplaySize>;
}

export interface MsContext extends BrowserContext {
  testbedConfigPath: string;
}

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BrowserContext, Page } from '@playwright/test';
import { OrganizationInformation, UserData } from '@tests/e2e/helpers/data';

export enum DisplaySize {
  Small = 'small',
  Large = 'large',
}

export interface MsPage extends Page {
  userData: UserData;
  orgInfo: OrganizationInformation;
  release: () => Promise<void>;
  isReleased: boolean;
  skipTestbedRelease: boolean;
  getContext: () => MsContext;
  openNewTab: () => Promise<MsPage>;
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

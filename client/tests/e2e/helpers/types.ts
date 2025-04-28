// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BrowserContext, Page } from '@playwright/test';
import { OrganizationInformation, UserData } from '@tests/e2e/helpers/data';

export interface MsPage extends Page {
  userData: UserData;
  orgInfo: OrganizationInformation;
  release: () => Promise<void>;
  isReleased: boolean;
  skipTestbedRelease: boolean;
  openNewTab: () => Promise<MsPage>;
}

export interface MsContext extends BrowserContext {
  testbedConfigPath: string;
}

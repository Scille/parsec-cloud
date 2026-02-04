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
  mockPki?: boolean;
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

export enum ImportDocuments {
  Docx = 1 << 0,
  Xlsx = 1 << 1,
  Pptx = 1 << 2,
  Py = 1 << 3,
  Txt = 1 << 4,
  Png = 1 << 5,
  Mp3 = 1 << 6,
  Mp4 = 1 << 7,
  Pdf = 1 << 8,
}

export const ImportAllDocuments =
  ImportDocuments.Docx |
  ImportDocuments.Xlsx |
  ImportDocuments.Pptx |
  ImportDocuments.Py |
  ImportDocuments.Txt |
  ImportDocuments.Png |
  ImportDocuments.Mp3 |
  ImportDocuments.Mp4 |
  ImportDocuments.Pdf;

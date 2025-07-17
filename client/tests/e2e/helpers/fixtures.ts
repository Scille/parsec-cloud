// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, TestInfo, test as base } from '@playwright/test';
import { expect } from '@tests/e2e/helpers/assertions';
import { MockBms, MockClientAreaOverload, MockRouteOptions } from '@tests/e2e/helpers/bms';
import { DEFAULT_USER_INFORMATION, generateDefaultOrganizationInformation, generateDefaultUserData } from '@tests/e2e/helpers/data';
import { mockExternalWebsites } from '@tests/e2e/helpers/externalWebsites';
import { dropTestbed, initTestBed } from '@tests/e2e/helpers/testbed';
import { DisplaySize, MsContext, MsPage } from '@tests/e2e/helpers/types';
import { createWorkspace, fillInputModal, fillIonInput, importDefaultFiles, logout } from '@tests/e2e/helpers/utils';

interface SetupOptions {
  testbedPath?: string;
  skipTestbed?: boolean;
  location?: string;
  skipGoto?: boolean;
  withParsecAccount?: boolean;
  parsecAccountAutoLogin?: boolean;
  withCustomBranding?: boolean;
  displaySize?: DisplaySize;
  mockBrowser?: 'Chrome' | 'Firefox' | 'Safari' | 'Edge' | 'Brave' | 'Chromium';
  trialServers?: string;
  saasServers?: string;
}

const DEV_TOOLS_OFFSET = 400;

export async function setupNewPage(page: MsPage, opts: SetupOptions = {}): Promise<void> {
  page.on('console', (msg) => console.log('> ', msg.text()));

  await page.addInitScript((options: SetupOptions) => {
    (window as any).TESTING = true;
    if (options.withParsecAccount) {
      (window as any).TESTING_ENABLE_ACCOUNT = true;
    }
    if (options.parsecAccountAutoLogin) {
      (window as any).TESTING_ACCOUNT_AUTO_LOGIN = true;
    }
    if (options.withCustomBranding) {
      (window as any).TESTING_ENABLE_CUSTOM_BRANDING = true;
    }
    if (options.mockBrowser) {
      (window as any).TESTING_MOCK_BROWSER = options.mockBrowser;
    }
    if (options.saasServers) {
      (window as any).TESTING_SAAS_SERVERS = options.saasServers;
    }
    if (options.trialServers) {
      (window as any).TESTING_TRIAL_SERVERS = options.trialServers;
    }
    (window as any).showSaveFilePicker = async (): Promise<FileSystemFileHandle> => {
      console.log('Show save file Picker');
      return {
        kind: 'file',
        name: 'downloadedFile.tmp',
        createWritable: async () => ({
          write: async (data: Uint8Array): Promise<any> => {
            if ((window as any).__downloadedFiles === undefined) {
              (window as any).__downloadedFiles = {
                default: data,
              };
            } else {
              (window as any).__downloadedFiles.default = data;
            }
          },
          close: async (): Promise<any> => {},
        }),
      } as FileSystemFileHandle;
    };
    (window as any).showDirectoryPicker = async (): Promise<FileSystemDirectoryHandle> => {
      return {
        kind: 'directory',
        name: 'folder',
        getFileHandle: async (name: string, _options?: FileSystemGetFileOptions): Promise<FileSystemFileHandle> => {
          return {
            kind: 'file',
            name: name,
            createWritable: async () => ({
              write: async (data: Uint8Array): Promise<any> => {
                if ((window as any).__downloadedFiles === undefined) {
                  (window as any).__downloadedFiles = {
                    [name]: data,
                  };
                } else {
                  (window as any).__downloadedFiles[name] = data;
                }
              },
              close: async (): Promise<any> => {},
            }),
          } as FileSystemFileHandle;
        },
        getDirectoryHandle: async (_name: string, _options?: FileSystemGetDirectoryOptions): Promise<FileSystemDirectoryHandle> => {
          return {} as FileSystemDirectoryHandle;
        },
        removeEntry: async (_name: string, _options?: FileSystemRemoveOptions): Promise<void> => {},
        resolve: async (_descendant: FileSystemHandle): Promise<string[] | null> => null,
        isSameEntry: async (_other: FileSystemHandle): Promise<boolean> => false,
      };
    };
  }, opts);
  if (!opts.skipGoto) {
    await page.goto(opts.location ?? '/');
  }
  await page.waitForLoadState('domcontentloaded');

  await expect(page.locator('#app')).toHaveAttribute('app-state', 'initializing');

  let testbed: string | undefined;
  if (!opts.skipTestbed) {
    testbed = await initTestBed(page, opts.testbedPath);
  } else {
    await page.evaluate(async () => {
      const [, nextStage] = window.nextStageHook();
      await nextStage(undefined, 'en-US');
    });
  }
  page.userData = generateDefaultUserData();
  page.orgInfo = generateDefaultOrganizationInformation();
  page.isReleased = false;
  if (!page.skipTestbedRelease && opts.skipTestbed) {
    page.skipTestbedRelease = true;
  }
  page.openNewTab = async (): Promise<MsPage> => {
    const newTab = (await page.context().newPage()) as MsPage;
    newTab.skipTestbedRelease = true;
    await setupNewPage(newTab, { ...opts, testbedPath: testbed });
    return newTab;
  };
  page.release = async (): Promise<void> => {
    if (!page.isReleased) {
      if (!page.skipTestbedRelease) {
        await dropTestbed(page);
      }
      await page.close();
    }
    page.isReleased = true;
  };

  page.isDebugEnabled = (): boolean => {
    return process.env.PWDEBUG === 'true';
  };
  page.defaultLargeSize = [1600, 900];
  page.defaultSmallSize = [700, 700];
  page.setDisplaySize = async (displaySize: DisplaySize): Promise<void> => {
    page.displaySize = displaySize;
    const width =
      (displaySize === DisplaySize.Large ? page.defaultLargeSize[0] : page.defaultSmallSize[0]) +
      (page.isDebugEnabled() ? DEV_TOOLS_OFFSET : 0);
    const height = displaySize === DisplaySize.Large ? page.defaultLargeSize[1] : page.defaultSmallSize[1];
    await page.setViewportSize({ width: width, height: height });
  };
  page.getDisplaySize = async (): Promise<DisplaySize> => {
    return page.displaySize;
  };
  await page.setDisplaySize(opts.displaySize ?? DisplaySize.Large);

  await expect(page.locator('#app')).toHaveAttribute('app-state', 'ready');
}

const debugTest = base.extend({
  ...(process.env.PWDEBUG && {
    browser: [
      async ({ playwright, browserName }, use): Promise<any> => {
        const args = [];
        // cspell:disable-next-line
        if (browserName.toLowerCase().includes('chrom')) {
          args.push('--auto-open-devtools-for-tabs');
        }
        const browser = await playwright[browserName].launch({
          headless: false,
          args: args,
        });
        await use(browser);
        await browser.close();
      },
      {
        scope: 'worker',
        timeout: 0,
      },
    ],
  }),
});

export const msTest = debugTest.extend<{
  context: MsContext;
  home: MsPage;
  secondTab: MsPage;
  connected: MsPage;
  workspacesStandard: MsPage;
  workspaces: MsPage;
  documents: MsPage;
  documentsReadOnly: MsPage;
  usersPage: MsPage;
  organizationPage: MsPage;
  myProfilePage: MsPage;
  userJoinModal: Locator;
  userGreetModal: Locator;
  createOrgModal: Locator;
  deviceGreetModal: Locator;
  workspaceSharingModal: Locator;
  clientArea: MsPage;
  clientAreaCustomOrder: MsPage;
  clientAreaCustomOrderInitialMocks?: Partial<
    Record<
      keyof typeof MockBms,
      {
        overload?: MockClientAreaOverload;
        options?: MockRouteOptions;
      }
    >
  >;
  parsecAccount: MsPage;
  parsecAccountLoggedIn: MsPage;
}>({
  context: async ({ browser }, use) => {
    const context = (await browser.newContext()) as MsContext;
    await context.grantPermissions(['clipboard-read']);
    await mockExternalWebsites(context);
    await use(context);
    await context.close();
  },

  home: async ({ context }, use) => {
    const page = (await context.newPage()) as MsPage;
    await setupNewPage(page);
    await use(page);
    await page.release();
  },

  connected: async ({ home }, use) => {
    await home.locator('.organization-card').first().click();
    await expect(home.locator('#password-input')).toBeVisible();

    await expect(home.locator('.login-button')).toHaveDisabledAttribute();

    await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
    await expect(home.locator('.login-button')).toBeEnabled();
    await home.locator('.login-button').click();
    await expect(home.locator('#connected-header')).toContainText('My workspaces');
    await expect(home.locator('.topbar-right').locator('.text-content-name')).toHaveText('Alicey McAliceFace');
    await expect(home).toBeWorkspacePage();

    await use(home);
  },

  workspacesStandard: async ({ home }, use) => {
    await home.locator('.organization-card').nth(1).click();
    await expect(home.locator('#password-input')).toBeVisible();

    await expect(home.locator('.login-button')).toHaveDisabledAttribute();

    await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
    await expect(home.locator('.login-button')).toBeEnabled();
    await home.locator('.login-button').click();
    await expect(home.locator('#connected-header')).toContainText('My workspaces');
    await expect(home.locator('.topbar-right').locator('.text-content-name')).toHaveText('Boby McBobFace');
    await expect(home).toBeWorkspacePage();

    await use(home);
  },

  workspaces: async ({ connected }, use) => {
    use(connected);
  },

  documents: async ({ workspaces }, use, testInfo: TestInfo) => {
    await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    await expect(workspaces).toHaveHeader(['wksp1'], true, true);
    await expect(workspaces.locator('.folder-container').locator('.no-files')).toBeVisible();
    await importDefaultFiles(workspaces, testInfo);
    use(workspaces);
  },

  documentsReadOnly: async ({ home }, use, testInfo: TestInfo) => {
    // Log in with Bob
    await home.locator('.organization-card').nth(1).click();
    await expect(home.locator('#password-input')).toBeVisible();
    await expect(home.locator('.login-button')).toHaveDisabledAttribute();
    await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
    await expect(home.locator('.login-button')).toBeEnabled();
    await home.locator('.login-button').click();
    await expect(home.locator('#connected-header')).toContainText('My workspaces');
    await expect(home).toBeWorkspacePage();

    await createWorkspace(home, 'wksp2-readonly');
    await home.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(1).click();
    await expect(home).toBeDocumentPage();
    await expect(home).toHaveHeader(['wksp2-readonly'], true, true);
    await expect(home.locator('.folder-container').locator('.no-files')).toBeVisible();
    await importDefaultFiles(home, testInfo);
    await home.locator('.topbar-left').locator('.back-button').click();
    await expect(home).toBeWorkspacePage();

    await home
      .locator('.workspaces-container-grid')
      .locator('.workspace-card-item')
      .nth(1)
      .locator('.workspace-card-bottom__icons')
      .locator('.icon-share-container')
      .nth(0)
      .click();
    const sharingModal = home.locator('.workspace-sharing-modal');
    await expect(sharingModal).toBeVisible();
    // Share the workspace with Alice
    const content = sharingModal.locator('.ms-modal-content');
    const users = content.locator('.user-list-suggestions').locator('.user-list-suggestions-item');
    await users.nth(0).locator('.dropdown-button').click();
    const roleDropdown = sharingModal.page().locator('.dropdown-popover');
    const roles = roleDropdown.getByRole('listitem');
    // Set reader
    await roles.nth(3).click();
    await expect(sharingModal.page()).toShowToast("Alicey McAliceFace's role has been updated to Reader.", 'Success');
    await sharingModal.locator('.closeBtn').click();
    await expect(sharingModal).toBeHidden();
    await logout(home);

    await expect(home).toBeHomePage();
    await home.locator('.organization-card').nth(0).click();
    await expect(home.locator('#password-input')).toBeVisible();
    await expect(home.locator('.login-button')).toHaveDisabledAttribute();
    await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
    await expect(home.locator('.login-button')).toBeEnabled();
    await home.locator('.login-button').click();
    await expect(home).toBeWorkspacePage();

    await home.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(1).click();
    await expect(home).toBeDocumentPage();
    await expect(home).toHaveHeader(['wksp2-readonly'], true, true);
    await expect(home.locator('.folder-container').locator('.no-files-content')).toBeHidden();
    use(home);
  },

  usersPage: async ({ connected }, use) => {
    await connected.locator('.sidebar').locator('#manageOrganization').click();
    await expect(connected).toHavePageTitle('Users');
    await expect(connected).toBeUserPage();
    use(connected);
  },

  organizationPage: async ({ connected }, use) => {
    await connected.locator('.sidebar').locator('#manageOrganization').click();
    const sidebarItem = connected
      .locator('.sidebar')
      .locator('.manage-organization')
      .locator('.organization-card-buttons')
      .locator('.organization-title');
    await expect(sidebarItem).toHaveTheClass('item-not-selected');
    await sidebarItem.click();
    await expect(connected).toHavePageTitle('Information');
    await expect(sidebarItem).toHaveTheClass('item-selected');
    await expect(connected).toBeOrganizationPage();
    use(connected);
  },

  myProfilePage: async ({ connected }, use) => {
    await connected.locator('.topbar').locator('.profile-header').click();
    const myProfileButton = connected.locator('.profile-header-organization-popover').locator('.main-list').getByRole('listitem').nth(0);
    await expect(myProfileButton).toHaveText('Settings');
    await myProfileButton.click();
    await expect(connected).toHavePageTitle('My profile');
    await expect(connected).toBeMyProfilePage();
    use(connected);
  },

  userJoinModal: async ({ home }, use) => {
    // cspell:disable-next-line
    const INVITATION_LINK = 'parsec3://parsec.cloud/Test?a=claim_user&p=xBBHJlEjlpxNZYTCvBWWDPIS';

    await home.locator('#create-organization-button').click();
    await expect(home.locator('.homepage-popover')).toBeVisible();
    await home.locator('.homepage-popover').getByRole('listitem').nth(1).click();
    await fillInputModal(home, INVITATION_LINK);
    const modal = home.locator('.join-organization-modal');
    await expect(home.locator('.join-organization-modal')).toBeVisible();
    await use(modal);
  },

  createOrgModal: async ({ home }, use) => {
    await home.locator('#create-organization-button').click();
    await expect(home.locator('.homepage-popover')).toBeVisible();
    await home.locator('.homepage-popover').getByRole('listitem').nth(0).click();
    const modal = home.locator('.create-organization-modal');
    await expect(home.locator('.create-organization-modal')).toBeVisible();
    await use(modal);
  },

  userGreetModal: async ({ connected }, use) => {
    await connected.locator('.topbar').locator('#invitations-button').click();
    const greetButton = connected.locator('.invitations-list-popover ').getByRole('listitem').nth(0).locator('ion-button').nth(2);
    await expect(greetButton).toHaveText('Greet');
    await greetButton.click();
    const modal = connected.locator('.greet-organization-modal');
    await use(modal);
  },

  deviceGreetModal: async ({ connected }, use) => {
    await connected.locator('.topbar').locator('.profile-header').click();
    const myProfileButton = connected.locator('.profile-header-organization-popover').locator('.main-list').getByRole('listitem').nth(1);
    await expect(myProfileButton).toHaveText('My devices');
    await myProfileButton.click();
    await expect(connected).toHavePageTitle('My profile');
    await expect(connected).toBeMyProfilePage();
    await expect(connected.locator('.menu-list__item').nth(1)).toHaveText('My devices');
    await connected.locator('.menu-list__item').nth(1).click();
    await connected.locator('#add-device-button').click();
    const modal = connected.locator('.greet-organization-modal');
    await expect(modal).toBeVisible();
    await use(modal);
  },

  workspaceSharingModal: async ({ workspaces }, use) => {
    await workspaces
      .locator('.workspaces-container-grid')
      .locator('.workspace-card-item')
      .nth(0)
      .locator('.workspace-card-bottom__icons')
      .locator('.icon-share-container')
      .nth(0)
      .click();
    const modal = workspaces.locator('.workspace-sharing-modal');
    await expect(modal).toBeVisible();
    await use(modal);
  },

  clientArea: async ({ home }, use) => {
    home.userData.reset();
    await MockBms.mockLogin(home);
    await MockBms.mockUserRoute(home);
    await MockBms.mockListOrganizations(home);
    await MockBms.mockOrganizationStats(home);
    await MockBms.mockOrganizationStatus(home);
    await MockBms.mockBillingDetails(home);
    await MockBms.mockGetInvoices(home);
    await MockBms.mockCustomOrderRequest(home);

    const button = home.locator('.homepage-header').locator('#trigger-customer-area-button');
    await expect(button).toHaveText('Customer area');
    await button.click();
    const saasContainer = home.locator('.saas-login');
    await expect(saasContainer).toBeVisible();
    await expect(saasContainer.locator('.saas-login__title')).toHaveText('Log in to your customer account');
    const loginButton = saasContainer.locator('.saas-login-button').locator('ion-button');
    await expect(loginButton).toHaveAttribute('disabled');
    await fillIonInput(saasContainer.locator('.input-container').nth(0).locator('ion-input'), DEFAULT_USER_INFORMATION.email);
    await fillIonInput(saasContainer.locator('.input-container').nth(1).locator('ion-input'), DEFAULT_USER_INFORMATION.password);
    await expect(loginButton).toHaveText('Log in');
    await expect(loginButton).toBeTrulyEnabled();
    await loginButton.click();
    await expect(home).toHaveURL(/.+\/clientArea$/);

    // Switch to first org
    const orgSwitchButton = home.locator('.sidebar-header').locator('.card-header-title');
    await orgSwitchButton.click();
    const popover = home.locator('.popover-switch');
    await expect(popover).toBeVisible();
    const orgs = popover.locator('.organization-list').getByRole('listitem');
    await orgs.nth(0).click();
    await expect(orgSwitchButton).toHaveText(home.orgInfo.name);
    await expect(popover).toBeHidden();

    await use(home);
  },

  clientAreaCustomOrder: async ({ home, clientAreaCustomOrderInitialMocks }, use) => {
    home.userData.reset();
    await MockBms.mockLogin(home);
    await MockBms.mockUserRoute(home, { billingSystem: 'CUSTOM_ORDER' });

    const mockKeys: (keyof typeof MockBms)[] = [
      'mockListOrganizations',
      'mockOrganizationStats',
      'mockOrganizationStatus',
      'mockBillingDetails',
      'mockGetInvoices',
      'mockCustomOrderStatus',
      'mockCustomOrderDetails',
      'mockCustomOrderRequest',
      'mockGetCustomOrderInvoices',
    ];

    for (const key of mockKeys) {
      const overload = clientAreaCustomOrderInitialMocks?.[key]?.overload;
      const options = clientAreaCustomOrderInitialMocks?.[key]?.options;
      await MockBms[key](home, overload as any, options);
    }

    const button = home.locator('.homepage-header').locator('#trigger-customer-area-button');
    await expect(button).toHaveText('Customer area');
    await button.click();
    const saasContainer = home.locator('.saas-login');
    await expect(saasContainer).toBeVisible();
    await expect(saasContainer.locator('.saas-login__title')).toHaveText('Log in to your customer account');
    const loginButton = saasContainer.locator('.saas-login-button').locator('ion-button');
    await expect(loginButton).toHaveAttribute('disabled');
    await fillIonInput(saasContainer.locator('.saas-login-content').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
    await fillIonInput(saasContainer.locator('.saas-login-content').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
    await expect(loginButton).not.toHaveAttribute('disabled');
    await loginButton.click();
    await expect(home).toHaveURL(/.+\/clientArea$/);

    await use(home);
  },

  clientAreaCustomOrderInitialMocks: [
    // eslint-disable-next-line no-empty-pattern
    async ({}, use): Promise<void> => {
      await use(undefined);
    },
    { option: true },
  ],

  parsecAccount: async ({ context }, use) => {
    const page = (await context.newPage()) as MsPage;
    await setupNewPage(page, { withParsecAccount: true });
    await expect(page).toHaveURL(/.+\/account$/);
    await use(page);
    await page.release();
  },

  parsecAccountLoggedIn: async ({ context }, use) => {
    const page = (await context.newPage()) as MsPage;
    await setupNewPage(page, { withParsecAccount: true, parsecAccountAutoLogin: true, location: '/home' });
    await expect(page).toHaveURL(/.+\/home$/);
    await use(page);
    await page.release();
  },
});

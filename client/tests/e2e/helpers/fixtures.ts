// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page, test as base } from '@playwright/test';
import { expect } from '@tests/e2e/helpers/assertions';
import { MockBms } from '@tests/e2e/helpers/bms';
import { DEFAULT_ORGANIZATION_INFORMATION, DEFAULT_USER_INFORMATION, UserData } from '@tests/e2e/helpers/data';
import { dropTestbed, newTestbed } from '@tests/e2e/helpers/testbed';
import { fillInputModal, fillIonInput } from '@tests/e2e/helpers/utils';

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
  home: Page;
  connected: Page;
  documents: Page;
  documentsReadOnly: Page;
  usersPage: Page;
  organizationPage: Page;
  myProfilePage: Page;
  userJoinModal: Locator;
  createOrgModal: Locator;
  userGreetModal: Locator;
  deviceGreetModal: Locator;
  workspaceSharingModal: Locator;
  clientArea: Page;
  clientAreaCustomOrder: Page;
}>({
  home: async ({ page, context }, use) => {
    page.on('console', (msg) => console.log('> ', msg.text()));
    await context.grantPermissions(['clipboard-read']);

    await page.addInitScript(() => {
      (window as any).TESTING = true;
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
    });
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');

    await expect(page.locator('#app')).toHaveAttribute('app-state', 'initializing');

    await newTestbed(page);

    if (process.env.PWDEBUG) {
      // Resize the viewport in debug mode to accomodate for the dev tools
      const viewport = page.viewportSize();

      if (viewport) {
        await page.setViewportSize({ width: viewport.width + 400, height: viewport.height });
      }
    }

    await expect(page.locator('#app')).toHaveAttribute('app-state', 'ready');
    await use(page);
    await dropTestbed(page);
  },

  connected: async ({ home }, use) => {
    await home.locator('.organization-card').first().click();
    await expect(home.locator('#password-input')).toBeVisible();

    await expect(home.locator('.login-button')).toHaveDisabledAttribute();

    await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
    await expect(home.locator('.login-button')).toBeEnabled();
    await home.locator('.login-button').click();
    await expect(home).toHaveURL(/\/loading\??.*$/);
    await expect(home.locator('#connected-header')).toContainText('My workspaces');
    await expect(home).toBeWorkspacePage();

    await use(home);
  },

  documents: async ({ connected }, use) => {
    await connected.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    await expect(connected).toHaveHeader(['The Copper Coronet'], true, true);
    await expect(connected).toBeDocumentPage();
    await expect(connected.locator('.folder-container').locator('.no-files-content')).toBeHidden();
    use(connected);
  },

  documentsReadOnly: async ({ connected }, use) => {
    await connected.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(2).click();
    await expect(connected).toHaveHeader(["Watcher's Keep"], true, true);
    await expect(connected).toBeDocumentPage();
    await expect(connected.locator('.folder-container').locator('.no-files-content')).toBeHidden();
    use(connected);
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
      .locator('.list-sidebar-content')
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
    const myProfileButton = connected.locator('.profile-header-popover').locator('.main-list').getByRole('listitem').nth(0);
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
    const myProfileButton = connected.locator('.profile-header-popover').locator('.main-list').getByRole('listitem').nth(1);
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

  workspaceSharingModal: async ({ connected }, use) => {
    await connected
      .locator('.workspaces-container-grid')
      .locator('.workspace-card-item')
      .nth(1)
      .locator('.workspace-card-bottom__icons')
      .locator('.icon-option')
      .nth(0)
      .click();
    const modal = connected.locator('.workspace-sharing-modal');
    await expect(modal).toBeVisible();
    await use(modal);
  },

  clientArea: async ({ home }, use) => {
    UserData.reset();
    await MockBms.mockLogin(home);
    await MockBms.mockUserRoute(home);
    await MockBms.mockListOrganizations(home);
    await MockBms.mockOrganizationStats(home);
    await MockBms.mockOrganizationStatus(home);
    await MockBms.mockBillingDetails(home);
    await MockBms.mockGetInvoices(home);
    await MockBms.mockCreateCustomOrderRequest(home);

    const button = home.locator('.topbar-right').locator('#trigger-customer-area-button');
    await expect(button).toHaveText('Customer area');
    await button.click();
    await fillIonInput(home.locator('.input-container').nth(0).locator('ion-input'), DEFAULT_USER_INFORMATION.email);
    await fillIonInput(home.locator('.input-container').nth(1).locator('ion-input'), DEFAULT_USER_INFORMATION.password);
    await home.locator('.saas-login-button__item').nth(1).click();
    await expect(home).toHaveURL(/.+\/clientArea$/);

    // Switch to first org
    const orgSwitchButton = home.locator('.sidebar-header').locator('.card-header-title');
    await orgSwitchButton.click();
    const popover = home.locator('.popover-switch');
    await expect(popover).toBeVisible();
    const orgs = popover.locator('.organization-list').getByRole('listitem');
    await orgs.nth(0).click();
    await expect(orgSwitchButton).toHaveText(DEFAULT_ORGANIZATION_INFORMATION.name);
    await expect(popover).toBeHidden();

    await use(home);
  },

  clientAreaCustomOrder: async ({ home }, use) => {
    UserData.reset();
    await MockBms.mockLogin(home);
    await MockBms.mockUserRoute(home, { billingSystem: 'CUSTOM_ORDER' });
    await MockBms.mockListOrganizations(home);
    await MockBms.mockOrganizationStats(home);
    await MockBms.mockOrganizationStatus(home);
    await MockBms.mockBillingDetails(home);
    await MockBms.mockGetInvoices(home);
    await MockBms.mockCustomOrderStatus(home);
    await MockBms.mockCustomOrderDetails(home);
    await MockBms.mockCreateCustomOrderRequest(home);
    await MockBms.mockGetCustomOrderInvoices(home);

    const button = home.locator('.topbar-right').locator('#trigger-customer-area-button');
    await expect(button).toHaveText('Customer area');
    await button.click();
    await fillIonInput(home.locator('.input-container').nth(0).locator('ion-input'), DEFAULT_USER_INFORMATION.email);
    await fillIonInput(home.locator('.input-container').nth(1).locator('ion-input'), DEFAULT_USER_INFORMATION.password);
    await home.locator('.saas-login-button__item').nth(1).click();
    await expect(home).toHaveURL(/.+\/clientArea$/);

    await use(home);
  },
});

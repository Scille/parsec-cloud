// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, TestInfo, test as base } from '@playwright/test';
import { expect } from '@tests/e2e/helpers/assertions';
import { MockBms } from '@tests/e2e/helpers/bms';
import { DEFAULT_USER_INFORMATION, generateDefaultOrganizationInformation, generateDefaultUserData } from '@tests/e2e/helpers/data';
import { dropTestbed, initTestBed } from '@tests/e2e/helpers/testbed';
import { MsContext, MsPage } from '@tests/e2e/helpers/types';
import { createWorkspace, fillInputModal, fillIonInput, importDefaultFiles, logout } from '@tests/e2e/helpers/utils';

async function setupNewPage(page: MsPage, testbedPath?: string): Promise<void> {
  page.on('console', (msg) => console.log('> ', msg.text()));

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

  await initTestBed(page, testbedPath);
  page.userData = generateDefaultUserData();
  page.orgInfo = generateDefaultOrganizationInformation();
  page.isReleased = false;
  page.openNewTab = async (): Promise<MsPage> => {
    const newTab = (await page.context().newPage()) as MsPage;
    newTab.skipTestbedRelease = true;
    await setupNewPage(newTab, testbedPath);
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

  if (process.env.PWDEBUG) {
    // Resize the viewport in debug mode to accomodate for the dev tools
    const viewport = page.viewportSize();

    if (viewport) {
      await page.setViewportSize({ width: viewport.width + 400, height: viewport.height });
    }
  }

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
}>({
  context: async ({ browser }, use) => {
    const context = await browser.newContext();
    await context.grantPermissions(['clipboard-read']);
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
    await users.nth(0).locator('.filter-button').click();
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

    const button = home.locator('.topbar-right').locator('#trigger-customer-area-button');
    await expect(button).toHaveText('Customer area');
    await button.click();
    await expect(home.locator('.saas-login')).toBeVisible();
    await expect(home.locator('.saas-login__title')).toHaveText('Log in to your customer account');
    const loginButton = home.locator('.saas-login-button').locator('ion-button');
    await expect(loginButton).toHaveAttribute('disabled');
    await fillIonInput(home.locator('.input-container').nth(0).locator('ion-input'), DEFAULT_USER_INFORMATION.email);
    await home.waitForTimeout(100);
    await fillIonInput(home.locator('.input-container').nth(1).locator('ion-input'), DEFAULT_USER_INFORMATION.password);
    await home.waitForTimeout(100);
    await expect(loginButton).not.toHaveAttribute('disabled');
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

  clientAreaCustomOrder: async ({ home }, use) => {
    home.userData.reset();
    await MockBms.mockLogin(home);
    await MockBms.mockUserRoute(home, { billingSystem: 'CUSTOM_ORDER' });
    await MockBms.mockListOrganizations(home);
    await MockBms.mockOrganizationStats(home);
    await MockBms.mockOrganizationStatus(home);
    await MockBms.mockBillingDetails(home);
    await MockBms.mockGetInvoices(home);
    await MockBms.mockCustomOrderStatus(home);
    await MockBms.mockCustomOrderDetails(home);
    await MockBms.mockCustomOrderRequest(home);
    await MockBms.mockGetCustomOrderInvoices(home);

    const button = home.locator('.topbar-right').locator('#trigger-customer-area-button');
    await expect(button).toHaveText('Customer area');
    await button.click();
    await expect(home.locator('.saas-login')).toBeVisible();
    await expect(home.locator('.saas-login__title')).toHaveText('Log in to your customer account');
    const loginButton = home.locator('.saas-login-button').locator('ion-button');
    await expect(loginButton).toHaveAttribute('disabled');
    await fillIonInput(home.locator('.saas-login-content').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
    await home.waitForTimeout(100);
    await fillIonInput(home.locator('.saas-login-content').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
    await home.waitForTimeout(100);
    await expect(loginButton).not.toHaveAttribute('disabled');
    await loginButton.click();
    await expect(home).toHaveURL(/.+\/clientArea$/);

    await use(home);
  },
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BrowserContext, Locator, Page, TestInfo, test as base } from '@playwright/test';
import { expect } from '@tests/e2e/helpers/assertions';
import { MockBms } from '@tests/e2e/helpers/bms';
import {
  DEFAULT_USER_INFORMATION,
  OrganizationInformation,
  UserData,
  generateDefaultOrganizationInformation,
  generateDefaultUserData,
} from '@tests/e2e/helpers/data';
import { dropTestbed, newTestbed } from '@tests/e2e/helpers/testbed';
import { createWorkspace, dismissToast, dragAndDropFile, fillInputModal, fillIonInput } from '@tests/e2e/helpers/utils';
import path from 'path';

export interface MsPage extends Page {
  userData: UserData;
  orgInfo: OrganizationInformation;
}

export const msTest = base.extend<{
  context: BrowserContext;
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
    await use(context);
    await context.close();
  },

  home: async ({ context }, use) => {
    const page = await context.newPage();

    page.on('console', (msg) => console.log('> ', msg.text()));
    await context.grantPermissions(['clipboard-read']);

    await page.addInitScript(() => {
      (window as any).TESTING = true;
    });
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');

    await expect(page.locator('#app')).toHaveAttribute('app-state', 'initializing');

    await newTestbed(page);
    (page as MsPage).userData = generateDefaultUserData();
    (page as MsPage).orgInfo = generateDefaultOrganizationInformation();

    await expect(page.locator('#app')).toHaveAttribute('app-state', 'ready');
    await use(page as MsPage);
    await dropTestbed(page);
  },

  secondTab: async ({ context }, use) => {
    const page = await context.newPage();

    page.on('console', (msg) => console.log('> ', msg.text()));
    await context.grantPermissions(['clipboard-read']);

    await page.addInitScript(() => {
      (window as any).TESTING = true;
    });
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');

    await expect(page.locator('#app')).toHaveAttribute('app-state', 'initializing');

    await newTestbed(page);
    (page as MsPage).userData = generateDefaultUserData();
    (page as MsPage).orgInfo = generateDefaultOrganizationInformation();

    await expect(page.locator('#app')).toHaveAttribute('app-state', 'ready');
    await use(page as MsPage);
    await dropTestbed(page);
  },

  connected: async ({ home }, use) => {
    await home.locator('.organization-card').first().click();
    await expect(home.locator('#password-input')).toBeVisible();

    await expect(home.locator('.login-button')).toHaveDisabledAttribute();

    await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
    await expect(home.locator('.login-button')).toBeEnabled();
    await home.locator('.login-button').click();
    await expect(home.locator('.loading-container')).toBeVisible();
    await expect(home).toHaveURL(/\/loading\??.*$/);
    await expect(home.locator('#connected-header')).toContainText('My workspaces');
    await expect(home).toBeWorkspacePage();

    await use(home);
  },

  workspaces: async ({ connected }, use) => {
    await createWorkspace(connected, 'The Copper Coronet');
    await dismissToast(connected);
    await createWorkspace(connected, 'Trademeet');
    await dismissToast(connected);
    await createWorkspace(connected, "Watcher's Keep");
    await dismissToast(connected);
    use(connected);
  },

  documents: async ({ workspaces }, use, testInfo: TestInfo) => {
    await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    await expect(workspaces).toHaveHeader(['The Copper Coronet'], true, true);
    await expect(workspaces).toBeDocumentPage();
    await expect(workspaces.locator('.folder-container').locator('.no-files')).toBeVisible();
    // Also create a folder here when available
    const dropZone = workspaces.locator('.folder-container').locator('.drop-zone').nth(0);
    await dragAndDropFile(workspaces, dropZone, [
      path.join(testInfo.config.rootDir, 'data', 'imports', 'image.png'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'document.docx'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'pdfDocument.pdf'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'video.mp4'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'audio.mp3'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'spreadsheet.xlsx'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'text.txt'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'code.py'),
    ]);
    // Hide the import menu
    await workspaces.locator('.upload-menu').locator('.menu-header-icons').locator('ion-icon').nth(1).click();
    use(workspaces);
  },

  documentsReadOnly: async ({ workspaces }, use) => {
    await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(2).click();
    await expect(workspaces).toHaveHeader(["Watcher's Keep"], true, true);
    await expect(workspaces).toBeDocumentPage();
    await expect(workspaces.locator('.folder-container').locator('.no-files-content')).toBeHidden();
    use(workspaces);
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
    await expect(myProfileButton).toHaveText('My profile');
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
    const myProfileButton = connected.locator('.profile-header-popover').locator('.main-list').getByRole('listitem').nth(0);
    await expect(myProfileButton).toHaveText('My profile');
    await myProfileButton.click();
    await expect(connected).toHavePageTitle('My profile');
    await expect(connected).toBeMyProfilePage();
    await connected.locator('.devices-header-button').click();
    const modal = connected.locator('.greet-organization-modal');
    await expect(modal).toBeVisible();
    await use(modal);
  },

  workspaceSharingModal: async ({ workspaces }, use) => {
    await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(1).locator('.shared-group').click();
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
    await MockBms.mockCreateCustomOrderRequest(home);

    const button = home.locator('.topbar-buttons').locator('#trigger-customer-area-button');
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
    await MockBms.mockCreateCustomOrderRequest(home);

    const button = home.locator('.topbar-buttons').locator('#trigger-customer-area-button');
    await expect(button).toHaveText('Customer area');
    await button.click();
    await fillIonInput(home.locator('.input-container').nth(0).locator('ion-input'), DEFAULT_USER_INFORMATION.email);
    await fillIonInput(home.locator('.input-container').nth(1).locator('ion-input'), DEFAULT_USER_INFORMATION.password);
    await home.locator('.saas-login-button__item').nth(1).click();
    await expect(home).toHaveURL(/.+\/clientArea$/);

    await use(home);
  },
});

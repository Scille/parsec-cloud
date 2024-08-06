// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page, test as base } from '@playwright/test';
import { expect } from '@tests/pw/helpers/assertions';
import { MockBms } from '@tests/pw/helpers/bms';
import { DEFAULT_USER_INFORMATION } from '@tests/pw/helpers/data';
import { dropTestbed, newTestbed } from '@tests/pw/helpers/testbed';
import { fillInputModal, fillIonInput } from '@tests/pw/helpers/utils';

export const msTest = base.extend<{
  home: Page;
  connected: Page;
  documents: Page;
  documentsReadOnly: Page;
  usersPage: Page;
  myProfilePage: Page;
  userJoinModal: Locator;
  createOrgModal: Locator;
  userGreetModal: Locator;
  workspaceSharingModal: Locator;
  clientArea: Page;
}>({
  home: async ({ page, context }, use) => {
    page.on('console', (msg) => console.log('> ', msg.text()));
    await context.grantPermissions(['clipboard-read']);

    await page.addInitScript(() => {
      (window as any).TESTING = true;
    });
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');

    await expect(page.locator('#app')).toHaveAttribute('app-state', 'initializing');

    await newTestbed(page);

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
    await expect(home.locator('#connected-header')).toContainText('My workspaces');
    await expect(home).toBeWorkspacePage();

    await use(home);
  },

  documents: async ({ connected }, use) => {
    await connected.locator('.workspaces-container-grid').locator('.workspaces-grid-item').nth(0).click();
    await expect(connected).toHaveHeader(['The Copper Coronet'], true, true);
    await expect(connected).toBeDocumentPage();
    use(connected);
  },

  documentsReadOnly: async ({ connected }, use) => {
    await connected.locator('.workspaces-container-grid').locator('.workspaces-grid-item').nth(2).click();
    await expect(connected).toHaveHeader(["Watcher's Keep"], true, true);
    await expect(connected).toBeDocumentPage();
    use(connected);
  },

  usersPage: async ({ connected }, use) => {
    await connected.locator('.sidebar').locator('.organization-card-manageBtn').click();
    await expect(connected).toHavePageTitle('Users');
    await expect(connected).toBeUserPage();
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

  workspaceSharingModal: async ({ connected }, use) => {
    await connected.locator('.workspaces-container-grid').locator('.workspaces-grid-item').nth(1).locator('.shared-group').click();
    const modal = connected.locator('.workspace-sharing-modal');
    await expect(modal).toBeVisible();
    await use(modal);
  },

  clientArea: async ({ home }, use) => {
    await MockBms.mockLogin(home);
    await MockBms.mockUserInfo(home);
    await MockBms.mockListOrganizations(home);
    await MockBms.mockOrganizationStats(home);
    await MockBms.mockOrganizationStatus(home);
    await MockBms.mockBillingDetails(home);
    await MockBms.mockGetInvoices(home, 2);

    const button = home.locator('.topbar-right-buttons').locator('ion-button').nth(2);
    await expect(button).toHaveText('Customer Area');
    await button.click();
    await expect(home).toHaveURL(/.+\/clientLogin$/);
    await fillIonInput(home.locator('.input-container').nth(0).locator('ion-input'), DEFAULT_USER_INFORMATION.email);
    await fillIonInput(home.locator('.input-container').nth(1).locator('ion-input'), DEFAULT_USER_INFORMATION.password);
    await home.locator('.login-button').locator('ion-button').click();
    await expect(home).toHaveURL(/.+\/clientArea$/);
    await use(home);
  },
});

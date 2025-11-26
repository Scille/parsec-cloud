// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page, TestInfo } from '@playwright/test';
import {
  DEFAULT_USER_INFORMATION,
  DisplaySize,
  expect,
  fillInputModal,
  fillIonInput,
  getServerAddr,
  getTestbedBootstrapAddr,
  mockLibParsec,
  MsPage,
  msTest,
  setupNewPage,
} from '@tests/e2e/helpers';
import { randomInt } from 'crypto';
import path from 'path';

async function openCreateOrganizationModal(page: MsPage): Promise<Locator> {
  await page.locator('#create-organization-button').click();
  const smallDisplay = (await page.getDisplaySize()) === DisplaySize.Small;

  if (smallDisplay) {
    await page.locator('.create-join-modal-list__item').nth(0).click();
  } else {
    await page.locator('.popover-viewport').getByRole('listitem').nth(0).click();
  }
  const modal = page.locator('.create-organization-modal');
  await modal.locator('.server-page-footer').locator('ion-button').nth(0).click();
  return modal;
}

async function cancelAndResume(page: Page, currentContainer: Locator): Promise<void> {
  await expect(currentContainer.locator('.closeBtn')).toBeVisible();
  await currentContainer.locator('.closeBtn').click();
  await expect(page.locator('.create-organization-modal')).toBeHidden();
  await expect(page.locator('.question-modal')).toBeVisible();
  await page.locator('.question-modal').locator('#cancel-button').click();
  await expect(page.locator('.question-modal')).toBeHidden();
  await expect(page.locator('.create-organization-modal')).toBeVisible();
}

msTest('Go through custom org creation process', { tag: '@important' }, async ({ home }) => {
  const modal = await openCreateOrganizationModal(home);

  const uniqueOrgName = `${home.orgInfo.name}-${randomInt(2 ** 47)}`;

  const orgServerContainer = modal.locator('.organization-name-and-server-page');
  await expect(orgServerContainer.locator('.modal-header-title__text')).toHaveText('Create organization on my Parsec server');
  const orgPrevious = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(0);
  const orgNext = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(1);
  await expect(orgPrevious).toBeVisible();
  await expect(orgNext).toHaveDisabledAttribute();
  await fillIonInput(orgServerContainer.locator('ion-input').nth(0), uniqueOrgName);
  await expect(orgNext).toHaveDisabledAttribute();
  await fillIonInput(orgServerContainer.locator('ion-input').nth(1), home.orgInfo.serverAddr);
  await expect(orgNext).toNotHaveDisabledAttribute();

  // Wrong org name
  await fillIonInput(orgServerContainer.locator('ion-input').nth(0), 'Invalid Org N@me');
  await expect(orgNext).toHaveDisabledAttribute();
  const orgNameError = orgServerContainer.locator('#org-name-input').locator('.form-error');
  await expect(orgNameError).toBeVisible();
  await expect(orgNameError).toHaveText('Only letters, digits, underscores and hyphens. No spaces.');

  // Correct org name again
  await fillIonInput(orgServerContainer.locator('ion-input').nth(0), uniqueOrgName);
  await expect(orgNext).toNotHaveDisabledAttribute();
  await expect(orgNameError).toBeHidden();

  // Now wrong server address
  await fillIonInput(orgServerContainer.locator('ion-input').nth(1), 'parsec.cloud');
  await expect(orgNext).toHaveDisabledAttribute();
  const orgServerError = orgServerContainer.locator('#server-addr-input').locator('.form-error');
  await expect(orgServerError).toBeVisible();
  await expect(orgServerError).toHaveText("Link should start with 'parsec3://'.");

  // And correct server address again
  await fillIonInput(orgServerContainer.locator('ion-input').nth(1), home.orgInfo.serverAddr);
  await expect(orgNext).toNotHaveDisabledAttribute();
  await expect(orgServerError).toBeHidden();

  await orgNext.click();

  const userInfoContainer = modal.locator('.user-information-page');
  const userPrevious = modal.locator('.user-information-page-footer').locator('ion-button').nth(0);
  const userNext = modal.locator('.user-information-page-footer').locator('ion-button').nth(1);
  await expect(orgServerContainer).toBeHidden();
  await expect(userInfoContainer).toBeVisible();
  await expect(userPrevious).toBeVisible();
  await expect(userPrevious).toNotHaveDisabledAttribute();
  await expect(userNext).toBeVisible();
  await expect(userNext).toHaveDisabledAttribute();
  await expect(userInfoContainer.locator('.modal-header-title__text')).toHaveText('Enter your personal information');
  await fillIonInput(userInfoContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.name);
  await expect(userNext).toHaveDisabledAttribute();
  await fillIonInput(userInfoContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.email);
  await expect(userNext).toNotHaveDisabledAttribute();

  // Try cancelling
  await cancelAndResume(home, userInfoContainer);

  // Try with a bad name
  await fillIonInput(userInfoContainer.locator('ion-input').nth(0), '"#"');
  const userNameError = userInfoContainer.locator('.input-container').nth(0).locator('.form-error');
  await expect(userNameError).toBeVisible();
  await expect(userNameError).toHaveText('Name contains invalid characters.');
  await expect(userNext).toHaveDisabledAttribute();

  // And correct name again
  await fillIonInput(userInfoContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.name);
  await expect(userNext).toNotHaveDisabledAttribute();
  await expect(userNameError).toBeHidden();

  // Now with bad email
  await fillIonInput(userInfoContainer.locator('ion-input').nth(1), 'invalid email');
  const userEmailError = userInfoContainer.locator('.input-container').nth(1).locator('.form-error');
  await expect(userEmailError).toBeVisible();
  await expect(userEmailError).toHaveText('Wrong email address format.');
  await expect(userNext).toHaveDisabledAttribute();

  // Correct email again
  await fillIonInput(userInfoContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.email);
  await expect(userNext).toNotHaveDisabledAttribute();
  await expect(userNameError).toBeHidden();

  await userNext.click();

  const authContainer = modal.locator('.authentication-page');
  const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
  const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
  await expect(orgServerContainer).toBeHidden();
  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeVisible();
  await expect(authPrevious).toBeVisible();
  await expect(authPrevious).toNotHaveDisabledAttribute();
  await expect(authNext).toBeVisible();
  await expect(authNext).toHaveDisabledAttribute();

  const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
  await expect(authRadio).toHaveCount(3);
  await expect(authRadio.nth(0)).toHaveTheClass('radio-disabled');
  await expect(authRadio.nth(0).locator('.authentication-card-text__title')).toHaveText('System authentication');
  await expect(authRadio.nth(1)).toHaveText('Password');
  await authRadio.nth(1).click();

  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toHaveDisabledAttribute();
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toNotHaveDisabledAttribute();

  // Try cancelling
  await cancelAndResume(home, authContainer);

  // Password too simple
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), 'EasyP@ssw0rd');
  await expect(authNext).toHaveDisabledAttribute();

  // Back to complicated password
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toNotHaveDisabledAttribute();

  // Check does not match
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), `${DEFAULT_USER_INFORMATION.password}-extra`);
  await expect(authNext).toHaveDisabledAttribute();
  const matchError = authContainer.locator('.choose-password').locator('.inputs-container-item').nth(1).locator('.form-helperText').nth(1);
  await expect(matchError).toBeVisible();
  await expect(matchError).toHaveText('Do not match');

  // Back to matching password
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toNotHaveDisabledAttribute();
  await expect(matchError).toBeHidden();

  await authNext.click();

  const summaryContainer = modal.locator('.summary-page');
  const summaryPrevious = modal.locator('.summary-page-footer').locator('ion-button').nth(0);
  const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
  const summaryEditButtons = modal.locator('.summary-item__button');
  await expect(orgServerContainer).toBeHidden();
  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeHidden();
  await expect(summaryContainer).toBeVisible();
  await expect(summaryContainer.locator('.modal-header-title__text')).toHaveText('Overview of your organization');
  await expect(summaryPrevious).toBeVisible();
  await expect(summaryPrevious).toNotHaveDisabledAttribute();
  await expect(summaryNext).toBeVisible();
  await expect(summaryNext).toNotHaveDisabledAttribute();

  // Everything can be updated
  await expect(summaryEditButtons.nth(0)).toBeVisible();
  await expect(summaryEditButtons.nth(1)).toBeVisible();
  await expect(summaryEditButtons.nth(2)).toBeVisible();
  await expect(summaryEditButtons.nth(3)).toBeVisible();
  await expect(summaryEditButtons.nth(4)).toBeVisible();

  await expect(summaryContainer.locator('.summary-item__label')).toHaveText([
    'Organization',
    'Full name',
    'Email',
    'Server choice',
    'Authentication method',
  ]);
  await expect(summaryContainer.locator('.summary-item__text')).toHaveText([
    uniqueOrgName,
    DEFAULT_USER_INFORMATION.name,
    DEFAULT_USER_INFORMATION.email,
    'Custom Server',
    'Password',
  ]);
  await summaryNext.click();

  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeHidden();
  await expect(summaryContainer).toBeHidden();
  await expect(modal.locator('.creation-page')).toBeVisible();
  await expect(modal.locator('.creation-page').locator('.closeBtn')).toBeHidden();
  await home.waitForTimeout(1000);

  await expect(modal.locator('.created-page')).toBeVisible();
  await expect(modal.locator('.creation-page')).toBeHidden();
  await expect(modal.locator('.created-page').locator('.closeBtn')).toBeHidden();
  await modal.locator('.created-page-footer').locator('ion-button').click();
  await expect(modal).toBeHidden();
  await home.waitForTimeout(1000);
  await expect(home).toBeWorkspacePage();
});

msTest('Go through custom org creation process from bootstrap link', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  // Making sure that the testbed is recognized as the custom server
  await setupNewPage(page, { trialServers: 'unknown.host', saasServers: 'unknown.host' });

  await page.locator('#create-organization-button').click();
  await page.locator('.popover-viewport').getByRole('listitem').nth(1).click();
  const uniqueOrgName = `${page.orgInfo.name}-${randomInt(2 ** 47)}`;
  await fillInputModal(page, getTestbedBootstrapAddr(uniqueOrgName));
  const modal = page.locator('.create-organization-modal');

  const orgServerContainer = modal.locator('.organization-name-and-server-page');
  await expect(orgServerContainer.locator('.modal-header-title__text')).toHaveText('Create organization on my Parsec server');
  const orgPrevious = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(0);
  const orgNext = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(1);
  await expect(orgPrevious).toBeHidden();
  await expect(orgNext).toNotHaveDisabledAttribute();

  await expect(orgServerContainer.locator('ion-input').nth(0)).toHaveTheClass('input-disabled');
  await expect(orgServerContainer.locator('ion-input').nth(0).locator('input')).toHaveValue(uniqueOrgName);
  await expect(orgServerContainer.locator('ion-input').nth(1)).toHaveTheClass('input-disabled');
  await expect(orgServerContainer.locator('ion-input').nth(1).locator('input')).toHaveValue(getServerAddr());
  await orgNext.click();

  const userInfoContainer = modal.locator('.user-information-page');
  const userPrevious = modal.locator('.user-information-page-footer').locator('ion-button').nth(0);
  const userNext = modal.locator('.user-information-page-footer').locator('ion-button').nth(1);
  await expect(orgServerContainer).toBeHidden();
  await expect(userInfoContainer).toBeVisible();
  await expect(userPrevious).toBeVisible();
  await expect(userPrevious).toNotHaveDisabledAttribute();
  await expect(userNext).toBeVisible();
  await expect(userNext).toHaveDisabledAttribute();
  await expect(userInfoContainer.locator('.modal-header-title__text')).toHaveText('Enter your personal information');
  await fillIonInput(userInfoContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.name);
  await expect(userNext).toHaveDisabledAttribute();
  await fillIonInput(userInfoContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.email);
  await expect(userNext).toNotHaveDisabledAttribute();
  await userNext.click();

  const authContainer = modal.locator('.authentication-page');
  const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
  const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
  await expect(orgServerContainer).toBeHidden();
  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeVisible();
  await expect(authPrevious).toBeVisible();
  await expect(authPrevious).toNotHaveDisabledAttribute();
  await expect(authNext).toBeVisible();
  await expect(authNext).toHaveDisabledAttribute();

  const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
  await expect(authRadio).toHaveCount(3);
  await expect(authRadio.nth(0)).toHaveTheClass('radio-disabled');
  await expect(authRadio.nth(0).locator('.authentication-card-text__title')).toHaveText('System authentication');
  await expect(authRadio.nth(1)).toHaveText('Password');
  await authRadio.nth(1).click();

  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toHaveDisabledAttribute();
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toNotHaveDisabledAttribute();
  await authNext.click();

  const summaryContainer = modal.locator('.summary-page');
  const summaryPrevious = modal.locator('.summary-page-footer').locator('ion-button').nth(0);
  const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
  const summaryEditButtons = modal.locator('.summary-item__button');
  await expect(orgServerContainer).toBeHidden();
  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeHidden();
  await expect(summaryContainer).toBeVisible();
  await expect(summaryContainer.locator('.modal-header-title__text')).toHaveText('Overview of your organization');
  await expect(summaryPrevious).toBeVisible();
  await expect(summaryPrevious).toNotHaveDisabledAttribute();
  await expect(summaryNext).toBeVisible();
  await expect(summaryNext).toNotHaveDisabledAttribute();

  // Name, email and authentication fields can be updated, server & org name cannot
  await expect(summaryEditButtons.nth(0)).not.toBeVisible();
  await expect(summaryEditButtons.nth(1)).toBeVisible();
  await expect(summaryEditButtons.nth(2)).toBeVisible();
  await expect(summaryEditButtons.nth(3)).not.toBeVisible();
  await expect(summaryEditButtons.nth(4)).toBeVisible();

  await expect(summaryContainer.locator('.summary-item__label')).toHaveText([
    'Organization',
    'Full name',
    'Email',
    'Server choice',
    'Authentication method',
  ]);
  await expect(summaryContainer.locator('.summary-item__text')).toHaveText([
    uniqueOrgName,
    DEFAULT_USER_INFORMATION.name,
    DEFAULT_USER_INFORMATION.email,
    'Custom Server',
    'Password',
  ]);
  await summaryNext.click();
  await page.waitForTimeout(1000);
  await expect(summaryContainer).toBeHidden();
  await expect(modal.locator('.created-page')).toBeVisible();
  await expect(modal.locator('.created-page-footer').locator('ion-button')).toHaveText('Access my organization');
  await modal.locator('.created-page-footer').locator('ion-button').click();
  await page.waitForTimeout(1000);
  await expect(page).toBeWorkspacePage();
  await page.release();
});

for (const displaySize of ['small', 'large']) {
  msTest(`Go through custom org creation process with authority key on ${displaySize} display`, async ({ home }, testInfo: TestInfo) => {
    if (displaySize === DisplaySize.Small) {
      await home.setDisplaySize(DisplaySize.Small);
    }
    const modal = await openCreateOrganizationModal(home);

    const uniqueOrgName = `${home.orgInfo.name}-${randomInt(2 ** 47)}`;

    const orgServerContainer = modal.locator('.organization-name-and-server-page');
    await expect(orgServerContainer.locator('.modal-header-title__text')).toHaveText('Create organization on my Parsec server');
    const orgPrevious = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(0);
    const orgNext = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(1);
    await expect(orgPrevious).toBeVisible();
    await expect(orgNext).toHaveDisabledAttribute();
    await fillIonInput(orgServerContainer.locator('ion-input').nth(0), uniqueOrgName);
    await expect(orgNext).toHaveDisabledAttribute();
    await fillIonInput(orgServerContainer.locator('ion-input').nth(1), home.orgInfo.serverAddr);
    await expect(orgNext).toNotHaveDisabledAttribute();

    await expect(orgServerContainer.locator('.advanced-settings')).toHaveText('Advanced Settings');
    await expect(orgServerContainer.locator('.sequester-container')).toBeHidden();
    await orgServerContainer.locator('.advanced-settings').click();
    await expect(orgServerContainer.locator('.sequester-container')).toBeVisible();

    await orgServerContainer.locator('.sequester-container').locator('ion-toggle').click();
    await expect(orgNext).toBeTrulyDisabled();
    await expect(orgServerContainer.locator('.sequester-container').locator('.upload-key__button')).toBeVisible();
    await expect(orgServerContainer.locator('.sequester-container').locator('.upload-key__button')).toHaveText('Add authority key');
    await expect(orgServerContainer.locator('.sequester-container').locator('.upload-key-update')).toBeHidden();

    const fileChooserPromise = home.waitForEvent('filechooser');
    await orgServerContainer.locator('.sequester-container').locator('.upload-key__button').click();
    const fileChooser = await fileChooserPromise;
    expect(fileChooser.isMultiple()).toBe(false);
    const importPath = path.join(testInfo.config.rootDir, 'data', 'public_key.pem');
    await fileChooser.setFiles([importPath]);

    await expect(orgServerContainer.locator('.sequester-container').locator('.upload-key__button')).toBeHidden();
    await expect(orgServerContainer.locator('.sequester-container').locator('.upload-key-update')).toBeVisible();
    await expect(orgServerContainer.locator('.sequester-container').locator('.upload-key-update').locator('ion-text')).toHaveText(
      'public_key.pem',
    );
    await expect(
      orgServerContainer.locator('.sequester-container').locator('.upload-key-update').locator('.upload-key-update__button'),
    ).toHaveText('Update');

    await expect(orgNext).toBeTrulyEnabled();

    await orgNext.click();

    const userInfoContainer = modal.locator('.user-information-page');
    const userPrevious = modal.locator('.user-information-page-footer').locator('ion-button').nth(0);
    const userNext = modal.locator('.user-information-page-footer').locator('ion-button').nth(1);
    await expect(orgServerContainer).toBeHidden();
    await expect(userInfoContainer).toBeVisible();
    await expect(userPrevious).toBeVisible();
    await expect(userPrevious).toNotHaveDisabledAttribute();
    await expect(userNext).toBeVisible();
    await expect(userNext).toHaveDisabledAttribute();
    await expect(userInfoContainer.locator('.modal-header-title__text')).toHaveText('Enter your personal information');
    await fillIonInput(userInfoContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.name);
    await expect(userNext).toHaveDisabledAttribute();
    await fillIonInput(userInfoContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.email);
    await expect(userNext).toNotHaveDisabledAttribute();

    await userNext.click();

    const authContainer = modal.locator('.authentication-page');
    const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
    const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
    await expect(orgServerContainer).toBeHidden();
    await expect(userInfoContainer).toBeHidden();
    await expect(authContainer).toBeVisible();
    await expect(authPrevious).toBeVisible();
    await expect(authPrevious).toNotHaveDisabledAttribute();
    await expect(authNext).toBeVisible();
    await expect(authNext).toHaveDisabledAttribute();

    const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
    await expect(authRadio).toHaveCount(3);
    await expect(authRadio.nth(0)).toHaveTheClass('radio-disabled');
    await expect(authRadio.nth(0).locator('.authentication-card-text__title')).toHaveText('System authentication');
    await expect(authRadio.nth(1)).toHaveText('Password');
    await authRadio.nth(1).click();
    await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
    await expect(authNext).toHaveDisabledAttribute();
    await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
    await expect(authNext).toNotHaveDisabledAttribute();
    await authNext.click();

    const summaryContainer = modal.locator('.summary-page');
    const summaryPrevious = modal.locator('.summary-page-footer').locator('ion-button').nth(0);
    const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
    const summaryEditButtons = modal.locator('.summary-item__button');
    await expect(orgServerContainer).toBeHidden();
    await expect(userInfoContainer).toBeHidden();
    await expect(authContainer).toBeHidden();
    await expect(summaryContainer).toBeVisible();
    await expect(summaryContainer.locator('.modal-header-title__text')).toHaveText('Overview of your organization');
    await expect(summaryPrevious).toBeVisible();
    await expect(summaryPrevious).toNotHaveDisabledAttribute();
    await expect(summaryNext).toBeVisible();
    await expect(summaryNext).toNotHaveDisabledAttribute();

    // Everything can be updated
    await expect(summaryEditButtons.nth(0)).toBeVisible();
    await expect(summaryEditButtons.nth(1)).toBeVisible();
    await expect(summaryEditButtons.nth(2)).toBeVisible();
    await expect(summaryEditButtons.nth(3)).toBeVisible();
    await expect(summaryEditButtons.nth(4)).toBeVisible();
    await expect(summaryEditButtons.nth(5)).toBeVisible();

    await expect(summaryContainer.locator('.summary-item__label')).toHaveText([
      'Organization',
      'Data sequester',
      'Full name',
      'Email',
      'Server choice',
      'Authentication method',
    ]);
    await expect(summaryContainer.locator('.summary-item__text')).toHaveText([
      uniqueOrgName,
      'Authority key added',
      DEFAULT_USER_INFORMATION.name,
      DEFAULT_USER_INFORMATION.email,
      'Custom Server',
      'Password',
    ]);
    await summaryNext.click();

    await expect(userInfoContainer).toBeHidden();
    await expect(authContainer).toBeHidden();
    await expect(summaryContainer).toBeHidden();
    await expect(modal.locator('.creation-page')).toBeVisible();
    await expect(modal.locator('.creation-page').locator('.closeBtn')).toBeHidden();
    await home.waitForTimeout(1000);

    await expect(modal.locator('.created-page')).toBeVisible();
    await expect(modal.locator('.creation-page')).toBeHidden();
    await expect(modal.locator('.created-page').locator('.closeBtn')).toBeHidden();
    await modal.locator('.created-page-footer').locator('ion-button').click();
    await expect(modal).toBeHidden();
    await home.waitForTimeout(1000);
    await expect(home).toBeWorkspacePage();
  });

  msTest(
    `Go through custom org creation process with invalid authority key on ${displaySize} display`,
    async ({ home }, testInfo: TestInfo) => {
      if (displaySize === DisplaySize.Small) {
        await home.setDisplaySize(DisplaySize.Small);
      }
      const modal = await openCreateOrganizationModal(home);

      const uniqueOrgName = `${home.orgInfo.name}-${randomInt(2 ** 47)}`;

      const orgServerContainer = modal.locator('.organization-name-and-server-page');
      await expect(orgServerContainer.locator('.modal-header-title__text')).toHaveText('Create organization on my Parsec server');
      const orgPrevious = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(0);
      const orgNext = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(1);
      await expect(orgPrevious).toBeVisible();
      await expect(orgNext).toHaveDisabledAttribute();
      await fillIonInput(orgServerContainer.locator('ion-input').nth(0), uniqueOrgName);
      await expect(orgNext).toHaveDisabledAttribute();
      await fillIonInput(orgServerContainer.locator('ion-input').nth(1), home.orgInfo.serverAddr);
      await expect(orgNext).toNotHaveDisabledAttribute();

      await orgServerContainer.locator('.advanced-settings').click();
      await orgServerContainer.locator('.sequester-container').locator('ion-toggle').click();
      await expect(orgNext).toBeTrulyDisabled();

      const fileChooserPromise = home.waitForEvent('filechooser');
      await orgServerContainer.locator('.sequester-container').locator('.upload-key__button').click();
      const fileChooser = await fileChooserPromise;
      expect(fileChooser.isMultiple()).toBe(false);
      const importPath = path.join(testInfo.config.rootDir, 'data', 'imports', 'text.txt');
      await fileChooser.setFiles([importPath]);

      await expect(orgServerContainer.locator('.sequester-container').locator('.upload-key-update').locator('ion-text')).toHaveText(
        'text.txt',
      );

      await expect(orgNext).toBeTrulyEnabled();

      await orgNext.click();

      const userInfoContainer = modal.locator('.user-information-page');
      const userPrevious = modal.locator('.user-information-page-footer').locator('ion-button').nth(0);
      const userNext = modal.locator('.user-information-page-footer').locator('ion-button').nth(1);
      await expect(orgServerContainer).toBeHidden();
      await expect(userInfoContainer).toBeVisible();
      await expect(userPrevious).toBeVisible();
      await expect(userPrevious).toNotHaveDisabledAttribute();
      await expect(userNext).toBeVisible();
      await expect(userNext).toHaveDisabledAttribute();
      await expect(userInfoContainer.locator('.modal-header-title__text')).toHaveText('Enter your personal information');
      await fillIonInput(userInfoContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.name);
      await expect(userNext).toHaveDisabledAttribute();
      await fillIonInput(userInfoContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.email);
      await expect(userNext).toNotHaveDisabledAttribute();

      await userNext.click();

      const authContainer = modal.locator('.authentication-page');
      const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
      const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
      await expect(orgServerContainer).toBeHidden();
      await expect(userInfoContainer).toBeHidden();
      await expect(authContainer).toBeVisible();
      await expect(authPrevious).toBeVisible();
      await expect(authPrevious).toNotHaveDisabledAttribute();
      await expect(authNext).toBeVisible();
      await expect(authNext).toHaveDisabledAttribute();

      const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
      await expect(authRadio).toHaveCount(3);
      await expect(authRadio.nth(0)).toHaveTheClass('radio-disabled');
      await expect(authRadio.nth(0).locator('.authentication-card-text__title')).toHaveText('System authentication');
      await expect(authRadio.nth(1)).toHaveText('Password');
      await authRadio.nth(1).click();
      await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
      await expect(authNext).toHaveDisabledAttribute();
      await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
      await expect(authNext).toNotHaveDisabledAttribute();
      await authNext.click();

      const summaryContainer = modal.locator('.summary-page');
      const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
      await summaryNext.click();

      await home.waitForTimeout(1000);
      await expect(summaryContainer.locator('.form-error')).toBeVisible();
      await expect(summaryContainer.locator('.form-error')).toHaveText('The provided authority key is not valid.');
    },
  );

  msTest(
    `Go through custom org creation process from bootstrap link with authority key on ${displaySize} display`,
    async ({ context }, testInfo: TestInfo) => {
      const page = (await context.newPage()) as MsPage;
      // Making sure that the testbed is recognized as the custom server
      await setupNewPage(page, { trialServers: 'unknown.host', saasServers: 'unknown.host' });

      if (displaySize === DisplaySize.Small) {
        await page.setDisplaySize(DisplaySize.Small);
      }

      await page.locator('#create-organization-button').click();
      if (displaySize === DisplaySize.Small) {
        await page.locator('.create-join-modal-list__item').nth(1).click();
      } else {
        await page.locator('.popover-viewport').getByRole('listitem').nth(1).click();
      }
      const uniqueOrgName = `${page.orgInfo.name}-${randomInt(2 ** 47)}`;
      await fillInputModal(page, getTestbedBootstrapAddr(uniqueOrgName));
      const modal = page.locator('.create-organization-modal');

      const orgServerContainer = modal.locator('.organization-name-and-server-page');
      await expect(orgServerContainer.locator('.modal-header-title__text')).toHaveText('Create organization on my Parsec server');
      const orgPrevious = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(0);
      const orgNext = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(1);
      await expect(orgPrevious).toBeHidden();
      await expect(orgNext).toNotHaveDisabledAttribute();

      await expect(orgServerContainer.locator('ion-input').nth(0)).toHaveTheClass('input-disabled');
      await expect(orgServerContainer.locator('ion-input').nth(0).locator('input')).toHaveValue(uniqueOrgName);
      await expect(orgServerContainer.locator('ion-input').nth(1)).toHaveTheClass('input-disabled');
      await expect(orgServerContainer.locator('ion-input').nth(1).locator('input')).toHaveValue(getServerAddr());

      await orgServerContainer.locator('.advanced-settings').click();
      await orgServerContainer.locator('.sequester-container').locator('ion-toggle').click();
      await expect(orgNext).toBeTrulyDisabled();

      const fileChooserPromise = page.waitForEvent('filechooser');
      await orgServerContainer.locator('.sequester-container').locator('.upload-key__button').click();
      const fileChooser = await fileChooserPromise;
      expect(fileChooser.isMultiple()).toBe(false);
      const importPath = path.join(testInfo.config.rootDir, 'data', 'public_key.pem');
      await fileChooser.setFiles([importPath]);

      await expect(orgServerContainer.locator('.sequester-container').locator('.upload-key-update').locator('ion-text')).toHaveText(
        'public_key.pem',
      );

      await expect(orgNext).toBeTrulyEnabled();

      await orgNext.click();

      const userInfoContainer = modal.locator('.user-information-page');
      const userPrevious = modal.locator('.user-information-page-footer').locator('ion-button').nth(0);
      const userNext = modal.locator('.user-information-page-footer').locator('ion-button').nth(1);
      await expect(orgServerContainer).toBeHidden();
      await expect(userInfoContainer).toBeVisible();
      await expect(userPrevious).toBeVisible();
      await expect(userPrevious).toNotHaveDisabledAttribute();
      await expect(userNext).toBeVisible();
      await expect(userNext).toHaveDisabledAttribute();
      await expect(userInfoContainer.locator('.modal-header-title__text')).toHaveText('Enter your personal information');
      await fillIonInput(userInfoContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.name);
      await expect(userNext).toHaveDisabledAttribute();
      await fillIonInput(userInfoContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.email);
      await expect(userNext).toNotHaveDisabledAttribute();
      await userNext.click();

      const authContainer = modal.locator('.authentication-page');
      const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
      const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
      await expect(orgServerContainer).toBeHidden();
      await expect(userInfoContainer).toBeHidden();
      await expect(authContainer).toBeVisible();
      await expect(authPrevious).toBeVisible();
      await expect(authPrevious).toNotHaveDisabledAttribute();
      await expect(authNext).toBeVisible();
      await expect(authNext).toHaveDisabledAttribute();

      const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
      await expect(authRadio).toHaveCount(3);
      await expect(authRadio.nth(0)).toHaveTheClass('radio-disabled');
      await expect(authRadio.nth(0).locator('.authentication-card-text__title')).toHaveText('System authentication');
      await expect(authRadio.nth(1)).toHaveText('Password');
      await authRadio.nth(1).click();
      await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
      await expect(authNext).toHaveDisabledAttribute();
      await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
      await expect(authNext).toNotHaveDisabledAttribute();
      await authNext.click();

      const summaryContainer = modal.locator('.summary-page');
      const summaryPrevious = modal.locator('.summary-page-footer').locator('ion-button').nth(0);
      const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
      const summaryEditButtons = modal.locator('.summary-item__button');
      await expect(orgServerContainer).toBeHidden();
      await expect(userInfoContainer).toBeHidden();
      await expect(authContainer).toBeHidden();
      await expect(summaryContainer).toBeVisible();
      await expect(summaryContainer.locator('.modal-header-title__text')).toHaveText('Overview of your organization');
      await expect(summaryPrevious).toBeVisible();
      await expect(summaryPrevious).toNotHaveDisabledAttribute();
      await expect(summaryNext).toBeVisible();
      await expect(summaryNext).toNotHaveDisabledAttribute();

      // Name, email, authority key and authentication fields can be updated, server & org name cannot
      await expect(summaryEditButtons.nth(0)).not.toBeVisible();
      await expect(summaryEditButtons.nth(1)).toBeVisible();
      await expect(summaryEditButtons.nth(2)).toBeVisible();
      await expect(summaryEditButtons.nth(3)).toBeVisible();
      await expect(summaryEditButtons.nth(4)).not.toBeVisible();
      await expect(summaryEditButtons.nth(5)).toBeVisible();

      await expect(summaryContainer.locator('.summary-item__label')).toHaveText([
        'Organization',
        'Data sequester',
        'Full name',
        'Email',
        'Server choice',
        'Authentication method',
      ]);
      await expect(summaryContainer.locator('.summary-item__text')).toHaveText([
        uniqueOrgName,
        'Authority key added',
        DEFAULT_USER_INFORMATION.name,
        DEFAULT_USER_INFORMATION.email,
        'Custom Server',
        'Password',
      ]);
      await summaryNext.click();
      await page.waitForTimeout(1000);
      await expect(summaryContainer).toBeHidden();
      await expect(modal.locator('.created-page')).toBeVisible();
      await expect(modal.locator('.created-page-footer').locator('ion-button')).toHaveText('Access my organization');
      await modal.locator('.created-page-footer').locator('ion-button').click();
      await page.waitForTimeout(1000);
      await expect(page).toBeWorkspacePage();
      await page.release();
    },
  );
}

msTest.skip('Go through custom org creation process with smartcard auth', async ({ home }) => {
  await mockLibParsec(home, [
    {
      name: 'showCertificateSelectionDialogWindowsOnly',
      result: {
        ok: true,
        value: {
          hash: 'ABCD',
          uris: [{ tag: 'X509URIFlavorValueWindowsCNG', x1: new Uint8Array([1, 2, 3, 4, 5, 6, 7, 8]) }],
        },
      },
    },
  ]);
  const modal = await openCreateOrganizationModal(home);

  const uniqueOrgName = `${home.orgInfo.name}-${randomInt(2 ** 47)}`;

  const orgServerContainer = modal.locator('.organization-name-and-server-page');
  await expect(orgServerContainer.locator('.modal-header-title__text')).toHaveText('Create organization on my Parsec server');
  const orgPrevious = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(0);
  const orgNext = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(1);
  await expect(orgPrevious).toBeVisible();
  await expect(orgNext).toHaveDisabledAttribute();
  await fillIonInput(orgServerContainer.locator('ion-input').nth(0), uniqueOrgName);
  await expect(orgNext).toHaveDisabledAttribute();
  await fillIonInput(orgServerContainer.locator('ion-input').nth(1), home.orgInfo.serverAddr);
  await expect(orgNext).toNotHaveDisabledAttribute();
  await orgNext.click();

  const userInfoContainer = modal.locator('.user-information-page');
  const userPrevious = modal.locator('.user-information-page-footer').locator('ion-button').nth(0);
  const userNext = modal.locator('.user-information-page-footer').locator('ion-button').nth(1);
  await expect(orgServerContainer).toBeHidden();
  await expect(userInfoContainer).toBeVisible();
  await expect(userPrevious).toBeVisible();
  await expect(userPrevious).toNotHaveDisabledAttribute();
  await expect(userNext).toBeVisible();
  await expect(userNext).toHaveDisabledAttribute();
  await expect(userInfoContainer.locator('.modal-header-title__text')).toHaveText('Enter your personal information');
  await fillIonInput(userInfoContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.name);
  await expect(userNext).toHaveDisabledAttribute();
  await fillIonInput(userInfoContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.email);
  await expect(userNext).toNotHaveDisabledAttribute();

  await userNext.click();

  const authContainer = modal.locator('.authentication-page');
  const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
  const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
  await expect(orgServerContainer).toBeHidden();
  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeVisible();
  await expect(authPrevious).toBeVisible();
  await expect(authPrevious).toNotHaveDisabledAttribute();
  await expect(authNext).toBeVisible();
  await expect(authNext).toHaveDisabledAttribute();

  const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
  await expect(authRadio).toHaveCount(3);
  await expect(authRadio.nth(0)).toHaveTheClass('radio-disabled');
  await expect(authRadio.nth(0).locator('.authentication-card-text__title')).toHaveText('System authentication');
  await expect(authRadio.nth(1)).toHaveText('Password');
  await expect(authRadio.nth(2).locator('.authentication-card-text__title')).toHaveText('Smartcard');
  const certBtn = authContainer.locator('.choose-certificate-button');
  await expect(certBtn).toBeHidden();
  await authRadio.nth(2).click();
  await expect(certBtn).toBeVisible();
  await expect(certBtn).toHaveText('Choose a certificate');
  await expect(authContainer.locator('.choose-certificate-selected__text')).toBeHidden();
  await certBtn.click();
  await expect(authContainer.locator('.choose-certificate-selected__text')).toBeVisible();
  await expect(authContainer.locator('.choose-certificate-selected__text')).toHaveText('Certificate selected');
  await expect(certBtn).toHaveText('Update');

  await expect(authNext).toNotHaveDisabledAttribute();
  await authNext.click();

  const summaryContainer = modal.locator('.summary-page');
  const summaryPrevious = modal.locator('.summary-page-footer').locator('ion-button').nth(0);
  const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
  const summaryEditButtons = modal.locator('.summary-item__button');
  await expect(orgServerContainer).toBeHidden();
  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeHidden();
  await expect(summaryContainer).toBeVisible();
  await expect(summaryContainer.locator('.modal-header-title__text')).toHaveText('Overview of your organization');
  await expect(summaryPrevious).toBeVisible();
  await expect(summaryPrevious).toNotHaveDisabledAttribute();
  await expect(summaryNext).toBeVisible();
  await expect(summaryNext).toNotHaveDisabledAttribute();

  // Everything can be updated
  await expect(summaryEditButtons.nth(0)).toBeVisible();
  await expect(summaryEditButtons.nth(1)).toBeVisible();
  await expect(summaryEditButtons.nth(2)).toBeVisible();
  await expect(summaryEditButtons.nth(3)).toBeVisible();
  await expect(summaryEditButtons.nth(4)).toBeVisible();

  await expect(summaryContainer.locator('.summary-item__label')).toHaveText([
    'Organization',

    'Full name',
    'Email',
    'Server choice',
    'Authentication method',
  ]);
  await expect(summaryContainer.locator('.summary-item__text')).toHaveText([
    uniqueOrgName,
    DEFAULT_USER_INFORMATION.name,
    DEFAULT_USER_INFORMATION.email,
    'Custom Server',
    'Certificate',
  ]);
});

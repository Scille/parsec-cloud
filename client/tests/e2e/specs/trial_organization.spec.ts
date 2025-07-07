// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, logout, MsPage, msTest, openExternalLink, setupNewPage } from '@tests/e2e/helpers';

msTest('Homepage with trial orgs', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  // Making sure that the testbed is recognized as the trial server
  const testbedUrl = new URL(process.env.TESTBED_SERVER ?? '');
  await setupNewPage(page, { trialServers: testbedUrl.host, saasServers: 'unknown.host' });

  const topBar = page.locator('.topbar');
  await expect(topBar.locator('.topbar-left-text__title')).toHaveText('Welcome to Parsec');
  await expect(topBar.locator('.topbar-left-text__subtitle')).toHaveText('Access your organizations');

  await page.release();
});

msTest('Sidebar with trial orgs', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  // Making sure that the testbed is recognized as the trial server
  const testbedUrl = new URL(process.env.TESTBED_SERVER ?? '');
  await setupNewPage(page, { trialServers: testbedUrl.host, saasServers: 'unknown.host' });

  const topBar = page.locator('.topbar');
  await expect(topBar.locator('.topbar-left-text__title')).toHaveText('Welcome to Parsec');

  await page.locator('.organization-card').first().click();
  await expect(page.locator('#password-input')).toBeVisible();

  await expect(page.locator('.login-button')).toHaveDisabledAttribute();

  await page.locator('#password-input').locator('input').fill('P@ssw0rd.');
  await expect(page.locator('.login-button')).toBeEnabled();
  await page.locator('.login-button').click();
  await expect(page.locator('#connected-header')).toContainText('My workspaces');
  await expect(page.locator('.topbar-right').locator('.text-content-name')).toHaveText('Alicey McAliceFace');
  await expect(page).toBeWorkspacePage();

  const trialMenu = page.locator('.sidebar').locator('.trial-card');
  await expect(trialMenu).toBeVisible();
  await expect(trialMenu.locator('.trial-card__tag')).toHaveText('Trial version');
  await expect(trialMenu.locator('.trial-card-text__time')).toHaveText('Expired');
  await expect(trialMenu.locator('.trial-card__button')).toHaveText('Subscribe now');
  await openExternalLink(page, trialMenu.locator('.trial-card__button'), /^https:\/\/parsec\.cloud\/en\/pricing\/?.*$/);

  await logout(page);
  await expect(topBar.locator('.topbar-left-text__title')).toHaveText('Welcome to Parsec');

  await expect(page.locator('.organization-card').first().locator('.organization-card-expiration')).toBeVisible();
  await expect(page.locator('.organization-card').first().locator('.organization-card-expiration')).toHaveText('Expired');
  await page.locator('.organization-card').first().click();

  await answerQuestion(page, false, {
    expectedTitleText: 'Expired organization',
    expectedQuestionText: "This device belongs to an expired organization. Do you wish to archive it (it won't be visible anymore)?",
    expectedPositiveText: 'Archive',
    expectedNegativeText: 'No, proceed to login',
  });

  await page.release();
});

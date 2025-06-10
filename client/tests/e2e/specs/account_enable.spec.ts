// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, MsPage, msTest, setupNewPage } from '@tests/e2e/helpers';

msTest('Default URL with account enabled', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  await setupNewPage(page, { withParsecAccount: true, location: '/' });
  await expect(page).toHaveURL(/.+\/account$/);
  await expect(page.locator('.account-login-container')).toBeVisible();
  await expect(page.locator('.organization-content')).toBeHidden();
  await page.release();
});

msTest('Default URL with account disabled', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  await setupNewPage(page, { withParsecAccount: false, location: '/' });
  await expect(page).toHaveURL(/.+\/home$/);
  await expect(page.locator('.account-login-container')).toBeHidden();
  await expect(page.locator('.organization-content')).toBeVisible();
  await page.release();
});

msTest('Account URLs unaccessible with account disabled', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  await setupNewPage(page, { withParsecAccount: false, location: '/account' });
  await expect(page).toHaveURL(/.+\/home$/);
  await page.goto('/createAccount');
  await expect(page).toHaveURL(/.+\/home$/);
  await page.release();
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest } from '@tests/e2e/helpers';

msTest('Parsec account login initial page', async ({ parsecAccount }) => {
  const container = parsecAccount.locator('.homepage-content');
  await expect(container.locator('.account-login__title')).toHaveText('My Parsec account');
});

msTest('Parsec account skip login', async ({ parsecAccount }) => {
  const container = parsecAccount.locator('.homepage-content');
  await expect(container.locator('.homepage-skip__button')).toHaveText('Skip this step');
  await container.locator('.homepage-skip__button').click();
  await expect(parsecAccount).toHaveURL(/.+\/home$/);
  await expect(parsecAccount.locator('.organization-content')).toBeVisible();
});

msTest('Parsec account go to customer area', async ({ parsecAccount }) => {
  const container = parsecAccount.locator('.homepage-content');
  const customerAreaContainer = container.locator('.homepage-client-area');
  await expect(customerAreaContainer.locator('.homepage-client-area__title')).toHaveText('Customer area');
  await expect(customerAreaContainer.locator('.homepage-client-area__button')).toHaveText('Access');
  await customerAreaContainer.locator('.homepage-client-area__button').click();
  await expect(parsecAccount).toHaveURL(/.+\/home$/);
  await expect(parsecAccount.locator('.saas-login-container')).toBeVisible();
});

msTest('Parsec account go to creation', async ({ parsecAccount }) => {
  const container = parsecAccount.locator('.homepage-content');
  await expect(container.locator('.account-create__button')).toHaveText('Create an account');
  await container.locator('.account-create__button').click();
  await expect(parsecAccount).toHaveURL(/.+\/createAccount$/);
});

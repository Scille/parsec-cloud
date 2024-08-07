// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';

msTest('Opens the about dialog', async ({ home }) => {
  await home.locator('#trigger-version-button').click();

  await expect(home.locator('.ms-modal')).toBeVisible();
  await expect(home.locator('.ms-modal').locator('.ms-modal-header__title')).toContainText('About');
});

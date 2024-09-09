// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';

msTest('Opens the about dialog', async ({ home }) => {
  await home.locator('#trigger-version-button').click();

  const modal = home.locator('.about-modal');
  await expect(modal).toBeVisible();
  await expect(modal.locator('.ms-modal-header__title')).toContainText('About');
  await expect(modal.locator('.app-info-key')).toHaveText(['Version', 'Developer', 'License', 'Project']);
  await expect(modal.locator('.app-info-value')).toHaveText([/^ v[a-z0-9-.+]+$/, 'Parsec Cloud', 'BUSL-1.1', ' GitHub ']);
  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
});

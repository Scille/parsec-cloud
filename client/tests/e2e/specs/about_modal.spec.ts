// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest, openExternalLink } from '@tests/e2e/helpers';

msTest('Opens the about dialog', async ({ home }) => {
  await home.locator('#trigger-version-button').click();

  const modal = home.locator('.about-modal');
  await expect(modal).toBeVisible();
  await expect(modal.locator('.ms-modal-header__title')).toContainText('About');
  const titles = modal.locator('.app-info-key');
  const values = modal.locator('.app-info-value');
  await expect(titles).toHaveText(['Version', 'Developer', 'License', 'Project']);
  await expect(values).toHaveText([/^ v[a-z0-9-.+]+$/, 'Parsec Cloud', 'BUSL-1.1', ' GitHub ']);

  await openExternalLink(home, values.nth(1), /^https:\/\/parsec\.cloud\/en\/?.*$/);
  await openExternalLink(home, values.nth(2), /^https:\/\/raw\.githubusercontent\.com\/Scille\/parsec-cloud\/.+\/LICENSE.*$/);
  await openExternalLink(home, values.nth(3), /^https:\/\/github\.com\/Scille\/parsec-cloud$/);
  await openExternalLink(home, modal.locator('.changelog-btn'), /^https:\/\/docs\.parsec\.cloud\/.+$/);

  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest } from '@tests/e2e/helpers';

msTest('Default URL with editics enabled', async ({ parsecEditics }) => {
  const entry = parsecEditics.locator('.folder-container').locator('.file-card-item').nth(2);
  await expect(entry).toBeVisible();
  await entry.hover();
  await entry.locator('.card-option').click();
  const popover = parsecEditics.locator('.file-context-menu');
  await expect(popover.getByRole('listitem').nth(1)).toHaveText('Edit');
});

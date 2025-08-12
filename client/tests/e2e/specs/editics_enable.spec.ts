// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest } from '@tests/e2e/helpers';

msTest('Default URL with editics enabled', async ({ parsecEditics }) => {
  const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(2);
  await expect(entry).toBeVisible();
  await entry.hover();
  await entry.locator('.options-button').click();
  const popover = parsecEditics.locator('.file-context-menu');
  await expect(popover.getByRole('listitem').nth(2)).toHaveText('Edit');
});

msTest('Default URL with editics disabled', async ({ documents }) => {
  const entry = documents.locator('.folder-container').locator('.file-list-item').nth(2);
  await expect(entry).toBeVisible();
  await entry.hover();
  await entry.locator('.options-button').click();
  const popover = documents.locator('.file-context-menu');
  await expect(popover.getByRole('listitem')).toHaveText([
    'File management',
    'Preview',
    'Rename',
    'Move to',
    'Make a copy',
    'History',
    'Download',
    'Details',
    'Delete',
    'Collaboration',
    'Copy link',
  ]);
});

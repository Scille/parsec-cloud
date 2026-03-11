// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, fillIonInput, msTest } from '@tests/e2e/helpers';

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });

  for (const file of [
    [0, 'docx', 'New document'],
    [1, 'xlsx', 'New spreadsheet'],
    [2, 'pptx', 'New presentation'],
    [3, 'txt', 'New text file'],
  ] as const) {
    msTest(`Create new ${file[1]} file from action bar`, async ({ documents }) => {
      const entries = documents.locator('.folder-container').locator('.file-list-item');
      await expect(entries).toHaveCount(0);
      const popover = documents.locator('.new-file-popover');
      await expect(popover).toBeHidden();
      const actionBar = documents.locator('.action-bar');
      await actionBar.locator('.ms-action-bar-button:visible').nth(1).click();
      await expect(popover).toBeVisible();
      await expect(popover.locator('ion-item')).toHaveText(['Document', 'Spreadsheet', 'Presentation', 'Text']);
      await popover.locator('ion-item').nth(file[0]).click();
      const modal = documents.locator('.text-input-modal');
      await expect(modal).toBeVisible();
      await expect(modal.locator('.ms-modal-header__title')).toHaveText('File name');
      await expect(modal.locator('.input-container').locator('.form-label')).toHaveText('Choose the new file name');
      await expect(modal.locator('ion-input').locator('input')).toHaveValue(file[2]);
      await fillIonInput(modal.locator('ion-input'), 'MyFile');
      await expect(modal.locator('#next-button')).toHaveText('Create the new file');
      await modal.locator('#next-button').click();
      await expect(entries).toHaveCount(1);
      await expect(entries.nth(0).locator('.file-name')).toHaveText(`MyFile.${file[1]}`);
    });
  }
});

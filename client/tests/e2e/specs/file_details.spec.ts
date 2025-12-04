// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import { expect, importDefaultFiles, ImportDocuments, msTest } from '@tests/e2e/helpers';

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });
  for (const isFile of [true, false]) {
    msTest(`Show ${isFile ? 'file' : 'folder'} details`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, isFile ? ImportDocuments.Png : 0, !isFile);
      const nameMatcher = isFile ? '[A-Za-z_.0-9]+' : 'Dir_[A-Za-z_.0-9]+';
      await expect(documents.locator('.topbar-left__breadcrumb')).toContainText('wksp1');
      const files = documents.locator('.folder-container').getByRole('listitem');
      await expect(files).toHaveCount(1);
      await expect(files.nth(0).locator('.file-name').locator('.label-name')).toHaveText(new RegExp(`^${nameMatcher}$`));
      await expect(files.nth(0).locator('.file-last-update')).toHaveText(/^((?:one|\d{1,2}) minutes? ago|< 1 minute|now)$/);
      expect(documents.locator('.file-context-menu')).toBeHidden();
      expect(documents.locator('.file-details-modal')).toBeHidden();

      await files.nth(0).hover();
      await files.nth(0).locator('.options-button').click();
      if (isFile) {
        expect(documents.locator('.file-context-menu').getByRole('listitem')).toHaveCount(11);
        await documents.locator('.file-context-menu').getByRole('listitem').nth(7).click();
      } else {
        expect(documents.locator('.file-context-menu').getByRole('listitem')).toHaveCount(9);
        await documents.locator('.file-context-menu').getByRole('listitem').nth(5).click();
      }
      await expect(documents.locator('.file-details-modal')).toBeVisible();
      const modal = documents.locator('.file-details-modal');
      await expect(modal.locator('.ms-modal-header__title ')).toHaveText(new RegExp(`^Details on ${nameMatcher}$`));

      const generalDetails = modal.locator('.file-info-details-content').nth(0);
      const generalDetailsItem = generalDetails.locator('.file-info-details-item');
      await expect(generalDetails.locator('.file-info-details-content__title')).toHaveText('General information');
      await expect(generalDetailsItem).toHaveCount(isFile ? 3 : 2);
      await expect(generalDetailsItem.nth(0).locator('.file-info-details-item__title')).toHaveText('Created');
      await expect(generalDetailsItem.nth(0).locator('.file-info-details-item__value')).toHaveText(/^[A-Za-z]{3} \d{1,2}, 20[0-9]{2}$/);

      if (isFile) {
        await expect(generalDetailsItem.nth(1).locator('.file-info-details-item__title')).toHaveText('Size');
        await expect(generalDetailsItem.nth(1).locator('.file-info-details-item__value')).toHaveText(/^[\d.]{1,4} (K|M)?B$/);
      }

      await expect(generalDetailsItem.nth(isFile ? 2 : 1).locator('.file-info-details-item__title')).toHaveText('Version');
      await expect(generalDetailsItem.nth(isFile ? 2 : 1).locator('.file-info-details-item__value')).toHaveText(/^\d+$/);

      await expect(modal.locator('.label-id')).toHaveText(/^(Internal ID: )[a-f0-9-]+$/);

      const updateDetails = modal.locator('.file-info-details-content').nth(1);
      const updateDetailsItem = updateDetails.locator('.file-info-details-item');
      await expect(updateDetails.locator('.file-info-details-content__title')).toHaveText('Last updated');
      await expect(updateDetailsItem).toHaveCount(2);
      await expect(updateDetailsItem.nth(0).locator('.file-info-details-item__title')).toHaveText('Updated');
      await expect(updateDetailsItem.nth(0).locator('.file-info-details-item__value')).toHaveText(/^[A-Za-z]{3} \d{1,2}, 20[0-9]{2}$/);

      await expect(updateDetailsItem.nth(1).locator('.file-info-details-item__title')).toHaveText('Editor');
      await expect(updateDetailsItem.nth(1).locator('.file-info-details-item__value')).toHaveText('Alicey McAliceFace');

      await expect(modal.locator('.file-info-path-value__text')).toHaveText(new RegExp(`^/${nameMatcher}$`));

      const icon = modal.locator('.cloud-overlay');
      const syncPopover = documents.locator('.tooltip-popover');
      await icon.click();
      await expect(syncPopover).toBeVisible();

      if (isFile) {
        await expect(syncPopover).toHaveText(/^\s*This file is (not)?synced with the server\.$/);
      } else {
        await expect(syncPopover).toHaveText(/^\s*This folder is (not)?synced with the server\.$/);
      }
    });
  }

  msTest('Show file details in grid mode', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);

    await expect(documents.locator('.topbar-left__breadcrumb')).toContainText('wksp1');
    await documents.locator('#folders-ms-action-bar').locator('#grid-view').click();
    const files = documents.locator('.folders-container-grid').locator('.file-card-item');
    await expect(files).toHaveCount(1);
    await expect(documents.locator('.file-context-menu')).toBeHidden();
    await expect(documents.locator('.file-details-modal')).toBeHidden();
    await files.nth(0).hover();
    await files.nth(0).locator('.card-option').click();
    await expect(documents.locator('.file-context-menu').getByRole('listitem')).toHaveCount(11);
    await documents.locator('.file-context-menu').getByRole('listitem').nth(7).click();
    await expect(documents.locator('.file-details-modal')).toBeVisible();
    const modal = documents.locator('.file-details-modal');
    await expect(modal.locator('.ms-modal-header__title ')).toHaveText(/^Details on [a-z0-9._]+$/);
  });
});

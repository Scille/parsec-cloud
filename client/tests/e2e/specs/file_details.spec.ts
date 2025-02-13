// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest } from '@tests/e2e/helpers';

const TEST_DATA = [
  {
    isFile: true,
    index: 3,
  },
  {
    isFile: false,
    index: 0,
  },
];

for (const testData of TEST_DATA) {
  msTest(`Show ${testData.isFile ? 'file' : 'folder'} details`, async ({ connected, context }) => {
    const nameMatcher = `${testData.isFile ? 'File' : 'Dir'}_[a-z_.0-9]+`;

    await expect(connected.locator('.workspace-card-item').nth(0)).toContainText('The Copper Coronet');
    await connected.locator('.workspace-card-item').nth(0).click();

    await expect(connected.locator('.topbar-left__breadcrumb')).toContainText('The Copper Coronet');
    const files = connected.locator('.folder-container').getByRole('listitem');
    await expect(files).toHaveCount(11);
    await expect(files.nth(testData.index).locator('.file-name').locator('.file-name__label')).toHaveText(new RegExp(`^${nameMatcher}$`));
    await expect(files.nth(testData.index).locator('.file-lastUpdate')).toHaveText(/^((?:one|\d{1,2}) minutes? ago|< 1 minute)$/);
    expect(connected.locator('.file-context-menu')).toBeHidden();
    expect(connected.locator('.file-details-modal')).toBeHidden();

    const syncElem = files.nth(testData.index).locator('.cloud-overlay');
    const classes = await syncElem.evaluate((node) => Array.from(node.classList.values()));
    const isSynced = classes.includes('cloud-overlay-ok');

    if (!isSynced) {
      await expect(syncElem).toHaveTheClass('cloud-overlay-ko');
    }

    await files.nth(testData.index).hover();
    await files.nth(testData.index).locator('.options-button').click();
    if (testData.isFile) {
      expect(connected.locator('.file-context-menu').getByRole('listitem')).toHaveCount(10);
      await connected.locator('.file-context-menu').getByRole('listitem').nth(7).click();
    } else {
      expect(connected.locator('.file-context-menu').getByRole('listitem')).toHaveCount(9);
      await connected.locator('.file-context-menu').getByRole('listitem').nth(6).click();
    }
    await expect(connected.locator('.file-details-modal')).toBeVisible();
    const modal = connected.locator('.file-details-modal');
    await expect(modal.locator('.ms-modal-header__title ')).toHaveText(new RegExp(`^Details on ${nameMatcher}$`));
    await expect(modal.locator('.file-info-basic__edit')).toHaveText(/^Updated: [A-Za-z]{3} \d{1,2}, 20[0-9]{2}$/);

    const details = modal.locator('.file-info-details-item');
    await expect(details).toHaveCount(testData.isFile ? 3 : 2);
    await expect(details.nth(0).locator('.file-info-details-item__title')).toHaveText('Created');
    await expect(details.nth(0).locator('.file-info-details-item__value')).toHaveText(/^[A-Za-z]{3} \d{1,2}, 20[0-9]{2}$/);

    if (testData.isFile) {
      await expect(details.nth(1).locator('.file-info-details-item__title')).toHaveText('Size');
      await expect(details.nth(1).locator('.file-info-details-item__value')).toHaveText(/^[\d.]{1,4} (K|M)?B$/);
    }

    await expect(details.nth(testData.isFile ? 2 : 1).locator('.file-info-details-item__title')).toHaveText('Version');
    await expect(details.nth(testData.isFile ? 2 : 1).locator('.file-info-details-item__value')).toHaveText(/^\d+$/);

    await expect(modal.locator('.label-id')).toHaveText(/^(Technical ID: )[a-f0-9-]+$/);

    await expect(modal.locator('.file-info-path-value__text')).toHaveText(new RegExp(`^/${nameMatcher}$`));
    const copyButton = modal.locator('#copy-link-btn');
    await expect(modal.locator('.file-info-path-value__copied')).toBeHidden();
    await expect(modal.locator('.file-info-path-value__not-copied')).toBeHidden();

    await copyButton.click();
    await expect(copyButton).toBeHidden();

    // Fails because the browser does not have the permissions
    await expect(modal.locator('.file-info-path-value__copied')).toBeHidden();
    await expect(modal.locator('.file-info-path-value__not-copied')).toBeVisible();
    await expect(modal.locator('.file-info-path-value__not-copied')).toHaveText('Failed to copy');

    // Wait for the button to be available again
    await connected.waitForTimeout(5000);
    // Grant the permissions
    await context.grantPermissions(['clipboard-write']);

    await expect(copyButton).toBeVisible();
    await copyButton.click();
    await expect(copyButton).toBeHidden();

    await expect(modal.locator('.file-info-path-value__copied')).toBeVisible();
    await expect(modal.locator('.file-info-path-value__copied')).toHaveText('Copied');
    await expect(modal.locator('.file-info-path-value__not-copied')).toBeHidden();

    const filePath = await connected.evaluate(() => navigator.clipboard.readText());
    expect(filePath).toMatch(new RegExp(`^/home/${nameMatcher}$`));

    const icon = modal.locator('.cloud-overlay');
    const syncPopover = connected.locator('.tooltip-popover');
    await icon.click();
    await expect(syncPopover).toBeVisible();
    if (isSynced) {
      await expect(icon).toHaveTheClass('cloud-overlay-ok');
      await expect(syncPopover).toHaveText(
        testData.isFile ? 'This file is synced with the server.' : 'This folder is synced with the server.',
      );
    } else {
      await expect(icon).toHaveTheClass('cloud-overlay-ko');
      await expect(syncPopover).toHaveText(
        testData.isFile ? 'This file is not synced with the server.' : 'This folder is not synced with the server.',
      );
    }
  });
}

msTest('Show file details in grid mode', async ({ connected }) => {
  await expect(connected.locator('.workspace-card-item').nth(0)).toContainText('The Copper Coronet');
  await connected.locator('.workspace-card-item').nth(0).click();
  await expect(connected.locator('.topbar-left__breadcrumb')).toContainText('The Copper Coronet');

  await connected.locator('#folders-ms-action-bar').locator('#grid-view').click();
  const files = connected.locator('.folders-container-grid').locator('.file-card-item');
  await expect(files).toHaveCount(11);
  expect(connected.locator('.file-context-menu')).toBeHidden();
  expect(connected.locator('.file-details-modal')).toBeHidden();
  await files.nth(3).hover();
  await files.nth(3).locator('.card-option').click();
  expect(connected.locator('.file-context-menu').getByRole('listitem')).toHaveCount(10);
  await connected.locator('.file-context-menu').getByRole('listitem').nth(7).click();
  await expect(connected.locator('.file-details-modal')).toBeVisible();
  const modal = connected.locator('.file-details-modal');
  await expect(modal.locator('.ms-modal-header__title ')).toHaveText(/^Details on File_[a-z0-9._]+$/);
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import { MockBms, expect, fillIonInput, fillIonTextArea, msTest } from '@tests/e2e/helpers';
import path from 'path';

msTest('Submit bug report', async ({ connected }, testInfo: TestInfo) => {
  MockBms.mockReportBug(connected);

  const modal = connected.locator('.bug-report-modal');
  await expect(modal).toBeHidden();
  await connected.locator('.topbar').locator('.profile-header').click();
  const reportButton = connected.locator('.profile-header-organization-popover').locator('.footer-list').getByRole('listitem').nth(4);
  await expect(reportButton).toHaveText('REPORT BUG');
  await reportButton.click();
  await expect(modal).toBeVisible();
  const sendButton = modal.locator('#next-button');
  await expect(sendButton).toHaveText('REPORT BUG');
  await expect(sendButton).toBeTrulyDisabled();
  await fillIonInput(modal.locator('ion-input').nth(0), 'gordon.freeman@blackmesa.nm');
  await expect(sendButton).toBeTrulyDisabled();
  await fillIonTextArea(modal.locator('ion-textarea').nth(0), 'Bug description');
  await expect(sendButton).toBeTrulyEnabled();

  const fileItems = modal.locator('.file-item');
  await expect(fileItems).toHaveCount(0);
  const addButton = modal.locator('.add-button');
  await expect(addButton).toHaveText('ADD FILE');
  let fileChooserPromise = connected.waitForEvent('filechooser');
  await addButton.click();
  let fileChooser = await fileChooserPromise;
  expect(fileChooser.isMultiple()).toBe(false);
  await fileChooser.setFiles([path.join(testInfo.config.rootDir, 'data', 'imports', 'hell_yeah.png')]);
  await expect(fileItems).toHaveCount(1);
  await expect(fileItems.nth(0).locator('.file-name')).toBeVisible();
  await expect(fileItems.nth(0).locator('.file-name')).toHaveText('hell_yeah.png');

  fileChooserPromise = connected.waitForEvent('filechooser');
  await addButton.click();
  fileChooser = await fileChooserPromise;
  expect(fileChooser.isMultiple()).toBe(false);
  await fileChooser.setFiles([path.join(testInfo.config.rootDir, 'data', 'imports', 'yo.png')]);
  await expect(fileItems).toHaveCount(2);
  await expect(fileItems.locator('.file-name')).toHaveText(['hell_yeah.png', 'yo.png']);
  await fileItems.nth(0).locator('.remove-button').click();
  await expect(fileItems).toHaveCount(1);
  await expect(fileItems.locator('.file-name')).toHaveText(['yo.png']);

  await expect(modal.locator('.include-logs')).not.toHaveTheClass('toggle-checked');
  // await modal.locator('.include-logs').click();
  // await expect(modal.locator('.include-logs')).toHaveTheClass('toggle-checked');

  await sendButton.click();
  await expect(modal).toBeHidden();
  await expect(connected).toShowToast('BUG REPORT SENT', 'Success');
});

msTest('Show logs', async ({ connected }) => {
  const modal = connected.locator('.log-modal');
  await expect(modal).toBeHidden();
  await connected.locator('.topbar').locator('.profile-header').click();
  const logButton = connected.locator('.profile-header-organization-popover').locator('.footer-list').getByRole('listitem').nth(3);
  await expect(logButton).toHaveText('SEE LOGS');
  await logButton.click();
  await expect(modal).toBeVisible();
  await expect(modal.locator('.ms-modal-header__title')).toHaveText('LOGS');
  await expect(modal.locator('.container-textinfo')).toHaveCount(3);
});

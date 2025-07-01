// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import { MockBms, expect, fillIonInput, fillIonTextArea, msTest } from '@tests/e2e/helpers';
import path from 'path';

msTest('Submit bug report', async ({ connected }, testInfo: TestInfo) => {
  MockBms.mockReportBug(connected);

  const modal = connected.locator('.bug-report-modal');
  await expect(modal).toBeHidden();
  await connected.locator('.topbar').locator('.profile-header').click();
  const reportButton = connected.locator('.profile-header-organization-popover').locator('.footer-list').getByRole('listitem').nth(3);
  await expect(reportButton).toHaveText('Report a bug');
  await reportButton.click();
  await expect(modal).toBeVisible();
  const error = modal.locator('.report-error');
  await expect(error).toBeHidden();
  await expect(modal.locator('.ms-modal-header__title-container')).toHaveText('Report a bug');
  const sendButton = modal.locator('#next-button');
  await expect(sendButton).toHaveText('Send report');
  await expect(sendButton).toBeTrulyDisabled();
  await fillIonInput(modal.locator('ion-input').nth(0), 'gordon.freeman@blackmesa.nm');
  await expect(sendButton).toBeTrulyDisabled();
  await fillIonTextArea(modal.locator('ion-textarea').nth(0), 'Bug description');
  await expect(sendButton).toBeTrulyEnabled();

  const fileItems = modal.locator('.file-item');
  await expect(fileItems).toHaveCount(0);
  const fileCount = modal.locator('.file-number');
  await expect(fileCount).toHaveText('Included files: 0 / 3');
  const addButton = modal.locator('.file-input__button').locator('.button-label');
  await expect(addButton).toHaveText('Add a file');
  let fileChooserPromise = connected.waitForEvent('filechooser');
  await addButton.click();
  let fileChooser = await fileChooserPromise;
  expect(fileChooser.isMultiple()).toBe(false);
  await fileChooser.setFiles([path.join(testInfo.config.rootDir, 'data', 'imports', 'hell_yeah.png')]);
  await expect(fileItems).toHaveCount(1);
  await expect(fileItems.nth(0).locator('.file-name')).toBeVisible();
  await expect(fileItems.nth(0).locator('.file-name')).toHaveText('hell_yeah.png');
  await expect(fileCount).toHaveText('Included files: 1 / 3');

  fileChooserPromise = connected.waitForEvent('filechooser');
  await addButton.click();
  fileChooser = await fileChooserPromise;
  expect(fileChooser.isMultiple()).toBe(false);
  await fileChooser.setFiles([path.join(testInfo.config.rootDir, 'data', 'imports', 'yo.png')]);
  await expect(fileItems).toHaveCount(2);
  await expect(fileCount).toHaveText('Included files: 2 / 3');
  await expect(fileItems.locator('.file-name')).toHaveText(['hell_yeah.png', 'yo.png']);
  await fileItems.nth(0).locator('.remove-button').click();
  await expect(fileItems).toHaveCount(1);
  await expect(fileCount).toHaveText('Included files: 1 / 3');
  await expect(fileItems.locator('.file-name')).toHaveText(['yo.png']);

  await expect(modal.locator('.report-logs__toggle')).not.toHaveTheClass('toggle-checked');
  await modal.locator('.report-logs__toggle').click();
  await expect(modal.locator('.report-logs__toggle')).toHaveTheClass('toggle-checked');

  await sendButton.click();
  await expect(error).toBeHidden();
  await expect(modal).toBeHidden();
  await expect(connected).toShowToast('The bug report has been sent. Thank you for your help!', 'Success');
});

msTest('Submit bug report failed', async ({ connected }) => {
  MockBms.mockReportBug(connected, { POST: { timeout: true } });

  const modal = connected.locator('.bug-report-modal');
  await expect(modal).toBeHidden();
  await connected.locator('.topbar').locator('.profile-header').click();
  const reportButton = connected.locator('.profile-header-organization-popover').locator('.footer-list').getByRole('listitem').nth(3);
  await expect(reportButton).toHaveText('Report a bug');
  await reportButton.click();
  await expect(modal).toBeVisible();
  const error = modal.locator('.report-error');
  await expect(error).toBeHidden();
  await expect(modal.locator('.ms-modal-header__title-container')).toHaveText('Report a bug');
  const sendButton = modal.locator('#next-button');
  await expect(sendButton).toHaveText('Send report');
  await expect(sendButton).toBeTrulyDisabled();
  await fillIonInput(modal.locator('ion-input').nth(0), 'gordon.freeman@blackmesa.nm');
  await expect(sendButton).toBeTrulyDisabled();
  await fillIonTextArea(modal.locator('ion-textarea').nth(0), 'Bug description');
  await expect(sendButton).toBeTrulyEnabled();

  await sendButton.click();
  await expect(error).toBeVisible();
  await expect(error).toHaveText('Failed to send the report');
  await expect(modal).toBeVisible();
});

msTest('Show logs when logged in', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(0)).toHaveText('Settings');
  await myProfilePage.locator('.menu-list__item').nth(0).click();
  const modal = myProfilePage.locator('.log-modal');
  await expect(modal).toBeHidden();
  const logButton = myProfilePage.locator('.settings-list').locator('.see-logs-button');
  await expect(logButton).toHaveText('View logs');
  await logButton.click();
  await expect(modal).toBeVisible();
  await expect(modal.locator('.ms-modal-header__title')).toHaveText('Logs');
});

msTest('Show logs on homepage', async ({ home }) => {
  const settingsModal = home.locator('.settings-modal');
  const logModal = home.locator('.log-modal');
  await expect(settingsModal).toBeHidden();
  await expect(logModal).toBeHidden();
  await home.locator('#trigger-settings-button').click();
  await expect(settingsModal).toBeVisible();
  await expect(logModal).toBeHidden();
  await settingsModal.locator('.see-logs-button').click();
  await expect(settingsModal).toBeHidden();
  await expect(logModal).toBeVisible();
  await logModal.locator('.closeBtn').click();
  await expect(settingsModal).toBeVisible();
  await expect(logModal).toBeHidden();
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, fillIonInput, msTest } from '@tests/e2e/helpers';
import path from 'path';
import { TestInfo } from 'playwright/test';

msTest('Import recovery device', async ({ home }, testInfo: TestInfo) => {
  await home.locator('.organization-card').nth(0).click();
  await home.locator('.login-popup').locator('#forgotten-password-button').click();
  const container = home.locator('.recovery-content');

  await expect(container.locator('.container-textinfo')).toHaveText(
    'You must have created a recovery file in order to reset your authentication method.',
  );

  const okButton = container.locator('#to-password-change-btn');
  await expect(okButton).toHaveText('Next');
  await expect(okButton).toBeTrulyDisabled();

  const items = container.locator('.recovery-list-item');
  await expect(items.nth(1)).toHaveTheClass('disabled');

  const importButton = items.nth(0).locator('#browse-button');
  await expect(importButton).toHaveText('Browse');

  await expect(items.nth(0).locator('.body')).toHaveText('No file selected');
  const fileChooserPromise = home.waitForEvent('filechooser');
  await importButton.click();
  const fileChooser = await fileChooserPromise;
  expect(fileChooser.isMultiple()).toBe(false);
  await fileChooser.setFiles([path.join(testInfo.config.rootDir, 'data', 'recovery', 'recovery_file.psrk')]);
  await expect(items.nth(0).locator('.body')).toHaveText('recovery_file.psrk');
  await expect(items.nth(0).locator('.body')).toHaveTheClass('file-added');

  await expect(items.nth(1)).not.toHaveTheClass('disabled');
  // cspell:disable-next-line
  await fillIonInput(items.nth(1).locator('ion-input'), 'ABCD-EFGH-IJKL-MNOP-QRST-UVWX-YZ12-3456-7890-ABCD-EFGH-IJKL-MNOP');

  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();

  const authChoices = container.locator('.choose-auth-page').locator('ion-radio');

  await expect(authChoices.nth(0).locator('.item-radio__label')).toHaveText('Use System Authentication');
  await expect(authChoices.nth(0).locator('.item-radio__text:visible')).toHaveText('Unavailable on web');
  await expect(authChoices.nth(0)).toHaveTheClass('radio-disabled');

  const authButton = container.locator('#validate-password-btn');
  await expect(authButton).toHaveText('Confirm');
  const inputs = container.locator('.choose-auth-page').locator('.choose-password').locator('ion-input');
  await fillIonInput(inputs.nth(0), 'YouGiv3Lov3aBadName#');
  await expect(authButton).toBeTrulyDisabled();
  await fillIonInput(inputs.nth(1), 'YouGiv3Lov3aBadName#');
  await expect(authButton).toBeTrulyEnabled();
  await authButton.click();

  await expect(container.locator('.success-card__title')).toHaveText('Authentication was successfully updated!');
  await container.locator('.success-card__button').click();
});

msTest('Import recovery device invalid recovery file', async ({ home }, testInfo: TestInfo) => {
  await home.locator('.organization-card').nth(0).click();
  await home.locator('.login-popup').locator('#forgotten-password-button').click();
  const container = home.locator('.recovery-content');

  const okButton = container.locator('#to-password-change-btn');
  const items = container.locator('.recovery-list-item');
  await expect(items.nth(0).locator('.recovery-list-item__title')).toHaveText('1 Recovery file');
  await expect(items.nth(1).locator('.recovery-list-item__title')).toHaveText('2 Secret key');

  await expect(items.nth(0).locator('.body')).toHaveText('No file selected');
  const fileChooserPromise = home.waitForEvent('filechooser');
  await items.nth(0).locator('ion-button').click();
  const fileChooser = await fileChooserPromise;
  expect(fileChooser.isMultiple()).toBe(false);
  await fileChooser.setFiles([path.join(testInfo.config.rootDir, 'data', 'recovery', 'fake_recovery_file.txt')]);
  await expect(items.nth(0).locator('.body')).toHaveText('fake_recovery_file.txt');
  await expect(items.nth(0).locator('.body')).toHaveTheClass('file-added');

  await expect(items.nth(1).locator('#checkmark-icon')).toBeHidden();
  // cspell:disable-next-line
  await fillIonInput(items.nth(1).locator('ion-input'), 'ABCD-EFGH-IJKL-MNOP-QRST-UVWX-YZ12-3456-7890-ABCD-EFGH-IJKL-MNOP');
  await expect(items.nth(1).locator('#checkmark-icon')).toBeVisible();

  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();

  const authButton = container.locator('#validate-password-btn');
  const inputs = container.locator('.choose-auth-page').locator('.choose-password').locator('ion-input');
  await fillIonInput(inputs.nth(0), 'YouGiv3Lov3aBadName#');
  await fillIonInput(inputs.nth(1), 'YouGiv3Lov3aBadName#');
  await authButton.click();

  await expect(home).toShowToast('Invalid recovery file.', 'Error');
});

msTest('Import recovery device invalid passphrase', async ({ home }, testInfo: TestInfo) => {
  await home.locator('.organization-card').nth(0).click();
  await home.locator('.login-popup').locator('#forgotten-password-button').click();
  const container = home.locator('.recovery-content');

  const okButton = container.locator('#to-password-change-btn');
  const items = container.locator('.recovery-list-item');

  await expect(items.nth(0).locator('.body')).toHaveText('No file selected');
  const fileChooserPromise = home.waitForEvent('filechooser');
  await items.nth(0).locator('ion-button').click();
  const fileChooser = await fileChooserPromise;
  expect(fileChooser.isMultiple()).toBe(false);
  await fileChooser.setFiles([path.join(testInfo.config.rootDir, 'data', 'recovery', 'recovery_file.psrk')]);
  await expect(items.nth(0).locator('.body')).toHaveText('recovery_file.psrk');
  await expect(items.nth(0).locator('.body')).toHaveTheClass('file-added');

  // cspell:disable-next-line
  await fillIonInput(items.nth(1).locator('ion-input'), 'ABCD-EFGH-IJKL-MNOP-QRST-UVWX-YZ12-3456-7890-ABCD-EFGH-IJKL-MNOX');
  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();

  const authButton = container.locator('#validate-password-btn');
  const inputs = container.locator('.choose-auth-page').locator('.choose-password').locator('ion-input');
  await fillIonInput(inputs.nth(0), 'YouGiv3Lov3aBadName#');
  await fillIonInput(inputs.nth(1), 'YouGiv3Lov3aBadName#');
  await authButton.click();

  await expect(home).toShowToast('The secret key does not match the recovery file.', 'Error');
});

msTest('Access recovery page from org list', async ({ home }) => {
  const btn = home.locator('.recovery-devices').locator('ion-button');
  await expect(btn).toHaveText('Recover my session');
  await btn.click();
  const container = home.locator('.recovery-content');

  await expect(container.locator('.container-textinfo')).toHaveText(
    'You must have created a recovery file in order to reset your authentication method.',
  );
});

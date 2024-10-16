// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, fillIonInput, msTest } from '@tests/pw/helpers';

msTest('Import recovery device', async ({ home }) => {
  await home.locator('.organization-card').nth(0).click();
  await home.locator('.login-popup').locator('#forgotten-password-button').click();
  const container = home.locator('.recovery-content');

  await expect(container.locator('.container-textinfo')).toHaveText(
    'You must have created recovery files in order to reset your password.',
  );

  const okButton = container.locator('#to-password-change-btn');
  await expect(okButton).toHaveText('Next');
  await expect(okButton).toBeTrulyDisabled();

  const items = container.locator('.recovery-list-item');
  await expect(items.nth(1)).toHaveTheClass('disabled');

  const importButton = items.nth(0).locator('#browse-button');
  await expect(importButton).toHaveText('Browse');

  const fileInput = items.nth(0).locator('input');
  await fileInput.setInputFiles({
    name: 'RecoveryFile.psrk',
    mimeType: 'text/plain',
    buffer: Buffer.from('Mock recovery file'),
  });

  await expect(items.nth(1)).not.toHaveTheClass('disabled');
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
  await fillIonInput(inputs.nth(0), 'YouGiv3Lov3aBadName');
  await expect(authButton).toBeTrulyDisabled();
  await fillIonInput(inputs.nth(1), 'YouGiv3Lov3aBadName');
  await expect(authButton).toBeTrulyEnabled();
  await authButton.click();

  await expect(container.locator('.success-card__title')).toHaveText('Authentication was successfully updated!');
  await container.locator('.success-card__button').click();
});

msTest('Import recovery device invalid recovery file', async ({ home }) => {
  await home.locator('.organization-card').nth(0).click();
  await home.locator('.login-popup').locator('#forgotten-password-button').click();
  const container = home.locator('.recovery-content');

  const okButton = container.locator('#to-password-change-btn');
  const items = container.locator('.recovery-list-item');
  const fileInput = items.nth(0).locator('input');
  await fileInput.setInputFiles({
    name: 'RecoveryFile.txt',
    mimeType: 'text/plain',
    buffer: Buffer.from('Mock recovery file'),
  });
  await fillIonInput(items.nth(1).locator('ion-input'), 'ABCD-EFGH-IJKL-MNOP-QRST-UVWX-YZ12-3456-7890-ABCD-EFGH-IJKL-MNOP');

  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();
  await expect(home).toShowToast('Invalid recovery file.', 'Error');
});

msTest('Import recovery device invalid passphrase', async ({ home }) => {
  await home.locator('.organization-card').nth(0).click();
  await home.locator('.login-popup').locator('#forgotten-password-button').click();
  const container = home.locator('.recovery-content');

  const okButton = container.locator('#to-password-change-btn');
  const items = container.locator('.recovery-list-item');
  const fileInput = items.nth(0).locator('input');
  await fileInput.setInputFiles({
    name: 'RecoveryFile.psrk',
    mimeType: 'text/plain',
    buffer: Buffer.from('Mock recovery file'),
  });
  await fillIonInput(items.nth(1).locator('ion-input'), 'ABCD-EFGH-IJKL-MNOP-QRST-UVWX-YZ12-3456-7890-ABCD-EFGH-IJKL-MNOX');

  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();
  await expect(home).toShowToast('The secret key does not match the recovery file.', 'Error');
});

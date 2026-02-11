// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DEFAULT_USER_INFORMATION, expect, fillIonInput, logout, msTest } from '@tests/e2e/helpers';

msTest('Export and use recovery files', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(3)).toHaveText('Recovery files');
  await myProfilePage.locator('.menu-list__item').nth(3).click();
  const recovery = myProfilePage.locator('.recovery');
  await expect(recovery.locator('.item-header__title')).toHaveText('Organization recovery files');
  await expect(recovery.locator('.organization-recovery-container').locator('.restore-password__advice')).toBeVisible();
  await expect(recovery.locator('.restore-password-button')).toHaveText('Create recovery files');
  const recoveryFiles = recovery.locator('.recovery-list');
  await expect(recoveryFiles).toBeHidden();
  await recovery.locator('.restore-password-button').click();
  await expect(recoveryFiles).toBeVisible();
  const recoveryItems = recoveryFiles.locator('.recovery-item');
  await expect(recoveryItems).toHaveCount(2);
  await expect(recoveryItems.locator('.recovery-item-text span')).toHaveText(['Recovery File', 'Secret Key']);
  await expect(recoveryItems.locator('.recovery-item-download ion-button')).toHaveText(['Download', 'Download']);
  await expect(recoveryItems.nth(0).locator('.checked')).toBeHidden();
  await expect(recoveryItems.nth(0).locator('.checked')).toBeHidden();

  const fileDownloadPromise = myProfilePage.waitForEvent('download');
  await recoveryItems.nth(0).locator('.recovery-item-download').locator('ion-button').click();
  const fileDownload = await fileDownloadPromise;
  expect(fileDownload.suggestedFilename()).toMatch(/^Parsec_Recovery_File_TestbedOrg\d+\.psrk$/);
  const fileStream = await fileDownload.createReadStream();
  const chunks: Array<Buffer> = [];
  for await (const chunk of fileStream) {
    chunks.push(chunk);
  }
  const fileContent = Buffer.concat(chunks);
  expect(fileContent.byteLength).toBeGreaterThan(0);
  await expect(myProfilePage).toShowToast('The recovery file was successfully downloaded', 'Success');
  await expect(recoveryItems.nth(0).locator('.checked')).toBeVisible();

  const passphraseDownloadPromise = myProfilePage.waitForEvent('download');
  await recoveryItems.nth(1).locator('.recovery-item-download').locator('ion-button').click();
  const passphraseDownload = await passphraseDownloadPromise;
  expect(passphraseDownload.suggestedFilename()).toMatch(/Parsec_Recovery_Code_TestbedOrg\d+\.txt/);
  const passphraseStream = await passphraseDownload.createReadStream();
  passphraseStream.setEncoding('utf8');

  const codes: Array<string> = [];
  for await (const chunk of passphraseStream) {
    codes.push(chunk);
  }
  const passphraseContent = codes.join();
  expect(passphraseContent).toMatch(/^([A-Z0-9]{4}-?){13}$/);
  await expect(myProfilePage).toShowToast('The secret key was successfully downloaded', 'Success');
  await expect(recoveryItems.nth(1).locator('.checked')).toBeVisible();

  await logout(myProfilePage);
  await expect(myProfilePage.locator('.recovery-devices').locator('ion-button')).toHaveText('Recover my session');
  await myProfilePage.locator('.recovery-devices').locator('ion-button').click();
  const recoveryContainer = myProfilePage.locator('.recovery-content');
  await expect(recoveryContainer).toBeVisible();

  const importItems = recoveryContainer.locator('.recovery-list-item');
  await expect(importItems.nth(0).locator('.recovery-list-item__button').locator('div')).toHaveText('No file selected');
  await expect(importItems.nth(0).locator('.recovery-list-item__button').locator('ion-button')).toHaveText('Browse');
  await expect(importItems.nth(1)).toHaveTheClass('disabled');

  const fileChooserPromise = myProfilePage.waitForEvent('filechooser');
  await importItems.nth(0).locator('ion-button').click();
  const fileChooser = await fileChooserPromise;
  expect(fileChooser.isMultiple()).toBe(false);
  await fileChooser.setFiles([
    {
      name: 'Parsec_Recovery_File_TestbedOrgXX.psrk',
      mimeType: 'application/octet-stream',
      buffer: fileContent,
    },
  ]);
  await expect(importItems.nth(0).locator('.recovery-list-item__button').locator('div')).toHaveText(
    'Parsec_Recovery_File_TestbedOrgXX.psrk',
  );
  await expect(importItems.nth(1)).not.toHaveTheClass('disabled');
  await expect(recoveryContainer.locator('.next-button').locator('ion-button')).toBeTrulyDisabled();
  await fillIonInput(importItems.nth(1).locator('div.recovery-list-item__input'), passphraseContent);
  await expect(recoveryContainer.locator('.next-button').locator('ion-button')).toBeTrulyEnabled();
  await recoveryContainer.locator('.next-button').locator('ion-button').click();

  await expect(importItems).toBeHidden();
  const authContainer = recoveryContainer.locator('.choose-auth-page');
  await expect(authContainer).toBeVisible();
  const authNext = recoveryContainer.locator('.validate-button');
  await expect(authNext).toHaveText('Confirm');

  const authRadio = authContainer.locator('.radio-list-item:visible');
  await expect(authRadio).toHaveAuthentication({ pkiDisabled: true, keyringDisabled: true, ssoDisabled: true });
  await authRadio.nth(0).click();

  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toHaveDisabledAttribute();
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toNotHaveDisabledAttribute();
  await authNext.click();

  await expect(recoveryContainer.locator('.success-card')).toBeVisible();
  await expect(recoveryContainer.locator('.success-card__title')).toHaveText('Authentication was successfully updated!');
  await recoveryContainer.locator('.success-card').locator('ion-button').click();
  await expect(myProfilePage).toBeWorkspacePage();
});

for (const error of ['invalid-passphrase', 'invalid-file']) {
  msTest(`Export and use recovery files with errors (${error})`, async ({ myProfilePage }) => {
    await expect(myProfilePage.locator('.menu-list__item').nth(3)).toHaveText('Recovery files');
    await myProfilePage.locator('.menu-list__item').nth(3).click();
    const recovery = myProfilePage.locator('.recovery');
    await expect(recovery.locator('.item-header__title')).toHaveText('Organization recovery files');
    await expect(recovery.locator('.organization-recovery-container').locator('.restore-password__advice')).toBeVisible();
    await expect(recovery.locator('.restore-password-button')).toHaveText('Create recovery files');
    const recoveryFiles = recovery.locator('.recovery-list');
    await expect(recoveryFiles).toBeHidden();
    await recovery.locator('.restore-password-button').click();
    await expect(recoveryFiles).toBeVisible();
    const recoveryItems = recoveryFiles.locator('.recovery-item');
    await expect(recoveryItems).toHaveCount(2);
    await expect(recoveryItems.locator('.recovery-item-text span')).toHaveText(['Recovery File', 'Secret Key']);
    await expect(recoveryItems.locator('.recovery-item-download ion-button')).toHaveText(['Download', 'Download']);
    await expect(recoveryItems.nth(0).locator('.checked')).toBeHidden();

    const fileDownloadPromise = myProfilePage.waitForEvent('download');
    await recoveryItems.nth(0).locator('.recovery-item-download ion-button').click();
    const fileDownload = await fileDownloadPromise;
    expect(fileDownload.suggestedFilename()).toMatch(/^Parsec_Recovery_File_TestbedOrg\d+\.psrk$/);
    const fileStream = await fileDownload.createReadStream();
    const chunks: Array<Buffer> = [];
    for await (const chunk of fileStream) {
      chunks.push(chunk);
    }
    const fileContent = Buffer.concat(chunks);
    expect(fileContent.byteLength).toBeGreaterThan(0);
    await expect(myProfilePage).toShowToast('The recovery file was successfully downloaded', 'Success');
    await expect(recoveryItems.nth(0).locator('.checked')).toBeVisible();

    const passphraseDownloadPromise = myProfilePage.waitForEvent('download');
    await recoveryItems.nth(1).locator('.recovery-item-download').locator('ion-button').click();
    const passphraseDownload = await passphraseDownloadPromise;
    expect(passphraseDownload.suggestedFilename()).toMatch(/Parsec_Recovery_Code_TestbedOrg\d+\.txt/);
    const passphraseStream = await passphraseDownload.createReadStream();
    passphraseStream.setEncoding('utf8');

    const codes: Array<string> = [];
    for await (const chunk of passphraseStream) {
      codes.push(chunk);
    }
    const passphraseContent = codes.join();
    expect(passphraseContent).toMatch(/^([A-Z0-9]{4}-?){13}$/);
    await expect(myProfilePage).toShowToast('The secret key was successfully downloaded', 'Success');
    await expect(recoveryItems.nth(1).locator('.checked')).toBeVisible();

    await logout(myProfilePage);
    await expect(myProfilePage.locator('.recovery-devices').locator('ion-button')).toHaveText('Recover my session');
    await myProfilePage.locator('.recovery-devices').locator('ion-button').click();
    const recoveryContainer = myProfilePage.locator('.recovery-content');
    await expect(recoveryContainer).toBeVisible();

    const importItems = recoveryContainer.locator('.recovery-list-item');
    await expect(importItems.nth(0).locator('.recovery-list-item__button').locator('div')).toHaveText('No file selected');
    await expect(importItems.nth(0).locator('.recovery-list-item__button').locator('ion-button')).toHaveText('Browse');
    await expect(importItems.nth(1)).toHaveTheClass('disabled');

    const fileChooserPromise = myProfilePage.waitForEvent('filechooser');
    await importItems.nth(0).locator('ion-button').click();
    const fileChooser = await fileChooserPromise;
    expect(fileChooser.isMultiple()).toBe(false);
    await fileChooser.setFiles([
      {
        name: 'Parsec_Recovery_File_TestbedOrgXX.psrk',
        mimeType: 'application/octet-stream',
        buffer: error === 'invalid-file' ? Buffer.from('meow', 'utf8') : fileContent,
      },
    ]);
    await expect(importItems.nth(0).locator('.recovery-list-item__button').locator('div')).toHaveText(
      'Parsec_Recovery_File_TestbedOrgXX.psrk',
    );
    await expect(importItems.nth(1)).not.toHaveTheClass('disabled');
    await expect(recoveryContainer.locator('.next-button').locator('ion-button')).toBeTrulyDisabled();
    await fillIonInput(
      importItems.nth(1).locator('div.recovery-list-item__input'),
      error === 'invalid-passphrase' ? 'UIKY-S9H9-KOII-QD51-9LHH-PHCE-JO28-T7TO-4JAO-9UR8-EO05-EJCZ-OH9P' : passphraseContent,
    );
    await expect(recoveryContainer.locator('.next-button').locator('ion-button')).toBeTrulyEnabled();
    await recoveryContainer.locator('.next-button').locator('ion-button').click();

    await expect(importItems).toBeHidden();
    const authContainer = recoveryContainer.locator('.choose-auth-page');
    await expect(authContainer).toBeVisible();
    const authNext = recoveryContainer.locator('.validate-button');
    await expect(authNext).toHaveText('Confirm');

    const authRadio = authContainer.locator('.radio-list-item:visible');
    await expect(authRadio).toHaveAuthentication({ pkiDisabled: true, keyringDisabled: true, ssoDisabled: true });
    await authRadio.nth(0).click();

    await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
    await expect(authNext).toHaveDisabledAttribute();
    await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
    await expect(authNext).toNotHaveDisabledAttribute();
    await authNext.click();

    await expect(myProfilePage).toShowToast(
      error === 'invalid-file' ? 'Invalid recovery file.' : 'The secret key does not match the recovery file.',
      'Error',
    );
  });
}

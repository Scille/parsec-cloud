// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, msTest } from '@tests/e2e/helpers';

msTest('Export recovery device', async ({ myProfilePage }) => {
  const container = myProfilePage.locator('.restore-password');
  await container.locator('.restore-password-button').click();
  await answerQuestion(myProfilePage, true, {
    expectedTitleText: 'Create a new recovery file?',
    expectedQuestionText: 'You have already created a recovery file in the past. Do you want to create a new one?',
    expectedPositiveText: 'Create a new recovery file',
    expectedNegativeText: 'Cancel',
  });
  await expect(myProfilePage.locator('.topbar-left').locator('.topbar-left__title')).toHaveText('Recovery file');
  const recoveryContainer = myProfilePage.locator('.recovery-container');
  const fileContainer = recoveryContainer.locator('.file-item').nth(0);
  const passphraseContainer = recoveryContainer.locator('.file-item').nth(1);

  await expect(fileContainer.locator('ion-button')).toHaveText('Download');
  await expect(passphraseContainer.locator('ion-button')).toHaveText('Download');

  /* Can't get the download to work properly */
  // const fileDownloadPromise = myProfilePage.waitForEvent('download');
  // await fileContainer.locator('ion-button').click();
  // const fileDownload = await fileDownloadPromise;
  // expect(fileDownload.suggestedFilename()).toBe('Parsec_Recovery_File_MyOrg.psrk');
  // const fileStream = await fileDownload.createReadStream();
  // fileStream.on('readable', async () => {
  //   fileStream.setEncoding('utf-8');
  //   expect(fileStream.read()).toBe('Q2lnYXJlQU1vdXN0YWNoZQ==');
  //   await fileDownload.delete();
  // });
  await fileContainer.locator('ion-button').click();
  await expect(myProfilePage).toShowToast('The recovery file was successfully downloaded', 'Success');

  // const passphraseDownloadPromise = myProfilePage.waitForEvent('download');
  // const passphraseDownload = await passphraseDownloadPromise;
  // expect(passphraseDownload.suggestedFilename()).toBe('Recovery.psrk');
  // const passphraseStream = await passphraseDownload.createReadStream();
  // passphraseStream.on('readable', async () => {
  //   passphraseStream.setEncoding('utf-8');
  //   expect(passphraseStream.read()).toBe('ABCDEF');
  // });

  await passphraseContainer.locator('ion-button').click();
  await expect(myProfilePage).toShowToast('The secret key was successfully downloaded', 'Success');

  await expect(fileContainer.locator('ion-button')).toHaveText('Download again');
  await expect(passphraseContainer.locator('ion-button')).toHaveText('Download again');
});

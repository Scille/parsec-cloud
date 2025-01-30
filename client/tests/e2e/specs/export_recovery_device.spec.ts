// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, msTest } from '@tests/e2e/helpers';

msTest('Export recovery device', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(3)).toHaveText('Organization recovery');
  await myProfilePage.locator('.menu-list__item').nth(3).click();
  const restorePassword = myProfilePage.locator('.recovery');
  await restorePassword.locator('.restore-password-button').locator('.button').click();
  await answerQuestion(myProfilePage, true, {
    expectedTitleText: 'Create a new recovery file?',
    expectedQuestionText: 'You have already created a recovery file in the past. Do you want to create a new one?',
    expectedPositiveText: 'Create a new recovery file',
    expectedNegativeText: 'Cancel',
  });
  const recoveryContainer = myProfilePage.locator('.organization-recovery-container');
  const fileContainer = recoveryContainer.locator('.recovery-item').nth(0);
  const passphraseContainer = recoveryContainer.locator('.recovery-item').nth(1);

  await expect(fileContainer.locator('ion-button')).toHaveText('Download');
  await expect(passphraseContainer.locator('ion-button')).toHaveText('Download');

  const fileDownloadPromise = myProfilePage.waitForEvent('download');
  await fileContainer.locator('ion-button').click();
  const fileDownload = await fileDownloadPromise;
  // expect(fileDownload.suggestedFilename()).toBe('XX');
  const fileStream = await fileDownload.createReadStream();
  let fileContent = '';
  for await (const chunk of fileStream) {
    fileContent += chunk.toString();
  }
  expect(fileContent).toBe('meow');
  await fileDownload.delete();
  await expect(myProfilePage).toShowToast('The recovery file was successfully downloaded', 'Success');

  const passphraseDownloadPromise = myProfilePage.waitForEvent('download');
  await passphraseContainer.locator('ion-button').click();
  const passphraseDownload = await passphraseDownloadPromise;
  // expect(passphraseDownload.suggestedFilename()).toBe('XX');
  const passphraseStream = await passphraseDownload.createReadStream();
  let codeContent = '';
  for await (const chunk of passphraseStream) {
    codeContent += chunk.toString();
  }
  expect(codeContent).toBe('ABCDEF');
  await expect(myProfilePage).toShowToast('The secret key was successfully downloaded', 'Success');

  await expect(fileContainer.locator('ion-button')).toHaveText('Download again');
  await expect(passphraseContainer.locator('ion-button')).toHaveText('Download again');
});

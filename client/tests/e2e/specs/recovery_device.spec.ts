// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DEFAULT_USER_INFORMATION, expect, fillIonInput, logout, msTest } from '@tests/e2e/helpers';

for (const authMode of ['password', 'sso']) {
  msTest(`Export and use recovery files with ${authMode}`, async ({ myProfilePage }) => {
    await expect(myProfilePage.locator('.menu-list__item').nth(3)).toHaveText('Access recovery');
    await myProfilePage.locator('.menu-list__item').nth(3).click();
    const recoveryPage = myProfilePage.locator('.recovery');
    const recoveryFileSection = recoveryPage.locator('.recovery-section--file');
    const exportRecoveryModal = myProfilePage.locator('.export-recovery-device-modal');
    await expect(recoveryPage.locator('.item-header__title')).toHaveText('Access recovery');
    await expect(exportRecoveryModal).toBeHidden();
    await expect(recoveryFileSection.locator('.action-button')).toHaveText('Create recovery files');
    await recoveryFileSection.locator('.action-button').click();
    await expect(exportRecoveryModal).toBeVisible();
    const recoverySteps = exportRecoveryModal.locator('.step-item');
    await expect(recoverySteps).toHaveCount(2);
    await expect(recoverySteps.locator('.step-item__title')).toContainText(['Secret Key', 'Recovery File']);
    await expect(exportRecoveryModal.locator('.confirmation-checkbox')).not.toBeChecked();
    const passphraseContent = ((await exportRecoveryModal.locator('.recovery-key .input-action-text').textContent()) ?? '').trim();
    expect(passphraseContent).toMatch(/^[A-Z0-9-]+$/);

    const fileDownloadPromise = myProfilePage.waitForEvent('download');
    await exportRecoveryModal.locator('.recovery-file .input-action-button').click();
    const fileDownload = await fileDownloadPromise;
    expect(fileDownload.suggestedFilename()).toMatch(/^Parsec_Recovery_File_TestbedOrg\d+\.psrk$/);
    const fileStream = await fileDownload.createReadStream();
    const chunks: Array<Buffer> = [];
    for await (const chunk of fileStream) {
      chunks.push(chunk);
    }
    const fileContent = Buffer.concat(chunks);
    expect(fileContent.byteLength).toBeGreaterThan(0);

    await exportRecoveryModal.locator('.confirmation-checkbox').click();
    await expect(exportRecoveryModal.locator('.confirmation-checkbox')).toBeChecked();
    await exportRecoveryModal.locator('#next-button').click();

    await expect(myProfilePage).toShowToast('Files downloaded', 'Success');

    await logout(myProfilePage);
    await expect(myProfilePage.locator('#recover-button-header')).toHaveText('Lost your access?');
    await myProfilePage.locator('#recover-button-header').click();

    const chooseRecoveryModal = myProfilePage.locator('.choose-recovery-device-modal');
    const importRecoveryModal = myProfilePage.locator('.import-recovery-device-modal');
    const authPage = myProfilePage.locator('.choose-auth-page');
    const recoveryMethodChoices = chooseRecoveryModal.locator('.recovery-method-item');
    const recoveryFilesContent = importRecoveryModal.locator('.recovery-device-files-page');

    await expect(chooseRecoveryModal).toBeVisible();
    await expect(recoveryFilesContent).toBeHidden();

    await expect(recoveryMethodChoices).toHaveCount(2);
    await expect(recoveryMethodChoices.locator('.recovery-method-item__title')).toHaveText([
      'Another connected device/browser',
      'Recovery file',
    ]);

    await recoveryMethodChoices.nth(1).click();
    await expect(recoveryMethodChoices.nth(1)).toHaveClass(/recovery-method-item--selected/);
    await chooseRecoveryModal.locator('#next-button').click();
    await expect(recoveryFilesContent).toBeVisible();

    const importItems = importRecoveryModal.locator('.recovery-device-files-page').locator('.recovery-item');
    await expect(importItems.nth(0).locator('.file-waiting__button')).toContainText('Add recovery file (.psrk)');

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
    await expect(importItems.nth(0).locator('.file-added__name')).toHaveText('Parsec_Recovery_File_TestbedOrgXX.psrk');
    await expect(importRecoveryModal.locator('#next-button')).toBeTrulyDisabled();
    await fillIonInput(importItems.nth(1).locator('ion-input.recovery-item__input'), passphraseContent);
    await expect(importRecoveryModal.locator('#next-button')).toBeTrulyEnabled();
    await expect(importRecoveryModal.locator('#next-button')).toBeVisible();
    await expect(importRecoveryModal.locator('.ms-modal-footer-buttons').locator('ion-button').nth(1)).toHaveText('Continue');
    await importRecoveryModal.locator('.ms-modal-footer-buttons').locator('ion-button').nth(1).click();

    await expect(authPage).toBeVisible();

    const authRadio = authPage.locator('.radio-list-item:visible');
    await expect(authRadio).toHaveAuthentication({ pkiDisabled: true, keyringDisabled: true, ssoDisabled: false });

    const nextButtonAuth = importRecoveryModal.locator('#next-button');
    if (authMode === 'password') {
      await authRadio.nth(0).click();

      await fillIonInput(authPage.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
      await expect(nextButtonAuth).toHaveDisabledAttribute();
      await fillIonInput(authPage.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
    } else {
      await authRadio.nth(3).click();
      await expect(authPage.locator('.proconnect-button')).toBeVisible();
      await authPage.locator('.proconnect-button').click();
    }
    await expect(nextButtonAuth).toNotHaveDisabledAttribute();
    await nextButtonAuth.click();

    const doneStep = myProfilePage.locator('.final-step');
    await expect(doneStep).toBeVisible();

    await expect(doneStep.locator('.final-step__title')).toHaveText('Your session has been recovered!');
    await doneStep.locator('.final-step__button').click();
    await expect(myProfilePage).toBeWorkspacePage();
    await logout(myProfilePage);

    await myProfilePage.locator('.organization-card').first().click();

    if (authMode === 'password') {
      await expect(myProfilePage.locator('#password-input')).toBeVisible();

      await expect(myProfilePage.locator('.login-button')).toHaveDisabledAttribute();

      await myProfilePage.locator('#password-input').locator('input').fill(DEFAULT_USER_INFORMATION.password);
      await expect(myProfilePage.locator('.login-button')).toBeEnabled();
      await myProfilePage.locator('.login-button').click();
    } else {
      await expect(myProfilePage.locator('.proconnect-button')).toBeVisible();
      await myProfilePage.locator('.proconnect-button').click();
    }

    await expect(myProfilePage.locator('#connected-header')).toContainText('My workspaces');
    await expect(myProfilePage.locator('.topbar-right').locator('.text-content-name')).toHaveText('Alicey McAliceFace');
    await expect(myProfilePage).toBeWorkspacePage();
  });
}

for (const error of ['invalid-passphrase', 'invalid-file']) {
  msTest(`Export and use recovery files with errors (${error})`, async ({ myProfilePage }) => {
    await expect(myProfilePage.locator('.menu-list__item').nth(3)).toHaveText('Access recovery');
    await myProfilePage.locator('.menu-list__item').nth(3).click();
    const recovery = myProfilePage.locator('.recovery');
    await expect(recovery.locator('.item-header__title')).toHaveText('Access recovery');
    const recoveryFileSection = recovery.locator('.recovery-section--file');
    await expect(recoveryFileSection).toBeVisible();
    await expect(recoveryFileSection.locator('.recovery-method-content-text__title')).toHaveText('Recovery files');
    await expect(recoveryFileSection.locator('.recovery-method-content-text__description')).toHaveText(
      'A recovery file combined with a secret key allow you to restore access to your data once assembled.',
    );
    await expect(recoveryFileSection.locator('.action-button')).toHaveText('Create recovery files');
    await expect(recoveryFileSection.locator('.badge-inactive')).toHaveText('Inactive');

    await recoveryFileSection.locator('.action-button').click();
    const exportRecoveryModal = myProfilePage.locator('.export-recovery-device-modal');
    await expect(exportRecoveryModal).toBeVisible();
    await expect(exportRecoveryModal.locator('.ms-modal-header__title')).toHaveText('Recovery files');
    await expect(exportRecoveryModal.locator('.step-item').nth(0).locator('.step-item__title')).toContainText('Secret Key');
    await expect(exportRecoveryModal.locator('.step-item').nth(1).locator('.step-item__title')).toContainText('Recovery File');

    const recoverySteps = exportRecoveryModal.locator('.step-item');
    await expect(recoverySteps).toHaveCount(2);
    await expect(recoverySteps.locator('.step-item__title')).toContainText(['Secret Key', 'Recovery File']);

    const passphraseContent = ((await exportRecoveryModal.locator('.recovery-key .input-action-text').textContent()) ?? '').trim();
    expect(passphraseContent).toMatch(/^[A-Z0-9-]+$/);

    const fileDownloadPromise = myProfilePage.waitForEvent('download');
    await exportRecoveryModal.locator('.recovery-file .input-action-button').click();
    const fileDownload = await fileDownloadPromise;
    expect(fileDownload.suggestedFilename()).toMatch(/^Parsec_Recovery_File_TestbedOrg\d+\.psrk$/);
    const fileStream = await fileDownload.createReadStream();
    const chunks: Array<Buffer> = [];
    for await (const chunk of fileStream) {
      chunks.push(chunk);
    }
    const fileContent = Buffer.concat(chunks);
    expect(fileContent.byteLength).toBeGreaterThan(0);

    await exportRecoveryModal.locator('.confirmation-checkbox').click();
    await expect(exportRecoveryModal.locator('.confirmation-checkbox')).toBeChecked();
    await exportRecoveryModal.locator('#next-button').click();

    await expect(myProfilePage).toShowToast('Files downloaded', 'Success');

    await logout(myProfilePage);
    await expect(myProfilePage.locator('#recover-button-header')).toHaveText('Lost your access?');
    await myProfilePage.locator('#recover-button-header').click();

    const chooseRecoveryModal = myProfilePage.locator('.choose-recovery-device-modal');
    const importRecoveryModal = myProfilePage.locator('.import-recovery-device-modal');
    const recoveryMethodChoices = chooseRecoveryModal.locator('.recovery-method-item');
    const recoveryFilesContent = importRecoveryModal.locator('.recovery-device-files-page');
    const authStep = importRecoveryModal.locator('.choose-auth-page');

    await expect(chooseRecoveryModal).toBeVisible();
    await expect(recoveryFilesContent).toBeHidden();

    await expect(recoveryMethodChoices).toHaveCount(2);
    await expect(recoveryMethodChoices.locator('.recovery-method-item__title')).toHaveText([
      'Another connected device/browser',
      'Recovery file',
    ]);

    await recoveryMethodChoices.nth(1).click();
    await expect(recoveryMethodChoices.nth(1)).toHaveClass(/recovery-method-item--selected/);
    await chooseRecoveryModal.locator('#next-button').click();
    await expect(recoveryFilesContent).toBeVisible();

    const importItems = recoveryFilesContent.locator('.recovery-item');
    await expect(importItems.nth(0).locator('.file-waiting__button')).toContainText('Add recovery file (.psrk)');

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
    await expect(importItems.nth(0).locator('.file-added__name')).toHaveText('Parsec_Recovery_File_TestbedOrgXX.psrk');
    await expect(importRecoveryModal.locator('#next-button')).toBeTrulyDisabled();
    await fillIonInput(
      importItems.nth(1).locator('ion-input.recovery-item__input'),
      error === 'invalid-passphrase' ? 'UIKY-S9H9-KOII-QD51-9LHH-PHCE-JO28-T7TO-4JAO-9UR8-EO05-EJCZ-OH9P' : passphraseContent,
    );
    await expect(importRecoveryModal.locator('#next-button')).toBeTrulyEnabled();
    await importRecoveryModal.locator('#next-button').click();

    await expect(myProfilePage).toShowToast(
      error === 'invalid-file' ? 'Invalid recovery file.' : 'The secret key does not match the recovery file.',
      'Error',
    );

    await expect(recoveryFilesContent).toBeVisible();
    await expect(authStep).toBeHidden();
  });
}

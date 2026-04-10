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
    await expect(myProfilePage.locator('.recovery-devices').locator('ion-button')).toHaveText('Recover my session');
    await myProfilePage.locator('.recovery-devices').locator('ion-button').click();

    const importStep = myProfilePage.locator('#recovery-import-step');
    const authStep = myProfilePage.locator('#recovery-auth-step');
    const doneStep = myProfilePage.locator('#recovery-success-step');

    await expect(importStep).toBeVisible();
    await expect(authStep).toBeHidden();
    await expect(doneStep).toBeHidden();

    const importItems = importStep.locator('.recovery-list-item');
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
    await expect(myProfilePage.locator('#to-password-change-btn')).toBeTrulyDisabled();
    await fillIonInput(importItems.nth(1).locator('div.recovery-list-item__input'), passphraseContent);
    await expect(myProfilePage.locator('#to-password-change-btn')).toBeTrulyEnabled();
    await myProfilePage.locator('#to-password-change-btn').click();

    await expect(importStep).toBeHidden();
    await expect(authStep).toBeVisible();
    await expect(doneStep).toBeHidden();

    const authContainer = authStep.locator('.choose-auth-page');
    await expect(authContainer).toBeVisible();
    const authNext = myProfilePage.locator('#validate-password-btn');
    await expect(authNext).toHaveText('Confirm');

    const authRadio = authContainer.locator('.radio-list-item:visible');
    await expect(authRadio).toHaveAuthentication({ pkiDisabled: true, keyringDisabled: true, ssoDisabled: false });

    if (authMode === 'password') {
      await authRadio.nth(0).click();

      await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
      await expect(authNext).toHaveDisabledAttribute();
      await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
    } else {
      await authRadio.nth(3).click();
      await expect(authContainer.locator('.proconnect-button')).toBeVisible();
      await authContainer.locator('.proconnect-button').click();
    }
    await expect(authNext).toNotHaveDisabledAttribute();
    await authNext.click();

    await expect(importStep).toBeHidden();
    await expect(authStep).toBeHidden();
    await expect(doneStep).toBeVisible();

    await expect(doneStep.locator('.success-card__title')).toHaveText('Authentication was successfully updated!');
    await doneStep.locator('ion-button').click();
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
    await expect(myProfilePage.locator('.recovery-devices').locator('ion-button')).toHaveText('Recover my session');
    await myProfilePage.locator('.recovery-devices').locator('ion-button').click();

    const importStep = myProfilePage.locator('#recovery-import-step');
    const authStep = myProfilePage.locator('#recovery-auth-step');

    await expect(importStep).toBeVisible();
    await expect(authStep).toBeHidden();

    const importItems = importStep.locator('.recovery-list-item');
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
    await expect(importStep.locator('#to-password-change-btn')).toBeTrulyDisabled();
    await fillIonInput(
      importItems.nth(1).locator('div.recovery-list-item__input'),
      error === 'invalid-passphrase' ? 'UIKY-S9H9-KOII-QD51-9LHH-PHCE-JO28-T7TO-4JAO-9UR8-EO05-EJCZ-OH9P' : passphraseContent,
    );
    await expect(importStep.locator('#to-password-change-btn')).toBeTrulyEnabled();
    await importStep.locator('#to-password-change-btn').click();

    await expect(myProfilePage).toShowToast(
      error === 'invalid-file' ? 'Invalid recovery file.' : 'The secret key does not match the recovery file.',
      'Error',
    );

    await expect(importStep).toBeVisible();
    await expect(authStep).toBeHidden();
  });
}

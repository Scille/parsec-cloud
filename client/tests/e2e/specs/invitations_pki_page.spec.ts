// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import { answerQuestion, expect, getClipboardText, MsPage, msTest, setWriteClipboardPermission } from '@tests/e2e/helpers';
import path from 'path';

async function setRootCertificate(page: MsPage, certificate = 'data/public_key.pem'): Promise<void> {
  const fileChooserPromise = page.waitForEvent('filechooser');

  if (await page.locator('.root-certificate').isVisible()) {
    await page.locator('.root-certificate').locator('ion-button').click();
  } else {
    await page.locator('.toggle-view-container').locator('#update-root-certificate-button').click();
  }
  const fileChooser = await fileChooserPromise;
  expect(fileChooser.isMultiple()).toBe(false);
  await fileChooser.setFiles([certificate]);
}

msTest.skip('PKI requests default state', async ({ invitationsPage }, testInfo: TestInfo) => {
  const viewToggle = invitationsPage.locator('.toggle-view-container');

  await expect(viewToggle.locator('.email-button')).toHaveText('Email invitation1');
  await expect(viewToggle.locator('.pki-button')).toHaveText('PKI requests3');
  await expect(viewToggle.locator('.email-button')).toBeTrulyDisabled();
  await expect(viewToggle.locator('.pki-button')).toBeTrulyEnabled();
  await viewToggle.locator('.pki-button').click();
  await expect(viewToggle.locator('.email-button')).toBeTrulyEnabled();
  await expect(viewToggle.locator('.pki-button')).toBeTrulyDisabled();

  const container = invitationsPage.locator('.invitations-container');
  const rootCertPopup = container.locator('.root-certificate');
  await expect(rootCertPopup).toBeVisible();
  await setRootCertificate(invitationsPage, path.join(testInfo.config.rootDir, 'data/public_key.pem'));
  await expect(rootCertPopup).toBeHidden();

  await expect(viewToggle.locator('#invite-user-button')).toBeHidden();
  await expect(viewToggle.locator('#update-root-certificate-button')).toBeVisible();
  await expect(viewToggle.locator('#copy-link-pki-request-button')).toBeVisible();
  await expect(viewToggle.locator('#copy-link-pki-request-button')).toHaveText('Copy link (PKI)');

  await expect(container.locator('.pkiRequests-container-list').locator('.pkiRequests-list-header__label')).toHaveText([
    'Name',
    'Email',
    'Request date',
    'Certificate',
    '',
  ]);
  const reqs = container.locator('.pkiRequests-container-list').locator('.pkiRequest-list-item');
  await expect(reqs).toHaveCount(3);
  await expect(reqs.nth(0).locator('.person-name')).toHaveText('Gordon Freeman');
  await expect(reqs.nth(0).locator('.pkiRequest-email')).toHaveText('gordon.freeman@blackmesa.nm');
  await expect(reqs.nth(0).locator('.pkiRequest-createdOn__label')).toHaveText(/^now|< 1 minute$/);
  await expect(reqs.nth(0).locator('.pkiRequest-certificate__label')).toHaveText('Invalid');
  const actions = reqs.nth(0).locator('.pkiRequest-actions').locator('ion-button');
  await expect(actions.nth(0)).toHaveText('Accept');

  const tooltip = invitationsPage.locator('.tooltip-popover');
  await expect(tooltip).toBeHidden();
  await actions.nth(1).hover();
  await expect(tooltip).toBeVisible();
  await expect(tooltip).toHaveText('Reject request');
  // Hide the tooltip
  await reqs.nth(0).locator('.pkiRequest-email').hover();
  // Not adding anything else right now because it's all mocked.
});

for (const hasPerms in [true, false]) {
  msTest.skip(
    `PKI requests copy link ${['without', 'with'][Number(hasPerms)]} permissions`,
    async ({ invitationsPage }, testInfo: TestInfo) => {
      const viewToggle = invitationsPage.locator('.toggle-view-container');
      await viewToggle.locator('.pki-button').click();
      const container = invitationsPage.locator('.invitations-container');
      const rootCertPopup = container.locator('.root-certificate');
      await expect(rootCertPopup).toBeVisible();
      await setRootCertificate(invitationsPage, path.join(testInfo.config.rootDir, 'data/public_key.pem'));
      await expect(rootCertPopup).toBeHidden();

      if (hasPerms) {
        await setWriteClipboardPermission(invitationsPage.context(), true);
      }
      await viewToggle.locator('#copy-link-pki-request-button').click();
      if (hasPerms) {
        await expect(invitationsPage).toShowToast('PKI link has been copied to clipboard.', 'Info');
        expect(await getClipboardText(invitationsPage)).toMatch(/^parsec3:\/\/.+$/);
      } else {
        await expect(invitationsPage).toShowToast(
          'Failed to copy the link. Your browser or device does not seem to support copy/paste.',
          'Error',
        );
      }
    },
  );
}

for (const answer of [true, false]) {
  msTest.skip(`PKI requests accept invalid cert answer ${answer}`, async ({ invitationsPage }, testInfo: TestInfo) => {
    const viewToggle = invitationsPage.locator('.toggle-view-container');
    await viewToggle.locator('.pki-button').click();
    const container = invitationsPage.locator('.invitations-container');
    const rootCertPopup = container.locator('.root-certificate');
    await expect(rootCertPopup).toBeVisible();
    await setRootCertificate(invitationsPage, path.join(testInfo.config.rootDir, 'data/public_key.pem'));
    await expect(rootCertPopup).toBeHidden();
    const reqs = container.locator('.pkiRequests-container-list').locator('.pkiRequest-list-item');
    await expect(reqs).toHaveCount(3);
    const modal = invitationsPage.locator('.select-profile-modal');
    await expect(modal).toBeHidden();
    const actions = reqs.nth(0).locator('.pkiRequest-actions').locator('ion-button');
    await actions.nth(0).click();
    await answerQuestion(invitationsPage, answer, {
      expectedNegativeText: 'Cancel',
      expectedPositiveText: 'Confirm adding user',
      expectedQuestionText:
        'The certificate provided by the user does not match the root certificate. Are you sure you want to add this user?',
      expectedTitleText: 'Adding a user with an invalid certificate',
    });
    if (answer) {
      await expect(modal).toBeVisible();
      const inputs = modal.locator('ion-input');
      await expect(inputs.locator('input').nth(0)).toHaveValue('Gordon Freeman');
      await expect(inputs.locator('input').nth(1)).toHaveValue('gordon.freeman@blackmesa.nm');
      await expect(modal.locator('.dropdown-button')).toHaveText('Select a profile');
      const dropdown = invitationsPage.locator('.dropdown-popover');
      await expect(dropdown).toBeHidden();
      await expect(modal.locator('#next-button')).toBeTrulyDisabled();
      await modal.locator('.dropdown-button').click();
      await expect(dropdown).toBeVisible();
      await expect(dropdown.locator('.option-text__label')).toHaveText(['Administrator', 'Member', 'External']);
      await dropdown.locator('.option-text__label').nth(0).click();
      await expect(modal.locator('#next-button')).toBeTrulyEnabled();
      await modal.locator('#next-button').click();
      await expect(modal).toBeHidden();
      await expect(invitationsPage).toShowToast('This request has been accepted', 'Success');
    } else {
      await expect(modal).toBeHidden();
    }
  });
}

msTest.skip('PKI requests reject request', async ({ invitationsPage }, testInfo: TestInfo) => {
  const viewToggle = invitationsPage.locator('.toggle-view-container');
  await viewToggle.locator('.pki-button').click();
  const container = invitationsPage.locator('.invitations-container');
  const rootCertPopup = container.locator('.root-certificate');
  await expect(rootCertPopup).toBeVisible();
  await setRootCertificate(invitationsPage, path.join(testInfo.config.rootDir, 'data/public_key.pem'));
  await expect(rootCertPopup).toBeHidden();
  const reqs = container.locator('.pkiRequests-container-list').locator('.pkiRequest-list-item');
  await expect(reqs).toHaveCount(3);
  const actions = reqs.nth(0).locator('.pkiRequest-actions').locator('ion-button');
  await actions.nth(1).click();
  await expect(invitationsPage).toShowToast('This request has been rejected', 'Info');
});

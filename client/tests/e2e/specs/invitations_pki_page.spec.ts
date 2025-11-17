// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, getClipboardText, msTest, setWriteClipboardPermission } from '@tests/e2e/helpers';

msTest('PKI requests default state', async ({ invitationsPage }) => {
  const viewToggle = invitationsPage.locator('.toggle-view-container');

  await expect(viewToggle.locator('.email-button')).toHaveText('Email invitation1');
  await expect(viewToggle.locator('.pki-button')).toHaveText('PKI requests6');
  await expect(viewToggle.locator('.email-button')).toBeTrulyDisabled();
  await expect(viewToggle.locator('.pki-button')).toBeTrulyEnabled();
  await viewToggle.locator('.pki-button').click();
  await expect(viewToggle.locator('.email-button')).toBeTrulyEnabled();
  await expect(viewToggle.locator('.pki-button')).toBeTrulyDisabled();

  const container = invitationsPage.locator('.invitations-container');
  const rootCertPopup = container.locator('.root-certificate');
  await expect(rootCertPopup).toBeVisible();
  await expect(rootCertPopup.locator('.root-certificate-text__title')).toHaveText('Missing your certificate');
  await expect(rootCertPopup.locator('.root-certificate-text__description')).toHaveText(
    `A certificate is required to validate requests received by PKI.
For security reasons, the certificate must be provided each time the organization is reopened.`,
  );

  await expect(rootCertPopup.locator('ion-button')).toHaveText('Set your certificate');
  await rootCertPopup.locator('ion-button').click();
  await expect(rootCertPopup).toBeHidden();

  await expect(viewToggle.locator('#invite-user-button')).toBeHidden();
  await expect(viewToggle.locator('#update-root-certificate-button')).toBeVisible();
  await expect(viewToggle.locator('#copy-link-pki-request-button')).toBeVisible();
  await expect(viewToggle.locator('#copy-link-pki-request-button')).toHaveText('Copy link (PKI)');

  const invalidRequests = container.locator('.requests-container-list').locator('.invalid-request');
  await expect(invalidRequests).toHaveCount(5);
  await expect(invalidRequests.locator('.invalid-request-text__description')).toHaveText([
    'This certificate cannot be trusted.',
    'This certificate is invalid.',
    'Could not find a root certificate matching the provided certificate.',
    'The provided certificate is expired.',
    'Could not get any information in the certificate.',
  ]);

  await expect(container.locator('.requests-container-list').locator('.requests-list-header__label')).toHaveText([
    'Name',
    'Email',
    'Request date',
    'Certificate',
    '',
  ]);
  const reqs = container.locator('.requests-container-list').locator('.request-list-item');
  await expect(reqs).toHaveCount(1);
  await expect(reqs.nth(0).locator('.person-name')).toHaveText('Gordon Freeman');
  await expect(reqs.nth(0).locator('.request-email')).toHaveText('gordon.freeman@blackmesa.nm');
  await expect(reqs.nth(0).locator('.request-createdOn__label')).toHaveText(/^now|< 1 minute$/);
  await expect(reqs.nth(0).locator('.request-certificate__label')).toHaveText('Valid');
  const actions = reqs.nth(0).locator('.request-actions').locator('ion-button');
  await expect(actions.nth(0)).toHaveText('Accept');

  const tooltip = invitationsPage.locator('.tooltip-popover');
  await expect(tooltip).toBeHidden();
  await actions.nth(1).hover();
  await expect(tooltip).toBeVisible();
  await expect(tooltip).toHaveText('Reject request');
  // Hide the tooltip
  await reqs.nth(0).locator('.request-email').hover();
  // Not adding anything else right now because it's all mocked.
});

msTest('PKI requests accept request', async ({ invitationsPage }) => {
  const viewToggle = invitationsPage.locator('.toggle-view-container');

  await expect(viewToggle.locator('.email-button')).toHaveText('Email invitation1');
  await expect(viewToggle.locator('.pki-button')).toHaveText('PKI requests6');
  await viewToggle.locator('.pki-button').click();

  const container = invitationsPage.locator('.invitations-container');
  const rootCertPopup = container.locator('.root-certificate');
  await expect(rootCertPopup).toBeVisible();
  await rootCertPopup.locator('ion-button').click();
  await expect(rootCertPopup).toBeHidden();

  const reqs = container.locator('.requests-container-list').locator('.request-list-item');
  await expect(reqs).toHaveCount(1);
  const actions = reqs.nth(0).locator('.request-actions').locator('ion-button');
  await expect(actions.nth(0)).toHaveText('Accept');
  const modal = invitationsPage.locator('.select-profile-modal');
  await expect(modal).toBeHidden();
  await actions.nth(0).click();
  await expect(modal).toBeVisible();
});

for (const hasPerms in [true, false]) {
  msTest(`PKI requests copy link ${['without', 'with'][Number(hasPerms)]} permissions`, async ({ invitationsPage }) => {
    const viewToggle = invitationsPage.locator('.toggle-view-container');
    await viewToggle.locator('.pki-button').click();

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
  });
}

msTest('PKI requests reject request', async ({ invitationsPage }) => {
  const viewToggle = invitationsPage.locator('.toggle-view-container');

  await expect(viewToggle.locator('.email-button')).toHaveText('Email invitation1');
  await expect(viewToggle.locator('.pki-button')).toHaveText('PKI requests6');
  await viewToggle.locator('.pki-button').click();

  const container = invitationsPage.locator('.invitations-container');
  const rootCertPopup = container.locator('.root-certificate');
  await rootCertPopup.locator('ion-button').click();
  await expect(rootCertPopup).toBeHidden();

  const invalidRequests = container.locator('.requests-container-list').locator('.invalid-request');
  await expect(invalidRequests).toHaveCount(5);
  await invalidRequests.nth(0).locator('.invalid-request__button').click();
  await expect(invitationsPage).toShowToast('This request has been rejected', 'Info');
});

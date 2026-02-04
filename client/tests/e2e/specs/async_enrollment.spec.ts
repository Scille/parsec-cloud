// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  answerQuestion,
  expect,
  fillInputModal,
  fillIonInput,
  getOrganizationAddr,
  login,
  logout,
  MsPage,
  msTest,
  setupNewPage,
} from '@tests/e2e/helpers';

function getAsyncEnrollmentJoinLink(orgName: string): string {
  const orgAddr = getOrganizationAddr(orgName);
  if (orgAddr.includes('?')) {
    return `${orgAddr}&a=async_enrollment`;
  } else {
    return `${orgAddr}?a=async_enrollment`;
  }
}

async function addRequest(page: MsPage, identitySystem: 'pki' | 'openbao'): Promise<void> {
  const requests = page.locator('.organization-request');
  await expect(requests).toBeHidden();
  const orgName = await page.locator('.organization-card').first().locator('.organization-name').textContent();
  expect(orgName).not.toBeNull();
  const link = getAsyncEnrollmentJoinLink(orgName as string);
  await page.locator('#create-organization-button').click();
  await expect(page.locator('.homepage-popover')).toBeVisible();
  await page.locator('.homepage-popover').getByRole('listitem').nth(1).click();
  await fillInputModal(page, link);
  const requestModal = page.locator('.async-enrollment-modal');
  await expect(requestModal).toBeVisible();
  const nextButton = requestModal.locator('#next-button');
  const previousButton = requestModal.locator('#cancel-button');

  await expect(requestModal.locator('.ms-modal-header__title')).toHaveText(`Join organization ${orgName}`);
  await expect(nextButton).toBeTrulyDisabled();
  await expect(nextButton).toHaveText('Continue');
  await expect(previousButton).toHaveText('Cancel');

  if (identitySystem === 'pki') {
    await expect(requestModal.locator('.choose-method')).toBeVisible();
    const options = requestModal.locator('.choose-method').locator('.choose-method-options-item');
    await expect(options.locator('.title-h4')).toHaveText(['Continue with PKI', 'Continue with Single Sign-On']);
    await options.nth(0).click();
    await expect(nextButton).toBeTrulyEnabled();
    await nextButton.click();
  }
  await expect(requestModal.locator('.modal-info')).toBeVisible();

  if (identitySystem === 'pki') {
    await expect(requestModal.locator('.async-authentication-modal-header__title')).toHaveText('Certificate Authentication (PKI)');
    await requestModal.locator('.choose-certificate-button').click();
  } else {
    await expect(requestModal.locator('.async-authentication-modal-header__title')).toHaveText(
      'Log in with SSO to access the organization.',
    );
    await requestModal.locator('.proconnect-button').click();
    await expect(requestModal.locator('.modal-info')).toBeHidden();
    await expect(requestModal.locator('.user-information')).toBeVisible();
    await fillIonInput(requestModal.locator('.user-information').locator('ion-input'), 'Gordon Freeman');
    await expect(requestModal.locator('.user-information').locator('.dropdown-button-content')).toHaveText(/^[a-f0-9-]+@example\.invalid$/);
  }
  await expect(nextButton).toBeTrulyEnabled();
  await nextButton.click();
  await expect(page).toShowToast('Your request to join the organization has been sent.', 'Success');
  await expect(requestModal).toBeHidden();
  await expect(requests).toHaveCount(1);
  await expect(requests.locator('.organization-request-organization')).toHaveText(orgName as string);
  await expect(requests.locator('.organization-request-username')).toHaveText('Gordon Freeman');
  await expect(requests.locator('.organization-request-status')).toHaveText('Pending');
}

for (const identitySystem of ['pki', 'openbao']) {
  msTest(`Async enrollment using ${identitySystem}`, async ({ context }) => {
    const page = (await context.newPage()) as MsPage;
    await setupNewPage(page, { mockPki: identitySystem === 'pki' });

    await addRequest(page, identitySystem as 'pki' | 'openbao');

    const requests = page.locator('.organization-request');

    await login(page, 'Alicey McAliceFace');

    await page.locator('.sidebar').locator('#sidebar-invitations').click();
    await expect(page).toHavePageTitle('Invitations & Requests');

    await expect(page.locator('.toggle-view-container').locator('.pki-button').locator('.toggle-view-button__label')).toHaveText(
      'Link requests',
    );
    await expect(page.locator('.toggle-view-container').locator('.pki-button').locator('.toggle-view-button__count')).toHaveText('1');
    await page.locator('.toggle-view-container').locator('.pki-button').click();

    const linkRequests = page.locator('.invitations-container').locator('.request-list-item');
    await expect(linkRequests).toHaveCount(1);
    await expect(linkRequests.nth(0).locator('.request-type__label')).toHaveText(identitySystem === 'pki' ? 'PKI' : 'SSO');
    await expect(linkRequests.nth(0).locator('.person-name')).toHaveText('Gordon Freeman');
    await expect(linkRequests.nth(0).locator('.request-email__label')).toHaveText(
      identitySystem === 'pki' ? 'gordon.freeman@blackmesa.nm' : /^ [a-f0-9-]+@example\.invalid$/,
    );

    const acceptButton = linkRequests.nth(0).locator('.request-actions').locator('ion-button').nth(0);
    await expect(acceptButton).toHaveText('Accept');
    const acceptModal = page.locator(identitySystem === 'pki' ? '.async-enrollment-pki-modal' : '.async-enrollment-openbao-modal');
    await expect(acceptModal).toBeHidden();
    await acceptButton.click();
    await expect(acceptModal).toBeVisible();

    if (identitySystem === 'pki') {
      await expect(acceptModal.locator('.async-authentication-modal-header__title')).toHaveText(
        'A certificate is required to accept the request',
      );
      await expect(acceptModal.locator('.async-authentication-modal-text')).toHaveText(
        'A certificate is required to validate requests received by PKI. ' +
          'For security reasons, it must be provided each time the organization is reopened.',
      );
      await acceptModal.locator('.choose-certificate-button').click();
    } else {
      await acceptModal.locator('.proconnect-button').click();
      await expect(acceptModal.locator('.proconnect-group--connected')).toBeVisible();
      await expect(acceptModal.locator('.proconnect-group--connected')).toHaveText('Connected');
      await acceptModal.locator('#next-button').click();
    }
    await expect(acceptModal).toBeHidden();
    const selectProfileModal = page.locator('.select-profile-modal');
    await expect(selectProfileModal).toBeVisible();
    await expect(selectProfileModal.locator('.user-details').locator('ion-input').nth(0).locator('input')).toHaveValue('Gordon Freeman');
    await expect(selectProfileModal.locator('.user-details').locator('ion-input').nth(1).locator('input')).toHaveValue(
      identitySystem === 'pki' ? 'gordon.freeman@blackmesa.nm' : /^[a-f0-9-]+@example\.invalid$/,
    );
    await expect(selectProfileModal.locator('#dropdown-popover-button')).toHaveText('Select a profile');
    await expect(selectProfileModal.locator('#next-button')).toHaveText('Validate request');
    await expect(selectProfileModal.locator('#next-button')).toBeTrulyDisabled();
    const profilePopover = page.locator('.dropdown-popover');
    await expect(profilePopover).toBeHidden();
    await selectProfileModal.locator('#dropdown-popover-button').click();
    await expect(profilePopover).toBeVisible();
    await expect(profilePopover.locator('.option').locator('.option-text__label')).toHaveText(['Administrator', 'Member', 'External']);
    await profilePopover.locator('.option').nth(1).click();
    await expect(selectProfileModal.locator('#next-button')).toBeTrulyEnabled();
    await selectProfileModal.locator('#next-button').click();
    await expect(page).toShowToast('This request has been accepted', 'Success');

    await logout(page);

    await expect(requests).toHaveCount(1);
    await expect(requests.nth(0).locator('.organization-request-button')).toBeVisible();
    await expect(requests.nth(0).locator('.organization-request-button')).toHaveText('Add to my organizations');
    await requests.nth(0).locator('.organization-request-button').click();

    if (identitySystem !== 'pki') {
      const finalizeModal = page.locator('.async-enrollment-openbao-modal');
      await expect(finalizeModal).toBeVisible();
      await finalizeModal.locator('.proconnect-button').click();
      await expect(finalizeModal.locator('#next-button')).toBeVisible();
      await finalizeModal.locator('#next-button').click();
      await expect(page).toShowToast('You successfully joined the organization.', 'Success');
      await expect(page).toBeWorkspacePage();
    } else {
      // Using a fake device so we can't log in, and testing for toast causes problems because two toasts
      // are opened one after the other.
      await expect(requests).toHaveCount(0);
    }

    await page.release();
  });

  msTest(`Cancel request using ${identitySystem}`, async ({ context }) => {
    const page = (await context.newPage()) as MsPage;
    await setupNewPage(page, { mockPki: identitySystem === 'pki' });

    await addRequest(page, identitySystem as 'pki' | 'openbao');

    const requests = page.locator('.organization-request');

    await expect(requests).toHaveCount(1);
    await requests.nth(0).locator('.organization-request-icon').click();
    await answerQuestion(page, true, {
      expectedNegativeText: 'No, wait for validation',
      expectedPositiveText: 'Cancel request',
      expectedQuestionText: 'Your request to join the organization is still pending. Are you sure you want to cancel it?',
      expectedTitleText: 'Organization request pending',
    });
    await expect(requests).toHaveCount(0);

    // Check that the admin doesn't see the request
    await login(page, 'Alicey McAliceFace');

    await page.locator('.sidebar').locator('#sidebar-invitations').click();
    await expect(page).toHavePageTitle('Invitations & Requests');

    await expect(page.locator('.toggle-view-container').locator('.pki-button').locator('.toggle-view-button__label')).toHaveText(
      'Link requests',
    );
    await expect(page.locator('.toggle-view-container').locator('.pki-button').locator('.toggle-view-button__count')).toHaveText('0');
    await page.locator('.toggle-view-container').locator('.pki-button').click();

    const linkRequests = page.locator('.invitations-container').locator('.request-list-item');
    await expect(linkRequests).toHaveCount(0);

    await expect(page.locator('.invitations-container').locator('.no-active-content')).toBeVisible();
    await expect(page.locator('.invitations-container').locator('.no-active-content')).toHaveText(
      'No pending link requests. These requests are for PKI or SSO users.',
    );

    await page.release();
  });

  msTest(`Reject async enrollment using ${identitySystem}`, async ({ context }) => {
    const page = (await context.newPage()) as MsPage;
    await setupNewPage(page, { mockPki: identitySystem === 'pki' });

    await addRequest(page, identitySystem as 'pki' | 'openbao');

    const requests = page.locator('.organization-request');

    await login(page, 'Alicey McAliceFace');

    await page.locator('.sidebar').locator('#sidebar-invitations').click();
    await expect(page).toHavePageTitle('Invitations & Requests');

    await expect(page.locator('.toggle-view-container').locator('.pki-button').locator('.toggle-view-button__label')).toHaveText(
      'Link requests',
    );
    await expect(page.locator('.toggle-view-container').locator('.pki-button').locator('.toggle-view-button__count')).toHaveText('1');
    await page.locator('.toggle-view-container').locator('.pki-button').click();

    const linkRequests = page.locator('.invitations-container').locator('.request-list-item');
    await expect(linkRequests).toHaveCount(1);
    await expect(linkRequests.nth(0).locator('.request-type__label')).toHaveText(identitySystem === 'pki' ? 'PKI' : 'SSO');
    await expect(linkRequests.nth(0).locator('.person-name')).toHaveText('Gordon Freeman');
    await expect(linkRequests.nth(0).locator('.request-email__label')).toHaveText(
      identitySystem === 'pki' ? 'gordon.freeman@blackmesa.nm' : /^ [a-f0-9-]+@example\.invalid$/,
    );

    const rejectButton = linkRequests.nth(0).locator('.request-actions').locator('ion-button').nth(1);
    await rejectButton.click();
    await expect(page).toShowToast('This request has been rejected', 'Info');
    await expect(linkRequests).toHaveCount(0);
    await expect(page.locator('.invitations-container').locator('.no-active-content')).toBeVisible();
    await expect(page.locator('.invitations-container').locator('.no-active-content')).toHaveText(
      'No pending link requests. These requests are for PKI or SSO users.',
    );

    await logout(page);

    await expect(requests).toHaveCount(1);
    await expect(requests.nth(0).locator('.organization-request-status')).toHaveText('Rejected');
    await requests.nth(0).locator('.organization-request-icon').click();
    await expect(page).toShowToast('Your request to join the organization has been deleted.', 'Info');
    await expect(requests).toHaveCount(0);

    await page.release();
  });
}

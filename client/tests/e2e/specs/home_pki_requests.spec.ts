// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, fillInputModal, fillIonInput, getOrganizationAddr, msTest } from '@tests/e2e/helpers';
import { randomInt } from 'crypto';

function getPkiJoinLink(orgName: string): string {
  const orgAddr = getOrganizationAddr(orgName);
  if (orgAddr.includes('?')) {
    return `${orgAddr}&a=pki_enrollment`;
  } else {
    return `${orgAddr}?a=pki_enrollment`;
  }
}

msTest('List pki requests', async ({ home }) => {
  const requests = home.locator('.organization-list').locator('.organization-request');
  await expect(requests).toHaveCount(4);
  await expect(home.locator('.organization-list__title').nth(0)).toHaveText('Pending requests');
  await expect(home.locator('.organization-list__title').nth(1)).toHaveText('Your organizations');

  await expect(requests.locator('.organization-request-organization')).toHaveText(['BlackMesa', 'BlackMesa', 'BlackMesa', 'BlackMesa']);
  // cspell: disable-next-line
  await expect(requests.locator('.organization-request-username')).toHaveText([
    'Gordon Freeman',
    'Gordon Freeman',
    'Gordon Freeman',
    'Gordon Freeman',
  ]);
  await expect(requests.locator('.organization-request-status')).toHaveText(['', 'Cancelled', 'Rejected', 'Pending']);
  await expect(requests.locator('.organization-request-button').nth(0)).toBeVisible();
  await expect(requests.locator('.organization-request-button').nth(0)).toHaveText('Join');
  await expect(requests.locator('.organization-request-status').nth(0)).toHaveTheClass('status-accepted');
  await expect(requests.locator('.organization-request-status').nth(1)).toHaveTheClass('status-cancelled');
  await expect(requests.locator('.organization-request-status').nth(2)).toHaveTheClass('status-rejected');
  await expect(requests.locator('.organization-request-status').nth(3)).toHaveTheClass('status-pending');
});

msTest('Create new pki request', async ({ home }) => {
  await home.locator('#create-organization-button').click();

  await expect(home.locator('.homepage-popover')).toBeVisible();
  await home.locator('.homepage-popover').getByRole('listitem').nth(1).click();
  const modal = home.locator('.join-pki-modal');
  await expect(modal).toBeHidden();
  const uniqueOrgName = `${home.orgInfo.name}-${randomInt(2 ** 47)}`;
  await fillInputModal(home, getPkiJoinLink(uniqueOrgName));
  await expect(modal).toBeVisible();
  await expect(modal.locator('.ms-modal-header__title')).toHaveText('Join organization with Smartcard');
  const confirmButton = modal.locator('#next-button');
  await expect(confirmButton).toBeTrulyDisabled();
  await fillIonInput(modal.locator('ion-input').nth(0), 'Gordon Freeman');
  await fillIonInput(modal.locator('ion-input').nth(1), 'gordon.freeman@blackmesa.nm');
  await expect(confirmButton).toBeTrulyDisabled();
  await expect(modal.locator('.certificate-valid')).toBeHidden();
  await modal.locator('.certificate-button').click();
  await expect(modal.locator('.certificate-valid')).toBeVisible();
  await expect(modal.locator('.certificate-valid__text')).toHaveText('Certificate selected');
  await expect(confirmButton).toBeTrulyEnabled();
  await confirmButton.click();
  await expect(home).toShowToast('Your request to join the organization has been sent.', 'Success');
});

msTest('Cancel pki request answer', async ({ home }) => {
  const requests = home.locator('.organization-list').locator('.organization-request');
  await expect(requests).toHaveCount(4);

  await requests.nth(2).locator('.organization-request-icon').click();
  await answerQuestion(home, true, {
    expectedNegativeText: 'Keep request',
    expectedPositiveText: 'Delete request',
    expectedQuestionText: 'Your request to join the organization has been rejected by an administrator. Do you want to delete it?',
    expectedTitleText: 'Organization request rejected',
  });
  await expect(home).toShowToast('Your request to join the organization has been deleted.', 'Success');
  await requests.nth(3).locator('.organization-request-icon').click();
  await answerQuestion(home, true, {
    expectedNegativeText: 'No, wait for validation',
    expectedPositiveText: 'Cancel request',
    expectedQuestionText: 'Your request to join the organization is still pending. Are you sure you want to cancel it?',
    expectedTitleText: 'Organization request pending',
  });
  await expect(home).toShowToast('Your request to join the organization has been cancelled.', 'Success');
});

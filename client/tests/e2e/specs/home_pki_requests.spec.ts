// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, fillInputModal, getOrganizationAddr, msTest } from '@tests/e2e/helpers';
import { randomInt } from 'crypto';

function getPkiJoinLink(orgName: string): string {
  const orgAddr = getOrganizationAddr(orgName);
  if (orgAddr.includes('?')) {
    return `${orgAddr}&a=pki_enrollment`;
  } else {
    return `${orgAddr}?a=pki_enrollment`;
  }
}

msTest('Create new pki request', async ({ home }) => {
  const requests = home.locator('.organization-list').locator('.organization-request');
  await expect(requests).toHaveCount(0);
  await expect(home.locator('.organization-list__title').nth(0)).toBeHidden();

  await home.locator('#create-organization-button').click();

  await expect(home.locator('.homepage-popover')).toBeVisible();
  await home.locator('.homepage-popover').getByRole('listitem').nth(1).click();

  const uniqueOrgName = `${home.orgInfo.name}-${randomInt(2 ** 47)}`;
  await fillInputModal(home, getPkiJoinLink(uniqueOrgName));
  console.log(getPkiJoinLink(uniqueOrgName));
  await expect(requests).toHaveCount(1);
  await expect(home).toShowToast('Your request to join the organization has been sent.', 'Success');
  await expect(requests.nth(0).locator('.organization-request-organization')).toHaveText('Black Mesa');
  // cspell: disable-next-line
  await expect(requests.nth(0).locator('.organization-request-username')).toHaveText('Isaac Kleiner');
  await expect(requests.nth(0).locator('.organization-request-status')).toHaveText('Pending');
  await expect(requests.nth(0).locator('.organization-request-status')).toHaveTheClass('status-pending');
  await expect(home.locator('.organization-list__title').nth(0)).toHaveText('Pending requests');
  await expect(home.locator('.organization-list__title').nth(1)).toHaveText('Your organizations');
});

for (const answer of [true, false]) {
  msTest(`Cancel pending pki request answer ${['no', 'yes'][Number(answer)]}`, async ({ home }) => {
    const requests = home.locator('.organization-list').locator('.organization-request');
    await expect(requests).toHaveCount(0);

    await home.locator('#create-organization-button').click();

    await expect(home.locator('.homepage-popover')).toBeVisible();
    await home.locator('.homepage-popover').getByRole('listitem').nth(1).click();

    const uniqueOrgName = `${home.orgInfo.name}-${randomInt(2 ** 47)}`;
    await fillInputModal(home, getPkiJoinLink(uniqueOrgName));
    await expect(requests).toHaveCount(1);
    await expect(home).toShowToast('Your request to join the organization has been sent.', 'Success');
    await requests.nth(0).locator('.organization-request-icon').click();
    await answerQuestion(home, answer, {
      expectedNegativeText: 'No, wait for validation',
      expectedPositiveText: 'Cancel request',
      expectedQuestionText: 'Your request to join the organization is still pending. Are you sure you want to cancel it?',
      expectedTitleText: 'Organization request pending',
    });
    if (answer === false) {
      await expect(requests).toHaveCount(1);
    } else {
      await expect(requests).toHaveCount(0);
    }
  });
}

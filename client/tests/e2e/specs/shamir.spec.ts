// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator } from '@playwright/test';
import {
  answerQuestion,
  expect,
  fillInputModal,
  fillIonInput,
  getClipboardText,
  login,
  logout,
  MsPage,
  msTest,
  setupNewPage,
  setWriteClipboardPermission,
} from '@tests/e2e/helpers';

msTest('Setup shamir recovery', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  await setupNewPage(page, {
    enableShamir: true,
    additionalUsers: [
      {
        // cspell:disable-next-line
        label: 'Quelaag',
        profile: 'UserProfileAdmin',
      },
      {
        label: 'Gwyn',
        profile: 'UserProfileAdmin',
      },
      {
        // cspell:disable-next-line
        label: 'Smough',
        profile: 'UserProfileStandard',
      },
      {
        label: 'Sif',
        profile: 'UserProfileStandard',
      },
      {
        label: 'Priscilla',
        profile: 'UserProfileOutsider',
      },
      {
        // cspell:disable-next-line
        label: 'Artorias',
        profile: 'UserProfileAdmin',
      },
      {
        label: 'Manus',
        profile: 'UserProfileStandard',
      },
    ],
  });

  await login(page, 'Alicey McAliceFace');
  await page.locator('.topbar').locator('.profile-header').click();
  const myProfileButton = page.locator('.profile-header-organization-popover').locator('.main-list').getByRole('listitem').nth(3);
  await expect(myProfileButton).toHaveText('Access recovery');
  await myProfileButton.click();
  await expect(page).toBeMyProfilePage();
  const container = page.locator('#recovery-profile-content');
  await expect(container).toBeVisible();
  await expect(page.locator('.recovery-section')).toHaveCount(3);
  const shamirContainer = page.locator('.recovery-section').nth(2);
  const status = shamirContainer.locator('.recovery-method-state');
  await expect(shamirContainer.locator('.recovery-method-content-text__description')).toHaveText(
    'Designate at least 2 trusted people. In case of lost access, they can help you regain access to your account.',
  );
  await expect(status).toHaveText('Inactive');
  const buttons = shamirContainer.locator('ion-button');
  await expect(buttons).toHaveText(['Manage my people of trust', 'Help recover an access']);
  const shamirModal = page.locator('.shamir-recovery-modal');
  await expect(shamirModal).toBeHidden();
  await buttons.nth(0).click();
  await expect(shamirModal).toBeVisible();
  await expect(shamirModal.locator('.shamir-information-header__title')).toHaveText('Designate trusted people');
  await expect(shamirModal.locator('.shamir-information-button')).toHaveText('Choose my trusted people');
  await shamirModal.locator('.shamir-information-button').click();
  await expect(shamirModal.locator('.shamir-setup-threshold-header__number')).toHaveText('0 / 2 minimum');
  const addedUsers = shamirModal.locator('.shamir-setup-list-item');
  await expect(addedUsers).toHaveCount(0);
  await expect(shamirModal.locator('#setup-shamir-button')).toBeTrulyDisabled();
  await expect(shamirModal.locator('#setup-shamir-button')).toHaveText('Create my circle of trust');
  await expect(shamirModal.locator('.shamir-setup-search-dropdown')).toBeHidden();
  const input = shamirModal.locator('.shamir-setup-search__input');
  await input.click();
  await expect(shamirModal.locator('.shamir-setup-search-dropdown')).toBeVisible();
  const dropdownItems = shamirModal.locator('.shamir-setup-search-dropdown').getByRole('listitem');
  await expect(dropdownItems).toHaveCount(7);
  await fillIonInput(input, 'u');
  await expect(dropdownItems).toHaveCount(3);
  await fillIonInput(input, '');
  await expect(dropdownItems).toHaveCount(7);
  const checkbox = shamirModal.locator('.ms-checkbox');
  await expect(checkbox).toBeHidden();
  await dropdownItems.nth(0).click();
  await expect(shamirModal.locator('.shamir-setup-threshold-header__number')).toHaveText('1 / 2 minimum');
  await dropdownItems.nth(1).click();
  await expect(shamirModal.locator('.shamir-setup-threshold-header__number')).toHaveText('2 selected');
  await dropdownItems.nth(2).click();
  await expect(shamirModal.locator('.shamir-setup-threshold-header__number')).toHaveText('3 selected');
  await expect(addedUsers).toHaveCount(3);
  // cspell:disable-next-line
  await expect(addedUsers.locator('.shamir-setup-list-item__text-label')).toHaveText(['Artorias', 'Boby McBobFace', 'Gwyn']);
  // Close the dropdown
  await shamirModal.locator('.shamir-setup-threshold-header__number').click();
  await expect(checkbox).toBeVisible();
  await checkbox.click();
  await expect(shamirModal.locator('#setup-shamir-button')).toBeTrulyEnabled();
  await shamirModal.locator('#setup-shamir-button').click();
  await expect(shamirModal.locator('.shamir-done')).toBeVisible();
  await expect(shamirModal.locator('.shamir-done-list-item').locator('.shamir-done-list-item__text-label')).toHaveText([
    // cspell:disable-next-line
    'Artorias',
    'Boby McBobFace',
    'Gwyn',
  ]);
  await shamirModal.locator('.shamir-done-footer__button').nth(1).click();
  await expect(shamirModal).toBeHidden();
  await expect(status).toHaveText('Active');
  await page.release();
});

msTest('Setup shamir recovery as outsider', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  await setupNewPage(page, {
    enableShamir: true,
    additionalUsers: [
      {
        // cspell:disable-next-line
        label: 'Quelaag',
        profile: 'UserProfileAdmin',
      },
      {
        label: 'Gwyn',
        profile: 'UserProfileAdmin',
      },
      {
        // cspell:disable-next-line
        label: 'Smough',
        profile: 'UserProfileStandard',
      },
      {
        label: 'Sif',
        profile: 'UserProfileStandard',
      },
      {
        label: 'Priscilla',
        profile: 'UserProfileOutsider',
      },
      {
        // cspell:disable-next-line
        label: 'Artorias',
        profile: 'UserProfileAdmin',
      },
      {
        label: 'Manus',
        profile: 'UserProfileStandard',
      },
    ],
  });

  // To setup the users
  await login(page, 'Alicey McAliceFace');
  await logout(page);
  await login(page, 'Malloryy McMalloryFace');
  await page.locator('.topbar').locator('.profile-header').click();
  const myProfileButton = page.locator('.profile-header-organization-popover').locator('.main-list').getByRole('listitem').nth(3);
  await expect(myProfileButton).toHaveText('Access recovery');
  await myProfileButton.click();
  await expect(page).toBeMyProfilePage();
  const container = page.locator('#recovery-profile-content');
  await expect(container).toBeVisible();
  await expect(page.locator('.recovery-section')).toHaveCount(3);
  const shamirContainer = page.locator('.recovery-section').nth(2);
  const status = shamirContainer.locator('.recovery-method-state');
  await expect(shamirContainer.locator('.recovery-method-content-text__description')).toHaveText(
    'Designate at least 2 trusted people. In case of lost access, they can help you regain access to your account.',
  );
  await expect(status).toHaveText('Inactive');
  const buttons = shamirContainer.locator('ion-button');
  await expect(buttons).toHaveText(['Manage my people of trust', 'Help recover an access']);
  const shamirModal = page.locator('.shamir-recovery-modal');
  await expect(shamirModal).toBeHidden();
  await buttons.nth(0).click();
  await expect(shamirModal).toBeVisible();
  await shamirModal.locator('.shamir-information-button').click();
  await expect(shamirModal.locator('.shamir-done')).toBeVisible();
  await expect(shamirModal.locator('.shamir-done-list-item').locator('.shamir-done-list-item__text')).toHaveText([
    'Anonymous',
    'Anonymous',
    'Anonymous',
    'Anonymous',
  ]);
  await shamirModal.locator('.shamir-done-footer__button').nth(1).click();

  await expect(shamirModal).toBeHidden();
  await expect(status).toHaveText('Active');
  await page.release();
});

msTest('Setup shamir recovery not enough users', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  await setupNewPage(page, {
    enableShamir: true,
  });

  await login(page, 'Alicey McAliceFace');
  await page.locator('.topbar').locator('.profile-header').click();
  const myProfileButton = page.locator('.profile-header-organization-popover').locator('.main-list').getByRole('listitem').nth(3);
  await expect(myProfileButton).toHaveText('Access recovery');
  await myProfileButton.click();
  await expect(page).toBeMyProfilePage();
  const container = page.locator('#recovery-profile-content');
  await expect(container).toBeVisible();
  const shamirContainer = page.locator('.recovery-section').nth(2);
  const buttons = shamirContainer.locator('ion-button');
  const shamirModal = page.locator('.shamir-recovery-modal');
  await expect(shamirModal).toBeHidden();
  await buttons.nth(0).click();
  await expect(shamirModal).toBeVisible();
  const error = shamirModal.locator('.ms-error');
  await expect(error).toBeVisible();
  await expect(error).toHaveText(
    'Not enough people in your organization. At least 2 people with a profile different than External are required.',
  );
  await expect(shamirModal.locator('.shamir-information-button')).toBeTrulyDisabled();
  await page.release();
});

async function openGreeterModal(page: MsPage, user: 'Boby McBobFace' | 'Mikey McMikeFace', copyLink = false): Promise<Locator> {
  await login(page, user);
  await page.locator('.topbar').locator('.profile-header').click();
  await expect(page.locator('.profile-header-organization-popover').locator('.main-list').getByRole('listitem').nth(3)).toHaveText(
    'Access recovery',
  );
  await page.locator('.profile-header-organization-popover').locator('.main-list').getByRole('listitem').nth(3).click();
  await expect(page).toHavePageTitle('My profile');
  await expect(page).toBeMyProfilePage();
  await expect(page.locator('.recovery-section').nth(2).locator('ion-button').nth(1)).toHaveText('Help recover an access');
  await page.locator('.recovery-section').nth(2).locator('ion-button').nth(1).click();
  const shamirModal = page.locator('.shamir-recovery-modal');
  await expect(shamirModal).toBeVisible();
  await expect(shamirModal.locator('.shamir-others-list-item').nth(0)).toHaveText(
    'Alicey McAliceFacealice@example.comCopy linkStart recovery',
  );
  if (copyLink) {
    await shamirModal.locator('.shamir-others-list-item').locator('ion-button').nth(0).click();
    await expect(page).toShowToast('Recovery link copied to clipboard.', 'Info');
  }
  return shamirModal;
}

msTest('Recover device using shamir', async ({ context }) => {
  msTest.setTimeout(120_000);
  const SAS_CODE_RE = /^[A-Z0-9]{4}$/;

  await setWriteClipboardPermission(context, true);
  const page = (await context.newPage()) as MsPage;
  await setupNewPage(page, { enableShamir: true, tesbedTemplate: 'shamir' });

  // Open page for greeter1 and start the process
  const greeter1Page = await page.openNewTab({ enableShamir: true });
  const greeter1Modal = await openGreeterModal(greeter1Page, 'Mikey McMikeFace', true);
  const link = await getClipboardText(greeter1Page);
  expect(link).toMatch(new RegExp('^https?://.+$'));
  await greeter1Modal.locator('.shamir-others-list-item').locator('ion-button').nth(1).click();
  await expect(greeter1Modal).toBeHidden();
  const exch1Modal = greeter1Page.locator('.greet-shamir-recovery-modal');
  await expect(exch1Modal).toBeVisible();
  await expect(exch1Modal.locator('.spinner')).toBeVisible();

  // Now start the process on the claimer's side
  await page.locator('#create-organization-button').click();
  await expect(page.locator('.homepage-popover')).toBeVisible();
  await page.locator('.homepage-popover').getByRole('listitem').nth(1).click();
  await fillInputModal(page, link);
  const claimModal = page.locator('.claim-shamir-recovery-modal');
  await expect(claimModal).toBeVisible();
  await expect(claimModal.locator('.parts')).toHaveText('PARTS: 0/2');
  await expect(claimModal.locator('.recipient-info')).toHaveText([
    'Boby McBobFacebob@example.com',
    'Malloryy McMalloryFacemallory@example.com',
    'Mikey McMikeFacemike@example.com',
  ]);
  await expect(claimModal.locator('.recipient').nth(0).locator('.button-start')).toBeVisible();
  await expect(claimModal.locator('.recipient').nth(1).locator('.button-start')).toBeVisible();
  await expect(claimModal.locator('.recipient').nth(2).locator('.button-start')).toBeVisible();
  // Start with Mikey as he only got 1 part (if we start with Bob the process would be finished immediately)
  await claimModal.locator('.recipient').nth(2).locator('.button-start').click();

  await expect(exch1Modal.locator('.provide-code')).toBeVisible();
  await expect(exch1Modal.locator('.provide-code')).toHaveText(SAS_CODE_RE);
  const greet1Code = await exch1Modal.locator('.provide-code').innerText();
  expect(greet1Code).toMatch(SAS_CODE_RE);

  await expect(claimModal.locator('.choose-code')).toBeVisible();
  await expect(claimModal.locator('.choose-code').locator('.button-choice')).toHaveText(Array(4).fill(SAS_CODE_RE));
  await claimModal.locator('.choose-code').locator('.button-choice', { hasText: greet1Code }).click();

  await expect(claimModal.locator('.provide-code')).toBeVisible();
  await expect(claimModal.locator('.provide-code')).toHaveText(SAS_CODE_RE);
  const claim1Code = await claimModal.locator('.provide-code').innerText();
  expect(claim1Code).toMatch(SAS_CODE_RE);

  await expect(exch1Modal.locator('.choose-code')).toBeVisible();
  await expect(exch1Modal.locator('.choose-code').locator('.button-choice')).toHaveText(Array(4).fill(SAS_CODE_RE));
  await exch1Modal.locator('.choose-code').locator('.button-choice', { hasText: claim1Code }).click();
  await expect(exch1Modal).toBeHidden();
  await expect(greeter1Page).toShowToast('SUCCESS', 'Success');
  await greeter1Page.close();

  await expect(claimModal.locator('.parts')).toHaveText('PARTS: 1/2');
  await expect(claimModal.locator('.recipient-info')).toHaveText([
    'Boby McBobFacebob@example.com',
    'Malloryy McMalloryFacemallory@example.com',
    'Mikey McMikeFacemike@example.com',
  ]);
  await expect(claimModal.locator('.recipient').nth(0).locator('.button-start')).toBeVisible();
  await expect(claimModal.locator('.recipient').nth(1).locator('.button-start')).toBeVisible();
  await expect(claimModal.locator('.recipient').nth(2).locator('.button-start')).toBeHidden();
  await expect(claimModal.locator('.recipient').nth(2).locator('.check-icon')).toBeVisible();

  // Open page for greeter2 and start the process
  const greeter2Page = await page.openNewTab({ enableShamir: true });
  const greeter2Modal = await openGreeterModal(greeter2Page, 'Boby McBobFace', false);
  await greeter2Modal.locator('.shamir-others-list-item').locator('ion-button').nth(1).click();
  await expect(greeter2Modal).toBeHidden();
  const exch2Modal = greeter2Page.locator('.greet-shamir-recovery-modal');
  await expect(exch2Modal).toBeVisible();
  await expect(exch2Modal.locator('.spinner')).toBeVisible();

  // Continue with Boby
  await claimModal.locator('.recipient').nth(0).locator('.button-start').click();

  await expect(exch2Modal.locator('.provide-code')).toBeVisible();
  await expect(exch2Modal.locator('.provide-code')).toHaveText(SAS_CODE_RE);
  const greet2Code = await exch2Modal.locator('.provide-code').innerText();
  expect(greet2Code).toMatch(SAS_CODE_RE);

  await expect(claimModal.locator('.choose-code')).toBeVisible();
  await expect(claimModal.locator('.choose-code').locator('.button-choice')).toHaveText(Array(4).fill(SAS_CODE_RE));
  await claimModal.locator('.choose-code').locator('.button-choice', { hasText: greet2Code }).click();

  await expect(claimModal.locator('.provide-code')).toBeVisible();
  await expect(claimModal.locator('.provide-code')).toHaveText(SAS_CODE_RE);
  const claim2Code = await claimModal.locator('.provide-code').innerText();
  expect(claim2Code).toMatch(SAS_CODE_RE);

  await expect(exch2Modal.locator('.choose-code')).toBeVisible();
  await expect(exch2Modal.locator('.choose-code').locator('.button-choice')).toHaveText(Array(4).fill(SAS_CODE_RE));
  await exch2Modal.locator('.choose-code').locator('.button-choice', { hasText: claim2Code }).click();
  await expect(exch2Modal).toBeHidden();
  await expect(greeter2Page).toShowToast('SUCCESS', 'Success');
  await greeter2Page.close();

  // Got all the parts, all that's left is choose the authentication
  await expect(claimModal.locator('.recipient-list')).toBeHidden();
  await expect(claimModal.locator('.finish-button')).toBeVisible();
  await claimModal.locator('.finish-button').click();
  await expect(claimModal).toBeHidden();
  const authModal = page.locator('.choose-authentication-modal');
  await expect(authModal).toBeVisible();

  const authRadio = authModal.locator('.radio-list-item:visible');
  await expect(authRadio).toHaveAuthentication({ keyringDisabled: true, pkiDisabled: true });
  await authRadio.nth(0).click();
  const passwordChoice = authModal.locator('.choose-password');
  await fillIonInput(passwordChoice.locator('ion-input').nth(0), 'AVeryL0ngP@ssw0rd');
  await fillIonInput(passwordChoice.locator('ion-input').nth(1), 'AVeryL0ngP@ssw0rd');
  await expect(authModal.locator('#next-button')).not.toHaveDisabledAttribute();
  await authModal.locator('#next-button').click();
  await expect(authModal).toBeHidden();
  await expect(page.locator('.ms-spinner-modal')).toBeVisible();
  await page.waitForTimeout(5000);
  await answerQuestion(page, false, {
    expectedTitleText: 'CANNOT REACH SERVER',
    expectedQuestionText: 'CANNOT REACH SERVER',
    expectedPositiveText: 'RETRY',
    expectedNegativeText: 'GIVE UP',
  });
  await expect(page).toBeHomePage();
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, fillIonInput, login, logout, MsPage, msTest, setupNewPage } from '@tests/e2e/helpers';

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

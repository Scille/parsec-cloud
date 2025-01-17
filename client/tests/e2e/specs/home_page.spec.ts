// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, fillIonInput, msTest, sortBy } from '@tests/e2e/helpers';

const USER_NAMES = ['Alicey McAliceFace', 'Boby McBobFace', 'Malloryy McMalloryFace'];

for (const displaySize of ['small', 'large']) {
  msTest(`Home default state with devices on ${displaySize} display`, async ({ home }) => {
    if (displaySize === 'small') {
      const viewport = home.viewportSize();
      await home.setViewportSize({ width: 700, height: viewport ? viewport.height : 700 });
    }

    const topBar = home.locator('.topbar');
    await expect(topBar.locator('.topbar-left__title')).toHaveText('Welcome to Parsec');

    const container = home.locator('.slider-container');
    await expect(container.locator('.organization-title')).toHaveText('Access your organizations');
    await expect(container.locator('#search-input-organization')).toBeVisible();
    await expect(container.locator('#organization-filter-select')).toHaveText('Organization name');

    const cards = home.locator('.organization-list').locator('.organization-card');
    await expect(cards).toHaveCount(USER_NAMES.length);

    for (const card of await cards.all()) {
      const lastLogin = card.locator('.login-text').nth(0);
      await expect(card.locator('.organization-card-content-text').locator('.title-h4')).toHaveText(/Org\d+/);
      await expect(lastLogin).toHaveText('--');
    }
    await expect(cards.locator('.login-name')).toHaveText(USER_NAMES.sort((u1, u2) => u1.localeCompare(u2)));

    if (displaySize === 'large') {
      await expect(topBar.locator('.topbar-right').locator('ion-button')).toHaveText(['Create or join', 'Customer area']);
      await expect(home.locator('.homepage-sidebar')).toBeVisible();
      await expect(home.locator('.menu-secondary-buttons').locator('ion-button')).toHaveText([
        'About',
        'Documentation',
        'Contact',
        'Settings',
      ]);
    } else {
      await expect(topBar.locator('.topbar-right').locator('ion-button')).toHaveText(['', 'Customer area']);
      await expect(home.locator('.homepage-sidebar')).toBeHidden();
      await expect(home.locator('.menu-secondary-buttons').locator('ion-button')).toHaveText(['About', 'Doc.', 'Contact', 'Settings']);
    }
  });
}

msTest('Sort devices', async ({ home }) => {
  const sortButton = home.locator('#organization-filter-select');
  const cards = home.locator('.organization-list').locator('.organization-card');
  await expect(cards.locator('.login-name')).toHaveText(USER_NAMES.sort((u1, u2) => u1.localeCompare(u2)));
  await sortBy(sortButton, 'Ascending');
  // Should not change anything right now because all devices have the same organization
  await expect(cards.locator('.login-name')).toHaveText(USER_NAMES.sort((u1, u2) => u1.localeCompare(u2)));
  await sortBy(sortButton, 'User name');
  // By name desc
  await expect(cards.locator('.login-name')).toHaveText(USER_NAMES.sort((u1, u2) => u2.localeCompare(u1)));
  await sortBy(sortButton, 'Descending');
  // By name asc
  await expect(cards.locator('.login-name')).toHaveText(USER_NAMES.sort((u1, u2) => u1.localeCompare(u2)));
});

msTest('Filter devices', async ({ home }) => {
  const cards = home.locator('.organization-list').locator('.organization-card');
  const searchInput = home.locator('#search-input-organization').locator('ion-input');
  await expect(cards.locator('.login-name')).toHaveText(USER_NAMES.sort((u1, u2) => u1.localeCompare(u2)));
  await fillIonInput(searchInput, 'cey');
  await expect(cards.locator('.login-name')).toHaveText(USER_NAMES.filter((u) => u.includes('cey')));
  await fillIonInput(searchInput, 'al');
  await expect(cards.locator('.login-name')).toHaveCount(2);
  await expect(cards.locator('.login-name')).toHaveText(USER_NAMES.filter((u) => u.toLowerCase().includes('al')));
});

msTest('Filter devices no match', async ({ home }) => {
  const cards = home.locator('.organization-list').locator('.organization-card');
  const searchInput = home.locator('#search-input-organization').locator('ion-input');
  await expect(cards.locator('.login-name')).toHaveText(USER_NAMES.sort((u1, u2) => u1.localeCompare(u2)));
  await fillIonInput(searchInput, 'nomatch');
  await expect(cards).toBeHidden();
  await expect(home.locator('.no-match-result')).toBeVisible();
  await expect(home.locator('.no-match-result')).toHaveText("No organization found that matches 'nomatch'.");
});

msTest('Check join link', async ({ home }) => {
  const LINKS = [
    {
      // cspell:disable-next-line
      link: 'http://parsec.cloud/Test?a=claim_user&p=xBBHJlEjlpxNZYTCvBWWDPIS',
      expectedError: 'This link is invalid.',
    },
    {
      // cspell:disable-next-line
      link: 'parsec3://parsec.cloud/Test?p=xBBHJlEjlpxNZYTCvBWWDPIS',
      expectedError: 'This link is invalid.',
    },
    {
      link: 'parsec3://parsec.cloud/Test?a=claim_user',
      expectedError: 'This link is invalid.',
    },
    {
      link: 'parsec3://parsec.cloud/Test?a=claim_user&p=abcde',
      expectedError: 'This link is invalid.',
    },
    {
      // cspell:disable-next-line
      link: 'parsec3://parsec.cloud/Test?a=claim_user&p=xBBHJlEjlpxNZYTCvBWWDPIS',
    },
  ];

  await home.locator('#create-organization-button').click();
  await home.locator('.homepage-popover').getByRole('listitem').nth(1).click();
  const confirmButton = home.locator('.text-input-modal').locator('#next-button');
  const errorForm = home.locator('.text-input-modal').locator('.form-error');
  const input = home.locator('.text-input-modal').locator('ion-input');

  await expect(confirmButton).toHaveDisabledAttribute();

  for (const linkInfo of LINKS) {
    await fillIonInput(input, linkInfo.link);
    if (linkInfo.expectedError) {
      await expect(errorForm).toBeVisible();
      await expect(errorForm).toHaveText(linkInfo.expectedError);
      await expect(confirmButton).toHaveDisabledAttribute();
    } else {
      await expect(errorForm).toBeHidden();
      await expect(confirmButton).toNotHaveDisabledAttribute();
    }
  }
});

msTest('Login', async ({ home }) => {
  await home.locator('.organization-list').locator('.organization-card').nth(0).click();
  await expect(home.locator('.login-header__title')).toHaveText('Log in');
  const loginButton = home.locator('.login-card-footer').locator('.login-button');
  await expect(loginButton).toHaveDisabledAttribute();
  const input = home.locator('#password-input').locator('ion-input');
  await fillIonInput(input, 'P@ssw0rd.');
  await expect(loginButton).toNotHaveDisabledAttribute();
  await loginButton.click();
  await expect(home).toBeWorkspacePage();
  await expect(home).toHaveHeader(['My workspaces'], false, false);
  const profile = home.locator('.topbar').locator('.profile-header');
  await expect(profile.locator('.text-content-name')).toHaveText('Alicey McAliceFace');
});

msTest('Login page and back to device list', async ({ home }) => {
  await home.locator('.organization-list').locator('.organization-card').nth(0).click();
  await expect(home.locator('.login-header__title')).toHaveText('Log in');
  await expect(home.locator('.organization-list')).toBeHidden();
  const backButton = home.locator('.topbar-left__back-button');
  await expect(backButton).toHaveText('Return to organizations');
  await backButton.click();
  await expect(home.locator('.organization-list')).toBeVisible();
});

msTest('Login with invalid password', async ({ home }) => {
  await home.locator('.organization-list').locator('.organization-card').nth(0).click();
  const loginButton = home.locator('.login-card-footer').locator('.login-button');
  await expect(loginButton).toHaveDisabledAttribute();
  await expect(home.locator('#password-input').locator('.form-error')).toBeHidden();
  const input = home.locator('#password-input').locator('ion-input');
  await fillIonInput(input, 'InvalidP@ssw0rd.');
  await expect(loginButton).toNotHaveDisabledAttribute();
  await loginButton.click();
  await expect(home.locator('#password-input').locator('.form-error')).toBeVisible();
  await expect(home.locator('#password-input').locator('.form-error')).toHaveText('Incorrect password.');
  await expect(home).toBeHomePage();
});

msTest.fail('Logout and go back to devices list', async ({ home }) => {
  await home.locator('.organization-list').locator('.organization-card').nth(0).click();
  await fillIonInput(home.locator('#password-input').locator('ion-input'), 'P@ssw0rd.');
  await home.locator('.login-card-footer').locator('.login-button').click();
  await expect(home).toBeWorkspacePage();
  await home.locator('.topbar').locator('.profile-header').click();
  const buttons = home.locator('.profile-header-popover').locator('.main-list').getByRole('listitem');
  await buttons.nth(4).click();
  await answerQuestion(home, true);
  await expect(home.locator('.organization-title')).toHaveText('Access your organizations');
  await expect(home).toBeHomePage();
});

msTest('Check header buttons', async ({ home }) => {
  await expect(home.locator('.menu-secondary-buttons').locator('ion-button')).toHaveCount(4);
});

msTest('Open documentation', async ({ home }) => {
  const newTabPromise = home.waitForEvent('popup');
  await home.locator('.menu-secondary-buttons').locator('ion-button').nth(1).click();
  const newTab = await newTabPromise;
  await newTab.waitForLoadState();
  await expect(newTab).toHaveURL(new RegExp('https://docs.parsec.cloud/(en|fr)/[a-z0-9-+.]+'));
});

msTest('Open feedback', async ({ home }) => {
  const newTabPromise = home.waitForEvent('popup');
  await home.locator('.menu-secondary-buttons').locator('ion-button').nth(2).click();
  const newTab = await newTabPromise;
  await newTab.waitForLoadState();
  await expect(newTab).toHaveURL(new RegExp('https://sign(-dev)?.parsec.cloud/contact'));
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  DisplaySize,
  expect,
  fillIonInput,
  generateMockInvitationLink,
  logout,
  MsPage,
  msTest,
  setupNewPage,
  sortBy,
} from '@tests/e2e/helpers';

const USER_NAMES = ['Alicey McAliceFace', 'Boby McBobFace', 'Malloryy McMalloryFace'];

for (const displaySize of [DisplaySize.Small, DisplaySize.Large]) {
  msTest(`Home default state with devices on ${displaySize} display`, { tag: '@important' }, async ({ home }) => {
    if (displaySize === 'small') {
      await home.setDisplaySize(DisplaySize.Small);
    }

    const topBar = home.locator('.topbar');
    await expect(topBar.locator('.topbar-left-text__title')).toHaveText('Welcome to Parsec');
    await expect(topBar.locator('.topbar-left-text__subtitle')).toHaveText('Access your organizations');

    const container = home.locator('.slider-container');
    await expect(container.locator('#search-input-organization')).toBeVisible();

    const cards = home.locator('.organization-list').locator('.organization-card');
    await expect(cards).toHaveCount(USER_NAMES.length);

    for (const card of await cards.all()) {
      const lastLogin = card.locator('.login-text').nth(0);
      await expect(card.locator('.organization-card-content-text').locator('.title-h4')).toHaveText(/TestbedOrg\d+/);
      await expect(lastLogin).toHaveText('--');
    }
    await expect(cards.locator('.login-name')).toHaveText(USER_NAMES.sort((u1, u2) => u1.localeCompare(u2)));

    if (displaySize === 'large') {
      await expect(container.locator('#organization-filter-select')).toHaveText('Organization name');
      await expect(topBar.locator('.topbar-right').locator('ion-button')).toHaveText(['Create or join']);
      await expect(home.locator('.homepage-sidebar')).toBeVisible();
      await expect(home.locator('.menu-secondary-buttons').locator('ion-button')).toHaveText([
        'About',
        'Documentation',
        'Contact',
        'Settings',
        'Customer area',
      ]);
    } else {
      await expect(topBar.locator('.topbar-right').locator('ion-button')).toHaveText(['Create or join']);
      await expect(home.locator('.homepage-sidebar')).toBeHidden();
      await expect(home.locator('.menu-secondary')).toBeHidden();
      await home.locator('.menu-button').isVisible();
      await home.locator('.menu-button').click();
      await expect(home.locator('.menu-secondary-collapse-buttons').locator('ion-button')).toHaveText([
        'About',
        'Documentation',
        'Contact',
        'Settings',
        'Customer area',
      ]);
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
  await expect(home.locator('.login-header__title')).toHaveText('Log in to an organization');
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

msTest('Logout and go back to devices list', async ({ home }) => {
  await home.locator('.organization-list').locator('.organization-card').nth(0).click();
  await fillIonInput(home.locator('#password-input').locator('ion-input'), 'P@ssw0rd.');
  await home.locator('.login-card-footer').locator('.login-button').click();
  await expect(home).toBeWorkspacePage();
  await logout(home);
});

msTest('Check header buttons', async ({ home }) => {
  await expect(home.locator('.menu-secondary-buttons').locator('ion-button')).toHaveCount(5);
  await expect(home.locator('.menu-secondary-buttons').locator('ion-button')).toHaveText([
    'About',
    'Documentation',
    'Contact',
    'Settings',
    'Customer area',
  ]);
});

for (const displaySize of [DisplaySize.Small, DisplaySize.Large]) {
  msTest(`Login page and back to device list ${displaySize} display`, async ({ home }) => {
    await home.locator('.organization-list').locator('.organization-card').nth(0).click();
    await expect(home.locator('.login-header__title')).toHaveText('Log in to an organization');
    await expect(home.locator('.organization-list')).toBeHidden();
    const backButton = home.locator('.topbar-left__back-button');

    if (displaySize === DisplaySize.Small) {
      await home.setDisplaySize(DisplaySize.Small);
      await expect(backButton).toHaveText('Return');
    } else {
      await expect(backButton).toHaveText('Return to organizations');
    }
    await backButton.click();
    await expect(home.locator('.organization-list')).toBeVisible();
  });

  msTest(`Open documentation ${displaySize} display`, async ({ home }) => {
    const newTabPromise = home.waitForEvent('popup');

    if (displaySize === DisplaySize.Small) {
      await home.setDisplaySize(DisplaySize.Small);
      await expect(home.locator('.menu-secondary')).toBeHidden();
      await home.locator('.menu-button').isVisible();
      await home.locator('.menu-button').click();
      await home.locator('.menu-secondary-collapse-buttons').locator('ion-button').nth(1).click();
    } else {
      await home.locator('.menu-secondary-buttons').locator('ion-button').nth(1).click();
    }
    const newTab = await newTabPromise;
    await newTab.waitForLoadState();
    await expect(newTab).toHaveURL(new RegExp('https://docs.parsec.cloud/(en|fr)/[a-z0-9-+.]+'));
  });

  msTest(`Open feedback ${displaySize} display`, async ({ home }) => {
    const newTabPromise = home.waitForEvent('popup');
    if (displaySize === DisplaySize.Small) {
      await home.setDisplaySize(DisplaySize.Small);
      await expect(home.locator('.menu-secondary')).toBeHidden();
      await home.locator('.menu-button').isVisible();
      await home.locator('.menu-button').click();
      await home.locator('.menu-secondary-collapse-buttons').locator('ion-button').nth(2).click();
    } else {
      await home.locator('.menu-secondary-buttons').locator('ion-button').nth(2).click();
    }
    const newTab = await newTabPromise;
    await newTab.waitForLoadState();
    await expect(newTab).toHaveURL(new RegExp('https://sign(-dev)?.parsec.cloud/contact'));
  });
}

msTest('Warn on Safari', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  // Tried changing the user agent but the detection is too good,
  // so we need a little extra step.
  await setupNewPage(page, { mockBrowser: 'Safari' });
  const modal = page.locator('.incompatible-environment');
  await expect(modal).toBeVisible();
  await expect(modal.locator('.incompatible-content__title')).toHaveText('Your browser is not compatible with Parsec yet.');
  await page.release();
});

msTest('Empty home page default state', { tag: '@important' }, async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  await setupNewPage(page, { libparsecMockFunctions: [{ name: 'listAvailableDevices', result: { ok: true, value: [] } }] });
  const container = page.locator('.no-devices');
  await expect(container).toBeVisible();
  await expect(container.locator('.create-organization-text__title')).toHaveText('New to Parsec?');
  const createOrgBtn = container.locator('#create-organization-button');
  await expect(createOrgBtn).toHaveText('Create an organization');

  await expect(container.locator('.invitation').locator('.invitation__title')).toHaveText('You have received an invitation link?');
  const joinBtn = container.locator('.invitation').locator('#join-organization-button');
  await expect(joinBtn).toHaveText('Join');
  await expect(joinBtn).toBeTrulyDisabled();
  await expect(container.locator('.webAccess').locator('.webAccess-header__title')).toHaveText(
    'Would you like to access your organization from this browser?',
  );
  await expect(container.locator('.webAccess').locator('.webAccess-step')).toBeHidden();
  await container.locator('.webAccess').locator('.webAccess-header-info').click();
  await expect(container.locator('.webAccess').locator('.webAccess-step')).toBeVisible();
  const continueBtn = container.locator('.webAccess').locator('#join-organization-button');
  await expect(continueBtn).toHaveText('Continue');
  await expect(continueBtn).toBeTrulyDisabled();

  await page.release();
});

msTest('Empty home page create org', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;

  await setupNewPage(page, { skipTestbed: true });
  const container = page.locator('.no-devices');
  await expect(container).toBeVisible();

  await expect(page.locator('.create-organization-modal')).toBeHidden();
  await container.locator('#create-organization-button').click();
  await expect(page.locator('.create-organization-modal')).toBeVisible();

  await page.release();
});

msTest('Empty home page recover', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;

  await setupNewPage(page, { skipTestbed: true });
  const container = page.locator('.no-devices');
  await expect(container).toBeVisible();

  const recoverBtn = container.locator('.recovery-no-devices').locator('ion-button');
  await expect(page.locator('.recovery-content')).toBeHidden();
  await recoverBtn.click();
  await expect(page.locator('.recovery-content')).toBeVisible();
  await page.locator('.topbar-left__back-button').click();
  await expect(page.locator('.recovery-content')).toBeHidden();
  await page.release();
});

msTest('Empty home page join1 with click', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;

  await setupNewPage(page, { skipTestbed: true });
  const container = page.locator('.no-devices');
  await expect(container).toBeVisible();

  const joinBtn = container.locator('.invitation').locator('#join-organization-button');
  const linkInput = container.locator('.invitation').locator('ion-input');
  const modal = page.locator('.join-organization-modal');
  await expect(joinBtn).toHaveText('Join');
  await expect(joinBtn).toBeTrulyDisabled();
  // cspell:disable-next-line
  await fillIonInput(linkInput, generateMockInvitationLink(page, 'user'));
  await expect(joinBtn).toBeTrulyEnabled();
  await expect(modal).toBeHidden();
  await joinBtn.click();
  await expect(modal).toBeVisible();
  await expect(page).toShowToast('An unexpected error occurred (reason: ClaimerRetrieveInfoErrorInternal).', 'Error');
  await expect(modal).toBeHidden();

  await page.release();
});

msTest('Empty home page join1 with enter', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;

  await setupNewPage(page, { skipTestbed: true });
  const container = page.locator('.no-devices');
  await expect(container).toBeVisible();

  const joinBtn = container.locator('.invitation').locator('#join-organization-button');
  const linkInput = container.locator('.invitation').locator('ion-input');
  const modal = page.locator('.join-organization-modal');
  await expect(joinBtn).toHaveText('Join');
  await expect(joinBtn).toBeTrulyDisabled();
  await expect(modal).toBeHidden();
  await fillIonInput(linkInput, generateMockInvitationLink(page, 'user'));
  await expect(joinBtn).toBeTrulyEnabled();
  await linkInput.locator('input').press('Enter');
  await expect(modal).toBeVisible();
  await expect(page).toShowToast('An unexpected error occurred (reason: ClaimerRetrieveInfoErrorInternal).', 'Error');
  await expect(modal).toBeHidden();

  await page.release();
});

msTest('Empty home page join2 with click', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;

  await setupNewPage(page, { skipTestbed: true });
  const container = page.locator('.no-devices');
  await expect(container).toBeVisible();

  const continueBtn = container.locator('.webAccess').locator('#join-organization-button');
  const webLinkInput = container.locator('.webAccess').locator('ion-input');
  const modal = page.locator('.join-organization-modal');
  await expect(continueBtn).toBeTrulyDisabled();
  await fillIonInput(webLinkInput, generateMockInvitationLink(page, 'user'));
  await expect(continueBtn).toBeTrulyEnabled();
  await expect(modal).toBeHidden();
  await continueBtn.click();
  await expect(modal).toBeVisible();
  await expect(page).toShowToast('An unexpected error occurred (reason: ClaimerRetrieveInfoErrorInternal).', 'Error');
  await expect(modal).toBeHidden();

  await page.release();
});

msTest('Empty home page join2 with enter', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;

  await setupNewPage(page, { skipTestbed: true });
  const container = page.locator('.no-devices');
  await expect(container).toBeVisible();

  const continueBtn = container.locator('.webAccess').locator('#join-organization-button');
  const webLinkInput = container.locator('.webAccess').locator('ion-input');
  const modal = page.locator('.join-organization-modal');
  await expect(continueBtn).toBeTrulyDisabled();
  await fillIonInput(webLinkInput, generateMockInvitationLink(page, 'user'));
  await expect(continueBtn).toBeTrulyEnabled();
  await expect(modal).toBeHidden();
  await webLinkInput.locator('input').press('Enter');
  await expect(modal).toBeVisible();
  await expect(page).toShowToast('An unexpected error occurred (reason: ClaimerRetrieveInfoErrorInternal).', 'Error');
  await expect(modal).toBeHidden();

  await page.release();
});

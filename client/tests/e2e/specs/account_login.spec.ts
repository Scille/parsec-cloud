// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { addUser, answerQuestion, expect, fillIonInput, logout, MsPage, msTest, setupNewPage } from '@tests/e2e/helpers';

msTest('Parsec account login initial page', async ({ parsecAccount }) => {
  const container = parsecAccount.locator('.homepage-content');
  await expect(container.locator('.account-login__title')).toHaveText('My Parsec account');
});

msTest('Parsec account skip login', async ({ parsecAccount }) => {
  const container = parsecAccount.locator('.homepage-content');
  await expect(container.locator('.homepage-skip__button')).toHaveText('Skip this step');
  await container.locator('.homepage-skip__button').click();
  await expect(parsecAccount).toHaveURL(/.+\/home$/);
  await expect(parsecAccount.locator('.organization-content')).toBeVisible();
});

msTest('Parsec account go to customer area', async ({ parsecAccount }) => {
  const container = parsecAccount.locator('.homepage-content');
  const customerAreaContainer = container.locator('.homepage-client-area');
  await expect(customerAreaContainer.locator('.homepage-client-area__title')).toHaveText('Customer area');
  await expect(customerAreaContainer.locator('.homepage-client-area__button')).toHaveText('Access');
  await customerAreaContainer.locator('.homepage-client-area__button').click();
  await expect(parsecAccount).toHaveURL(/.+\/home$/);
  await expect(parsecAccount.locator('.saas-login-container')).toBeVisible();
});

msTest('Parsec account go to creation', async ({ parsecAccount }) => {
  const container = parsecAccount.locator('.homepage-content');
  await expect(container.locator('.account-create__button')).toHaveText('Create an account');
  await container.locator('.account-create__button').click();
  await expect(parsecAccount).toHaveURL(/.+\/createAccount$/);
});

msTest('Open settings in profile homepage popover ', async ({ parsecAccountLoggedIn }) => {
  const accountNameButton = parsecAccountLoggedIn.locator('.profile-header-homepage');
  await expect(accountNameButton).toBeVisible();
  await expect(accountNameButton).toHaveText(/^Agent\d+$/);
  await expect(parsecAccountLoggedIn.locator('.profile-header-homepage-popover')).toBeHidden();
  await accountNameButton.click();
  await expect(parsecAccountLoggedIn.locator('.profile-header-homepage-popover')).toBeVisible();
  const popover = parsecAccountLoggedIn.locator('.profile-header-homepage-popover');
  const header = popover.locator('.header-list');
  await expect(header.locator('.header-list-email')).toHaveText(/^agent\d+@example.com$/);

  const buttons = popover.locator('.main-list').getByRole('listitem');
  await expect(buttons).toHaveText(['Settings', 'Account', 'Authentication', 'Log out']);
  await popover.locator('.main-list').getByRole('listitem').nth(0).click();
});

msTest('Open settings from header (secondary menu) ', async ({ parsecAccountLoggedIn }) => {
  const settingsButton = parsecAccountLoggedIn.locator('.menu-secondary').locator('#trigger-settings-button');
  await expect(settingsButton).toBeVisible();
  await settingsButton.click();
  await expect(parsecAccountLoggedIn.locator('.profile-content-item').nth(0).locator('.settings-list-group__title')).toHaveText([
    'Display',
    'Configuration',
    'Account',
  ]);
});

msTest('Switch tab from popover ', async ({ parsecAccountLoggedIn }) => {
  const accountNameButton = parsecAccountLoggedIn.locator('.profile-header-homepage');
  await expect(accountNameButton).toBeVisible();
  await expect(accountNameButton).toHaveText(/^Agent\d+$/);
  await expect(parsecAccountLoggedIn.locator('.profile-header-homepage-popover')).toBeHidden();
  await accountNameButton.click();
  await expect(parsecAccountLoggedIn.locator('.profile-header-homepage-popover')).toBeVisible();
  const popover = parsecAccountLoggedIn.locator('.profile-header-homepage-popover');
  const buttons = popover.locator('.main-list').getByRole('listitem');
  await expect(buttons).toHaveText(['Settings', 'Account', 'Authentication', 'Log out']);
  await popover.locator('.main-list').getByRole('listitem').nth(1).click();
  await expect(parsecAccountLoggedIn.locator('.profile-content-item').nth(0).locator('.item-header__title')).toHaveText('My account');
  await accountNameButton.click();
  await expect(parsecAccountLoggedIn.locator('.profile-header-homepage-popover')).toBeVisible();
  await popover.locator('.main-list').getByRole('listitem').nth(2).click();
  await expect(parsecAccountLoggedIn.locator('.profile-content-item').nth(0).locator('.item-header__title')).toHaveText('Authentication');
});

msTest('Account create registration device and replace current', async ({ parsecAccountLoggedIn }) => {
  msTest.setTimeout(120_000);

  const home = parsecAccountLoggedIn;

  const accountNameButton = parsecAccountLoggedIn.locator('.profile-header-homepage');
  await expect(accountNameButton).toBeVisible();
  await expect(accountNameButton).toHaveText(/^Agent\d+$/);
  await accountNameButton.click();
  const popover = parsecAccountLoggedIn.locator('.profile-header-homepage-popover');
  await expect(popover).toBeVisible();
  const header = popover.locator('.header-list');
  const email = (await header.locator('.header-list-email').textContent()) ?? 'incorrect';
  await popover.locator('ion-backdrop').click();

  await home.locator('.organization-list').locator('.organization-card').nth(0).click();
  await fillIonInput(home.locator('#password-input').locator('ion-input'), 'P@ssw0rd.');
  await home.locator('.login-card-footer').locator('.login-button').click();
  await expect(home).toBeWorkspacePage();
  await home.locator('.sidebar').locator('#sidebar-users').click();
  await expect(home).toHavePageTitle('Users');
  await expect(home).toBeUserPage();

  const secondTab = await home.openNewTab();

  await addUser(home, secondTab, 'UserName', email, 'admin');

  await secondTab.release();

  await logout(home);
  await expect(home.locator('.organization-list').locator('.organization-card')).toHaveCount(4);

  await home.locator('.organization-list').locator('.organization-card').nth(3).click();
  await fillIonInput(home.locator('#password-input').locator('ion-input'), 'AVeryL0ngP@ssw0rd');
  await home.locator('.login-card-footer').locator('.login-button').click();
  await answerQuestion(home, true, {
    expectedNegativeText: 'No',
    expectedPositiveText: 'Store to Parsec Account',
    expectedQuestionText: "This device is not stored in Parsec Account. Do you want to store it so that it'll be available everywhere?",
    expectedTitleText: 'Store this device to Parsec Account',
  });
  await expect(home).toBeWorkspacePage();
  await logout(home);
  await home.locator('.organization-list').locator('.organization-card').nth(3).click();
  await expect(home).toBeWorkspacePage();
});

msTest('Account create registration device and auto-register', async ({ browser, parsecAccountLoggedIn }) => {
  msTest.setTimeout(180_000);

  const home = parsecAccountLoggedIn;

  const accountNameButton = parsecAccountLoggedIn.locator('.profile-header-homepage');
  await expect(accountNameButton).toBeVisible();
  await expect(accountNameButton).toHaveText(/^Agent\d+$/);
  await accountNameButton.click();
  const popover = parsecAccountLoggedIn.locator('.profile-header-homepage-popover');
  await expect(popover).toBeVisible();
  const header = popover.locator('.header-list');
  const email = (await header.locator('.header-list-email').textContent()) ?? 'incorrect';
  await popover.locator('ion-backdrop').click();

  await home.locator('.organization-list').locator('.organization-card').nth(0).click();
  await fillIonInput(home.locator('#password-input').locator('ion-input'), 'P@ssw0rd.');
  await home.locator('.login-card-footer').locator('.login-button').click();
  await expect(home).toBeWorkspacePage();
  await home.locator('.sidebar').locator('#sidebar-users').click();
  await expect(home).toHavePageTitle('Users');
  await expect(home).toBeUserPage();

  // Opening a new context, else we'll be automatically logged into parsec account
  const context2 = await browser.newContext();
  const secondTab = (await context2.newPage()) as MsPage;
  await setupNewPage(secondTab, {
    withParsecAccount: true,
    location: '/account',
    parsecAccountAutoLogin: false,
  });
  const loginContainer1 = secondTab.locator('.account-login-container');
  const loginButton1 = loginContainer1.locator('.account-login-button').locator('ion-button');
  await fillIonInput(loginContainer1.locator('ion-input').nth(2), email);
  await fillIonInput(loginContainer1.locator('ion-input').nth(3), 'P@ssw0rd.');
  await expect(loginButton1).toBeTrulyEnabled();
  await loginButton1.click();
  await expect(secondTab).toBeHomePage();

  await addUser(home, secondTab, 'UserName', email, 'admin');
  await secondTab.locator('.join-organization-modal').locator('#next-button').click();

  await answerQuestion(secondTab, true, {
    expectedNegativeText: 'No',
    expectedPositiveText: 'Store to Parsec Account',
    expectedQuestionText: "This device is not stored in Parsec Account. Do you want to store it so that it'll be available everywhere?",
    expectedTitleText: 'Store this device to Parsec Account',
  });
  await expect(secondTab).toBeWorkspacePage();
  await expect(secondTab).toShowToast('The device has been stored in Parsec Account.', 'Success');

  await secondTab.release();
  await context2.close();
  await logout(home);

  // Opening a second tab with the same account, device should be present
  const newTab = await home.openNewTab({ location: '/home', withParsecAccount: true, parsecAccountAutoLogin: false });
  await expect(newTab).toBeHomePage();

  await expect(newTab.locator('.organization-list').locator('.organization-card')).toHaveCount(4);
  await expect(newTab.locator('.organization-list').locator('.organization-card').nth(3).locator('.login-name')).toHaveText('UserName');
  await newTab.locator('.organization-list').locator('.organization-card').nth(3).click();
  await expect(newTab).toBeWorkspacePage();
});

msTest('Account auto logged in on second tab', async ({ parsecAccountLoggedIn }) => {
  const home = parsecAccountLoggedIn;

  await expect(parsecAccountLoggedIn.locator('.profile-header-homepage')).toBeVisible();
  await expect(parsecAccountLoggedIn.locator('.profile-header-homepage')).toHaveText(/^Agent\d+$/);

  const secondTab = await home.openNewTab({ withParsecAccount: true, location: '/account', parsecAccountAutoLogin: false });
  await expect(secondTab).toBeHomePage();
  await expect(secondTab.locator('.profile-header-homepage')).toBeVisible();
  await expect(secondTab.locator('.profile-header-homepage')).toHaveText(/^Agent\d+$/);

  await secondTab.release();
});

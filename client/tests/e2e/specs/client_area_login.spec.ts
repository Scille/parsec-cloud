// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  DEFAULT_USER_INFORMATION,
  DisplaySize,
  MockBms,
  MsPage,
  answerQuestion,
  expect,
  fillIonInput,
  msTest,
  setupNewPage,
} from '@tests/e2e/helpers';

msTest('Log into the customer area', { tag: '@important' }, async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  await setupNewPage(home, { enableStripe: true });

  await MockBms.mockLogin(home);
  await MockBms.mockUserRoute(home);
  await MockBms.mockListOrganizations(home);
  await MockBms.mockOrganizationStats(home);
  await MockBms.mockOrganizationStatus(home);
  await MockBms.mockBillingDetails(home);
  await MockBms.mockGetInvoices(home);

  const button = home.locator('.homepage-header').locator('#trigger-customer-area-button');
  await expect(button).toHaveText('Customer area');
  await button.click();
  await expect(home).toHaveURL(/.+\/home$/);
  await expect(home.locator('.saas-login-container').locator('.saas-login__title')).toHaveText('Log in to your customer account');
  await fillIonInput(home.locator('.input-container').nth(0).locator('ion-input'), DEFAULT_USER_INFORMATION.email);
  await fillIonInput(home.locator('.input-container').nth(1).locator('ion-input'), DEFAULT_USER_INFORMATION.password);
  await home.locator('.saas-login-button__item').nth(1).click();
  await expect(home.locator('.topbar').locator('.topbar-left-text__title')).toHaveText('Dashboard');
  const logOutButton = home.locator('.sidebar-content').locator('.bottom-section-buttons-logout');
  await expect(logOutButton).toHaveText('Log out');
  await logOutButton.click();
  await answerQuestion(home, true, {
    expectedTitleText: 'Log out',
    expectedQuestionText: 'Do you want to log out?',
    expectedPositiveText: 'Log out',
    expectedNegativeText: 'Stay connected',
  });
  await expect(home).toHaveURL(/.+\/home$/);
});

msTest('Log into the customer area failed', async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  await setupNewPage(home, { enableStripe: true });

  await MockBms.mockLogin(home, { POST: { errors: { status: 401, attribute: 'email' } } });

  const button = home.locator('.homepage-header').locator('#trigger-customer-area-button');
  await expect(button).toHaveText('Customer area');
  await button.click();
  await expect(home).toHaveURL(/.+\/home$/);
  await fillIonInput(home.locator('.input-container').nth(0).locator('ion-input'), DEFAULT_USER_INFORMATION.email);
  await fillIonInput(home.locator('.input-container').nth(1).locator('ion-input'), 'invalid_password');
  await home.locator('.saas-login-button__item').nth(1).click();
  const error = home.locator('.saas-login-container').locator('.login-button-error');
  await expect(error).toBeVisible();
  await expect(error).toHaveText('Cannot log in. Please check your email and password.');
});

for (const displaySize of [DisplaySize.Small, DisplaySize.Large]) {
  msTest(`Switch pages ${displaySize} display`, async ({ clientArea }) => {
    const pages = [
      { button: 'Dashboard', title: 'Dashboard', url: 'dashboard' },
      { button: 'My profile', title: 'My profile', url: 'personal-data' },
      { button: 'Statistics', title: 'Statistics', url: 'statistics' },
      { button: 'Invoices', title: 'Invoices', url: 'invoices' },
      { button: 'Payment methods', title: 'Payment methods', url: 'payment-methods' },
      { button: 'Billing details', title: 'Billing details', url: 'billing-details' },
    ];

    if (displaySize === DisplaySize.Small) {
      await clientArea.setDisplaySize(DisplaySize.Small);
    }

    const title = clientArea.locator('.topbar').locator('.topbar-left-text__title');
    const sidebar = clientArea.locator('.sidebar');
    const sidebarMenuButton = clientArea.locator('.topbar').locator('#trigger-toggle-menu-button');
    await expect(clientArea).toHaveURL(/.+\/clientArea\\?(?:.*)$/);
    await expect(title).toHaveText('Dashboard');

    if (displaySize === DisplaySize.Small) {
      await expect(sidebarMenuButton).toBeVisible();
      await expect(sidebar).toBeHidden();
      await sidebarMenuButton.click();
      await expect(sidebar).toBeVisible();
    }

    await expect(clientArea.locator('.sidebar-header').locator('.card-header-title')).toBeVisible();
    const menuButtons = clientArea.locator('.menu-client').locator('.menu-client-list').getByRole('listitem');
    const buttonTexts = pages.map((texts) => texts.button);
    await expect(menuButtons).toHaveText(buttonTexts);
    for (let i = 0; i < pages.length; i++) {
      if (displaySize === DisplaySize.Small && i > 0) {
        await sidebarMenuButton.click();
        await expect(sidebar).toBeVisible();
      }
      await menuButtons.nth(i).click();
      await expect(title).toHaveText(pages[i].title);

      if (displaySize === DisplaySize.Small) {
        await expect(sidebar).toBeHidden();
      }
      // eslint-disable-next-line max-len
      const urlMatch = `https?://[a-z:0-9.]+/clientArea\\?(?:organization=[a-f0-9-]+&)?(?:page=${pages[i].url})&?(?:organization=[a-f0-9-]+)?`;
      await expect(clientArea).toHaveURL(new RegExp(urlMatch));
    }
  });

  msTest(`Switch org ${displaySize} display`, async ({ clientArea }) => {
    const org1 = clientArea.orgInfo.name;
    const org2 = `${clientArea.orgInfo.name}-2`;

    let orgSwitchButton;

    if (displaySize === DisplaySize.Large) {
      orgSwitchButton = clientArea.locator('.sidebar-header').locator('.card-header-title');
    } else {
      orgSwitchButton = clientArea.locator('#connected-header').locator('.current-organization__name');
    }

    if (displaySize === DisplaySize.Small) {
      await clientArea.setDisplaySize(DisplaySize.Small);
    }
    await expect(orgSwitchButton).toContainText(org1);
    const popover = clientArea.locator('.popover-switch');
    await expect(popover).toBeHidden();
    await orgSwitchButton.click();
    await expect(popover).toBeVisible();
    const orgs = popover.locator('.organization-list').getByRole('listitem');
    const orgNames = orgs.locator('.organization-name');
    await expect(orgNames).toHaveText([org1, org2, 'All organizations']);
    await expect(orgs.nth(0).locator('.organization-icon-current')).toBeVisible();
    await expect(orgs.nth(1).locator('.organization-icon-current')).toBeHidden();
    await expect(orgs.nth(2).locator('.organization-icon-current')).toBeHidden();
    // Click on backdrop, nothing should change
    await clientArea.locator('.backdrop-hide').click();
    await expect(orgSwitchButton).toHaveText(org1);

    await expect(popover).toBeHidden();
    await orgSwitchButton.click();
    await expect(popover).toBeVisible();
    await expect(orgNames).toHaveText([org1, org2, 'All organizations']);
    await expect(orgs.nth(0).locator('.organization-icon-current')).toBeVisible();
    await expect(orgs.nth(1).locator('.organization-icon-current')).toBeHidden();
    await expect(orgs.nth(2).locator('.organization-icon-current')).toBeHidden();
    // Click on second org, should switch
    await orgs.nth(1).click();
    await expect(popover).toBeHidden();
    await expect(orgSwitchButton).toHaveText(org2);

    await orgSwitchButton.click();
    await expect(popover).toBeVisible();
    await expect(orgNames).toHaveText([org1, org2, 'All organizations']);
    // Current one should be second one
    await expect(orgs.nth(0).locator('.organization-icon-current')).toBeHidden();
    await expect(orgs.nth(1).locator('.organization-icon-current')).toBeVisible();
    await expect(orgs.nth(2).locator('.organization-icon-current')).toBeHidden();
  });
}

msTest('Open settings modal', async ({ clientArea }) => {
  const settingsButton = clientArea.locator('.topbar').locator('.custom-button');
  const modal = clientArea.locator('.settings-modal');
  await expect(modal).toBeHidden();
  await settingsButton.click();
  await expect(modal).toBeVisible();
  await expect(modal.locator('.ms-modal-header__title')).toHaveText('Settings');
});

for (const frozen of [false, true]) {
  msTest(`Check org state ${frozen ? 'frozen' : 'active'}`, async ({ context }) => {
    const home = (await context.newPage()) as MsPage;
    await setupNewPage(home, { enableStripe: true });

    await MockBms.mockLogin(home);
    await MockBms.mockUserRoute(home);
    await MockBms.mockListOrganizations(home);
    await MockBms.mockOrganizationStats(home);
    await MockBms.mockBillingDetails(home);
    await MockBms.mockGetInvoices(home);
    await MockBms.mockOrganizationStatus(home, { isFrozen: frozen });

    const button = home.locator('.homepage-header').locator('#trigger-customer-area-button');
    await button.click();
    await fillIonInput(home.locator('.input-container').nth(0).locator('ion-input'), DEFAULT_USER_INFORMATION.email);
    await fillIonInput(home.locator('.input-container').nth(1).locator('ion-input'), DEFAULT_USER_INFORMATION.password);
    const loginButton = home.locator('.saas-login-button__item').nth(1);
    await expect(loginButton).toHaveText('Log in');
    await expect(loginButton).toBeTrulyEnabled();
    await loginButton.click();
    const orgSwitchButton = home.locator('.sidebar-header').locator('.card-header-title');
    await expect(orgSwitchButton).toHaveText('All organizations');
    await orgSwitchButton.click();
    const popover = home.locator('.popover-switch');
    const orgs = popover.locator('.organization-list').getByRole('listitem');
    await orgs.nth(0).click();
    await expect(orgSwitchButton).toHaveText(home.orgInfo.name);

    const orgState = home.locator('.sidebar').locator('.sidebar-header').locator('.organization-card-state');
    await expect(orgState).toHaveText(frozen ? 'State: Frozen' : 'State: Active');
  });
}

msTest('Login in and refresh no remember me', async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  await setupNewPage(home, { enableStripe: true });

  await MockBms.mockLogin(home);
  await MockBms.mockUserRoute(home);
  await MockBms.mockListOrganizations(home);
  await MockBms.mockOrganizationStats(home);
  await MockBms.mockOrganizationStatus(home);
  await MockBms.mockBillingDetails(home);
  await MockBms.mockGetInvoices(home);

  const button = home.locator('.homepage-header').locator('#trigger-customer-area-button');
  await expect(button).toHaveText('Customer area');
  await button.click();
  await expect(home).toBeHomePage();
  const loginContainer = home.locator('.saas-login-container');
  await expect(loginContainer).toBeVisible();
  await expect(loginContainer.locator('.saas-login__title')).toHaveText('Log in to your customer account');
  await fillIonInput(loginContainer.locator('.input-container').nth(0).locator('ion-input'), DEFAULT_USER_INFORMATION.email);
  await fillIonInput(loginContainer.locator('.input-container').nth(1).locator('ion-input'), DEFAULT_USER_INFORMATION.password);
  await loginContainer.locator('.saas-login-button__item').nth(1).click();
  await expect(home.locator('.topbar').locator('.topbar-left-text__title')).toHaveText('Dashboard');
  await expect(home).toBeClientAreaPage();
  await expect(loginContainer).toBeHidden();
  await home.reload();
  await setupNewPage(home, { skipGoto: true });
  await expect(home).toBeHomePage();
  await expect(loginContainer).toBeVisible();
});

msTest('Login in and refresh with remember me', async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  await setupNewPage(home, { enableStripe: true });

  await MockBms.mockLogin(home);
  await MockBms.mockUserRoute(home);
  await MockBms.mockListOrganizations(home);
  await MockBms.mockOrganizationStats(home);
  await MockBms.mockOrganizationStatus(home);
  await MockBms.mockBillingDetails(home);
  await MockBms.mockGetInvoices(home);

  const button = home.locator('.homepage-header').locator('#trigger-customer-area-button');
  await expect(button).toHaveText('Customer area');
  await button.click();
  await expect(home).toBeHomePage();
  const loginContainer = home.locator('.saas-login-container');
  await expect(loginContainer).toBeVisible();
  await expect(loginContainer.locator('.saas-login__title')).toHaveText('Log in to your customer account');
  await fillIonInput(loginContainer.locator('.input-container').nth(0).locator('ion-input'), DEFAULT_USER_INFORMATION.email);
  await fillIonInput(loginContainer.locator('.input-container').nth(1).locator('ion-input'), DEFAULT_USER_INFORMATION.password);
  await loginContainer.locator('.saas-login-link').locator('.ms-checkbox').check();
  await loginContainer.locator('.saas-login-button__item').nth(1).click();
  await expect(home.locator('.topbar').locator('.topbar-left-text__title')).toHaveText('Dashboard');
  await expect(home).toBeClientAreaPage();
  await expect(loginContainer).toBeHidden();
  await home.reload();
  await setupNewPage(home, { skipGoto: true });
  await expect(home).toBeClientAreaPage();
  await expect(loginContainer).toBeHidden();
  await expect(home.locator('.topbar').locator('.topbar-left-text__title')).toHaveText('Dashboard');
});

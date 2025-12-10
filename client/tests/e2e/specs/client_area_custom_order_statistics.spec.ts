// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MockBms, clientAreaNavigateTo, clientAreaSwitchOrganization, expect, msTest, setupNewPage } from '@tests/e2e/helpers';

msTest('Test initial status', async ({ clientAreaCustomOrder }) => {
  await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
  await clientAreaNavigateTo(clientAreaCustomOrder, 'Statistics');

  const page = clientAreaCustomOrder.locator('.client-page-statistics');
  const error = page.locator('.statistics-error');
  await expect(error).toBeHidden();
  const active = page.locator('.users-cards-list').nth(0);
  const activeUserItems = active.locator('.users-cards-list-item').locator('.users-cards-list-item-text');
  await expect(activeUserItems).toHaveText(['Administrators4', 'Members54', 'External1', 'Total59']);

  const revoked = page.locator('.users-cards-list').nth(1);
  const revokedUserItems = revoked.locator('.users-cards-list-item').locator('.users-cards-list-item-text');
  await expect(revokedUserItems).toHaveText(['Administrator1', 'Member1', 'Externals142']);

  const storage = page.locator('.storage-data');
  const storageGlobal = storage.locator('.storage-data-global');
  await expect(storageGlobal.locator('ion-text')).toHaveText(['373 GB', '373 GBData', '381 MBMetadata']);
  const usage = storage.locator('.storage-data-usage');
  await expect(usage.locator('.ms-warning')).toBeVisible();
  await expect(usage.locator('.ms-warning')).toHaveText('You have reached your storage limit.');
});

msTest('Test page is not accessible by click if org is not bootstrapped', async ({ clientAreaCustomOrder }) => {
  await MockBms.mockOrganizationStatus(clientAreaCustomOrder, { isBootstrapped: false });

  await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
  const button = clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').filter({
    hasText: 'Statistics',
  });
  await expect(button).toBeVisible();
  await expect(button.locator('button')).toBeDisabled();
});

msTest('Test initial status for all orgs', async ({ clientAreaCustomOrder }) => {
  const orgSelector = clientAreaCustomOrder.locator('.sidebar-header').locator('.organization-card-header').locator('.card-header-title');
  await expect(orgSelector).toHaveText('All organizations');
  await clientAreaNavigateTo(clientAreaCustomOrder, 'Statistics');

  const container = clientAreaCustomOrder.locator('.client-page-statistics');
  const error = container.locator('.statistics-error');
  await expect(error).toBeHidden();

  const orgChoice = container.locator('.organization-choice-title');
  await expect(orgChoice).toBeVisible();
  await expect(orgChoice).toHaveText(
    'You have multiple organizations. Please select one from the top-left button or select it below in order to check its statistics.',
  );

  await expect(container.locator('.organization-list')).toBeVisible();
  const orgs = container.locator('.organization-list').locator('.organization-list-item');
  await expect(orgs).toHaveText(['BlackMesa', 'BlackMesa-2']);
  await orgs.nth(1).click();
  await expect(orgChoice).toBeHidden();
  await expect(container.locator('.organization-list')).toBeHidden();
  await expect(orgSelector).toHaveText('BlackMesa-2');
});

msTest.describe(() => {
  msTest.use({ clientAreaInitialParams: { rememberMe: true } });

  msTest('Test initial status org not bootstrapped', async ({ clientAreaCustomOrder }) => {
    await MockBms.mockOrganizationStatus(clientAreaCustomOrder, { isBootstrapped: false });

    await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
    const button = clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').filter({
      hasText: 'Statistics',
    });
    await expect(button).toBeVisible();
    await expect(button).toBeTrulyDisabled();
    // Use setupNewPage with skipTestbed to navigate with full page reload and proper initialization
    await setupNewPage(clientAreaCustomOrder, {
      skipTestbed: true,
      location: '/clientArea?organization=42&page=custom-order-statistics',
    });

    // Now verify the page loaded correctly with the error message
    const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');
    await expect(title).toHaveText('Statistics');
    const page = clientAreaCustomOrder.locator('.client-page-statistics');
    const error = page.locator('.statistics-error');
    await expect(error).toBeVisible();
    await expect(error).toHaveText('Your organization is not bootstrapped yet.');
  });

  msTest('Custom order stats generic error', async ({ clientAreaCustomOrder }) => {
    await MockBms.mockOrganizationStats(clientAreaCustomOrder, {}, { GET: { errors: { status: 400 } } });

    await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
    const button = clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').filter({
      hasText: 'Statistics',
    });
    await expect(button).toBeVisible();
    await expect(button).toBeTrulyDisabled();
    await setupNewPage(clientAreaCustomOrder, {
      skipTestbed: true,
      location: '/clientArea?organization=42&page=custom-order-statistics',
    });

    const container = clientAreaCustomOrder.locator('.client-page-statistics');
    const error = container.locator('.statistics-error');
    await expect(error).toBeVisible();
    await expect(error).toHaveText('Failed to retrieve organization data.');
  });

  msTest('Custom order stats timeout error', async ({ clientAreaCustomOrder }) => {
    await MockBms.mockOrganizationStats(clientAreaCustomOrder, {}, { GET: { timeout: true } });

    await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
    const button = clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').filter({
      hasText: 'Statistics',
    });
    await expect(button).toBeVisible();
    await expect(button).toBeTrulyDisabled();
    await setupNewPage(clientAreaCustomOrder, {
      skipTestbed: true,
      location: '/clientArea?organization=42&page=custom-order-statistics',
    });

    const container = clientAreaCustomOrder.locator('.client-page-statistics');
    const error = container.locator('.statistics-error');
    await expect(error).toBeVisible();
    await expect(error).toHaveText('Failed to retrieve organization data.');
  });

  msTest('Custom order org status generic error', async ({ clientAreaCustomOrder }) => {
    await MockBms.mockOrganizationStatus(clientAreaCustomOrder, {}, { GET: { errors: { status: 400 } } });

    await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
    const button = clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').filter({
      hasText: 'Statistics',
    });
    await expect(button).toBeVisible();
    await expect(button).toBeTrulyDisabled();
    await setupNewPage(clientAreaCustomOrder, {
      skipTestbed: true,
      location: '/clientArea?organization=42&page=custom-order-statistics',
    });

    const container = clientAreaCustomOrder.locator('.client-page-statistics');
    const error = container.locator('.statistics-error');
    await expect(error).toBeVisible();
    await expect(error).toHaveText('Failed to retrieve organization data.');
  });

  msTest('Custom order org status timeout error', async ({ clientAreaCustomOrder }) => {
    await MockBms.mockOrganizationStatus(clientAreaCustomOrder, {}, { GET: { timeout: true } });

    await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
    const button = clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').filter({
      hasText: 'Statistics',
    });
    await expect(button).toBeVisible();
    await expect(button).toBeTrulyDisabled();
    await setupNewPage(clientAreaCustomOrder, {
      skipTestbed: true,
      location: '/clientArea?organization=42&page=custom-order-statistics',
    });

    const container = clientAreaCustomOrder.locator('.client-page-statistics');
    const error = container.locator('.statistics-error');
    await expect(error).toBeVisible();
    await expect(error).toHaveText('Failed to retrieve organization data.');
  });
});

msTest('Custom order details generic error', async ({ clientAreaCustomOrder }) => {
  await MockBms.mockCustomOrderDetails(clientAreaCustomOrder, {}, { POST: { errors: { status: 400 } } });

  await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
  await clientAreaNavigateTo(clientAreaCustomOrder, 'Statistics');

  const container = clientAreaCustomOrder.locator('.client-page-statistics');
  const error = container.locator('.statistics-error');
  await expect(error).toBeVisible();
  await expect(error).toHaveText('Failed to retrieve organization data.');
});

msTest('Custom order details timeout error', async ({ clientAreaCustomOrder }) => {
  await MockBms.mockCustomOrderDetails(clientAreaCustomOrder, {}, { POST: { timeout: true } });

  await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
  await clientAreaNavigateTo(clientAreaCustomOrder, 'Statistics');

  const container = clientAreaCustomOrder.locator('.client-page-statistics');
  const error = container.locator('.statistics-error');
  await expect(error).toBeVisible();
  await expect(error).toHaveText('Failed to retrieve organization data.');
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, generateOrganizationLink, MsPage, msTest, openExternalLink, setupNewPage } from '@tests/e2e/helpers';

for (const [action, link] of [
  ['async_enrollment', generateOrganizationLink('BlackMesa', 'async_enrollment')],
  ['claim_user', generateOrganizationLink('BlackMesa', 'claim_user')],
  ['claim_device', generateOrganizationLink('BlackMesa', 'claim_device')],
  ['path', generateOrganizationLink('BlackMesa', 'path')],
  ['bootstrap_organization', generateOrganizationLink('BlackMesa', 'bootstrap_organization')],
]) {
  msTest(`Handle ${action} link web redirect`, async ({ context }) => {
    const page = (await context.newPage()) as MsPage;

    await setupNewPage(page, { location: `/webRedirect?webRedirectUrl=${encodeURIComponent(link)}` });
    await expect(page).toHaveURL(/\/webRedirect\?.*/);
    await expect(page.locator('.redirect-text__title')).toHaveText('Launching Parsec App');
    await expect(page.locator('.redirect-buttons__item')).toHaveText(['Open in Parsec app', 'Continue in browser']);
    await page.locator('.redirect-buttons__item').nth(1).click();
    await expect(page).toBeHomePage();
    if (action === 'async_enrollment') {
      await expect(page.locator('.async-enrollment-modal')).toBeVisible();
    } else if (action === 'claim_user') {
      await expect(page.locator('.join-organization-modal')).toBeVisible();
    } else if (action === 'claim_device') {
      await expect(page.locator('.join-organization-modal')).toBeVisible();
    } else if (action === 'path') {
      await expect(page).toShowInformationModal(
        'You do not have access to the organization BlackMesa in which this file is stored.',
        'Error',
      );
    } else if (action === 'bootstrap_organization') {
      await expect(page.locator('.create-organization-modal')).toBeVisible();
    }
  });
}

msTest('Handle invalid protocol link web redirect', async ({ context }) => {
  const LINK = 'http://invalid';

  const page = (await context.newPage()) as MsPage;

  await setupNewPage(page, { location: `/webRedirect?webRedirectUrl=${encodeURIComponent(LINK)}` });
  await expect(page).toHaveURL(/\/webRedirect\?.*/);
  await expect(page.locator('.redirect-text__title')).toHaveText('Launching Parsec App');
  await expect(page.locator('.redirect-buttons__item')).toHaveText(['Open in Parsec app', 'Continue in browser']);
  await page.locator('.redirect-buttons__item').nth(1).click();
  await expect(page).toBeHomePage();
});

msTest('Handle invalid action link web redirect', async ({ context }) => {
  const LINK = 'parsec3://localhost:6770/BlackMesa?a=invalid&no_ssl=true';

  const page = (await context.newPage()) as MsPage;

  await setupNewPage(page, { location: `/webRedirect?webRedirectUrl=${encodeURIComponent(LINK)}` });
  await expect(page).toHaveURL(/\/webRedirect\?.*/);
  await expect(page.locator('.redirect-text__title')).toHaveText('Launching Parsec App');
  await expect(page.locator('.redirect-buttons__item')).toHaveText(['Open in Parsec app', 'Continue in browser']);
  await page.locator('.redirect-buttons__item').nth(1).click();
  await expect(page).toBeHomePage();
});

msTest('Open download link', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;

  await setupNewPage(page, {
    location: `/webRedirect?webRedirectUrl=${encodeURIComponent(generateOrganizationLink('BlackMesa', 'bootstrap_organization'))}`,
  });
  await openExternalLink(page, page.locator('.redirect-download__link'), /^https:\/\/parsec\.cloud\/en\/?.*$/);
});

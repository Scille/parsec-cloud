// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import {
  checkEntryContextMenu,
  checkWorkspaceContextMenu,
  createFolder,
  expect,
  generateOrganizationLink,
  getClipboardText,
  importDefaultFiles,
  ImportDocuments,
  logout,
  MsPage,
  msTest,
  openExternalLink,
  setupNewPage,
} from '@tests/e2e/helpers';

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
    if (action === 'async_enrollment') {
      await expect(page).toBeHomePage();
      await expect(page.locator('.async-enrollment-modal')).toBeVisible();
    } else if (action === 'claim_user') {
      await expect(page).toBeHomePage();
      await expect(page.locator('.join-organization-modal')).toBeVisible();
    } else if (action === 'claim_device') {
      await expect(page).toBeHomePage();
      await expect(page.locator('.join-organization-modal')).toBeVisible();
    } else if (action === 'path') {
      await expect(page).toShowInformationModal(
        'You do not have access to the organization BlackMesa in which this file is stored.',
        'Error',
      );
    } else if (action === 'bootstrap_organization') {
      await expect(page).toBeHomePage();
      await expect(page.locator('.create-organization-modal')).toBeVisible();
    }
    await expect(page).toHaveURL(/\/home$/);
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

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });
  msTest('Fully consume file link', async ({ workspaces }, testInfo: TestInfo) => {
    // Share the workspace with Mallory, since it's the device that'll be selected to consume the link.
    await workspaces.locator('.workspace-card-item').nth(0).locator('.icon-share-container').nth(0).click();
    const sharingModal = workspaces.locator('.workspace-sharing-modal');
    const suggestions = sharingModal.locator('.ms-modal-content').locator('.user-list-suggestions-item');

    await expect(suggestions).toHaveCount(1);

    const user = suggestions.nth(0);
    await expect(user.locator('.dropdown-button')).toHaveText('Not shared');
    await user.locator('.dropdown-button').click();
    const roleDropdown = workspaces.locator('.dropdown-popover');
    const roles = roleDropdown.getByRole('listitem');
    await expect(roles.locator('.option-text__label')).toHaveText(['Owner', 'Manager', 'Contributor', 'Reader', 'Not shared']);
    // Set contributor
    await roles.nth(2).click();
    await expect(workspaces).toShowToast("Malloryy McMalloryFace's role has been updated to Contributor.", 'Success');
    await sharingModal.locator('.closeBtn').click();
    await expect(sharingModal).toBeHidden();
    await workspaces.locator('.workspace-card-item').nth(0).click();
    // Alias
    const documents = workspaces;

    // Create a folder to have some depth
    await createFolder(documents, 'New Folder');
    await documents.locator('.folder-container').locator('.file-list-item').nth(0).dblclick();
    await importDefaultFiles(documents, testInfo, ImportDocuments.Pdf, true);
    await documents.context().grantPermissions(['clipboard-write']);
    await documents.locator('.folder-container').locator('.file-list-item').nth(1).click({ button: 'right' });
    await checkEntryContextMenu(documents, 'file-full', 'Copy link');
    await expect(documents).toShowToast('Link has been copied to clipboard.', 'Info');
    const link = await getClipboardText(documents);
    await logout(documents);
    const encoded = encodeURIComponent(link);
    expect(encoded).toMatch(/^http%3A%2F%2F.+$/);
    const url = new URL(documents.url());
    const redirectUrl = `${url.origin}/webRedirect?webRedirectUrl=${encoded}`;
    const newTab = await documents.openNewTab({ location: redirectUrl });
    await expect(newTab.locator('.redirect-container')).toBeVisible();
    await expect(newTab.locator('.redirect-buttons__item').nth(1)).toHaveText('Continue in browser');
    await newTab.locator('.redirect-buttons__item').nth(1).click();
    await newTab.waitForTimeout(500);
    await newTab.locator('#password-input').locator('input').fill('P@ssw0rd.');
    await expect(newTab.locator('.login-button')).toBeEnabled();
    await newTab.locator('.login-button').click();
    await expect(newTab).toBeDocumentPage();
    await expect(newTab.locator('.folder-container').locator('.file-list-item')).toHaveCount(2);
    await expect(newTab.locator('.folder-container').locator('.file-list-item').nth(1).locator('.ms-checkbox')).toBeChecked();
    await expect(newTab).toHaveHeader(['wksp1', 'New Folder'], false, true);
  });
});

msTest('Fully consume workspace link', async ({ workspaces }) => {
  // Share the workspace with Mallory, since it's the device that'll be selected to consume the link.
  await workspaces.locator('.workspace-card-item').nth(0).locator('.icon-share-container').nth(0).click();
  const sharingModal = workspaces.locator('.workspace-sharing-modal');
  const suggestions = sharingModal.locator('.ms-modal-content').locator('.user-list-suggestions-item');

  await expect(suggestions).toHaveCount(1);

  const user = suggestions.nth(0);
  await expect(user.locator('.dropdown-button')).toHaveText('Not shared');
  await user.locator('.dropdown-button').click();
  const roleDropdown = workspaces.locator('.dropdown-popover');
  const roles = roleDropdown.getByRole('listitem');
  await expect(roles.locator('.option-text__label')).toHaveText(['Owner', 'Manager', 'Contributor', 'Reader', 'Not shared']);
  // Set contributor
  await roles.nth(2).click();
  await expect(workspaces).toShowToast("Malloryy McMalloryFace's role has been updated to Contributor.", 'Success');
  await sharingModal.locator('.closeBtn').click();
  await expect(sharingModal).toBeHidden();
  await workspaces.context().grantPermissions(['clipboard-write']);
  await workspaces.locator('.workspace-card-item').nth(0).locator('.icon-share-container').nth(0).click({ button: 'right' });
  await checkWorkspaceContextMenu(workspaces, 'owner', 'Copy link');

  await expect(workspaces).toShowToast('Workspace link has been copied to clipboard.', 'Info');
  const link = await getClipboardText(workspaces);
  await logout(workspaces);
  const encoded = encodeURIComponent(link);
  expect(encoded).toMatch(/^http%3A%2F%2F.+$/);
  const url = new URL(workspaces.url());
  const redirectUrl = `${url.origin}/webRedirect?webRedirectUrl=${encoded}`;
  const newTab = await workspaces.openNewTab({ location: redirectUrl });
  await expect(newTab.locator('.redirect-container')).toBeVisible();
  await expect(newTab.locator('.redirect-buttons__item').nth(1)).toHaveText('Continue in browser');
  await newTab.locator('.redirect-buttons__item').nth(1).click();
  await newTab.waitForTimeout(500);
  await newTab.locator('#password-input').locator('input').fill('P@ssw0rd.');
  await expect(newTab.locator('.login-button')).toBeEnabled();
  await newTab.locator('.login-button').click();
  await expect(newTab).toBeDocumentPage();
  await expect(newTab.locator('.folder-container').locator('.file-list-item')).toHaveCount(0);
});

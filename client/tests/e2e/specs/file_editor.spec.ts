// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, login, msTest } from '@tests/e2e/helpers';

msTest('Open editor with header option', async ({ documents }) => {
  const entries = documents.locator('.folder-container').locator('.file-list-item');

  await entries.nth(2).click();
  const actionBar = documents.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('ion-button').nth(1)).toHaveText('Edit');
  await actionBar.locator('ion-button').nth(1).click();
  await expect(documents.locator('#cryptpad-editor')).toBeVisible();
});

msTest('Open editor with contextual menu', async ({ documents }) => {
  const entries = documents.locator('.folder-container').locator('.file-list-item');
  await entries.nth(2).click({ button: 'right' });
  const menu = documents.locator('#file-context-menu');
  await expect(menu).toBeVisible();
  await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
  await menu.getByRole('listitem').nth(2).click();
  await expect(documents.locator('#cryptpad-editor')).toBeVisible();
});

msTest('Edit file in editor', async ({ documents }) => {
  const entries = documents.locator('.folder-container').locator('.file-list-item');

  // Promote Bob
  await documents.locator('.sidebar').locator('.sidebar-content-workspaces').nth(1).getByRole('listitem').click({ button: 'right' });
  await expect(documents.locator('ion-popover').locator('ion-item').nth(9)).toHaveText('Sharing and roles');
  await documents.locator('ion-popover').locator('ion-item').nth(9).click();
  const bobDropdown = documents.locator('ion-modal').locator('.user-list-members-item').locator('.dropdown-container');
  await expect(bobDropdown).toHaveText('Reader');
  await bobDropdown.click();
  await documents.locator('ion-popover').locator('ion-item').nth(0).click();
  await expect(bobDropdown).toHaveText('Owner');
  await documents.locator('ion-modal').locator('.closeBtn').click();

  // Open in editor with Alice
  await entries.nth(7).click({ button: 'right' });
  const menu = documents.locator('#file-context-menu');
  await expect(menu).toBeVisible();
  await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
  await menu.getByRole('listitem').nth(2).click();
  await expect(documents.locator('#cryptpad-editor')).toBeVisible();

  // Open editor with Bob
  const secondTab = await documents.openNewTab();
  await login(secondTab, 'Boby McBobFace');
  await secondTab.locator('.workspaces-container-grid').locator('.workspace-card-item').click();
  const secondEntries = secondTab.locator('.folder-container').locator('.file-list-item');
  await secondEntries.nth(7).click();
  const actionBar = secondTab.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('ion-button').nth(1)).toHaveText('Edit');
  await actionBar.locator('ion-button').nth(1).click();
  await expect(secondTab.locator('#cryptpad-editor')).toBeVisible();

  // Make some edits and check from the other user
  const editorAlice = documents
    .locator('iframe[name="frameEditor"]')
    .contentFrame()
    .locator('#sbox-iframe')
    .contentFrame()
    .locator('iframe[title="Editor\\, editor1"]')
    .contentFrame();
  const editorBob = secondTab
    .locator('iframe[name="frameEditor"]')
    .contentFrame()
    .locator('#sbox-iframe')
    .contentFrame()
    .locator('iframe[title="Editor\\, editor1"]')
    .contentFrame();
  await expect(editorAlice.getByText('A simple text file')).toBeVisible();
  await expect(editorBob.getByText('A simple text file')).toBeVisible();

  await editorAlice.getByText('A simple text file').fill('A brand new text file!');

  await expect(editorBob.getByText('A simple text file')).not.toBeVisible();
  await expect(editorBob.getByText('A brand new text file!')).toBeVisible();
  await editorBob.getByText('A brand new text file!').fill('A brand new text file, modified once again...');

  await expect(editorAlice.getByText('A simple text file')).not.toBeVisible();
  await expect(editorAlice.getByText('A brand new text file!')).not.toBeVisible();
  await expect(editorAlice.getByText('A brand new text file, modified once again...')).toBeVisible();
});

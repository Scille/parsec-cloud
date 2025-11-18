// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, login, msTest, waitUntilLoaded, waitUntilSaved } from '@tests/e2e/helpers';

msTest('Open editor with header option', async ({ parsecEditics }) => {
  msTest.setTimeout(120_000);
  const entries = parsecEditics.locator('.folder-container').locator('.file-list-item');

  await entries.nth(2).hover();
  await entries.nth(2).locator('ion-checkbox').click();
  const actionBar = parsecEditics.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('ion-button').nth(1)).toHaveText('Edit');
  await actionBar.locator('ion-button').nth(1).click();
  const frame = await waitUntilLoaded(parsecEditics);
  const editor = frame.locator('#cp-app-code-editor').locator('.CodeMirror-code');
  await expect(editor).toBeVisible();
  await expect(editor.locator('pre').nth(0)).toHaveText('A simple text file', { timeout: 30000 });
});

msTest('Open editor with contextual menu', async ({ parsecEditics }) => {
  msTest.setTimeout(120_000);
  const entries = parsecEditics.locator('.folder-container').locator('.file-list-item');
  await entries.nth(2).click({ button: 'right' });
  const menu = parsecEditics.locator('#file-context-menu');
  await expect(menu).toBeVisible();
  await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
  await menu.getByRole('listitem').nth(2).click();
  const frame = await waitUntilLoaded(parsecEditics);
  const editor = frame.locator('#cp-app-code-editor').locator('.CodeMirror-code');
  await expect(editor).toBeVisible();
  await expect(editor.locator('pre').nth(0)).toHaveText('A simple text file', { timeout: 30000 });
});

msTest('Open editor from viewer', async ({ parsecEditics }) => {
  msTest.setTimeout(120_000);
  const entries = parsecEditics.locator('.folder-container').locator('.file-list-item');
  await entries.nth(2).dblclick();
  await expect(parsecEditics.locator('.ms-spinner-modal')).toBeVisible();
  await expect(parsecEditics.locator('.ms-spinner-modal').locator('.spinner-label__text')).toHaveText('Opening file...');
  await expect(parsecEditics.locator('.ms-spinner-modal')).toBeHidden();
  await expect(parsecEditics).toBeViewerPage();

  const topbarEditButton = parsecEditics
    .locator('.file-handler-topbar')
    .locator('.file-handler-topbar-buttons')
    .locator('ion-button')
    .nth(1);
  await expect(topbarEditButton).toHaveText('Edit');
  await topbarEditButton.click();
  const frame = await waitUntilLoaded(parsecEditics);
  const editor = frame.locator('#cp-app-code-editor').locator('.CodeMirror-code');
  await expect(editor).toBeVisible();
  await expect(editor.locator('pre').nth(0)).toHaveText('A simple text file', { timeout: 30000 });
});

msTest('Check edited file in viewer', async ({ parsecEditics }) => {
  msTest.setTimeout(120_000);
  await parsecEditics.locator('.header-label-name').click();

  const entries = parsecEditics.locator('.folder-container').locator('.file-list-item');

  await entries.nth(2).hover();
  await entries.nth(2).locator('ion-checkbox').click();
  const actionBar = parsecEditics.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('ion-button').nth(1)).toHaveText('Edit');
  await actionBar.locator('ion-button').nth(1).click();
  const frame = await waitUntilLoaded(parsecEditics);
  const editor = frame.locator('#cp-app-code-editor').locator('.CodeMirror-code');
  await expect(editor).toBeVisible();
  await expect(editor.locator('pre').nth(0)).toHaveText(
    '# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS',
    { timeout: 30000 },
  );
  await parsecEditics.waitForTimeout(200);
  await editor.locator('pre').nth(0).fill('ABCD');
  await expect(editor.locator('pre').nth(0)).toHaveText('ABCD');
  await editor.blur();
  await expect(parsecEditics.locator('#unsaved-changes')).toBeVisible();
  await parsecEditics.waitForTimeout(500);
  await waitUntilSaved(parsecEditics);

  await parsecEditics.locator('.file-handler-topbar').locator('.back-button').click();
  await entries.nth(2).dblclick();
  await expect(parsecEditics.locator('.ms-spinner-modal')).toBeVisible();
  await expect(parsecEditics.locator('.ms-spinner-modal').locator('.spinner-label__text')).toHaveText('Opening file...');
  await expect(parsecEditics.locator('.ms-spinner-modal')).toBeHidden();
  await expect(parsecEditics).toBeViewerPage();
  const content = await parsecEditics.locator('.file-viewer-wrapper').locator('.editor-scrollable').textContent();
  expect(content?.startsWith('ABCD')).toBeTruthy();
});

// TODO: re-enable when collaborative editing is properly supported
msTest.skip('Edit file in editor with two users', async ({ parsecEditics }) => {
  msTest.setTimeout(120_000);
  const entries = parsecEditics.locator('.folder-container').locator('.file-list-item');

  // Promote Bob
  await parsecEditics.locator('.sidebar').locator('.sidebar-content-workspaces').nth(1).getByRole('listitem').click({ button: 'right' });
  await expect(parsecEditics.locator('ion-popover').locator('ion-item').nth(9)).toHaveText('Sharing and roles');
  await parsecEditics.locator('ion-popover').locator('ion-item').nth(9).click();
  const bobDropdown = parsecEditics.locator('ion-modal').locator('.user-list-members-item').locator('.dropdown-container');
  await expect(bobDropdown).toHaveText('Reader');
  await bobDropdown.click();
  await parsecEditics.locator('ion-popover').locator('ion-item').nth(0).click();
  await expect(bobDropdown).toHaveText('Owner');
  await parsecEditics.locator('ion-modal').locator('.closeBtn').click();

  // Open in editor with Alice
  await entries.nth(2).click({ button: 'right' });
  const menu = parsecEditics.locator('#file-context-menu');
  await expect(menu).toBeVisible();
  await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
  await menu.getByRole('listitem').nth(2).click();
  await expect(parsecEditics.locator('#cryptpad-editor')).toBeVisible();
  const mainFrameAlice = parsecEditics.locator('#cryptpad-editor').contentFrame();
  await expect(mainFrameAlice.locator('.placeholder-message-container')).toBeVisible();
  await expect(mainFrameAlice.locator('.placeholder-message-container')).toHaveText('Loading...');
  // Takes an incredibly long time to load on the CI
  await parsecEditics.waitForTimeout(10000);

  // Open editor with Bob
  const secondTab = await parsecEditics.openNewTab();
  await login(secondTab, 'Boby McBobFace');
  await secondTab.locator('.workspaces-container-grid').locator('.workspace-card-item').click();
  const secondEntries = secondTab.locator('.folder-container').locator('.file-list-item');
  await secondEntries.nth(2).hover();
  await secondEntries.nth(2).locator('ion-checkbox').click();
  const actionBar = secondTab.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('ion-button').nth(1)).toHaveText('Edit');
  await actionBar.locator('ion-button').nth(1).click();
  await expect(secondTab.locator('#cryptpad-editor')).toBeVisible();
  const mainFrameBob = secondTab.locator('#cryptpad-editor').contentFrame();
  await expect(mainFrameBob.locator('.placeholder-message-container')).toBeVisible();
  await expect(mainFrameBob.locator('.placeholder-message-container')).toHaveText('Loading...');
  // Takes an incredibly long time to load on the CI
  await secondTab.waitForTimeout(10000);

  // Make some edits and check from the other user
  await expect(mainFrameAlice.locator('#sbox-iframe')).toBeVisible();
  const editorAlice = parsecEditics
    .locator('#cryptpad-editor')
    .contentFrame()
    .locator('#sbox-iframe')
    .contentFrame()
    .locator('#cp-app-code-editor')
    .locator('.CodeMirror-code')
    .locator('pre')
    .nth(0);
  const editorBob = secondTab
    .locator('#cryptpad-editor')
    .contentFrame()
    .locator('#sbox-iframe')
    .contentFrame()
    .locator('#cp-app-code-editor')
    .locator('.CodeMirror-code')
    .locator('pre')
    .nth(0);
  await expect(editorAlice).toHaveText('# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS');
  await expect(editorBob).toHaveText('# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS');

  await editorAlice.fill('New first line!');
  await expect(editorAlice).toHaveText('New first line!');
  await waitUntilSaved(parsecEditics);
  await expect(editorBob).toHaveText('New first line!');
  await editorBob.fill('New NEWER first line!');
  await expect(editorBob).toHaveText('New NEWER first line!');
  await waitUntilSaved(secondTab);
  await expect(editorAlice).toHaveText('New NEWER first line!');
});

msTest.skip('Check file edited by other user', async ({ parsecEditics }) => {
  msTest.setTimeout(120_000);
  await parsecEditics.locator('.header-label-name').click();
  const entries = parsecEditics.locator('.folder-container').locator('.file-list-item');

  // Promote Bob
  await parsecEditics.locator('.sidebar').locator('.sidebar-content-workspaces').nth(1).getByRole('listitem').click({ button: 'right' });
  await expect(parsecEditics.locator('ion-popover').locator('ion-item').nth(9)).toHaveText('Sharing and roles');
  await parsecEditics.locator('ion-popover').locator('ion-item').nth(9).click();
  const bobDropdown = parsecEditics.locator('ion-modal').locator('.user-list-members-item').locator('.dropdown-container');
  await expect(bobDropdown).toHaveText('Reader');
  await bobDropdown.click();
  await parsecEditics.locator('ion-popover').locator('ion-item').nth(0).click();
  await expect(bobDropdown).toHaveText('Owner');
  await parsecEditics.locator('ion-modal').locator('.closeBtn').click();

  // Open in editor with Alice
  await entries.nth(2).click({ button: 'right' });
  const menu = parsecEditics.locator('#file-context-menu');
  await expect(menu).toBeVisible();
  await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
  await menu.getByRole('listitem').nth(2).click();
  await expect(parsecEditics.locator('#cryptpad-editor')).toBeVisible();
  const mainFrameAlice = parsecEditics.locator('#cryptpad-editor').contentFrame();
  await expect(mainFrameAlice.locator('.placeholder-message-container')).toBeVisible();
  await expect(mainFrameAlice.locator('.placeholder-message-container')).toHaveText('Loading...');
  // Takes an incredibly long time to load on the CI
  await parsecEditics.waitForTimeout(10000);

  // Make some edits and check from the other user
  await expect(mainFrameAlice.locator('#sbox-iframe')).toBeVisible();
  const editorAlice = parsecEditics
    .locator('#cryptpad-editor')
    .contentFrame()
    .locator('#sbox-iframe')
    .contentFrame()
    .locator('#cp-app-code-editor')
    .locator('.CodeMirror-code')
    .locator('pre')
    .nth(0);
  await expect(editorAlice).toBeVisible();
  await expect(editorAlice).toHaveText('# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS');
  await editorAlice.fill('New first line!');
  await waitUntilSaved(parsecEditics);

  // Open editor with Bob
  const secondTab = await parsecEditics.openNewTab();
  await login(secondTab, 'Boby McBobFace');
  await secondTab.locator('.workspaces-container-grid').locator('.workspace-card-item').click();
  const secondEntries = secondTab.locator('.folder-container').locator('.file-list-item');
  await secondEntries.nth(2).hover();
  await secondEntries.nth(2).locator('ion-checkbox').click();
  const actionBar = secondTab.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('ion-button').nth(1)).toHaveText('Edit');
  await actionBar.locator('ion-button').nth(1).click();
  await expect(secondTab.locator('#cryptpad-editor')).toBeVisible();

  const mainFrameBob = secondTab.locator('#cryptpad-editor').contentFrame();
  await expect(mainFrameBob.locator('.placeholder-message-container')).toBeVisible();
  await expect(mainFrameBob.locator('.placeholder-message-container')).toHaveText('Loading...');
  // Takes an incredibly long time to load on the CI
  await secondTab.waitForTimeout(10000);

  // Check modified text
  const editorBob = secondTab
    .locator('#cryptpad-editor')
    .contentFrame()
    .locator('#sbox-iframe')
    .contentFrame()
    .locator('#cp-app-code-editor')
    .locator('.CodeMirror-code')
    .locator('pre')
    .nth(0);
  await expect(editorBob).toBeVisible();
  await expect(editorBob).toHaveText('New first line!');
});

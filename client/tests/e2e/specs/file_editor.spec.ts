// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import {
  answerQuestion,
  expect,
  fillInputModal,
  getClipboardText,
  importDefaultFiles,
  ImportDocuments,
  login,
  mockCryptpadServer,
  msTest,
  waitUntilSaved,
} from '@tests/e2e/helpers';

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });

  for (const [mode, method] of [
    ['edit', 'context'],
    ['view', 'context'],
    ['edit', 'header'],
    ['view', 'header'],
  ]) {
    msTest(`Open cryptpad in ${mode} mode with ${method}`, async ({ parsecEditics }, testInfo: TestInfo) => {
      await mockCryptpadServer(parsecEditics);
      await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Docx, false);
      const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);

      if (method === 'header') {
        await entry.locator('.file-last-update').click();
        const actionBar = parsecEditics.locator('#folders-ms-action-bar');
        if (mode === 'edit') {
          await expect(actionBar.locator('ion-button').nth(1)).toHaveText('Edit');
          await actionBar.locator('ion-button').nth(1).click();
        } else {
          await expect(actionBar.locator('ion-button').nth(0)).toHaveText('Preview');
          await actionBar.locator('ion-button').nth(0).click();
        }
      } else {
        await entry.click({ button: 'right' });
        const menu = parsecEditics.locator('#file-context-menu');
        await expect(menu).toBeVisible();
        if (mode === 'edit') {
          await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
          await menu.getByRole('listitem').nth(2).click();
        } else {
          await expect(menu.getByRole('listitem').nth(1)).toHaveText('Preview');
          await menu.getByRole('listitem').nth(1).click();
        }
      }
      await expect(parsecEditics.locator('.file-editor')).toBeVisible();
      const frame = parsecEditics.frameLocator('.file-editor');
      await expect(frame.locator('#editor-container')).toBeVisible();
      const topbar = parsecEditics.locator('.file-handler-topbar');
      await expect(topbar.locator('.file-handler-topbar__title')).toHaveText('document.docx');
      await expect(topbar.locator('.back-button')).toBeVisible();
      const topbarButtons = topbar.locator('.file-handler-topbar-buttons').locator('.file-handler-topbar-buttons__item:visible');
      if (mode === 'edit') {
        await expect(topbar.locator('.save-info')).toBeHidden();
        await expect(topbarButtons).toHaveCount(4);
        await expect(topbarButtons).toHaveText(['Details', 'Copy link', 'Download', 'Show menu']);
      } else {
        await expect(topbar.locator('.save-info')).toBeVisible();
        await expect(topbar.locator('.save-info')).toHaveText('Read only document');
        await expect(topbarButtons).toHaveCount(5);
        await expect(topbarButtons).toHaveText(['Details', 'Copy link', 'Edit', 'Download', 'Show menu']);
      }
      await expect(frame.locator('#editor-container')).toHaveText('document.docx');
    });
  }

  for (const action of ['details', 'copy_link', 'edit', 'download', 'show_menu']) {
    msTest(`File editor header '${action}' action`, async ({ parsecEditics }, testInfo: TestInfo) => {
      await mockCryptpadServer(parsecEditics);
      await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Docx, false);
      const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);

      await entry.click({ button: 'right' });
      const menu = parsecEditics.locator('#file-context-menu');
      await expect(menu).toBeVisible();
      await expect(menu.getByRole('listitem').nth(1)).toHaveText('Preview');
      await menu.getByRole('listitem').nth(1).click();

      await expect(parsecEditics.locator('.file-editor')).toBeVisible();
      const frame = parsecEditics.frameLocator('.file-editor');
      await expect(frame.locator('#editor-container')).toBeVisible();
      await expect(frame.locator('#editor-container')).toHaveText('document.docx');
      const topbar = parsecEditics.locator('.file-handler-topbar');
      await expect(topbar.locator('.file-handler-topbar__title')).toHaveText('document.docx');
      await expect(topbar.locator('.back-button')).toBeVisible();
      const topbarButtons = topbar.locator('.file-handler-topbar-buttons').locator('.file-handler-topbar-buttons__item:visible');
      await expect(topbarButtons).toHaveText(['Details', 'Copy link', 'Edit', 'Download', 'Show menu']);
      await expect(topbar.locator('.save-info')).toBeVisible();
      await expect(topbar.locator('.save-info')).toHaveText('Read only document');

      if (action === 'details') {
        const modal = parsecEditics.locator('.file-details-modal');
        await expect(modal).toBeHidden();
        await topbarButtons.nth(0).click();
        await expect(modal).toBeVisible();
      } else if (action === 'copy_link') {
        await parsecEditics.context().grantPermissions(['clipboard-write']);
        await topbarButtons.nth(1).click();
        await expect(parsecEditics).toShowToast('Link has been copied to clipboard.', 'Info');
        expect(await getClipboardText(parsecEditics)).toMatch(/^https?:\/\/.+\/redirect\/.+a=path&p=.+$/);
      } else if (action === 'edit') {
        await topbarButtons.nth(2).click();
        await expect(topbar.locator('.save-info')).toBeHidden();
        await expect(topbarButtons).toHaveText(['Details', 'Copy link', 'Download', 'Show menu']);
      } else if (action === 'download') {
        const modal = parsecEditics.locator('.download-warning-modal');
        await expect(modal).toBeHidden();
        await topbarButtons.nth(3).click();
        await expect(modal).toBeVisible();
      } else if (action === 'show_menu') {
        const header = parsecEditics.locator('#connected-header');
        await expect(header).toBeHidden();
        await topbarButtons.nth(4).click();
        await expect(header).toBeVisible();
      }
    });
  }

  for (const error of ['404', 'timeout']) {
    msTest(`File editor failing to get frame because of ${error}`, async ({ parsecEditics }, testInfo: TestInfo) => {
      await mockCryptpadServer(parsecEditics, { timeout: error === 'timeout', httpErrorCode: error === '404' ? 404 : 200 });
      await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Docx, false);
      const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);

      await entry.click({ button: 'right' });
      const menu = parsecEditics.locator('#file-context-menu');
      await expect(menu).toBeVisible();
      await expect(menu.getByRole('listitem').nth(1)).toHaveText('Preview');
      await menu.getByRole('listitem').nth(1).click();

      await expect(parsecEditics.locator('.file-editor')).toBeHidden();
      const errorContainer = parsecEditics.locator('.file-editor-error');
      await expect(errorContainer).toBeVisible();
      await expect(errorContainer.locator('.error-content-text__title')).toHaveText('Cannot open file');
      await expect(errorContainer.locator('.error-content-text__message')).toHaveText(
        'Could not load the editor. Please check your network connection.',
      );
    });
  }

  for (const error of ['initFail', 'openFail']) {
    msTest(`File editor failing to get frame because of ${error}`, async ({ parsecEditics }, testInfo: TestInfo) => {
      await mockCryptpadServer(parsecEditics, { failInit: error === 'initFail', failOpen: error === 'openFail' });
      await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Docx, false);
      const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);

      await entry.click({ button: 'right' });
      const menu = parsecEditics.locator('#file-context-menu');
      await expect(menu).toBeVisible();
      await expect(menu.getByRole('listitem').nth(1)).toHaveText('Preview');
      await menu.getByRole('listitem').nth(1).click();

      await expect(parsecEditics.locator('.file-editor')).toBeHidden();
      const errorContainer = parsecEditics.locator('.file-editor-error');
      await expect(errorContainer).toBeVisible();
      await expect(errorContainer.locator('.error-content-text__title')).toHaveText('Cannot open file');
      if (error === 'initFail') {
        await expect(errorContainer.locator('.error-content-text__message')).toHaveText(
          'If you want to edit this file, you can download it and open it locally on your device.',
        );
      } else {
        await expect(errorContainer.locator('.error-content-text__message')).toHaveText('Could not open the file.');
      }
    });
  }

  msTest('Error after load', async ({ parsecEditics }, testInfo: TestInfo) => {
    await mockCryptpadServer(parsecEditics, {
      customOpenFunction: `
        sendToParent({ command: 'editics-open-result', success: true });
        setTimeout(() => {
          sendToParent({ command: 'editics-event', event: 'ready' });
        }, 100);
        setTimeout(() => {
          sendToParent({ command: 'editics-event', event: 'error', 'details': 'file error' });
        }, 800);
      `,
    });
    await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Docx, false);
    const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);

    await entry.click({ button: 'right' });
    const menu = parsecEditics.locator('#file-context-menu');
    await expect(menu).toBeVisible();
    await expect(menu.getByRole('listitem').nth(1)).toHaveText('Preview');
    await menu.getByRole('listitem').nth(1).click();

    await expect(parsecEditics.locator('.file-editor')).toBeHidden();
    const errorContainer = parsecEditics.locator('.file-editor-error');
    await expect(errorContainer).toBeVisible();
    await expect(errorContainer.locator('.error-content-text__title')).toHaveText('Cannot open file');
    await expect(errorContainer.locator('.error-content-text__message')).toHaveText('Failed to open the file');
  });

  msTest('File editor save status', async ({ parsecEditics }, testInfo: TestInfo) => {
    /* eslint-disable max-len */
    await mockCryptpadServer(parsecEditics, {
      customOpenFunction: `
        sendToParent({ command: 'editics-open-result', success: true });
        document.getElementById('editor-container').innerText = data.documentName;
        setTimeout(() => {
          sendToParent({ command: 'editics-event', event: 'ready' });
        }, 100);
        setTimeout(() => {
          sendToParent({ command: 'editics-event', event: 'save-status', saved: false });
        }, 300);
        setTimeout(() => {
          sendToParent({ command: 'editics-event', event: 'save', documentContent: new Blob([42, 42, 42, 42, 42, 42, 42], { type: 'application/octet-stream' }) });
        }, 800);
        setTimeout(() => {
          sendToParent({ command: 'editics-event', event: 'save-status', saved: true });
          }, 1000);
      `,
    });
    /* eslint-enable max-len */
    await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Docx, false);
    const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);

    await entry.click({ button: 'right' });
    const menu = parsecEditics.locator('#file-context-menu');
    await expect(menu).toBeVisible();
    await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
    await menu.getByRole('listitem').nth(2).click();

    const frame = parsecEditics.frameLocator('.file-editor');
    await expect(frame.locator('#editor-container')).toBeVisible();
    await expect(frame.locator('#editor-container')).toHaveText('document.docx');
    await expect(parsecEditics.locator('.file-editor-error')).toBeHidden();
    const topbar = parsecEditics.locator('.file-handler-topbar');
    await expect(topbar.locator('.save-info')).toBeVisible();
    await expect(topbar.locator('.save-info-text')).toBeVisible();
    await expect(topbar.locator('.save-info-text')).toHaveText('Changes unsaved');
    await parsecEditics.waitForTimeout(800);
    await expect(topbar.locator('.save-info-text')).toHaveText('File saved');
  });

  msTest('Go back with unsaved status', async ({ parsecEditics }, testInfo: TestInfo) => {
    await mockCryptpadServer(parsecEditics, {
      customOpenFunction: `
        sendToParent({ command: 'editics-open-result', success: true });
        document.getElementById('editor-container').innerText = data.documentName;
        setTimeout(() => {
          sendToParent({ command: 'editics-event', event: 'ready' });
        }, 100);
        setTimeout(() => {
          sendToParent({ command: 'editics-event', event: 'save-status', saved: false });
        }, 300);
      `,
    });
    await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Docx, false);
    const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);

    await entry.click({ button: 'right' });
    const menu = parsecEditics.locator('#file-context-menu');
    await expect(menu).toBeVisible();
    await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
    await menu.getByRole('listitem').nth(2).click();

    const frame = parsecEditics.frameLocator('.file-editor');
    await expect(frame.locator('#editor-container')).toBeVisible();
    await expect(frame.locator('#editor-container')).toHaveText('document.docx');
    await expect(parsecEditics.locator('.file-editor-error')).toBeHidden();
    const topbar = parsecEditics.locator('.file-handler-topbar');
    await parsecEditics.waitForTimeout(500);
    await expect(topbar.locator('.save-info')).toBeVisible();
    await expect(topbar.locator('.save-info-text')).toBeVisible();
    await expect(topbar.locator('.save-info-text')).toHaveText('Changes unsaved');
    await topbar.locator('.back-button').click();
    await answerQuestion(parsecEditics, true, {
      expectedNegativeText: 'Stay on page',
      expectedPositiveText: 'Discard changes',
      expectedQuestionText: 'Some changes have not been saved yet. Do you want to discard them?',
      expectedTitleText: 'Discard changes?',
    });
    await expect(parsecEditics).toBeDocumentPage();
  });

  msTest('Update text file', async ({ parsecEditics }, testInfo: TestInfo) => {
    /* eslint-disable max-len */
    await mockCryptpadServer(parsecEditics, {
      customOpenFunction: `
        sendToParent({ command: 'editics-open-result', success: true });
        document.getElementById('editor-container').innerText = new TextDecoder().decode(data.documentContent);
        setTimeout(() => {
          sendToParent({ command: 'editics-event', event: 'ready' });
        }, 100);
        setTimeout(() => {
          const bytes = new TextEncoder().encode('A simple text file with updates');
          sendToParent({ command: 'editics-event', event: 'save', documentContent: new Blob([new Uint8Array(bytes)], { type: 'application/octet-stream' }) });
        }, 300);
      `,
    });
    /* eslint-enable max-len */
    await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Txt, false);
    const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);

    await expect(entry.locator('.file-size')).toHaveText('19 B');

    await entry.click({ button: 'right' });
    const menu = parsecEditics.locator('#file-context-menu');
    await expect(menu).toBeVisible();
    await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
    await menu.getByRole('listitem').nth(2).click();
    await expect(menu).toBeHidden();

    const frame = parsecEditics.frameLocator('.file-editor');
    await expect(frame.locator('#editor-container')).toBeVisible();
    await expect(frame.locator('#editor-container')).toHaveText('A simple text file');
    await expect(parsecEditics.locator('.file-editor-error')).toBeHidden();
    const topbar = parsecEditics.locator('.file-handler-topbar');
    await parsecEditics.waitForTimeout(800);
    await topbar.locator('.back-button').click();
    await expect(parsecEditics).toBeDocumentPage();
    await expect(entry.locator('.file-size')).toHaveText('31 B');

    await entry.click({ button: 'right' });
    parsecEditics.locator('#file-context-menu');
    await expect(menu).toBeVisible();
    await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
    await menu.getByRole('listitem').nth(2).click();
    await expect(menu).toBeHidden();

    await expect(frame.locator('#editor-container')).toBeVisible();
    await expect(frame.locator('#editor-container')).toHaveText('A simple text file with updates');
  });

  msTest('Check files handled', async ({ parsecEditics }, testInfo: TestInfo) => {
    // Makes sure that some files cannot be opened, and also checks that the opening + back + opening + ...
    // works properly.

    msTest.setTimeout(60_000);

    const FILES = [
      { fileName: 'file.txt', opener: 'editor', renameIndex: 3 },
      { fileName: 'file.html', opener: 'editor', renameIndex: 3 },
      { fileName: 'file.odt', opener: 'editor', renameIndex: 3 },
      { fileName: 'file.docx', opener: 'editor', renameIndex: 3 },
      { fileName: 'file.doc', opener: undefined, renameIndex: 3 },
      { fileName: 'file.xls', opener: 'editor', renameIndex: 2 },
      { fileName: 'file.xlsx', opener: 'editor', renameIndex: 3 },
      { fileName: 'file.ods', opener: 'editor', renameIndex: 3 },
      { fileName: 'file.pptx', opener: 'editor', renameIndex: 3 },
      { fileName: 'file.odp', opener: undefined, renameIndex: 3 },
      { fileName: 'file.ppt', opener: undefined, renameIndex: 2 },
      { fileName: 'file.rtf', opener: undefined, renameIndex: 2 },
      { fileName: 'file.log', opener: 'editor', renameIndex: 2 },
      { fileName: 'file.png', opener: 'viewer', renameIndex: 3 },
      { fileName: 'file.pdf', opener: 'viewer', renameIndex: 2 },
      { fileName: 'file.mp3', opener: 'viewer', renameIndex: 2 },
      { fileName: 'file.mp4', opener: 'viewer', renameIndex: 2 },
      { fileName: 'file', opener: undefined, renameIndex: 2 },
    ];

    await mockCryptpadServer(parsecEditics);
    // Doesn't matter which one we import initially
    await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Txt, false);
    const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);

    for (const fileData of FILES) {
      const menu = parsecEditics.locator('#file-context-menu');

      console.log(`Checking file '${fileData.fileName}'`);

      // Rename the file first, we match the extension
      await entry.click({ button: 'right' });
      await expect(menu).toBeVisible();
      await expect(menu.getByRole('listitem').nth(fileData.renameIndex)).toHaveText('Rename');
      await menu.getByRole('listitem').nth(fileData.renameIndex).click();
      await fillInputModal(parsecEditics, fileData.fileName);
      expect(menu).toBeHidden();
      await expect(entry.locator('.label-name')).toHaveText(fileData.fileName);
      await entry.click({ button: 'right' });
      await expect(menu).toBeVisible();
      await expect(menu.getByRole('listitem').nth(1)).toHaveText('Preview');
      await menu.getByRole('listitem').nth(1).click();

      if (!fileData.opener) {
        await expect(parsecEditics).toShowInformationModal(
          'Parsec cannot preview this type of document. You can download it ' +
            "by selecting the file or showing options then click 'Download'.",
          'Info',
          'Cannot preview this file',
        );
        await expect(parsecEditics).toBeDocumentPage();
      } else if (fileData.opener === 'viewer') {
        await expect(parsecEditics).toBeViewerPage();
        const topbar = parsecEditics.locator('.file-handler-topbar');
        await expect(topbar.locator('.file-handler-topbar__title')).toHaveText(fileData.fileName);
        await expect(topbar.locator('.back-button')).toBeVisible();
        await topbar.locator('.back-button').click();
        await expect(parsecEditics).toBeDocumentPage();
      } else {
        await expect(parsecEditics).toBeEditorPage();
        await expect(parsecEditics.locator('.file-editor')).toBeVisible();
        const frame = parsecEditics.frameLocator('.file-editor');
        await expect(frame.locator('#editor-container')).toBeVisible();
        const topbar = parsecEditics.locator('.file-handler-topbar');
        await expect(topbar.locator('.file-handler-topbar__title')).toHaveText(fileData.fileName);
        await expect(topbar.locator('.back-button')).toBeVisible();
        await topbar.locator('.back-button').click();
        await expect(parsecEditics).toBeDocumentPage();
      }
    }
  });
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
  await secondEntries.nth(2).locator('.ms-checkbox').check();
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
  await secondEntries.nth(2).locator('.ms-checkbox').check();
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

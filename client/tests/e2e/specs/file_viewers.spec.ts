// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { expect, msTest } from '@tests/e2e/helpers';

async function openFileType(documentsPage: Page, type: 'xlsx' | 'docx' | 'png' | 'pdf' | 'mp3' | 'mp4' | 'txt' | 'py'): Promise<void> {
  const entries = documentsPage.locator('.folder-container').locator('.file-list-item');

  for (const entry of await entries.all()) {
    const entryName = (await entry.locator('.file-name').locator('.file-name__label').textContent()) ?? '';
    if (entryName.endsWith(`.${type}`)) {
      await entry.dblclick();
      return;
    }
  }
}

msTest('Documents page default state', async ({ documents }) => {
  const entries = documents.locator('.folder-container').locator('.file-list-item');

  await entries.nth(2).dblclick();
  await expect(documents.locator('.ms-spinner-modal')).toBeVisible();
  await documents.waitForTimeout(500);
  await expect(documents.locator('.ms-spinner-modal')).toBeHidden();
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_.]+$/);
});

msTest('Spreadsheet viewer', async ({ documents }) => {
  await openFileType(documents, 'xlsx');
  await documents.waitForTimeout(500);
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.xlsx$/);
});

msTest('Document viewer', async ({ documents }) => {
  await openFileType(documents, 'docx');
  await documents.waitForTimeout(500);
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.docx$/);
});

msTest('PDF viewer', async ({ documents }) => {
  await openFileType(documents, 'pdf');
  await documents.waitForTimeout(500);
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.pdf$/);
});

msTest('Image viewer', async ({ documents }) => {
  await openFileType(documents, 'png');
  await documents.waitForTimeout(500);
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.png$/);
});

msTest('Audio viewer', async ({ documents }) => {
  await openFileType(documents, 'mp3');
  await documents.waitForTimeout(500);
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.mp3$/);
});

msTest('Video viewer', async ({ documents }) => {
  await openFileType(documents, 'mp4');
  await documents.waitForTimeout(500);
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.mp4$/);
});

msTest('Text viewer', async ({ documents }) => {
  await openFileType(documents, 'py');
  await documents.waitForTimeout(500);
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.py$/);
});

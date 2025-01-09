// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { Media, expect, expectMedia, msTest } from '@tests/e2e/helpers';

async function openFileType(documentsPage: Page, type: 'xlsx' | 'docx' | 'png' | 'pdf' | 'mp3' | 'mp4' | 'txt' | 'py'): Promise<void> {
  const entries = documentsPage.locator('.folder-container').locator('.file-list-item');

  for (const entry of await entries.all()) {
    const entryName = (await entry.locator('.file-name').locator('.file-name__label').textContent()) ?? '';
    if (entryName.endsWith(`.${type}`)) {
      await entry.dblclick();
      await expect(documentsPage.locator('.ms-spinner-modal')).toBeVisible();
      await expect(documentsPage.locator('.ms-spinner-modal').locator('.spinner-label__text')).toHaveText('Opening file...');
      await expect(documentsPage.locator('.ms-spinner-modal')).toBeHidden();
      await expect(documentsPage).toBeViewerPage();
      return;
    }
  }
}

msTest('Documents page default state', async ({ documents }) => {
  const entries = documents.locator('.folder-container').locator('.file-list-item');

  await entries.nth(2).dblclick();
  await expect(documents.locator('.ms-spinner-modal')).toBeVisible();
  await expect(documents.locator('.ms-spinner-modal').locator('.spinner-label__text')).toHaveText('Opening file...');
  await expect(documents.locator('.ms-spinner-modal')).toBeHidden();
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_.]+$/);
});

msTest('Spreadsheet viewer', async ({ documents }) => {
  await openFileType(documents, 'xlsx');
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.xlsx$/);
  const bottomBar = documents.locator('.file-viewer-bottombar');
  await expect(bottomBar.locator('.file-controls-button')).toHaveText(['Sheet1', 'Sheet2']);
  const wrapper = documents.locator('.file-viewer-wrapper');
  await expect(wrapper.locator('.spreadsheet-content').locator('td')).toHaveText(['A', '1', 'B', '2', 'C', '3', 'D', '4']);
  // Switch to second sheet
  await bottomBar.locator('.file-controls-button').nth(1).click();
  await expect(wrapper.locator('.spreadsheet-content').locator('td')).toHaveText(['E', '5', 'F', '6', 'G', '7', 'H', '8']);
});

msTest('Document viewer', async ({ documents }) => {
  await openFileType(documents, 'docx');
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.docx$/);
  const bottomBar = documents.locator('.file-viewer-bottombar');
  await expect(bottomBar).toBeEmpty();
  const wrapper = documents.locator('.file-viewer-wrapper');
  await expect(wrapper.locator('.document-content').locator('p')).toHaveText([
    'Title',
    '',
    'BOLD',
    'Italic',
    'Underline',
    'RED',
    'parsec.cloud',
  ]);
  await expect(wrapper.locator('.document-content').locator('h2')).toHaveText('Subtitle');
});

msTest('PDF viewer', async ({ documents }) => {
  await openFileType(documents, 'pdf');
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.pdf$/);
  const wrapper = documents.locator('.file-viewer-wrapper');
  const canvas = wrapper.locator('.pdf-container').locator('canvas');
  const canvasWidth = Number(await canvas.getAttribute('width'));
  const canvasHeight = Number(await canvas.getAttribute('height'));

  const bottomBar = documents.locator('.file-viewer-bottombar');
  const buttons = bottomBar.locator('.file-controls-button');
  await expect(buttons).toHaveText(['', '', '', '']);

  // Pagination
  const pagination = bottomBar.locator('.file-controls-pagination');
  await expect(pagination).toHaveCount(1);
  const input = pagination.locator('.file-controls-input');
  await expect(input).toHaveText('Page 1 / 2');

  // Zoom-
  const zoom = bottomBar.locator('.file-controls-zoom');
  await expect(zoom).toHaveCount(1);
  await buttons.nth(1).click();
  expect(Number(await canvas.getAttribute('width'))).toBeLessThan(canvasWidth);
  expect(Number(await canvas.getAttribute('height'))).toBeLessThan(canvasHeight);

  // Restore zoom
  await buttons.nth(0).click();
  expect(Number(await canvas.getAttribute('width'))).toBe(canvasWidth);
  expect(Number(await canvas.getAttribute('height'))).toBe(canvasHeight);

  // Zoom+
  await buttons.nth(2).click();
  expect(Number(await canvas.getAttribute('width'))).toBeGreaterThan(canvasWidth);
  expect(Number(await canvas.getAttribute('height'))).toBeGreaterThan(canvasHeight);
});

msTest('Image viewer', async ({ documents }) => {
  await openFileType(documents, 'png');
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.png$/);

  const wrapper = documents.locator('.file-viewer-wrapper');
  await expect(wrapper.locator('img')).toBeVisible();
  const bottomBar = documents.locator('.file-viewer-bottombar');
  const zoom = bottomBar.locator('.file-controls-zoom');
  await expect(zoom).toHaveCount(1);
});

msTest('Audio viewer', async ({ documents }) => {
  await openFileType(documents, 'mp3');
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.mp3$/);

  const bottomBar = documents.locator('.file-viewer-bottombar');
  const buttons = bottomBar.locator('.file-controls-button');
  const wrapper = documents.locator('.file-viewer-wrapper');
  const audio = wrapper.locator('audio');
  const fluxBar = bottomBar.locator('.slider').nth(0);
  const volumeSlider = bottomBar.locator('.slider').nth(1);

  await expectMedia(audio).toHaveDuration(7.967347);
  await expectMedia(audio).toHaveCurrentTime(0.0);

  // Volume control
  const volumeButton = buttons.nth(1);
  await expectMedia(audio).toHaveVolume(1);
  await volumeSlider.click();
  await expectMedia(audio).toHaveVolume(0.48);
  await volumeButton.click();
  await expectMedia(audio).toHaveVolume(0);
  await volumeButton.click();
  await expectMedia(audio).toHaveVolume(0.48);

  // Stream control
  await buttons.nth(0).click();
  await documents.waitForTimeout(500);
  await buttons.nth(0).click();
  expect(await Media.getCurrentTime(audio)).toBeGreaterThan(0.1);

  await fluxBar.click();
  await expectMedia(audio).toHaveCurrentTime(3.98);
});

msTest('Video viewer', async ({ documents }) => {
  await openFileType(documents, 'mp4');
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.mp4$/);

  const bottomBar = documents.locator('.file-viewer-bottombar');
  const buttons = bottomBar.locator('.file-controls-button');
  const wrapper = documents.locator('.file-viewer-wrapper');
  const video = wrapper.locator('video');
  const fluxBar = bottomBar.locator('.slider').nth(0);
  const volumeSlider = bottomBar.locator('.slider').nth(1);

  await expect(buttons).toHaveCount(3);

  const readyState = await video.evaluate((videoEl) => {
    return (videoEl as HTMLMediaElement).readyState;
  });
  expect(readyState).toBe(4);

  const videoWidth = await video.evaluate((el) => {
    return (el as HTMLVideoElement).videoWidth;
  });
  const videoHeight = await video.evaluate((el) => {
    return (el as HTMLVideoElement).videoHeight;
  });
  expect(videoWidth).toBe(870);
  expect(videoHeight).toBe(692);

  expect(await Media.getDuration(video)).toBe(3.562646);
  expect(await Media.getCurrentTime(video)).toBe(0.0);
  await expectMedia(video).toHaveVolume(1);

  await buttons.nth(0).click();
  await documents.waitForTimeout(500);
  await buttons.nth(0).click();
  expect(await Media.getCurrentTime(video)).toBeGreaterThan(0.1);

  // Volume control
  await expectMedia(video).toHaveVolume(1);
  await volumeSlider.click();
  await expectMedia(video).toHaveVolume(0.49);
  await buttons.nth(1).click();
  await expectMedia(video).toHaveVolume(0);
  await buttons.nth(1).click();
  await expectMedia(video).toHaveVolume(0.49);

  // Stream control
  await fluxBar.click();
  await expectMedia(video).toHaveCurrentTime(1.77);
});

msTest('Text viewer', async ({ documents }) => {
  await openFileType(documents, 'py');
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.py$/);

  const container = documents.locator('.file-viewer').locator('.text-container');
  const linesCount = (await container.locator('.margin').locator('.line-numbers').all()).length;
  await expect(container.locator('.margin').locator('.line-numbers')).toHaveText(new Array(linesCount).fill(/^\d+$/));
  // Didn't manage to make a better regex, I have no idea why but nothing matches
  await expect(container.locator('.editor-scrollable')).toHaveText(new RegExp('^.*Parsec.*$'));
});

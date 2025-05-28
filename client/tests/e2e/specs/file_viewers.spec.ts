// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, expectMedia, Media, msTest, openFileType, sliderClick } from '@tests/e2e/helpers';

msTest('File viewer page default state', async ({ documents }) => {
  const entries = documents.locator('.folder-container').locator('.file-list-item');

  await entries.nth(2).dblclick();
  await expect(documents.locator('.ms-spinner-modal')).toBeVisible();
  await expect(documents.locator('.ms-spinner-modal').locator('.spinner-label__text')).toHaveText('Opening file...');
  await expect(documents.locator('.ms-spinner-modal')).toBeHidden();
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^[A-Za-z0-9_.-]+$/);
});

msTest('File viewer page details', async ({ documents }) => {
  const entries = documents.locator('.folder-container').locator('.file-list-item');

  await entries.nth(2).dblclick();
  await expect(documents.locator('.ms-spinner-modal')).toBeVisible();
  await expect(documents.locator('.ms-spinner-modal').locator('.spinner-label__text')).toHaveText('Opening file...');
  await expect(documents.locator('.ms-spinner-modal')).toBeHidden();
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^[A-Za-z0-9_.-]+$/);
  const buttons = documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-button');
  await expect(buttons).toHaveCount(4);
});

msTest('Quick access loads correct document', async ({ documents }) => {
  const entries = documents.locator('.folder-container').locator('.file-list-item');

  await entries.nth(2).dblclick();
  await expect(documents.locator('.ms-spinner-modal')).toBeVisible();
  await expect(documents.locator('.ms-spinner-modal').locator('.spinner-label__text')).toHaveText('Opening file...');
  await expect(documents.locator('.ms-spinner-modal')).toBeHidden();
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  const doc1Name = (await documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text').textContent()) ?? '';
  await documents.locator('.topbar-left').locator('.back-button').click();
  await entries.nth(3).dblclick();
  await expect(documents.locator('.ms-spinner-modal')).toBeVisible();
  await expect(documents.locator('.ms-spinner-modal').locator('.spinner-label__text')).toHaveText('Opening file...');
  await expect(documents.locator('.ms-spinner-modal')).toBeHidden();
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  const doc2Name = (await documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text').textContent()) ?? '';

  const sidebar = documents.locator('.sidebar');
  const recentDocs = sidebar.locator('.file-workspaces').locator('.list-sidebar-content').getByRole('listitem');
  await expect(recentDocs).toHaveCount(2);
  await expect(recentDocs.nth(0)).toHaveText(doc2Name);
  await expect(recentDocs.nth(1)).toHaveText(doc1Name);

  await recentDocs.nth(1).click();
  await expect(documents.locator('.ms-spinner-modal')).toBeVisible();
  await expect(documents.locator('.ms-spinner-modal').locator('.spinner-label__text')).toHaveText('Opening file...');
  await expect(documents.locator('.ms-spinner-modal')).toBeHidden();
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(doc1Name);

  await recentDocs.nth(1).click();
  await expect(documents.locator('.ms-spinner-modal')).toBeVisible();
  await expect(documents.locator('.ms-spinner-modal').locator('.spinner-label__text')).toHaveText('Opening file...');
  await expect(documents.locator('.ms-spinner-modal')).toBeHidden();
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(doc2Name);
});

msTest.skip('Audio viewer', async ({ documents }) => {
  msTest.setTimeout(60_000);

  await openFileType(documents, 'mp3');
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^[A-Za-z0-9_-]+\.mp3$/);

  const bottomBar = documents.locator('.file-viewer-bottombar');
  const volume = bottomBar.locator('.volume');
  const playPause = documents.locator('.file-controls-playback');
  const wrapper = documents.locator('.file-viewer-wrapper');
  const audio = wrapper.locator('audio');
  const fluxBar = bottomBar.locator('.slider').nth(0);
  const volumeSlider = bottomBar.locator('.slider').nth(1);

  // check if illustration is displayed
  const fileViewerBackground = wrapper.locator('.file-viewer-background');
  await expect(fileViewerBackground).toBeVisible();
  await expect(fileViewerBackground.locator('.file-viewer-background-icon')).toBeVisible();

  await expectMedia(audio).toHaveDuration(7.967347);
  await expectMedia(audio).toHaveCurrentTime(0.0);

  // Volume control
  const volumeButton = volume.locator('.file-controls-button').nth(0);
  await expectMedia(audio).toHaveVolume(1);
  await sliderClick(documents, volumeSlider, 70);
  await expectMedia(audio).toHaveVolume(0.7);
  await volumeButton.click();
  await expectMedia(audio).toHaveVolume(0);
  await volumeButton.click();
  await expectMedia(audio).toHaveVolume(0.7);

  // Stream control
  await playPause.click();
  await documents.waitForTimeout(500);
  await playPause.click();
  expect(await Media.getCurrentTime(audio)).toBeGreaterThan(0.0);

  await sliderClick(documents, fluxBar, 90);
  expect(await Media.getCurrentTime(audio)).toBeGreaterThan(5);
  expect(await Media.getCurrentTime(audio)).toBeLessThan(8);

  // Dropdown menu
  const dropdownButton = documents.locator('.file-controls-dropdown');
  await dropdownButton.click();
  const popover = documents.locator('.file-controls-dropdown-popover');
  await expect(popover).toBeVisible();
  const backdrop = popover.locator('..').locator('..').locator('ion-backdrop');
  const popoverButtons = popover.locator('.file-controls-dropdown-item');
  await expect(popoverButtons).toHaveCount(2);

  // Loop
  await expect(popoverButtons.nth(1)).not.toHaveClass('file-controls-dropdown-item file-controls-dropdown-item-active');
  await popoverButtons.nth(1).click();
  await expect(popoverButtons.nth(1)).toHaveClass('file-controls-dropdown-item file-controls-dropdown-item-active');
  await backdrop.click();

  await playPause.click();
  expect(await Media.getPausedState(audio)).toBe(false);
  await documents.waitForTimeout(2000);
  expect(await Media.getPausedState(audio)).toBe(false);

  await dropdownButton.click();
  await popoverButtons.nth(1).click();
  await backdrop.click();
  await sliderClick(documents, fluxBar, 90);
  await documents.waitForTimeout(1000);
  await expectMedia(audio).toHaveCurrentTime(7.967347);
  expect(await Media.getPausedState(audio)).toBe(true);

  // Playback speed
  await dropdownButton.click();
  await popoverButtons.nth(0).click();
  await expect(popoverButtons).toHaveCount(5);
  await expect(popoverButtons.nth(2)).toHaveClass('file-controls-dropdown-item file-controls-dropdown-item-active');
  // x0.25
  await popoverButtons.nth(0).click();
  await expect(popoverButtons.nth(2)).not.toHaveClass('file-controls-dropdown-item file-controls-dropdown-item-active');
  await expect(popoverButtons.nth(0)).toHaveClass('file-controls-dropdown-item file-controls-dropdown-item-active');
  await backdrop.click();
  await playPause.click();
  await documents.waitForTimeout(2000);
  await playPause.click();
  expect(await Media.getCurrentTime(audio)).toBeLessThan(6);
  await dropdownButton.click();
  await popoverButtons.nth(0).click();
  await popoverButtons.nth(4).click();
  await expect(popoverButtons.nth(0)).not.toHaveClass('file-controls-dropdown-item file-controls-dropdown-item-active');
  await expect(popoverButtons.nth(4)).toHaveClass('file-controls-dropdown-item file-controls-dropdown-item-active');
  await backdrop.click();
  await playPause.click();
  await documents.waitForTimeout(4000);
  await expectMedia(audio).toHaveCurrentTime(7.967347);
  expect(await Media.getPausedState(audio)).toBe(true);
});

msTest('Video viewer', async ({ documents }) => {
  msTest.setTimeout(60_000);

  await openFileType(documents, 'mp4');
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^[A-Za-z0-9_-]+\.mp4$/);

  const bottomBar = documents.locator('.file-viewer-bottombar');
  const volume = bottomBar.locator('.volume');
  const volumeButton = volume.locator('.file-controls-button').nth(0);
  const buttons = bottomBar.locator('.file-controls-group');
  const wrapper = documents.locator('.file-viewer-wrapper');
  const video = wrapper.locator('video');
  const fluxBar = bottomBar.locator('.slider').nth(0);
  const volumeSlider = bottomBar.locator('.slider').nth(1);

  await expect(buttons).toHaveCount(5);

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
  expect(await Media.getCurrentTime(video)).toBeGreaterThan(0.0);

  await buttons.nth(0).click();
  expect(await Media.getPausedState(video)).toBe(false);
  await documents.waitForTimeout(4000);
  expect(await Media.getCurrentTime(video)).toBe(3.562646);
  expect(await Media.getPausedState(video)).toBe(true);

  // Volume control
  await expectMedia(video).toHaveVolume(1);
  await sliderClick(documents, volumeSlider, 40);
  await expectMedia(video).toHaveVolume(0.4);
  await volumeButton.click();
  await expectMedia(video).toHaveVolume(0);
  await volumeButton.click();
  await expectMedia(video).toHaveVolume(0.4);

  // Stream control
  await sliderClick(documents, fluxBar, 90);
  expect(await Media.getCurrentTime(video)).toBeGreaterThan(2);
  expect(await Media.getCurrentTime(video)).toBeLessThan(3.5);

  // Dropdown menu
  const dropdownButton = buttons.nth(3);
  await dropdownButton.click();
  const popover = documents.locator('.file-controls-dropdown-popover');
  await expect(popover).toBeVisible();
  const backdrop = popover.locator('..').locator('..').locator('ion-backdrop');
  const popoverButtons = popover.locator('.file-controls-dropdown-item');
  await expect(popoverButtons).toHaveCount(3);

  // Loop
  await expect(popoverButtons.nth(2)).not.toHaveTheClass('file-controls-dropdown-item-active');
  await popoverButtons.nth(2).click();
  await expect(popoverButtons.nth(2)).toHaveTheClass('file-controls-dropdown-item-active');
  await backdrop.click();
  await buttons.nth(0).click();
  expect(await Media.getPausedState(video)).toBe(false);
  await documents.waitForTimeout(500);
  expect(await Media.getPausedState(video)).toBe(false);

  // Unloop
  await dropdownButton.click();
  await popoverButtons.nth(2).click();
  await backdrop.click();
  await sliderClick(documents, fluxBar, 90);
  await documents.waitForTimeout(3000);
  expect(await Media.getCurrentTime(video)).toBeGreaterThan(1);
  expect(await Media.getCurrentTime(video)).toBeLessThan(4);
  expect(await Media.getPausedState(video)).toBe(true);

  // Playback speed
  await dropdownButton.click();
  await popoverButtons.nth(0).click();
  await expect(popoverButtons).toHaveCount(5);
  await expect(popoverButtons.nth(2)).toHaveTheClass('file-controls-dropdown-item-active');
  // x0.25
  await popoverButtons.nth(0).click();
  await expect(popoverButtons.nth(2)).not.toHaveTheClass('file-controls-dropdown-item-active');
  await expect(popoverButtons.nth(0)).toHaveTheClass('file-controls-dropdown-item-active');
  await backdrop.click();
  await buttons.nth(0).click();
  await documents.waitForTimeout(2000);
  await buttons.nth(0).click();
  expect(await Media.getCurrentTime(video)).toBeLessThan(2);

  await dropdownButton.click();
  await popoverButtons.nth(0).click();
  await popoverButtons.nth(4).click();
  await expect(popoverButtons.nth(0)).not.toHaveTheClass('file-controls-dropdown-item-active');
  await expect(popoverButtons.nth(4)).toHaveTheClass('file-controls-dropdown-item-active');
  await backdrop.click();
  await buttons.nth(0).click();
  await documents.waitForTimeout(2000);
  expect(await Media.getCurrentTime(video)).toBe(3.562646);
  expect(await Media.getPausedState(video)).toBe(true);
});

msTest('Text viewer', async ({ documents }) => {
  await openFileType(documents, 'py');
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^[A-Za-z0-9_-]+\.py$/);

  const container = documents.locator('.file-viewer').locator('.text-container');
  const linesCount = (await container.locator('.margin').locator('.line-numbers').all()).length;
  await expect(container.locator('.margin').locator('.line-numbers')).toHaveText(new Array(linesCount).fill(/^\d+$/));
  // Didn't manage to make a better regex, I have no idea why but nothing matches
  await expect(container.locator('.editor-scrollable')).toHaveText(new RegExp('^.*Parsec.*$'));
});

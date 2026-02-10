// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import {
  DisplaySize,
  expect,
  expectMedia,
  getDownloadedFile,
  importDefaultFiles,
  ImportDocuments,
  Media,
  msTest,
  openFileType,
  sliderClick,
} from '@tests/e2e/helpers';

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });

  msTest('File viewer page default state', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await expect(entries).toHaveCount(1);

    await entries.nth(0).dblclick();
    await expect(documents).toBeViewerPage();
    await expect(documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text')).toHaveText('image.png');
    await expect(documents.locator('#connected-header .topbar')).toBeHidden();
    await expect(documents.locator('.sidebar')).toBeHidden();
  });

  msTest('Open viewer with header option', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);

    const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
    await expect(entry.locator('.label-name')).toHaveText('image.png');
    await entry.hover();
    await expect(entry).toBeVisible();
    await entry.locator('.ms-checkbox').click();
    await expect(entry).toContainClass('selected');
    const actionBar = documents.locator('#folders-ms-action-bar');
    await expect(actionBar.locator('ion-button').nth(0)).toHaveText('Preview');
    await actionBar.locator('ion-button').nth(0).click();
    await expect(documents).toBeViewerPage();
    await expect(documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text')).toHaveText('image.png');
    await expect(documents.locator('#connected-header .topbar')).toBeHidden();
    await expect(documents.locator('.sidebar')).toBeHidden();
  });

  msTest('Open viewer with contextual menu', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);

    const entries = documents.locator('.folder-container').locator('.file-list-item');

    await entries.nth(0).click({ button: 'right' });
    const menu = documents.locator('#file-context-menu');
    await expect(menu).toBeVisible();
    await expect(menu.getByRole('listitem').nth(1)).toHaveText('Preview');
    await menu.getByRole('listitem').nth(1).click();
    await expect(documents).toBeViewerPage();
    await expect(documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text')).toHaveText('image.png');
    await expect(documents.locator('#connected-header .topbar')).toBeHidden();
    await expect(documents.locator('.sidebar')).toBeHidden();
  });

  msTest('File viewer back to documents page', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
    const entries = documents.locator('.folder-container').locator('.file-list-item');

    await entries.nth(0).dblclick();
    await expect(documents).toBeViewerPage();
    await expect(documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text')).toHaveText('image.png');
    await expect(documents.locator('#connected-header .topbar')).toBeHidden();
    await expect(documents.locator('.sidebar')).toBeHidden();
    await expect(documents.locator('.file-handler-topbar')).toBeVisible();
    await documents.locator('.file-handler-topbar').locator('.back-button').click();
    await expect(documents.locator('#connected-header .topbar')).toBeVisible();
    await expect(documents).toBeDocumentPage();
  });

  msTest('File viewer page details', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
    const entries = documents.locator('.folder-container').locator('.file-list-item');

    await entries.nth(0).dblclick();
    await expect(documents).toBeViewerPage();
    await expect(documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text')).toHaveText('image.png');
    const buttons = documents.locator('.file-handler').locator('.file-handler-topbar-buttons').locator('ion-button:visible');
    await expect(buttons).toHaveCount(4);
    await expect(buttons).toHaveText(['Details', 'Copy link', 'Download', 'Show menu']);
    const modal = documents.locator('.file-details-modal');
    await expect(modal).toBeHidden();
    await buttons.nth(0).click();
    await expect(modal).toBeVisible();
  });

  msTest('File viewer header control functionality', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Pdf, false);
    const entries = documents.locator('.folder-container').locator('.file-list-item');

    await entries.nth(0).dblclick();
    await expect(documents).toBeViewerPage();
    await expect(documents.locator('#connected-header .topbar')).toBeHidden();

    const toggleButton = documents.locator('.file-handler-topbar-buttons__item.toggle-menu');
    const toggleSidebarButton = documents.locator('.file-handler-topbar').locator('#trigger-toggle-menu-button');
    await expect(toggleButton).toBeVisible();
    await expect(toggleButton).toContainText('Show menu');

    await toggleButton.click();
    await expect(toggleSidebarButton).toBeHidden();
    await expect(documents.locator('#connected-header .topbar')).toBeVisible();
    await expect(toggleButton).toContainText('Hide menu');

    await toggleButton.click();
    await expect(toggleSidebarButton).toBeVisible();
    await expect(documents.locator('#connected-header .topbar')).toBeHidden();
    await expect(toggleButton).toContainText('Show menu');

    // Test that header state persists when navigating between files in viewer
    await toggleButton.click();
    await expect(documents.locator('#connected-header .topbar')).toBeVisible();
    await documents.locator('.topbar-left').locator('.back-button').click();
    await entries.nth(1).dblclick();
    await expect(documents.locator('#connected-header .topbar')).toBeHidden();

    await toggleSidebarButton.click();
    await expect(documents.locator('.sidebar')).toBeVisible();
    await toggleSidebarButton.click();
    await expect(documents.locator('.sidebar')).toBeHidden();

    // Test leaving file viewer restores header
    const backButton = documents.locator('.file-handler-topbar').locator('.back-button');
    await expect(backButton).toBeVisible();
    await backButton.click();
    await expect(documents.locator('#connected-header .topbar')).toBeVisible();
    await expect(documents.locator('.sidebar')).toBeVisible();
  });

  for (const displaySize of ['small', 'large']) {
    msTest(`Quick access loads correct document ${displaySize} display`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Pdf, false);
      const entries = documents.locator('.folder-container').locator('.file-list-item');

      if (displaySize === DisplaySize.Small) {
        await documents.setDisplaySize(DisplaySize.Small);
      }

      await entries.nth(0).locator('.label-name').click();
      await expect(documents).toBeViewerPage();
      const doc1Name = (await documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text').textContent()) ?? '';

      // Ensure the main header is visible
      const isTopbarVisible = await documents.locator('#connected-header .topbar').isVisible();
      const fileViewerToggleMenuButton = documents.locator('.file-handler-topbar-buttons__item.toggle-menu');
      if (!isTopbarVisible && fileViewerToggleMenuButton && displaySize === DisplaySize.Large) {
        await fileViewerToggleMenuButton.click();
      }

      if (displaySize === DisplaySize.Small) {
        await expect(fileViewerToggleMenuButton).toBeHidden();
      }

      await documents.locator('.topbar-left-content').locator('.back-button').click();
      await entries.nth(1).locator('.label-name').click();
      await expect(documents).toBeViewerPage();
      const doc2Name = (await documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text').textContent()) ?? '';

      if (displaySize === 'large') {
        const sidebar = documents.locator('.sidebar');
        const recentDocs = sidebar.locator('#sidebar-files').locator('.list-sidebar-content').getByRole('listitem');
        await expect(recentDocs).toHaveCount(2);
        await expect(recentDocs.nth(0)).toHaveText(doc2Name);
        await expect(recentDocs.nth(1)).toHaveText(doc1Name);
        const toggleSidebarButton = documents.locator('.file-handler-topbar').locator('#trigger-toggle-menu-button');
        await toggleSidebarButton.click();
        await recentDocs.nth(1).click();
        await expect(documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text')).toHaveText(doc1Name);
        await recentDocs.nth(1).click();
        await expect(documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text')).toHaveText(doc2Name);
      }
    });

    msTest(`File viewer download ${displaySize} display`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Pdf, false);
      if (displaySize === 'small') {
        await documents.setDisplaySize(DisplaySize.Small);
      }
      await openFileType(documents, 'pdf');
      await expect(documents).toBeViewerPage();
      await expect(documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text')).toHaveText(
        /^[A-Za-z0-9_-]+\.pdf$/,
      );

      const modal = documents.locator('.download-warning-modal');

      // showSaveFilePicker is not yet supported by Playwright: https://github.com/microsoft/playwright/issues/31162
      if (displaySize === 'small') {
        const actionMenuButton = documents.locator('.file-handler-topbar-buttons__item.action-menu');
        const actionMenuModal = documents.locator('.viewer-action-menu-modal');
        await expect(actionMenuModal).toBeHidden();
        await expect(actionMenuButton).toBeVisible();
        await expect(documents.locator('.file-handler-topbar-buttons')).toBeHidden();
        await actionMenuButton.click();

        await expect(documents.locator('.list-group-item__label-small').nth(2)).toBeVisible();
        await documents.locator('.list-group-item__label-small').nth(2).click();

        await expect(actionMenuModal).toBeVisible();
        await modal.locator('#next-button').click();
      } else {
        await documents.locator('.file-handler-topbar-buttons__item').nth(2).click();
        await modal.locator('#next-button').click();
      }

      await expect(modal).toBeHidden();
      await documents.waitForTimeout(1000);

      const uploadMenu = documents.locator('.upload-menu');
      await expect(uploadMenu).toBeVisible();
      const opItems = uploadMenu.locator('.upload-menu-list').locator('.file-operation-item');
      await expect(opItems).toHaveCount(2);
      await expect(opItems.nth(0).locator('.element-details-title__name')).toHaveText('Downloading pdfDocument.pdf');
      await expect(opItems.nth(0).locator('.element-details-info').locator('ion-text').nth(0)).toHaveText(' wksp1');

      const content = await getDownloadedFile(documents);
      expect(content).toBeTruthy();
      if (content) {
        expect(content.length).toEqual(78731);
      }
    });
  }

  msTest('Audio viewer', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(60_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3, false);

    await openFileType(documents, 'mp3');
    await expect(documents).toBeViewerPage();
    await expect(documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text')).toHaveText('audio.mp3');

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

    expect((await Media.getDuration(audio)).toString()).toMatch(/^7.9\d+$/);
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
    expect((await Media.getDuration(audio)).toString()).toMatch(/^7.9\d+$/);
    await expectMedia(audio).toBePaused();

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
    expect((await Media.getDuration(audio)).toString()).toMatch(/^7.9\d+$/);
    await expectMedia(audio).toBePaused();
  });

  msTest('Video viewer', async ({ documents }, testInfo: TestInfo) => {
    console.log(testInfo.project.name);
    // Codecs are not included in chromium
    if (testInfo.project.name.toLocaleLowerCase() === 'chromium') {
      return;
    }

    msTest.setTimeout(60_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp4, false);

    await openFileType(documents, 'mp4');
    await expect(documents).toBeViewerPage();
    await expect(documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text')).toHaveText('video.mp4');

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

  msTest('File viewer auto-hides sidebar on entry', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Pdf, false);
    // Start with visible sidebar
    await expect(documents.locator('.sidebar')).toBeVisible();

    // Open file viewer
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await entries.nth(0).dblclick();
    await expect(documents).toBeViewerPage();

    // Sidebar should be auto-hidden in file viewer
    await expect(documents.locator('.sidebar')).toBeHidden();
    await expect(documents.locator('#connected-header .topbar')).toBeHidden();
  });

  msTest('File viewer sidebar can be toggled in large display', async ({ documents }, testInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Pdf, false);
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await entries.nth(0).dblclick();
    await expect(documents).toBeViewerPage();

    // Sidebar initially hidden
    await expect(documents.locator('.sidebar')).toBeHidden();

    const toggleButton = documents.locator('.file-handler-topbar-buttons__item.toggle-menu');
    const toggleSidebarButton = documents.locator('#trigger-toggle-menu-button');

    await expect(toggleButton).toBeVisible();
    await expect(toggleButton).toContainText('Show menu');

    // Show menu and sidebar
    await toggleButton.click();
    await expect(documents.locator('#connected-header .topbar')).toBeVisible();
    await expect(toggleSidebarButton).toBeVisible();
    await expect(toggleButton).toContainText('Hide menu');

    // Toggle sidebar specifically
    await toggleSidebarButton.click();
    await expect(documents.locator('.sidebar')).toBeVisible();

    await toggleSidebarButton.click();
    await expect(documents.locator('.sidebar')).toBeHidden();
  });

  msTest('File viewer sidebar state persists between files', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3 | ImportDocuments.Png, false);
    const entries = documents.locator('.folder-container').locator('.file-list-item');

    const sidebar = documents.locator('.sidebar');
    const header = documents.locator('#connected-header');
    await expect(sidebar).toBeVisible();
    await expect(header).toBeVisible();

    // Open first file and ensure it's loaded
    await entries.nth(0).locator('.file-name .file-mobile-text .label-name').click();
    await expect(documents).toBeViewerPage();
    await expect(documents.locator('.file-handler-topbar .file-handler-topbar__title')).toHaveText('audio.mp3');

    await expect(sidebar).toBeHidden();
    await expect(header).toBeHidden();

    const toggleButton = documents.locator('.file-handler-topbar-buttons__item.toggle-menu');
    const toggleSidebarButton = documents.locator('#trigger-toggle-menu-button');

    // Show header and sidebar
    await toggleButton.click();
    await expect(header).toBeVisible();

    await toggleSidebarButton.click();
    await expect(sidebar).toBeVisible();

    // Go back to documents page and open a second file to ensure we have 2 recent files
    const backButton = documents.locator('.topbar-left-content').locator('.back-button');

    await backButton.click();
    await expect(documents).toBeDocumentPage();
    await expect(documents.locator('.folder-container')).toBeVisible();

    await expect(sidebar).toBeVisible();

    const recentDocs = sidebar.locator('#sidebar-files').locator('.list-sidebar-content').getByRole('listitem');
    await expect(recentDocs).toHaveCount(1);
    await expect(recentDocs).toHaveText(['audio.mp3']);

    // Hide sidebar again before opening second file
    await toggleSidebarButton.click();
    await expect(sidebar).toBeHidden();

    // Open second file
    await entries.nth(1).locator('.file-name .file-mobile-text .label-name').click();
    await expect(documents.locator('.file-handler-topbar .file-handler-topbar__title')).toHaveText('image.png');
    await expect(documents).toBeViewerPage();

    await expect(header).toBeHidden();
    await expect(sidebar).toBeHidden();

    // Show header and sidebar again
    await toggleButton.click();
    await expect(header).toBeVisible();

    await toggleSidebarButton.click();
    await expect(sidebar).toBeVisible();

    // Now we should have 2 recent files
    await expect(recentDocs).toHaveCount(2);
    await expect(recentDocs).toHaveText(['image.png', 'audio.mp3']);

    // Navigate via sidebar to the previous file
    await recentDocs.nth(1).click();
    await expect(documents).toBeViewerPage();
    await expect(documents.locator('.file-handler-topbar .file-handler-topbar__title')).toHaveText('audio.mp3');

    // Sidebar should remain visible
    await expect(sidebar).toBeVisible();
    await expect(header).toBeVisible();
  });

  msTest('File viewer restores sidebar state on exit', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3 | ImportDocuments.Png, false);
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    const toggleButton = documents.locator('#trigger-toggle-menu-button');

    // Sidebar starts visible (default state)
    await expect(documents.locator('.sidebar')).toBeVisible();
    await expect(documents.locator('#connected-header .topbar')).toBeVisible();

    // Open file viewer
    await entries.nth(0).locator('.file-name .file-mobile-text .label-name').click();
    await expect(documents).toBeViewerPage();
    await expect(documents.locator('.file-handler-topbar .file-handler-topbar__title')).toHaveText('audio.mp3');

    // Sidebar should be hidden in file viewer
    await expect(documents.locator('.sidebar')).toBeHidden();
    await expect(documents.locator('#connected-header .topbar')).toBeHidden();

    // Exit file viewer
    const backButton = documents.locator('.file-handler-topbar').locator('.back-button');
    await expect(backButton).toBeVisible();
    await backButton.click();
    await expect(documents).toBeDocumentPage();
    await expect(entries).toHaveCount(2);
    await expect(documents.locator('.folder-container')).toBeVisible();

    // Sidebar should be restored to visible as it was before
    await expect(documents.locator('.sidebar')).toBeVisible();
    await expect(documents.locator('#connected-header .topbar')).toBeVisible();

    // Hide sidebar and test restoration of hidden state
    await toggleButton.click();
    await expect(documents.locator('.sidebar')).toBeHidden();

    // Open file viewer again
    await entries.nth(1).locator('.file-name .file-mobile-text .label-name').click();
    await expect(documents).toBeViewerPage();
    await expect(documents.locator('.file-handler-topbar .file-handler-topbar__title')).toHaveText('image.png');

    // Exit file viewer
    const backButton2 = documents.locator('.file-handler-topbar').locator('.back-button');
    await expect(backButton2).toBeVisible();
    await backButton2.click();
    await expect(documents).toBeDocumentPage();
    await expect(entries).toHaveCount(2);
    await expect(documents.locator('.folder-container')).toBeVisible();

    // Sidebar should remain hidden as it was before
    await expect(documents.locator('.sidebar')).toBeHidden();
    await expect(documents.locator('#connected-header .topbar')).toBeVisible();
  });

  msTest('File viewer sidebar in small display', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
    await documents.setDisplaySize(DisplaySize.Small);

    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await entries.nth(0).locator('.label-name').click();
    await expect(documents).toBeViewerPage();

    await expect(documents.locator('.sidebar')).toBeHidden();
    await expect(documents.locator('#connected-header .topbar')).toBeHidden();
    const toggleButton = documents.locator('.file-handler-topbar-buttons__item.toggle-menu');
    await expect(toggleButton).toBeHidden();

    // File viewer functionality should still work
    await expect(documents.locator('.file-handler')).toBeVisible();
    await expect(documents.locator('.file-handler-topbar')).toBeVisible();

    // Back button should work in small display
    const backButton = documents.locator('.file-handler-topbar').locator('.back-button');
    await expect(backButton).toBeVisible();
    await backButton.click();

    // After exiting file viewer, should return to documents page
    await expect(documents.locator('.folder-container')).toBeVisible();
    await expect(documents.locator('#connected-header .topbar')).toBeVisible();
    await expect(documents.locator('.sidebar')).toBeHidden();
  });
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest, openFileType, testFileViewerZoomLevel } from '@tests/e2e/helpers';

msTest.skip('Spreadsheet viewer', async ({ documents }) => {
  await openFileType(documents, 'xlsx');
  await expect(documents).toBeViewerPage();
  await expect(documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text')).toHaveText(/^[A-Za-z0-9_-]+\.xlsx$/);
  const spinner = documents.locator('#spreadsheet-spinner');
  await expect(spinner).toBeVisible();
  const bottomBar = documents.locator('.file-viewer-bottombar');
  const dropdownButton = bottomBar.locator('.file-controls-dropdown');
  await expect(dropdownButton).toHaveText('Sheet1');
  await expect(spinner).toBeHidden();
  const wrapper = documents.locator('.file-viewer-wrapper');
  const spreadsheet = wrapper.locator('.inner-content-table').nth(1).locator('.content-wrapper');
  await expect(spreadsheet.locator('.rgCell')).toHaveText(['A', '1', 'B', '2', 'C', '3', 'D', '4']);

  // Open sheet popover
  const dropdownPopover = documents.locator('.dropdown-popover');
  await expect(dropdownPopover).toBeHidden();
  await dropdownButton.click();
  await expect(dropdownPopover).toBeVisible();
  const dropdownItems = dropdownPopover.locator('.file-controls-dropdown-item');
  await expect(dropdownItems).toHaveText(['Sheet1', 'Sheet2']);
  await expect(dropdownItems.nth(0)).toHaveTheClass('file-controls-dropdown-item-active');
  await expect(dropdownItems.nth(1)).not.toHaveTheClass('file-controls-dropdown-item-active');

  // Trying to switch to Sheet1, nothing should happen
  await dropdownItems.nth(0).click();
  await expect(dropdownPopover).toBeHidden();
  await expect(dropdownButton).toHaveText('Sheet1');
  await dropdownButton.click();
  await expect(dropdownPopover).toBeVisible();

  // Switching to Sheet2
  await dropdownItems.nth(1).click();
  await expect(spinner).toBeVisible();
  await expect(dropdownPopover).toBeHidden();
  await expect(dropdownButton).toHaveText('Sheet2');
  await expect(spreadsheet.locator('.rgCell')).toHaveText(['E', '5', 'F', '6', 'G', '7', 'H', '8']);
  await expect(spinner).toBeHidden();
});

msTest.skip('Spreadsheet viewer zoom', async ({ documents }) => {
  await openFileType(documents, 'xlsx');
  await expect(documents).toBeViewerPage();
  await expect(documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text')).toHaveText(/^[A-Za-z0-9_-]+\.xlsx$/);
  const bottomBar = documents.locator('.file-viewer-bottombar');
  const wrapper = documents.locator('.file-viewer-wrapper');
  const spreadsheet = wrapper.locator('.inner-content-table').nth(1).locator('.content-wrapper');
  await expect(spreadsheet.locator('.rgCell')).toHaveText(['A', '1', 'B', '2', 'C', '3', 'D', '4']);
  const zoom = bottomBar.locator('.zoom-controls');
  const zoomReset = bottomBar.locator('#reset-zoom');
  const zoomOut = zoom.locator('.file-controls-button-container').nth(0);
  const zoomIn = zoom.locator('.file-controls-button-container').nth(1);
  const zoomLevel = zoom.locator('ion-text.file-controls-input');
  const zoomLevelInput = zoom.locator('ion-input.file-controls-input');

  await expect(zoomLevelInput).toBeHidden();

  await expect(zoomLevel).toHaveText('100 %');
  await testFileViewerZoomLevel(wrapper, '1');

  await zoomOut.click();
  await zoomOut.click();
  await expect(zoomLevel).toHaveText('80 %');
  await testFileViewerZoomLevel(wrapper, '0.8');
  for (let i = 0; i < 8; i++) {
    await zoomOut.click();
  }
  await expect(zoomOut).toBeTrulyDisabled();
  await expect(zoomLevel).toHaveText('5 %');
  await testFileViewerZoomLevel(wrapper, '0.05');
  await zoomReset.click();
  await expect(zoomLevel).toHaveText('100 %');
  await testFileViewerZoomLevel(wrapper, '1');
  await expect(zoomOut).toBeTrulyEnabled();

  await zoomIn.click();
  await zoomIn.click();
  await expect(zoomLevel).toHaveText('150 %');
  await testFileViewerZoomLevel(wrapper, '1.5');
  for (let i = 0; i < 6; i++) {
    await zoomIn.click();
  }
  await expect(zoomIn).toBeTrulyDisabled();
  await expect(zoomLevel).toHaveText('500 %');
  await testFileViewerZoomLevel(wrapper, '5');
  await zoomReset.click();

  await expect(zoomLevel).toHaveText('100 %');
  await testFileViewerZoomLevel(wrapper, '1');
  await expect(zoomIn).toBeTrulyEnabled();

  await zoomLevel.click();
  await expect(zoomLevel).toBeHidden();
  await expect(zoomLevelInput).toBeVisible();

  // Can't use fillIonInput because the input disappears once filled
  const input = zoomLevelInput.locator('input');
  await input.fill('42');
  await input.blur();
  await expect(zoomLevelInput).toBeHidden();
  await expect(zoomLevel).toBeVisible();
  await expect(zoomLevel).toHaveText('40 %');
  await testFileViewerZoomLevel(wrapper, '0.4');
});

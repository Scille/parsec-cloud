// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, fillIonInput, msTest, openFileType, testFileViewerZoomLevel } from '@tests/e2e/helpers';

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

msTest('Image viewer zoom', async ({ documents }) => {
  await openFileType(documents, 'png');
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.png$/);
  const bottomBar = documents.locator('.file-viewer-bottombar');
  const wrapper = documents.locator('.file-viewer-wrapper');
  const zoomControls = bottomBar.locator('.file-controls-zoom').locator('ion-button');
  const zoomReset = zoomControls.nth(0);
  const zoomMinus = zoomControls.nth(1);
  const zoomPlus = zoomControls.nth(2);
  const zoomLevel = bottomBar.locator('.file-controls-zoom').locator('span.zoom-level-input');
  const zoomLevelInput = bottomBar.locator('.file-controls-zoom').locator('ion-input.zoom-level-input');

  await expect(zoomLevelInput).toBeHidden();

  await expect(zoomLevel).toHaveText('100 %');
  await testFileViewerZoomLevel(wrapper, '1');

  await zoomMinus.click();
  await zoomMinus.click();
  await expect(zoomLevel).toHaveText('80 %');
  await testFileViewerZoomLevel(wrapper, '0.8');
  for (let i = 0; i < 8; i++) {
    await zoomMinus.click();
  }
  await expect(zoomMinus).toBeTrulyDisabled();
  await expect(zoomLevel).toHaveText('5 %');
  await testFileViewerZoomLevel(wrapper, '0.05');
  await zoomReset.click();
  await expect(zoomLevel).toHaveText('100 %');
  await testFileViewerZoomLevel(wrapper, '1');
  await expect(zoomMinus).toBeTrulyEnabled();

  await zoomPlus.click();
  await zoomPlus.click();
  await expect(zoomLevel).toHaveText('150 %');
  await testFileViewerZoomLevel(wrapper, '1.5');
  for (let i = 0; i < 6; i++) {
    await zoomPlus.click();
  }
  await expect(zoomPlus).toBeTrulyDisabled();
  await expect(zoomLevel).toHaveText('500 %');
  await testFileViewerZoomLevel(wrapper, '5');
  await zoomReset.click();

  await expect(zoomLevel).toHaveText('100 %');
  await testFileViewerZoomLevel(wrapper, '1');
  await expect(zoomPlus).toBeTrulyEnabled();

  await zoomLevel.click();
  await expect(zoomLevel).toBeHidden();
  await expect(zoomLevelInput).toBeVisible();
  await fillIonInput(zoomLevelInput, '42');
  await expect(zoomLevel).toHaveText('40 %');
  await testFileViewerZoomLevel(wrapper, '0.4');
});

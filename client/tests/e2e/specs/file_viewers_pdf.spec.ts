// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator } from '@playwright/test';
import { expect, msTest, openFileType } from '@tests/e2e/helpers';

msTest('PDF viewer: content', async ({ documents }) => {
  await openFileType(documents, 'pdf');
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text')).toHaveText(/^File_[a-z0-9_]+\.pdf$/);

  const wrapper = documents.locator('.file-viewer-wrapper');
  const pages = wrapper.locator('.pdf-container').locator('canvas');
  await expect(pages).toHaveCount(3);
});

msTest('PDF viewer: pagination', async ({ documents }) => {
  await openFileType(documents, 'pdf');
  const bottomBar = documents.locator('.file-viewer-bottombar');
  const wrapper = documents.locator('.file-viewer-wrapper');
  const pages = wrapper.locator('.pdf-container').locator('canvas');

  const pagination = bottomBar.locator('.file-controls-pagination');
  await expect(pagination).toHaveCount(1);
  const paginationElement = pagination.locator('.file-controls-input');

  async function expectPage(pageNumber: number): Promise<void> {
    await expect(paginationElement).toHaveText(`Page ${pageNumber} / 3`);
    for (const [index, page] of (await pages.all()).entries()) {
      index === pageNumber - 1 ? await expect(page).toBeInViewport() : await expect(page).not.toBeInViewport();
    }
  }

  await expect(paginationElement).toHaveTheClass('text-only');
  await expectPage(1);

  await pagination.click();
  await expect(paginationElement).toHaveTheClass('editing-input');
  await paginationElement.locator('input').fill('2');
  await paginationElement.locator('input').blur();
  await expect(paginationElement).toHaveTheClass('text-only');
  await expectPage(2);

  await pagination.click();
  await expect(paginationElement).toHaveTheClass('editing-input');
  await paginationElement.locator('input').fill('3');
  await paginationElement.locator('input').press('Enter');
  await expect(paginationElement).toHaveTheClass('text-only');
  await expectPage(3);

  await pagination.click();
  await expect(paginationElement).toHaveTheClass('editing-input');
  await paginationElement.locator('input').fill('1');
  await paginationElement.locator('input').press('Escape');
  await expect(paginationElement).toHaveTheClass('text-only');
  await expectPage(1);

  await pagination.click();
  await expect(paginationElement).toHaveTheClass('editing-input');
  await paginationElement.locator('input').press('ArrowUp');
  await paginationElement.locator('input').press('Enter');
  await expect(paginationElement).toHaveTheClass('text-only');
  await expectPage(2);

  await pagination.click();
  await expect(paginationElement).toHaveTheClass('editing-input');
  await paginationElement.locator('input').press('ArrowRight');
  await paginationElement.locator('input').press('Enter');
  await expect(paginationElement).toHaveTheClass('text-only');
  await expectPage(3);

  await pagination.click();
  await expect(paginationElement).toHaveTheClass('editing-input');
  await paginationElement.locator('input').press('ArrowDown');
  await paginationElement.locator('input').press('Enter');
  await expect(paginationElement).toHaveTheClass('text-only');
  await expectPage(2);

  await pagination.click();
  await expect(paginationElement).toHaveTheClass('editing-input');
  await paginationElement.locator('input').press('ArrowLeft');
  await paginationElement.locator('input').press('Enter');
  await expect(paginationElement).toHaveTheClass('text-only');
  await expectPage(1);
});

msTest('PDF viewer: scroll', async ({ documents }) => {
  await openFileType(documents, 'pdf');
  const wrapper = documents.locator('.file-viewer-wrapper');
  const bottomBar = documents.locator('.file-viewer-bottombar');
  const pages = wrapper.locator('.pdf-container').locator('canvas');
  const firstPage = pages.nth(0);
  const secondPage = pages.nth(1);
  const thirdPage = pages.nth(2);

  const pagination = bottomBar.locator('.file-controls-pagination');
  const paginationElement = pagination.locator('.file-controls-input');

  await thirdPage.scrollIntoViewIfNeeded();
  await expect(firstPage).not.toBeInViewport();
  await expect(secondPage).not.toBeInViewport();
  await expect(thirdPage).toBeInViewport();
  await expect(paginationElement).toHaveText('Page 3 / 3');

  await secondPage.scrollIntoViewIfNeeded();
  await expect(firstPage).not.toBeInViewport();
  await expect(secondPage).toBeInViewport();
  await expect(thirdPage).not.toBeInViewport();
  await expect(paginationElement).toHaveText('Page 2 / 3');

  await firstPage.scrollIntoViewIfNeeded();
  await expect(firstPage).toBeInViewport();
  await expect(secondPage).not.toBeInViewport();
  await expect(thirdPage).not.toBeInViewport();
  await expect(paginationElement).toHaveText('Page 1 / 3');
});

msTest('PDF viewer: zoom', async ({ documents }) => {
  await openFileType(documents, 'pdf');
  const wrapper = documents.locator('.file-viewer-wrapper');
  const canvas = wrapper.locator('.pdf-container').locator('canvas');
  const bottomBar = documents.locator('.file-viewer-bottombar');

  // Zoom
  const zoom = bottomBar.locator('.file-controls-zoom');
  await expect(zoom).toHaveCount(1);
  const zoomReset = zoom.locator('.file-controls-button').nth(0);
  const zoomOut = zoom.locator('.file-controls-button').nth(1);
  const zoomIn = zoom.locator('.file-controls-button').nth(2);
  const zoomInput = zoom.locator('.file-controls-input');
  const initialHeight = Number(await canvas.nth(0).getAttribute('height'));
  const initialWidth = Number(await canvas.nth(0).getAttribute('width'));

  async function expectZoomLevel(expectedZoomLevel: number): Promise<void> {
    for (const page of await canvas.all()) {
      await expect(page).toHaveCSS('width', `${Math.floor((initialWidth * expectedZoomLevel) / 100)}px`);
      await expect(page).toHaveCSS('height', `${Math.floor((initialHeight * expectedZoomLevel) / 100)}px`);
    }
  }

  await expectZoomLevel(100);

  await zoomOut.click();
  await expectZoomLevel(90);

  await zoomReset.click();
  await expectZoomLevel(100);

  await zoomIn.click();
  await expectZoomLevel(125);

  await zoomInput.click();
  await expect(zoomInput).toHaveTheClass('editing-input');
  await zoomInput.locator('input').fill('150');
  await zoomInput.locator('input').press('Enter');
  await expect(zoomInput).toHaveTheClass('text-only');
  await expectZoomLevel(150);

  await zoomInput.click();
  await expect(zoomInput).toHaveTheClass('editing-input');
  await zoomInput.locator('input').fill('600'); // forbidden value should be ignored
  await zoomInput.locator('input').press('Escape');
  await expect(zoomInput).toHaveTheClass('text-only');
  await expectZoomLevel(150);

  await zoomInput.click();
  await expect(zoomInput).toHaveTheClass('editing-input');
  await zoomInput.locator('input').fill('100');
  await zoomInput.locator('input').blur();
  await expect(zoomInput).toHaveTheClass('text-only');
  await expectZoomLevel(100);

  await zoomInput.click();
  await expect(zoomInput).toHaveTheClass('editing-input');
  await zoomInput.locator('input').press('ArrowUp');
  await zoomInput.locator('input').press('Enter');
  await expect(zoomInput).toHaveTheClass('text-only');
  await expectZoomLevel(125);

  await zoomInput.click();
  await expect(zoomInput).toHaveTheClass('editing-input');
  await zoomInput.locator('input').press('ArrowDown');
  await zoomInput.locator('input').press('Enter');
  await expect(zoomInput).toHaveTheClass('text-only');
  await expectZoomLevel(100);
});

msTest('PDF viewer: progressive loading', async ({ documents }) => {
  await openFileType(documents, 'pdf');
  const wrapper = documents.locator('.file-viewer-wrapper');
  const bottomBar = documents.locator('.file-viewer-bottombar');
  const zoom = bottomBar.locator('.file-controls-zoom');
  const zoomReset = zoom.locator('.file-controls-button').nth(0);
  const zoomIn = zoom.locator('.file-controls-button').nth(2);
  const pagination = bottomBar.locator('.file-controls-pagination');
  const paginationElement = pagination.locator('.file-controls-input');
  const pages = wrapper.locator('.pdf-container').locator('canvas');
  const firstPage = pages.nth(0);
  const secondPage = pages.nth(1);
  const thirdPage = pages.nth(2);
  await expect(pages).toHaveCount(3);

  async function expectPageToBeRendered(page: Locator, rendered: boolean): Promise<void> {
    rendered ? await expect(page).toHaveAttribute('data-rendered') : await expect(page).not.toHaveAttribute('data-rendered');
  }
  await expectPageToBeRendered(firstPage, true);
  await expectPageToBeRendered(secondPage, false);
  await expectPageToBeRendered(thirdPage, false);

  await secondPage.scrollIntoViewIfNeeded();
  await expectPageToBeRendered(firstPage, true);
  await expectPageToBeRendered(secondPage, true);
  await expectPageToBeRendered(thirdPage, false);

  await thirdPage.scrollIntoViewIfNeeded();
  await expectPageToBeRendered(firstPage, true);
  await expectPageToBeRendered(secondPage, true);
  await expectPageToBeRendered(thirdPage, true);

  await zoomIn.click(); // zoom in to force re-rendering
  await expectPageToBeRendered(firstPage, false);
  await expectPageToBeRendered(secondPage, false);
  await expectPageToBeRendered(thirdPage, true);

  await pagination.click();
  await paginationElement.locator('input').fill('1');
  await paginationElement.locator('input').press('Enter');
  await expectPageToBeRendered(firstPage, true);
  await expectPageToBeRendered(secondPage, true);
  await expectPageToBeRendered(thirdPage, true);

  await documents.waitForTimeout(500); // wait for the scroll animation to be done
  await zoomReset.click(); // reset zoom to force re-rendering
  await expectPageToBeRendered(firstPage, true);
  await expectPageToBeRendered(secondPage, false);
  await expectPageToBeRendered(thirdPage, false);
});

msTest('PDF viewer: fullscreen', async ({ documents }) => {
  await openFileType(documents, 'pdf');
  const bottomBar = documents.locator('.file-viewer-bottombar');
  const fullscreenButton = bottomBar.locator('.file-controls-fullscreen');

  // Fullscreen
  const isNotFullScreen = await documents.evaluate(() => {
    return document.fullscreenElement === null;
  });
  await expect(isNotFullScreen).toBe(true);
  await fullscreenButton.click();
  const isFullScreen = await documents.evaluate(() => {
    return document.fullscreenElement !== null;
  });
  await expect(isFullScreen).toBe(true);
});

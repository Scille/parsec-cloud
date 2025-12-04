// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, TestInfo } from '@playwright/test';
import { expect, importDefaultFiles, ImportDocuments, msTest, openFileType } from '@tests/e2e/helpers';

const enum CanvasStates {
  Rendered = 'rendered',
  Failed = 'failed',
  Blank = 'blank',
}

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });
  msTest('PDF viewer: content', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Pdf, false);
    await openFileType(documents, 'pdf');
    await expect(documents).toBeViewerPage();
    await expect(documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text')).toHaveText('pdfDocument.pdf');

    const wrapper = documents.locator('.file-viewer-wrapper');
    const pages = wrapper.locator('.pdf-container').locator('canvas');
    await expect(pages).toHaveCount(4);
  });

  msTest('PDF viewer: pagination', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Pdf, false);
    await openFileType(documents, 'pdf');
    const bottomBar = documents.locator('.file-viewer-bottombar');
    const wrapper = documents.locator('.file-viewer-wrapper');
    const pages = wrapper.locator('.pdf-container').locator('canvas');

    const pagination = bottomBar.locator('.file-controls-pagination');
    await expect(pagination).toHaveCount(1);
    const paginationElement = pagination.locator('.file-controls-input');
    await expect(pagination.locator('.file-controls-input-prefix')).toHaveText('Page');

    async function expectPage(pageNumber: number): Promise<void> {
      await expect(paginationElement).toHaveText(`${pageNumber} / 4`);
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

  msTest('PDF viewer: scroll', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Pdf, false);

    await openFileType(documents, 'pdf');
    const wrapper = documents.locator('.file-viewer-wrapper');
    const bottomBar = documents.locator('.file-viewer-bottombar');
    const pages = wrapper.locator('.pdf-container').locator('canvas');
    const firstPage = pages.nth(0);
    const secondPage = pages.nth(1);
    const thirdPage = pages.nth(2);
    const fourthPage = pages.nth(3);

    const pagination = bottomBar.locator('.file-controls-pagination');
    const paginationElement = pagination.locator('.file-controls-input');

    await fourthPage.scrollIntoViewIfNeeded();
    await expect(firstPage).not.toBeInViewport();
    await expect(secondPage).not.toBeInViewport();
    await expect(thirdPage).toBeInViewport(); // the last page is an error canvas, leaving space for the previous one
    await expect(fourthPage).toBeInViewport();
    // Depending on the viewport size, the last canvas may not take enough of the page
    // to arrive at 4/4.
    await expect(paginationElement).toHaveText(/^(3|4) \/ 4$/);

    await thirdPage.scrollIntoViewIfNeeded();
    await expect(firstPage).not.toBeInViewport();
    await expect(secondPage).not.toBeInViewport();
    await expect(thirdPage).toBeInViewport();
    await expect(fourthPage).not.toBeInViewport();
    await expect(paginationElement).toHaveText('3 / 4');

    await secondPage.scrollIntoViewIfNeeded();
    await expect(firstPage).not.toBeInViewport();
    await expect(secondPage).toBeInViewport();
    await expect(thirdPage).not.toBeInViewport();
    await expect(fourthPage).not.toBeInViewport();
    await expect(paginationElement).toHaveText('2 / 4');

    await firstPage.scrollIntoViewIfNeeded();
    await expect(firstPage).toBeInViewport();
    await expect(secondPage).not.toBeInViewport();
    await expect(thirdPage).not.toBeInViewport();
    await expect(fourthPage).not.toBeInViewport();
    await expect(paginationElement).toHaveText('1 / 4');
  });

  msTest('PDF viewer: zoom', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Pdf, false);

    await openFileType(documents, 'pdf');
    const wrapper = documents.locator('.file-viewer-wrapper');
    const canvas = wrapper.locator('.pdf-container').locator('canvas');
    const bottomBar = documents.locator('.file-viewer-bottombar');

    // Zoom
    const zoom = bottomBar.locator('.zoom-controls');
    await expect(zoom).toHaveCount(1);
    const zoomReset = bottomBar.locator('#reset-zoom');
    const zoomOut = zoom.locator('.file-controls-button').nth(0);
    const zoomIn = zoom.locator('.file-controls-button').nth(1);
    const zoomInput = zoom.locator('.file-controls-input');

    let initialHeight = 0;
    let initialWidth = 0;
    let i = 0;

    while ((initialHeight === 0 || initialWidth === 0) && i < 10) {
      initialHeight = Number(await canvas.nth(0).getAttribute('height'));
      initialWidth = Number(await canvas.nth(0).getAttribute('width'));
      i++;
      await documents.waitForTimeout(50);
    }

    expect(initialHeight).toBeGreaterThan(0);
    expect(initialWidth).toBeGreaterThan(0);

    async function expectZoomLevel(expectedZoomLevel: number): Promise<void> {
      for (const page of await canvas.all()) {
        if ((await page.getAttribute('data-canvas-state')) !== CanvasStates.Failed) {
          await expect(page).toHaveCSS('width', `${Math.floor((initialWidth * expectedZoomLevel) / 100)}px`);
          await expect(page).toHaveCSS('height', `${Math.floor((initialHeight * expectedZoomLevel) / 100)}px`);
        }
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

  msTest('PDF viewer: progressive loading', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Pdf, false);

    await openFileType(documents, 'pdf');
    const wrapper = documents.locator('.file-viewer-wrapper');
    const bottomBar = documents.locator('.file-viewer-bottombar');
    const zoom = bottomBar.locator('.zoom-controls');
    await expect(zoom).toHaveCount(1);
    const zoomReset = bottomBar.locator('#reset-zoom');
    const zoomIn = zoom.locator('.file-controls-button').nth(1);
    const pagination = bottomBar.locator('.file-controls-pagination');
    const paginationElement = pagination.locator('.file-controls-input');
    const pages = wrapper.locator('.pdf-container').locator('canvas');
    const firstPage = pages.nth(0);
    const secondPage = pages.nth(1);
    const thirdPage = pages.nth(2);
    const fourthPage = pages.nth(3);
    await expect(pages).toHaveCount(4);

    async function expectCanvasStateToBe(page: Locator, state: CanvasStates): Promise<void> {
      switch (state) {
        case CanvasStates.Rendered:
          await expect(page).toHaveAttribute('data-canvas-state', CanvasStates.Rendered);
          break;
        case CanvasStates.Failed:
          await expect(page).toHaveAttribute('data-canvas-state', CanvasStates.Failed);
          break;
        case CanvasStates.Blank:
          await expect(page).not.toHaveAttribute('data-canvas-state');
          break;
      }
    }
    await expectCanvasStateToBe(firstPage, CanvasStates.Rendered);
    await expectCanvasStateToBe(secondPage, CanvasStates.Blank);
    await expectCanvasStateToBe(thirdPage, CanvasStates.Blank);
    await expectCanvasStateToBe(fourthPage, CanvasStates.Failed);

    await secondPage.scrollIntoViewIfNeeded();
    await expectCanvasStateToBe(firstPage, CanvasStates.Rendered);
    await expectCanvasStateToBe(secondPage, CanvasStates.Rendered);
    await expectCanvasStateToBe(thirdPage, CanvasStates.Blank);
    await expectCanvasStateToBe(fourthPage, CanvasStates.Failed);

    await thirdPage.scrollIntoViewIfNeeded();
    await expectCanvasStateToBe(firstPage, CanvasStates.Rendered);
    await expectCanvasStateToBe(secondPage, CanvasStates.Rendered);
    await expectCanvasStateToBe(thirdPage, CanvasStates.Rendered);
    await expectCanvasStateToBe(fourthPage, CanvasStates.Failed);

    await fourthPage.scrollIntoViewIfNeeded();
    await expectCanvasStateToBe(firstPage, CanvasStates.Rendered);
    await expectCanvasStateToBe(secondPage, CanvasStates.Rendered);
    await expectCanvasStateToBe(thirdPage, CanvasStates.Rendered);
    await expectCanvasStateToBe(fourthPage, CanvasStates.Failed);

    await secondPage.scrollIntoViewIfNeeded();
    await documents.waitForTimeout(500); // wait for the scroll animation to be done
    await zoomIn.click(); // zoom in to force re-rendering
    await expectCanvasStateToBe(firstPage, CanvasStates.Blank);
    await expectCanvasStateToBe(secondPage, CanvasStates.Rendered);
    await expectCanvasStateToBe(thirdPage, CanvasStates.Blank);
    await expectCanvasStateToBe(fourthPage, CanvasStates.Failed);

    await pagination.click();
    await paginationElement.locator('input').fill('1');
    await paginationElement.locator('input').press('Enter');
    await expectCanvasStateToBe(firstPage, CanvasStates.Rendered);
    await expectCanvasStateToBe(secondPage, CanvasStates.Rendered);
    await expectCanvasStateToBe(thirdPage, CanvasStates.Blank);
    await expectCanvasStateToBe(fourthPage, CanvasStates.Failed);

    await documents.waitForTimeout(500); // wait for the scroll animation to be done
    await zoomReset.click(); // reset zoom to force re-rendering
    await expectCanvasStateToBe(firstPage, CanvasStates.Rendered);
    await expectCanvasStateToBe(secondPage, CanvasStates.Blank);
    await expectCanvasStateToBe(thirdPage, CanvasStates.Blank);
    await expectCanvasStateToBe(fourthPage, CanvasStates.Failed);
  });

  msTest('PDF viewer: fullscreen', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Pdf, false);

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
});

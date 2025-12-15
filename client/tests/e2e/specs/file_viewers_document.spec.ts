// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest, openFileType } from '@tests/e2e/helpers';

msTest.skip('Open file viewer with single click', async ({ documents }) => {
  const entries = documents.locator('.folder-container').locator('.file-list-item');
  await entries.nth(0).locator('.label-name').click();
  await documents.locator('.topbar-left-content').locator('.back-button').click();
  await entries.nth(1).locator('.label-name').click();
  await expect(documents).toBeViewerPage();
});

msTest.skip('Document viewer: content', async ({ documents }) => {
  await openFileType(documents, 'docx');
  await expect(documents).toBeViewerPage();
  await expect(documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text')).toHaveText(/^[A-Za-z0-9_-]+\.docx$/);

  const wrapper = documents.locator('.file-viewer-wrapper');
  const documentContent = wrapper.locator('.document-content');
  const pages = documentContent.locator('.docx-page');
  await expect(pages).toHaveCount(3);

  // Pages content
  const firstPage = pages.nth(0);
  await expect(firstPage.locator('header')).toHaveText('Header');
  const firstPageElements = firstPage.locator('article').locator('> p');
  await expect(firstPageElements.nth(0)).toHaveText('Title');
  await expect(firstPageElements.nth(1)).toHaveText('Subtitle');
  await expect(firstPageElements.nth(2)).toHaveText(''); // Image
  await expect(firstPageElements.nth(2).locator('img')).toBeDefined();
  await expect(firstPageElements.nth(3)).toHaveText('BOLD');
  await expect(firstPageElements.nth(3).locator('span')).toHaveCSS('font-weight', '700');
  await expect(firstPageElements.nth(4)).toHaveText('Italic');
  await expect(firstPageElements.nth(4).locator('span')).toHaveCSS('font-style', 'italic');
  await expect(firstPageElements.nth(5)).toHaveText('Underline');
  await expect(firstPageElements.nth(5).locator('span')).toHaveCSS('text-decoration', /underline/);
  await expect(firstPageElements.nth(6)).toHaveText('RED');
  await expect(firstPageElements.nth(6).locator('span')).toHaveCSS('color', 'rgb(255, 0, 0)');
  await expect(firstPageElements.nth(7)).toHaveText('parsec.cloud');
  await expect(firstPageElements.nth(7).locator('a')).toHaveAttribute('href', 'https://parsec.cloud/');
  await expect(firstPageElements.nth(8)).toHaveText(''); // Page break
  await expect(firstPage.locator('footer')).toHaveText('Footer');

  const secondPage = pages.nth(1);
  await expect(secondPage.locator('header')).toHaveText('Header');
  const secondPageElements = secondPage.locator('article').locator('> p');
  await expect(secondPageElements.nth(0)).toHaveText('Second page');
  await expect(secondPageElements.nth(1)).toHaveText(''); // empty line
  await expect(secondPageElements.nth(2)).toHaveText('One');
  await expect(secondPageElements.nth(2)).toHaveTheClass('docx-page-num-1-0');
  await expect(secondPageElements.nth(3)).toHaveText('Two');
  await expect(secondPageElements.nth(3)).toHaveTheClass('docx-page-num-1-0');
  await expect(secondPageElements.nth(4)).toHaveText('Three');
  await expect(secondPageElements.nth(4)).toHaveTheClass('docx-page-num-1-0');
  await expect(secondPageElements.nth(5)).toHaveText('A');
  await expect(secondPageElements.nth(5)).toHaveTheClass('docx-page-num-1-1');
  await expect(secondPageElements.nth(6)).toHaveText('B');
  await expect(secondPageElements.nth(6)).toHaveTheClass('docx-page-num-1-1');
  await expect(secondPageElements.nth(7)).toHaveText(''); // empty line
  await expect(secondPageElements.nth(8)).toHaveText('Bullet 1');
  await expect(secondPageElements.nth(8)).toHaveTheClass('docx-page-num-2-0');
  await expect(secondPageElements.nth(9)).toHaveText('Bullet 2');
  await expect(secondPageElements.nth(9)).toHaveTheClass('docx-page-num-2-0');
  await expect(secondPageElements.nth(10)).toHaveText('Bullet 3');
  await expect(secondPageElements.nth(10)).toHaveTheClass('docx-page-num-2-0');
  await expect(secondPageElements.nth(11)).toHaveText('Bullet A');
  await expect(secondPageElements.nth(11)).toHaveTheClass('docx-page-num-2-1');
  await expect(secondPageElements.nth(12)).toHaveText('Bullet B');
  await expect(secondPageElements.nth(12)).toHaveTheClass('docx-page-num-2-1');
  await expect(secondPageElements.nth(13)).toHaveText(''); // empty line
  const secondPageTable = secondPage.locator('article').locator('> table');
  await expect(secondPageTable).toHaveCount(1);
  await expect(secondPageTable.locator('tr')).toHaveCount(2);
  await expect(secondPageTable.locator('tr').nth(0).locator('td')).toHaveText(['Table', 'Column 1', 'Column 2']);
  await expect(secondPageTable.locator('tr').nth(1).locator('td')).toHaveText(['Line 1', 'Value A', 'Value B']);
  await expect(secondPageElements.nth(14)).toHaveText(''); // page break
  await expect(secondPage.locator('footer')).toHaveText('Footer');

  const thirdPage = pages.nth(2);
  await expect(thirdPage.locator('header')).toHaveText('Header');
  const thirdPageElements = thirdPage.locator('article').locator('> p');
  /* eslint-disable max-len */
  const loremIpsumText =
    'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam.';
  /* eslint-enable max-len */
  await expect(thirdPageElements.nth(0)).toHaveText(''); // empty line
  await expect(thirdPageElements.nth(1)).toHaveText('Third page');
  await expect(thirdPageElements.nth(2)).toHaveText(''); // empty line
  await expect(thirdPageElements.nth(3)).toHaveText(loremIpsumText);
  await expect(thirdPageElements.nth(3)).toHaveCSS('text-align', 'start');
  await expect(thirdPageElements.nth(4)).toHaveText(loremIpsumText);
  await expect(thirdPageElements.nth(4)).toHaveCSS('text-align', 'center');
  await expect(thirdPageElements.nth(5)).toHaveText(loremIpsumText);
  await expect(thirdPageElements.nth(5)).toHaveCSS('text-align', 'right');
  await expect(thirdPageElements.nth(6)).toHaveText(loremIpsumText);
  await expect(thirdPageElements.nth(6)).toHaveCSS('text-align', 'justify');
  await expect(thirdPage.locator('footer')).toHaveText('Footer');
});

msTest.skip('Document viewer: pagination', async ({ documents }) => {
  await openFileType(documents, 'docx');
  await expect(documents).toBeViewerPage();
  const bottomBar = documents.locator('.file-viewer-bottombar');
  const wrapper = documents.locator('.file-viewer-wrapper');
  const documentContent = wrapper.locator('.document-content');
  const pages = documentContent.locator('.docx-page');

  const pagination = bottomBar.locator('.file-controls-pagination');
  await expect(pagination).toHaveCount(1);
  const paginationElement = pagination.locator('.file-controls-input');
  await expect(pagination.locator('.file-controls-input-prefix')).toHaveText('Page');

  async function expectPage(pageNumber: number): Promise<void> {
    await expect(paginationElement).toHaveText(`${pageNumber} / 3`);
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

msTest.skip('Document viewer: scroll', async ({ documents }) => {
  await openFileType(documents, 'docx');
  await expect(documents).toBeViewerPage();
  const bottomBar = documents.locator('.file-viewer-bottombar');
  const wrapper = documents.locator('.file-viewer-wrapper');
  const documentContent = wrapper.locator('.document-content');
  const pages = documentContent.locator('.docx-page');
  const firstPage = pages.nth(0);
  const secondPage = pages.nth(1);
  const thirdPage = pages.nth(2);
  const pagination = bottomBar.locator('.file-controls-pagination');
  const paginationElement = pagination.locator('.file-controls-input');

  await thirdPage.scrollIntoViewIfNeeded();
  await expect(firstPage).not.toBeInViewport();
  await expect(secondPage).not.toBeInViewport();
  await expect(thirdPage).toBeInViewport();
  await expect(paginationElement).toHaveText('3 / 3');

  await secondPage.scrollIntoViewIfNeeded();
  await expect(firstPage).not.toBeInViewport();
  await expect(secondPage).toBeInViewport();
  await expect(thirdPage).not.toBeInViewport();
  await expect(paginationElement).toHaveText('2 / 3');

  await firstPage.scrollIntoViewIfNeeded();
  await expect(firstPage).toBeInViewport();
  await expect(secondPage).not.toBeInViewport();
  await expect(thirdPage).not.toBeInViewport();
  await expect(paginationElement).toHaveText('1 / 3');
});

msTest.skip('Document viewer: zoom', async ({ documents }) => {
  await openFileType(documents, 'docx');
  await expect(documents).toBeViewerPage();
  const bottomBar = documents.locator('.file-viewer-bottombar');
  const wrapper = documents.locator('.file-viewer-wrapper');
  const documentContent = wrapper.locator('.document-content');

  // Zoom
  const zoom = bottomBar.locator('.zoom-controls');
  await expect(zoom).toHaveCount(1);
  const zoomReset = bottomBar.locator('#reset-zoom');
  const zoomOut = zoom.locator('.file-controls-button').nth(0);
  const zoomIn = zoom.locator('.file-controls-button').nth(1);
  const zoomInput = zoom.locator('.file-controls-input');

  async function expectZoomLevel(expectedZoomLevel: number): Promise<void> {
    await expect(zoomInput).toHaveText(`${expectedZoomLevel} %`);
    await expect(documentContent).toHaveCSS('transform', `matrix(${expectedZoomLevel / 100}, 0, 0, ${expectedZoomLevel / 100}, 0, 0)`);
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

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import { createFolder, expect, fillIonInput, importDefaultFiles, ImportDocuments, MsPage, msTest } from '@tests/e2e/helpers';

async function populateWorkspace(page: MsPage, testInfo: TestInfo): Promise<void> {
  // Folder Work
  //    - image.png
  //    - document.docx
  //    - text.txt
  // Folder Home
  //    - image.png
  //    - document.docx
  //    - text.txt
  // - image.png
  // - document.docx
  // - text.txt
  const entries = page.locator('.folder-container').locator('.file-list-item');
  await importDefaultFiles(page, testInfo, ImportDocuments.Png | ImportDocuments.Docx | ImportDocuments.Txt, false);
  await createFolder(page, 'Work');
  await createFolder(page, 'Home');
  await expect(entries).toHaveCount(5);
  await entries.nth(0).locator('.file-name').click();
  await expect(entries).toHaveCount(0);
  await importDefaultFiles(page, testInfo, ImportDocuments.Png | ImportDocuments.Docx | ImportDocuments.Txt, false);
  await expect(entries).toHaveCount(3);
  await page.locator('.topbar-left__breadcrumb').locator('.breadcrumb-element').nth(1).click();
  await expect(entries).toHaveCount(5);
  await entries.nth(1).locator('.file-name').click();
  await expect(entries).toHaveCount(0);
  await importDefaultFiles(page, testInfo, ImportDocuments.Png | ImportDocuments.Docx | ImportDocuments.Txt, false);
  await expect(entries).toHaveCount(3);
  await page.locator('.topbar-left__breadcrumb').locator('.breadcrumb-element').nth(1).click();
  await expect(entries).toHaveCount(5);
}

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });

  msTest('Search files state', async ({ documents }, testInfo: TestInfo) => {
    const entries = documents.locator('.files-container-list').locator('.file-list-item');
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Docx | ImportDocuments.Txt, false);
    await expect(entries).toHaveCount(3);

    const actionBar = documents.locator('#folders-ms-action-bar');
    const searchContainer = documents.locator('.file-search-results');
    const searchInput = actionBar.locator('.ms-search-input').locator('ion-input');
    const results = searchContainer.locator('.result-list-item');
    const actionBarButtons = actionBar.locator('.ms-action-bar-button:visible');
    const labelRole = actionBar.locator('.label-role');
    const counter = actionBar.locator('.counter');
    const sortButton = actionBar.locator('.sorter-button');
    const gridListButton = actionBar.locator('.ms-grid-list-toggle');

    await expect(actionBarButtons).toHaveCount(3);
    await expect(labelRole).toBeVisible();
    await expect(counter).toBeVisible();
    await expect(sortButton).toBeVisible();
    await expect(gridListButton).toBeVisible();

    await fillIonInput(searchInput, 'image');

    await expect(searchContainer).toBeVisible();
    await expect(documents.locator('.files-container-list')).toBeHidden();
    await expect(results).toHaveCount(1);
    await expect(actionBarButtons).toHaveCount(0);
    await expect(labelRole).toBeVisible();
    await expect(counter).toBeHidden();
    await expect(sortButton).toBeHidden();
    await expect(gridListButton).toBeHidden();

    await searchInput.locator('.input-clear-icon').click();

    await expect(searchContainer).toBeHidden();
    await expect(documents.locator('.files-container-list')).toBeVisible();
    await expect(actionBarButtons).toHaveCount(3);
    await expect(labelRole).toBeVisible();
    await expect(counter).toBeVisible();
    await expect(sortButton).toBeVisible();
    await expect(gridListButton).toBeVisible();
  });

  msTest('Search files with filters', async ({ documents }, testInfo: TestInfo) => {
    await populateWorkspace(documents, testInfo);
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await expect(entries).toHaveCount(5);
    const searchContainer = documents.locator('.file-search-results');
    const searchInput = documents.locator('#folders-ms-action-bar').locator('.ms-search-input').locator('ion-input');
    const results = searchContainer.locator('.result-list-item');
    await expect(searchContainer).toBeHidden();
    // Check case insensitive and partial match
    await fillIonInput(searchInput, 'wor');
    await expect(searchContainer).toBeVisible();

    // 4 results : the folder Work and the three files inside it
    await expect(results).toHaveCount(4);
    const titlesToggle = searchContainer.locator('.only-titles-toggle');
    await expect(titlesToggle).not.toHaveTheClass('active');
    const allNames = (await results.locator('.file-name-content').locator('.label-name').allTextContents()).sort();
    expect(allNames).toEqual(['Work', 'document.docx', 'image.png', 'text.txt']);
    await expect(results.locator('.file-name-content').locator('.label-path')).toHaveText(['/', '/Work', '/Work', '/Work']);

    // Only matching the titles, not the paths. Only the folder is left.
    await titlesToggle.click();
    await expect(titlesToggle).toHaveTheClass('active');
    const titlesOnly = (await results.locator('.file-name-content').locator('.label-name').allTextContents()).sort();
    await expect(results).toHaveCount(1);
    expect(titlesOnly).toEqual(['Work']);

    // Matching title and path, all four results are listed
    await titlesToggle.click();
    await expect(titlesToggle).not.toHaveTheClass('active');
    await expect(results).toHaveCount(4);
    const filterButton = searchContainer.locator('#select-filter-popover-button');

    // Looking at document types
    await expect(filterButton).toHaveText('Document types');
    const filterPopover = documents.locator('.document-filter-popover');
    await expect(filterPopover).toBeHidden();
    await filterButton.click();
    await expect(filterPopover).toBeVisible();
    await expect(filterPopover.getByRole('listitem')).toHaveText([
      'Folder',
      'Document',
      'Spreadsheet',
      'Presentation',
      'PDF',
      'Audio',
      'Video',
      'Image',
      'Text',
    ]);
    // Only showing Document, Text and Video
    await filterPopover.getByRole('listitem').nth(1).click();
    await filterPopover.getByRole('listitem').nth(6).click();
    await filterPopover.getByRole('listitem').nth(8).click();
    await filterPopover.locator('.filter-apply-button').click();
    await expect(filterPopover).toBeHidden();
    await expect(filterButton).toHaveText(' Document, Text +1 ');
    // Only document.docx and text.txt are visible
    await expect(results).toHaveCount(2);
    const filteredNames = (await results.locator('.file-name-content').locator('.label-name').allTextContents()).sort();
    expect(filteredNames).toEqual(['document.docx', 'text.txt']);
  });

  msTest('Search files no match', async ({ documents }, testInfo: TestInfo) => {
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Docx | ImportDocuments.Txt, false);
    await expect(entries).toHaveCount(3);

    const searchContainer = documents.locator('.file-search-results');
    const searchInput = documents.locator('#folders-ms-action-bar').locator('.ms-search-input').locator('ion-input');
    const results = searchContainer.locator('.result-list-item');

    await expect(searchContainer).toBeHidden();
    await fillIonInput(searchInput, 'Black Mesa');
    await expect(searchContainer).toBeVisible();
    await expect(results).toBeHidden();
    await expect(searchContainer.locator('.results-empty')).toBeVisible();
    await expect(searchContainer.locator('.results-empty').locator('.results-empty__title')).toHaveText(
      'No results match your search in this workspace',
    );
  });
});

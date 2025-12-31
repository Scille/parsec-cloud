// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BrowserContext, Locator, Page, TestInfo } from '@playwright/test';
import { expect } from '@tests/e2e/helpers/assertions';
import { DisplaySize, ImportAllDocuments, ImportDocuments, MsPage } from '@tests/e2e/helpers/types';
import { readFileSync } from 'fs';
import path from 'path';

interface QuestionOptions {
  expectedTitleText?: string | RegExp;
  expectedQuestionText?: string | RegExp;
  expectedPositiveText?: string | RegExp;
  expectedNegativeText?: string | RegExp;
}

export async function answerQuestion(page: MsPage, positiveAnswer: boolean, options?: QuestionOptions): Promise<void> {
  const smallDisplay = (await page.getDisplaySize()) === DisplaySize.Small;
  const modal = page.locator('.question-modal');
  const positiveButton = modal.locator(smallDisplay ? '#confirm-button' : '#next-button');
  const negativeButton = modal.locator('#cancel-button');

  await expect(modal).toBeVisible();

  if (options) {
    if (options.expectedTitleText) {
      if (smallDisplay) {
        await expect(modal.locator('.ms-small-display-modal-header__title')).toHaveText(options.expectedTitleText);
      } else {
        await expect(modal.locator('.ms-modal-header__title')).toHaveText(options.expectedTitleText);
      }
    }
    if (options.expectedQuestionText) {
      if (smallDisplay) {
        await expect(modal.locator('.ms-small-display-modal-header__text')).toHaveText(options.expectedQuestionText);
      } else {
        await expect(modal.locator('.ms-modal-header__text')).toHaveText(options.expectedQuestionText);
      }
    }
    if (options.expectedPositiveText) {
      await expect(positiveButton).toHaveText(options.expectedPositiveText);
    }
    if (options.expectedNegativeText) {
      await expect(negativeButton).toHaveText(options.expectedNegativeText);
    }
  }
  if (positiveAnswer) {
    await positiveButton.click();
  } else {
    await negativeButton.click();
  }
  await expect(modal).toBeHidden();
}

export async function fillIonInput(ionInput: Locator, text: string): Promise<void> {
  const input = ionInput.locator('input');
  await input.fill(text);
  await input.blur();
  await expect(ionInput).not.toBeFocused();
}

export async function fillIonTextArea(ionTextArea: Locator, text: string): Promise<void> {
  const textArea = ionTextArea.locator('textarea');
  await textArea.fill(text);
  await textArea.blur();
  await expect(textArea).toHaveValue(text);
  await expect(ionTextArea).not.toBeFocused();
}

export async function fillInputModal(root: Locator | Page, text: string, clear?: boolean): Promise<void> {
  const modal = root.locator('.text-input-modal');
  await expect(modal).toBeVisible();
  if (clear) {
    await fillIonInput(modal.locator('ion-input'), '');
  }
  const okButton = modal.locator('.ms-modal-footer-buttons').locator('#next-button');
  await fillIonInput(modal.locator('ion-input'), text);
  await okButton.click();
  await expect(modal).toBeHidden();
}

export async function inviteUsers(page: MsPage, emails: Array<string> | string): Promise<void> {
  const modal = page.locator('.invite-modal');
  await expect(modal).toBeVisible();
  const okButton = modal.locator('#next-button');
  await expect(okButton).toBeTrulyDisabled();
  await fillIonInput(modal.locator('ion-input'), Array.isArray(emails) ? emails.join(';') : emails);
  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();
  await expect(modal).toBeHidden();
}

export async function getClipboardText(page: Page): Promise<string> {
  return await page.evaluate(() => navigator.clipboard.readText());
}

export async function setWriteClipboardPermission(context: BrowserContext, allow: boolean): Promise<void> {
  if (allow) {
    await context.grantPermissions(['clipboard-write']);
  } else {
    // There doesn't seem to be a function to remove specific permissions, so we clear
    // them all and re-add clipboard-read.
    await context.clearPermissions();
    await context.grantPermissions(['clipboard-read']);
  }
}

export async function selectDropdown(button: Locator, select: string, currentlySelected?: string): Promise<void> {
  const page = button.page();
  await expect(page.locator('.dropdown-popover')).toBeHidden();
  await button.click();
  const dropdown = page.locator('.dropdown-popover');
  await expect(dropdown).toBeVisible();

  const options = dropdown.getByRole('listitem');
  if (currentlySelected) {
    await expect(options.filter({ hasText: currentlySelected })).toHaveTheClass('selected');
  }
  await options.filter({ hasText: select }).click();
  await expect(page.locator('.dropdown-popover')).toBeHidden();
}

export async function sortBy(sortButton: Locator, clickOnLabel: string): Promise<void> {
  await sortButton.click();
  const popover = sortButton.page().locator('.sorter-popover');
  await popover.getByRole('listitem').filter({ hasText: clickOnLabel }).click();
  await expect(popover).toBeHidden();
}

export async function dragAndDropFile(page: Page, element: Locator, filePaths: Array<string>): Promise<void> {
  const files: Array<{ content: string; name: string }> = [];

  for (const filePath of filePaths) {
    files.push({ content: readFileSync(filePath).toString('base64'), name: path.basename(filePath) });
  }

  const dataTransfer = await page.evaluateHandle(async (filesInfo: Array<{ content: string; name: string }>) => {
    const dt = new DataTransfer();

    for (const fileInfo of filesInfo) {
      const blobData = await fetch(`data:application/octet-stream;base64,${fileInfo.content}`).then((res) => res.blob());
      const upload = new File([blobData], fileInfo.name, { type: 'application/octet-stream' });
      dt.items.add(upload);
    }
    return dt;
  }, files);

  await element.dispatchEvent('dragenter');
  await element.dispatchEvent('drop', { dataTransfer });
}

export const Media = {
  async getDuration(media: Locator): Promise<number> {
    return await media.evaluate((el) => {
      return (el as HTMLVideoElement).duration;
    });
  },
  async getCurrentTime(media: Locator): Promise<number> {
    return await media.evaluate((el) => {
      return (el as HTMLVideoElement).currentTime;
    });
  },
  async getPausedState(media: Locator): Promise<boolean> {
    return await media.evaluate((el) => {
      return (el as HTMLVideoElement).paused;
    });
  },
};

export async function openFileType(
  documentsPage: MsPage,
  type: 'xlsx' | 'docx' | 'png' | 'pdf' | 'mp3' | 'mp4' | 'txt' | 'py',
): Promise<void> {
  const entries = documentsPage.locator('.folder-container').locator('.file-list-item');

  for (const entry of await entries.all()) {
    const entryName = (await entry.locator('.file-name').locator('.label-name').textContent()) ?? '';
    if (entryName.endsWith(`.${type}`)) {
      await entry.dblclick();
      await expect(documentsPage).toBeFileHandlerPage();
      return;
    }
  }
}

export async function testFileViewerZoomLevel(fileWrapper: Locator, zoom: string): Promise<void> {
  const value = await fileWrapper.evaluate((el) => {
    return (el.attributes.getNamedItem('style') as Attr).value;
  });
  expect(value).toMatch(new RegExp(`^--[a-f0-9]+-zoomLevel: ${zoom};$`));
}

export async function addDownloadedFile(win: Window, data: Uint8Array, name: string = 'default'): Promise<void> {
  if ((win as any).__downloadedFiles === undefined) {
    (win as any).__downloadedFiles = {
      [name]: data,
    };
  } else {
    (win as any).__downloadedFiles[name] = data;
  }
}

export async function getDownloadedFile(page: Page, name: string = 'default'): Promise<Uint8Array | undefined> {
  return await page.evaluate((name) => {
    if ((window as any).__downloadedFiles && (window as any).__downloadedFiles[name]) {
      return Array.from((window as any).__downloadedFiles[name]) as unknown as Uint8Array;
    }
    return undefined;
  }, name);
}

export async function sliderClick(page: Page, slider: Locator, progressPercent: number): Promise<void> {
  const box = await slider.boundingBox();
  if (box) {
    await page.mouse.click(box.x + (box.width * progressPercent) / 100, box.y);
  }
}

export async function expandSheetModal(page: Page, modal: Locator): Promise<void> {
  await page.waitForTimeout(500);
  const box = await modal.boundingBox();
  if (box) {
    await page.mouse.move(box.x + box.width / 2, box.y + box.height * 0.95);
    await page.mouse.down();
    await page.mouse.move(box.x + box.width / 2, box.y);
    await page.mouse.up();
    await page.waitForTimeout(500);
  }
}

export async function clientAreaSwitchOrganization(page: Page, organization: string | 'all'): Promise<void> {
  const selector = page.locator('.sidebar-header').locator('.organization-card-header');
  await selector.click();
  const popover = page.locator('#organization-switch-popover');
  await expect(popover).toBeVisible();
  const items = popover.locator('.organization-list__item').locator('.organization-name');
  for (const item of await items.all()) {
    const text = await item.textContent();
    if (text === organization || (text === 'All organizations' && organization === 'all')) {
      await item.click();
      break;
    }
  }
  await expect(popover).toBeHidden();
}

export async function clientAreaNavigateTo(page: Page, label: string): Promise<void> {
  const button = page.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').filter({ hasText: label });
  await expect(button).toBeVisible();
  await expect(button).toBeTrulyEnabled();

  await button.click();
  await expect(page.locator('.topbar').locator('.topbar-left-text__title')).toHaveText(label);
}

export async function workspacesInGridMode(workspacesPage: Page): Promise<boolean> {
  return (await workspacesPage.locator('#workspaces-ms-action-bar').locator('#grid-view').getAttribute('disabled')) !== null;
}

export async function createWorkspace(workspacesPage: MsPage, name: string): Promise<void> {
  const displaySize = await workspacesPage.getDisplaySize();
  let workspacesCount = 0;
  await expect(workspacesPage.locator('.no-workspaces-loading')).toBeHidden();

  if (displaySize === DisplaySize.Small) {
    const addButton = workspacesPage.locator('#add-menu-fab-button');
    await expect(addButton).toBeVisible();
    await addButton.click();
    const modal = workspacesPage.locator('.tab-menu-modal');
    await expect(modal).toBeVisible();
    await modal.locator('.list-group-item').filter({ hasText: 'New workspace' }).click();
    await fillInputModal(workspacesPage, name);
  } else {
    if (await workspacesInGridMode(workspacesPage)) {
      workspacesCount = await workspacesPage.locator('.workspaces-container').locator('.workspace-card-item').count();
    } else {
      workspacesCount = await workspacesPage.locator('.workspaces-container').locator('.workspace-list-item').count();
    }
    const actionBar = workspacesPage.locator('#workspaces-ms-action-bar');
    await actionBar.getByText('New workspace').click();
    await fillInputModal(workspacesPage, name);

    if (await workspacesInGridMode(workspacesPage)) {
      await expect(workspacesPage.locator('.workspaces-container').locator('.workspace-card-item')).toHaveCount(workspacesCount + 1);
    } else {
      await expect(workspacesPage.locator('.workspaces-container').locator('.workspace-list-item')).toHaveCount(workspacesCount + 1);
    }
  }
  await dismissToast(workspacesPage);
}

export async function createFolder(documentsPage: MsPage, name: string): Promise<void> {
  const displaySize = await documentsPage.getDisplaySize();
  if (displaySize === DisplaySize.Small) {
    await documentsPage.locator('#add-menu-fab-button').click();
    await documentsPage.locator('.tab-menu-modal').locator('.list-group-item').nth(0).click();
  } else {
    const actionBar = documentsPage.locator('#folders-ms-action-bar');
    const createFolderButton = actionBar.locator('.ms-action-bar-button:visible').nth(0);
    await expect(createFolderButton).toHaveText('New folder');
    await createFolderButton.click();
  }
  await fillInputModal(documentsPage, name);
}

export async function dismissToast(page: Page): Promise<void> {
  await page.locator('.notification-toast').locator('.toast-button-confirm').click();
  await expect(page.locator('.notification-toast')).toHaveCount(0);
}

export function getTestbedBootstrapAddr(orgName: string): string {
  const url = new URL(process.env.TESTBED_SERVER ?? '');
  const port = url.port ? `:${url.port}` : '';
  let search = '?a=bootstrap_organization&p=wA';

  for (const [key, val] of url.searchParams.entries()) {
    search = `${search}&${key}=${val}`;
  }
  return `${url.protocol}//${url.hostname}${port}/${orgName}${search}`;
}

export function getOrganizationAddr(orgName: string): string {
  const url = new URL(process.env.TESTBED_SERVER ?? '');
  const port = url.port ? `:${url.port}` : '';

  return `${url.protocol}//${url.hostname}${port}/${orgName}${url.search}`;
}

export function getServerAddr(): string {
  return process.env.TESTBED_SERVER ?? '';
}

export async function login(homePage: Page, name: string): Promise<void> {
  await homePage.locator('.organization-card', { hasText: name }).click();
  await expect(homePage.locator('#password-input')).toBeVisible();

  await expect(homePage.locator('.login-button')).toHaveDisabledAttribute();

  await homePage.locator('#password-input').locator('input').fill('P@ssw0rd.');
  await expect(homePage.locator('.login-button')).toBeEnabled();
  await homePage.locator('.login-button').click();
  await expect(homePage.locator('#connected-header')).toContainText('My workspaces');
  await expect(homePage).toBeWorkspacePage();
}

export async function logout(page: MsPage): Promise<void> {
  await page.locator('#connected-header').locator('#profile-button').click();
  const buttons = page.locator('.profile-header-organization-popover').locator('.main-list').getByRole('listitem');
  await buttons.nth(4).click();
  await answerQuestion(page, true);
  await expect(page).toBeHomePage();
  await expect(page.locator('.homepage-header').locator('.topbar-left-text__title')).toHaveText('Welcome to Parsec');
  await expect(page.locator('.homepage-header').locator('.topbar-left-text__subtitle')).toHaveText('Access your organizations');
}

export async function importDefaultFiles(
  documentsPage: MsPage,
  testInfo: TestInfo,
  imports: number = ImportAllDocuments,
  mkdir: boolean = true,
): Promise<void> {
  await expect(documentsPage).toBeDocumentPage();
  if (mkdir) {
    await createFolder(documentsPage, 'Dir_Folder');
  }
  const dropZone = documentsPage.locator('.folder-container').locator('.drop-zone').nth(0);
  let imported = 0;
  if (imports === ImportAllDocuments) {
    // Speed up a bit by importing everything
    await dragAndDropFile(documentsPage, dropZone, [
      path.join(testInfo.config.rootDir, 'data', 'imports', 'image.png'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'document.docx'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'pdfDocument.pdf'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'video.mp4'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'audio.mp3'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'spreadsheet.xlsx'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'text.txt'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'code.py'),
    ]);
    imported = 8;
  } else {
    const FILES = new Map([
      [ImportDocuments.Png, 'image.png'],
      [ImportDocuments.Docx, 'document.docx'],
      [ImportDocuments.Pdf, 'pdfDocument.pdf'],
      [ImportDocuments.Mp4, 'video.mp4'],
      [ImportDocuments.Mp3, 'audio.mp3'],
      [ImportDocuments.Xlsx, 'spreadsheet.xlsx'],
      [ImportDocuments.Txt, 'text.txt'],
      [ImportDocuments.Py, 'code.py'],
    ]);

    for (const [flag, document] of FILES.entries()) {
      if (imports & flag) {
        await dragAndDropFile(documentsPage, dropZone, [path.join(testInfo.config.rootDir, 'data', 'imports', document)]);
        imported += 1;
      }
    }
  }
  // Hide the import menu
  if (imported > 0) {
    const uploadMenu = documentsPage.locator('.upload-menu');
    await expect(uploadMenu).toBeVisible();
    const tabs = uploadMenu.locator('.upload-menu-tabs').getByRole('listitem');
    await expect(tabs.locator('.text-counter')).toHaveText(['0', `${imported}`, '0']);
    await uploadMenu.locator('.menu-header-icons').locator('ion-icon').nth(1).click();
  }
  await expect(documentsPage.locator('.folder-container').locator('.no-files-content')).toBeHidden();
}

export async function renameDocument(documentsPage: MsPage, entry: Locator, newName: string, gridMode = false): Promise<void> {
  if (gridMode) {
    await entry.locator('.card-option').click();
  } else {
    await entry.hover();
    await entry.locator('.options-button').click();
  }

  const smallDisplay = (await documentsPage.getDisplaySize()) === DisplaySize.Small;
  let popover!: Locator;
  if (smallDisplay) {
    await expandSheetModal(documentsPage, documentsPage.locator('.file-context-sheet-modal'));
    popover = documentsPage.locator('.file-context-sheet-modal');
  } else {
    popover = documentsPage.locator('.file-context-menu');
  }

  await popover.getByRole('listitem').filter({ hasText: 'Rename' }).click();
  await fillInputModal(documentsPage, newName, true);
  // Can't check if the entry's been renamed, since the renaming may change the sort order and therefore,
  // what element the entry points to.
}

export async function resizePage(page: Page, width: number, height?: number): Promise<void> {
  await page.setViewportSize({ width, height: height ?? 900 });
  await page.waitForTimeout(100);
}

export async function sendUpdateEvent(page: MsPage): Promise<void> {
  await page.evaluate(() => {
    window.electronAPI.getUpdateAvailability(true);
  });
}

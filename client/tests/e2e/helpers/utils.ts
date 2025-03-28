// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BrowserContext, Locator, Page } from '@playwright/test';
import { expect } from '@tests/e2e/helpers/assertions';
import { readFileSync } from 'fs';
import path from 'path';

interface QuestionOptions {
  expectedTitleText?: string | RegExp;
  expectedQuestionText?: string | RegExp;
  expectedPositiveText?: string | RegExp;
  expectedNegativeText?: string | RegExp;
}

export async function answerQuestion(page: Page, positiveAnswer: boolean, options?: QuestionOptions): Promise<void> {
  const modal = page.locator('.question-modal');
  const positiveButton = modal.locator('#next-button');
  const negativeButton = modal.locator('#cancel-button');

  await expect(modal).toBeVisible();

  if (options) {
    if (options.expectedTitleText) {
      await expect(modal.locator('.ms-modal-header__title')).toHaveText(options.expectedTitleText);
    }
    if (options.expectedQuestionText) {
      await expect(modal.locator('.ms-modal-header__text')).toHaveText(options.expectedQuestionText);
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

// eslint-disable-next-line
export async function openFileType(
  documentsPage: Page,
  type: 'xlsx' | 'docx' | 'png' | 'pdf' | 'mp3' | 'mp4' | 'txt' | 'py',
): Promise<void> {
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

export async function workspacesInGridMode(workspacesPage: Page): Promise<boolean> {
  return (await workspacesPage.locator('#workspaces-ms-action-bar').locator('#grid-view').getAttribute('disabled')) !== null;
}

export async function createWorkspace(workspacesPage: Page, name: string): Promise<void> {
  let workspacesCount = 0;
  await expect(workspacesPage.locator('.no-workspaces-loading')).toBeHidden();
  if (await workspacesInGridMode(workspacesPage)) {
    workspacesCount = await workspacesPage.locator('.workspaces-container').locator('.workspace-card-item').count();
  } else {
    workspacesCount = await workspacesPage.locator('.workspaces-container').locator('.workspace-list-item').count();
  }
  const actionBar = workspacesPage.locator('#workspaces-ms-action-bar');
  await actionBar.locator('#button-new-workspace').click();
  await fillInputModal(workspacesPage, name);
  if (await workspacesInGridMode(workspacesPage)) {
    await expect(workspacesPage.locator('.workspaces-container').locator('.workspace-card-item')).toHaveCount(workspacesCount + 1);
  } else {
    await expect(workspacesPage.locator('.workspaces-container').locator('.workspace-list-item')).toHaveCount(workspacesCount + 1);
  }
  await dismissToast(workspacesPage);
}

export async function createFolder(documentsPage: Page, name: string): Promise<void> {
  const actionBar = documentsPage.locator('#folders-ms-action-bar');
  await actionBar.locator('.ms-action-bar-button:visible').nth(0).click();
  await fillInputModal(documentsPage, name);
}

export async function dismissToast(page: Page): Promise<void> {
  await page.locator('.notification-toast').locator('.toast-button-confirm').click();
  await expect(page.locator('.notification-toast')).toBeHidden();
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

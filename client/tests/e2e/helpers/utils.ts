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
  if (clear) {
    await fillIonInput(modal.locator('ion-input'), '');
  }
  const okButton = modal.locator('.ms-modal-footer-buttons').locator('#next-button');
  await expect(okButton).toHaveDisabledAttribute();
  await fillIonInput(modal.locator('ion-input'), text);
  await expect(okButton).toNotHaveDisabledAttribute();
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

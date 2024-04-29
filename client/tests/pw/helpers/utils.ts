// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page } from '@playwright/test';
import { expect } from '@tests/pw/helpers/assertions';

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

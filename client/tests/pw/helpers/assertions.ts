// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect as baseExpect, Locator, Page } from '@playwright/test';

interface AssertReturnType {
  message: () => string;
  pass: boolean;
}

export const expect = baseExpect.extend({
  async toHaveDisabledAttribute(locator: Locator): Promise<AssertReturnType> {
    const attr1 = await locator.getAttribute('disabled');
    const attr2 = await locator.getAttribute('aria-disabled');
    return {
      message: () => 'failed',
      pass: attr1 !== null || (attr2 !== null && attr2 === 'true'),
    };
  },

  async toHaveTheClass(locator: Locator, className: string): Promise<AssertReturnType> {
    const pass = await locator.evaluate((node, clsName) => node.classList.contains(clsName), className);
    return {
      message: () => `Does not have class '${className}'`,
      pass: pass,
    };
  },

  async toShowToast(page: Page, message: string, theme: 'Success' | 'Warning' | 'Error' | 'Info'): Promise<AssertReturnType> {
    const toast = page.locator('.notification-toast');
    let errorMessage = '';
    let pass = await toast.evaluate((node, theme: string) => {
      const expectedClass = `ms-${theme.toLowerCase()}`;
      return node.classList.contains(expectedClass);
    }, theme);

    if (pass) {
      try {
        await baseExpect(toast.locator('.toast-message')).toHaveText(message);
      } catch (_error) {
        errorMessage = `Toast does not contain the text '${message}'`;
        pass = false;
      }
    } else {
      errorMessage = `Toast does not have the theme '${theme}'`;
    }

    return {
      message: () => errorMessage,
      pass: pass,
    };
  },
});

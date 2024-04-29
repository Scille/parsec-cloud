// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect as baseExpect, Locator } from '@playwright/test';

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
});

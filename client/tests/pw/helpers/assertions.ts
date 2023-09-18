// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect as baseExpect, Locator } from '@playwright/test';

export const expect = baseExpect.extend({
  async toHaveDisabledAttribute(locator: Locator): Promise<{ message: () => string; pass: boolean }> {
    const attr = await locator.getAttribute('disabled');
    return {
      message: () => 'failed',
      pass: attr !== null,
    };
  },
});

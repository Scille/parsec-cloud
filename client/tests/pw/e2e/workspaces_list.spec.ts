// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';

msTest('Opens the about dialog', async ({ connected }) => {
  await expect(connected.locator('.workspaces-grid-item')).toHaveCount(3);
});

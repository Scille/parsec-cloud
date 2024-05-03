// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';
import { fillInputModal } from '@tests/pw/helpers/utils';

msTest('Rename a workspace', async ({ connected }) => {
  const firstWorkspace = connected.locator('.workspaces-container-grid').locator('.workspaces-grid-item').nth(1);
  await firstWorkspace.locator('.card-option').click();
  const popover = connected.locator('.popover-viewport');
  await popover.getByRole('listitem').nth(1).click();
  await fillInputModal(connected, 'New Workspace Name', true);
  await expect(connected).toShowToast('Workspace has been successfully renamed to New Workspace Name.', 'Success');
});

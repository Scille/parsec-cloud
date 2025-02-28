// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest } from '@tests/e2e/helpers';

msTest('Navigate to unknown URL from home', async ({ home }) => {
  await home.goto('/unknownURL');
  await expect(home).toBeHomePage();
  await expect(home).toHaveURL(new RegExp('/home$'));
});

msTest('Navigate to unknown URL from logged in', async ({ connected }) => {
  await connected.goto('/unknownURL');
  await expect(connected).toBeHomePage();
  await expect(connected).toHaveURL(new RegExp('/home$'));
});

msTest('Navigate to unknown logged in URL from home', async ({ home }) => {
  await home.goto('/42/unknownURL');
  await expect(home).toBeWorkspacePage();
  await expect(home).toHaveURL(new RegExp('/\\d+/workspaces$'));
});

msTest('Navigate to unknown logged in URL from logged in', async ({ connected }) => {
  await connected.goto('/42/unknownURL');
  await expect(connected).toBeWorkspacePage();
  await expect(connected).toHaveURL(new RegExp('/\\d+/workspaces$'));
});

msTest('Navigate to unknown with query logged in URL from logged in', async ({ connected }) => {
  await connected.goto('/42/unknownURL?param1=test');
  await expect(connected).toHaveURL(new RegExp('/\\d+/workspaces$'));
  await expect(connected).toBeWorkspacePage();
});

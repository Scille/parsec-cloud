// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest, setupNewPage } from '@tests/e2e/helpers';

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
  await setupNewPage(home, undefined, '/42/unknownURL');
  await expect(home).toBeHomePage();
  await expect(home).toHaveURL(new RegExp('/home$'));
});

msTest('Navigate to unknown logged in URL from logged in', async ({ connected }) => {
  await setupNewPage(connected, undefined, '/42/unknownURL');
  await expect(connected).toBeHomePage();
  await expect(connected).toHaveURL(new RegExp('/home$'));
});

msTest('Navigate to unknown with query logged in URL from logged in', async ({ connected }) => {
  await setupNewPage(connected, undefined, '/42/unknownURL?param1=test');
  await expect(connected).toBeHomePage();
  await expect(connected).toHaveURL(new RegExp('/home$'));
});

msTest('Navigate to an unknown logged in URL with a valid handle and another tab open', async ({ connected }) => {
  // Open another tab to keep the web worker running
  const secondTab = await connected.openNewTab();
  await setupNewPage(connected, undefined, '/1/unknownURL');
  await expect(connected).toBeWorkspacePage();
  await expect(connected).toHaveURL(new RegExp('/workspaces$'));
  await secondTab.release();
});

msTest('Navigate to an unknown logged in URL with a valid handle', async ({ connected }) => {
  // The web worker is killed, and the handle becomes invalid
  await setupNewPage(connected, undefined, '/1/unknownURL');
  await expect(connected).toBeHomePage();
  await expect(connected).toHaveURL(new RegExp('/home$'));
});

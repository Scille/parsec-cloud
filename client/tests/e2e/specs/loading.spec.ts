// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, MsPage, msTest, setupNewPage } from '@tests/e2e/helpers';

for (const lang of ['en', 'fr']) {
  msTest(`App initialization loading ${lang}`, async ({ context }) => {
    await context.addInitScript((locale) => {
      Object.defineProperty(navigator, 'language', {
        get: () => (locale === 'fr' ? 'fr-FR' : 'en-US'),
      });
    }, lang);
    const page = (await context.newPage()) as MsPage;
    // Setup with a next stage hook that does nothing to interrupt the loading
    await setupNewPage(page, { skipTestbed: true, customNextStage: async (_page: MsPage) => {} });
    const container = page.locator('.loading-container');
    if (lang === 'fr') {
      await expect(container.locator('#loading-title')).toHaveText("Lancement de l'application Parsec");
      await expect(container.locator('#loading-subtitle')).toHaveText('Chargement des modules...');
    } else {
      await expect(container.locator('#loading-title')).toHaveText('Loading Parsec app');
      await expect(container.locator('#loading-subtitle')).toHaveText('Initializing modules...');
    }
  });
}

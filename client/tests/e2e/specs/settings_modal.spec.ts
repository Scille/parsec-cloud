// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator } from '@playwright/test';
import { expect, msTest, selectDropdown } from '@tests/e2e/helpers';

async function checkModal(modal: Locator): Promise<void> {
  await expect(modal.locator('.ms-modal-header__title')).toHaveText('Settings');
  const menuItems = modal.locator('.menu-list').locator('.menu-list__item:visible');
  await expect(menuItems).toHaveCount(1);
  await expect(menuItems).toHaveText('General');
  const content = modal.locator('.menu-item-content');
  const options = content.locator('.settings-option');
  await expect(options).toHaveCount(3);
  const lang = options.nth(0);
  await expect(lang.locator('.settings-option__content').locator('.title')).toHaveText('Language');
  await expect(lang.locator('.settings-option__content').locator('.description')).toHaveText('Choose application language');
  await expect(lang.locator('.dropdown-container')).toHaveText('English');
  await selectDropdown(lang.locator('ion-button'), 'Français', 'English');
  await expect(lang.locator('.dropdown-container')).toHaveText('Français');
  const theme = options.nth(1);
  // Now we're in French
  await expect(theme.locator('.settings-option__content').locator('.title')).toHaveText('Thème');
  await expect(theme.locator('.settings-option__content').locator('.description')).toHaveText("Choisir l'apparence de l'application");
  await expect(theme.locator('.dropdown-container')).toHaveText('Clair');

  const telemetry = options.nth(2);
  await expect(telemetry.locator('.settings-option__content').locator('.title')).toHaveText("Envoyer les rapports d'erreur");
  await expect(telemetry.locator('.settings-option__content').locator('.description')).toHaveText(
    "Les rapports d'erreur aident à améliorer Parsec",
  );
}

msTest('Settings modal on home page', async ({ home }) => {
  await expect(home.locator('.settings-modal')).toBeHidden();
  await home.locator('.menu-secondary').locator('#trigger-settings-button').click();
  const modal = home.locator('.settings-modal');
  await expect(modal).toBeVisible();
  await checkModal(modal);
  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
});

msTest('Settings modal when connected', async ({ connected }) => {
  await expect(connected.locator('.settings-modal')).toBeHidden();
  await connected.locator('.topbar').locator('.profile-header').click();
  await connected.locator('.profile-header-popover').locator('.main-list').getByRole('listitem').nth(1).click();
  const modal = connected.locator('.settings-modal');
  await expect(modal).toBeVisible();
  await checkModal(modal);
  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DEFAULT_USER_INFORMATION, MockBms, expect, fillIonInput, msTest } from '@tests/e2e/helpers';

msTest('Opens the create organization modal', async ({ home }) => {
  await expect(home.locator('#create-organization-button')).toHaveText('Create or join');
  await expect(home.locator('.homepage-popover')).toBeHidden();
  await home.locator('#create-organization-button').click();
  await expect(home.locator('.homepage-popover')).toBeVisible();
  await expect(home.locator('.create-organization-modal')).toBeHidden();
  const createButton = home.locator('.homepage-popover').getByRole('listitem').nth(0);
  await expect(createButton.locator('ion-label')).toHaveText('Create');
  await expect(createButton.locator('ion-text')).toHaveText('I want to create an organization');
  await createButton.click();
  const modal = home.locator('.create-organization-modal');
  await expect(modal).toBeVisible();
  await expect(modal.locator('.modal-header-title__text')).toHaveText('Create an organization');
  await expect(modal.locator('.modal-header__text')).toHaveText('Choose the option that best suits your needs:');
  await expect(modal.locator('.server-choice-item').locator('.server-choice-item__label')).toHaveText([
    'Create my organization on Parsec',
    'Try Parsec for 15 days (Your organization and associated data will be deleted at the end of the trial period)',
  ]);
  await expect(modal.locator('.server-modal-footer').locator('ion-button').nth(0)).toHaveText('Use a different Parsec server');

  // Select one option
  const nextButton = modal.locator('.server-modal-footer').locator('ion-button').nth(1);
  await expect(nextButton).toBeTrulyDisabled();
  await expect(nextButton).toHaveText('Continue');
  await modal.locator('.server-choice-item').nth(0).click();
  await expect(nextButton).toBeTrulyEnabled();

  // Check if the close button closes the modal
  await expect(modal.locator('.closeBtn')).toBeVisible();
  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
});

msTest('Return to server selection after selecting server type', async ({ home }) => {
  await home.locator('#create-organization-button').click();
  await home.locator('.popover-viewport').getByRole('listitem').nth(0).click();
  const modal = home.locator('.create-organization-modal');
  await expect(modal.locator('.server-modal')).toBeVisible();

  // Go to saas and back
  await modal.locator('.server-choice-item').nth(0).click();
  await modal.locator('.server-modal-footer').locator('ion-button').nth(1).click();
  await expect(modal.locator('.saas-login')).toBeVisible();
  await modal.locator('.saas-login-button__item').nth(0).click();
  await expect(modal.locator('.server-modal')).toBeVisible();

  // Go to saas, login, and back
  await modal.locator('.server-choice-item').nth(0).click();
  await modal.locator('.server-modal-footer').locator('ion-button').nth(1).click();

  await MockBms.mockLogin(home);
  await MockBms.mockUserRoute(home);
  const bmsContainer = modal.locator('.saas-login');
  const bmsNext = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);

  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await expect(bmsNext).toHaveDisabledAttribute();
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(bmsNext).not.toHaveDisabledAttribute();
  await bmsNext.click();
  await expect(bmsContainer).toBeHidden();
  await expect(modal.locator('.organization-name-page')).toBeVisible();
  await modal.locator('.organization-name-page').locator('#previous-button').click();
  await expect(modal.locator('.server-modal')).toBeVisible();

  // To to trial and back
  await modal.locator('.server-choice-item').nth(1).click();
  await modal.locator('.server-modal-footer').locator('ion-button').nth(1).click();
  await expect(modal.locator('.user-information-page')).toBeVisible();
  await modal.locator('.user-information-page').locator('#previous-button').click();
  await expect(modal.locator('.server-modal')).toBeVisible();

  // Go to custom and back
  await modal.locator('.server-modal-footer').locator('ion-button').nth(0).click();
  await expect(modal.locator('.organization-name-and-server-page')).toBeVisible();
  await modal.locator('.organization-name-and-server-page').locator('#previous-button').click();
  await expect(modal.locator('.server-modal')).toBeVisible();
});

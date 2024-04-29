// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator } from '@playwright/test';
import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';
import { answerQuestion, fillIonInput } from '@tests/pw/helpers/utils';

const createOrgTest = msTest.extend<{ createOrgModal: Locator }>({
  createOrgModal: async ({ home }, use) => {
    await home.locator('#create-organization-button').click();
    await expect(home.locator('.homepage-popover')).toBeVisible();
    await home.locator('.homepage-popover').getByRole('listitem').nth(0).click();
    const modal = home.locator('.create-organization-modal');
    await expect(home.locator('.create-organization-modal')).toBeVisible();
    await use(modal);
  },
});

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
});

msTest('Close the create organization modal', async ({ home }) => {
  await home.locator('#create-organization-button').click();
  await expect(home.locator('.create-organization-modal')).toBeHidden();
  const createButton = home.locator('.homepage-popover').getByRole('listitem').nth(0);
  await createButton.click();
  await expect(home.locator('.create-organization-modal')).toBeVisible();
  // Go at least to second page
  await fillIonInput(home.locator('.create-organization-modal').locator('#org-name-input').locator('ion-input'), 'MyOrg');
  await home.locator('.create-organization-modal').locator('#next-button').click();

  await home.locator('.create-organization-modal').locator('.closeBtn').click();
  await expect(home.locator('.create-organization-modal')).toBeHidden();

  await answerQuestion(home, false, {
    expectedTitleText: 'Cancel organization creation',
    expectedQuestionText: 'Are you sure you want to cancel the process? Information will not be saved, you will have to restart.',
    expectedNegativeText: 'Resume',
    expectedPositiveText: 'Cancel process',
  });

  await expect(home.locator('.create-organization-modal')).toBeVisible();
  await home.locator('.create-organization-modal').locator('.closeBtn').click();

  await answerQuestion(home, true);
  await expect(home.locator('.create-organization-modal')).toBeHidden();
});

createOrgTest('Go through the organization create process', async ({ createOrgModal }) => {
  const modalTitle = createOrgModal.locator('.modal-header__title');
  const modalSubtitle = createOrgModal.locator('.modal-header__text');
  const nextButton = createOrgModal.locator('#next-button');
  const modalContent = createOrgModal.locator('.modal-content');

  async function ensureCurrentStepIs(modalContent: Locator, stepClass: string): Promise<void> {
    const stepsClasses = ['org-name', 'user-info', 'org-server', 'org-password', 'org-summary', 'org-loading', 'org-created'];

    for (const cls of stepsClasses) {
      const block = modalContent.locator(`.${cls}`);
      if (stepClass === cls) {
        await expect(block).toBeVisible();
      } else {
        await expect(block).toBeHidden();
      }
    }
  }

  await ensureCurrentStepIs(modalContent, 'org-name');
  await fillIonInput(modalContent.locator('#org-name-input').locator('ion-input'), 'MyOrg');
  await nextButton.click();

  await ensureCurrentStepIs(modalContent, 'user-info');
  await expect(modalTitle).toHaveText('Enter your personal information');
  await expect(modalSubtitle).toHaveText('Fill in your name and email.');
  await expect(nextButton).toHaveDisabledAttribute();
  await fillIonInput(modalContent.locator('.user-info').locator('ion-input').nth(0), 'Banjo');
  await expect(nextButton).toHaveDisabledAttribute();
  await fillIonInput(modalContent.locator('.user-info').locator('ion-input').nth(1), 'banjo@rare');
  await expect(nextButton).not.toHaveDisabledAttribute();
  await nextButton.click();

  await ensureCurrentStepIs(modalContent, 'org-server');
  await expect(modalTitle).toHaveText('Choose the server you need');
  await expect(modalSubtitle).toHaveText('Choose your desired server type.');
  const serverRadios = modalContent.locator('.choose-server-page').locator('ion-radio');
  await expect(serverRadios.nth(0)).toContainText('Use the main PARSEC server');
  await expect(serverRadios.nth(1)).toContainText('I have my own PARSEC server');
  await expect(serverRadios.nth(0)).toBeChecked();
  await expect(serverRadios.nth(1)).not.toBeChecked();
  await nextButton.click();

  await ensureCurrentStepIs(modalContent, 'org-password');
  await expect(modalTitle).toHaveText('Authentication');
  await expect(modalSubtitle).toHaveText('Lastly, choose an authentication method.');
  await expect(nextButton).toHaveDisabledAttribute();
  const authRadios = modalContent.locator('.choose-auth-page').locator('ion-radio');
  await expect(authRadios.nth(0)).toContainText('Use System Authentication');
  await expect(authRadios.nth(1)).toContainText('Use Password');
  await expect(authRadios.nth(0)).not.toBeChecked();
  await expect(authRadios.nth(0)).toHaveDisabledAttribute();
  await expect(authRadios.nth(1)).toBeChecked();
  await expect(nextButton).toHaveDisabledAttribute();
  const passwordInputs = modalContent.locator('.choose-password').locator('ion-input');
  await fillIonInput(passwordInputs.nth(0), 'AVeryL0ngP@ssw0rd');
  await fillIonInput(passwordInputs.nth(1), 'AVeryL0ngP@ssw0rd');
  await expect(nextButton).toBeEnabled();
  await nextButton.click();

  await ensureCurrentStepIs(modalContent, 'org-summary');
  await expect(modalTitle).toHaveText('Overview of your organization');
  await expect(modalSubtitle).toHaveText('Verify that the information is correct.');

  const summaryItems = modalContent.locator('.summary-list').getByRole('listitem');
  await expect(summaryItems).toHaveCount(5);
  await expect(summaryItems.locator('.summary-item__label')).toHaveText([
    'Organization',
    'Full name',
    'Email',
    'Server choice',
    'Authentication method',
  ]);
  await expect(summaryItems.locator('.summary-item__text')).toHaveText(['MyOrg', 'Banjo', 'banjo@rare', 'Parsec SaaS', 'Password']);

  await nextButton.click();
  await ensureCurrentStepIs(modalContent, 'org-loading');

  await ensureCurrentStepIs(modalContent, 'org-created');
  await expect(modalTitle).toHaveText('Your organization has been created!');
  await expect(createOrgModal.locator('.org-created')).toHaveText('You can access your new organization');
  await nextButton.click();
  await expect(createOrgModal).toBeHidden();
});

createOrgTest('Invalid org name', async ({ createOrgModal }) => {
  const nextButton = createOrgModal.locator('#next-button');
  const modalContent = createOrgModal.locator('.modal-content');
  const nameInput = modalContent.locator('#org-name-input').locator('ion-input');
  const errorForm = modalContent.locator('#org-name-input').locator('.form-error');

  await expect(createOrgModal.locator('.modal-header__title')).toHaveText('Create an organization');
  await expect(createOrgModal.locator('.modal-header__text')).toHaveText('Choose your organization name.');
  await expect(modalContent.locator('.org-name-criteria')).toHaveText(
    'Letters, digits, underscores and hyphens only. No spaces. Limited to 32 characters.',
  );
  await expect(nextButton).toHaveText('Next');
  await expect(nextButton).toHaveDisabledAttribute();

  await expect(errorForm).toBeHidden();
  await fillIonInput(nameInput, 'My Organization');
  await expect(errorForm).toBeVisible();
  await expect(errorForm).toHaveText('Only letters, digits, underscores and hyphens. No spaces.');
  await expect(nextButton).toHaveDisabledAttribute();

  await fillIonInput(nameInput, '');
  await expect(errorForm).toBeHidden();
  await fillIonInput(nameInput, 'MyOrganization');
  await expect(errorForm).toBeHidden();
  await expect(nextButton).not.toHaveDisabledAttribute();
  await expect(nextButton).toBeEnabled();
});

createOrgTest('Invalid custom server', async ({ createOrgModal }) => {
  const nextButton = createOrgModal.locator('#next-button');
  const modalContent = createOrgModal.locator('.modal-content');

  await fillIonInput(modalContent.locator('#org-name-input').locator('ion-input'), 'MyOrganization');
  await nextButton.click();
  await fillIonInput(modalContent.locator('.user-info').locator('ion-input').nth(0), 'Banjo');
  await fillIonInput(modalContent.locator('.user-info').locator('ion-input').nth(1), 'banjo@rare');
  await nextButton.click();

  const serverRadios = modalContent.locator('.choose-server-page').locator('ion-radio');
  const serverInput = modalContent.locator('.choose-server-page').locator('ion-input');
  const formError = modalContent.locator('.choose-server-page').locator('.input-container').locator('.form-error');

  await expect(serverRadios.nth(0)).toContainText('Use the main PARSEC server');
  await expect(serverRadios.nth(1)).toContainText('I have my own PARSEC server');
  await expect(serverRadios.nth(0)).toBeChecked();
  await expect(serverRadios.nth(1)).not.toBeChecked();
  await expect(serverInput).toBeHidden();

  await serverRadios.nth(1).click();
  await expect(serverRadios.nth(1)).toBeChecked();
  await expect(serverRadios.nth(0)).not.toBeChecked();
  await expect(serverInput).toBeVisible();
  await expect(formError).toBeHidden();

  await fillIonInput(serverInput, 'https://lightning-jaguar:6777');
  await expect(nextButton).toHaveDisabledAttribute();
  await expect(formError).toBeVisible();
  await expect(formError).toHaveText("Link should start with 'parsec3://'.");

  await fillIonInput(serverInput, '');
  await expect(nextButton).toHaveDisabledAttribute();
  await expect(formError).toBeHidden();

  await fillIonInput(serverInput, 'parsec3://lightning-jaguar:6777');
  await expect(nextButton).not.toHaveDisabledAttribute();
});

// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator } from '@playwright/test';
import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';
import { fillInputModal, fillIonInput } from '@tests/pw/helpers/utils';

// cspell:disable-next-line
const BOOTSTRAP_LINK = 'parsec3://localhost:6770/MyOrg?no_ssl=true&a=bootstrap_organization&p=xBCy2YVGB31DPzcxGZbGVUt7';

enum Steps {
  OrgName = 'org-name',
  UserInfo = 'user-info',
  Server = 'org-server',
  Authentication = 'org-password',
  Summary = 'org-summary',
  Creation = 'org-loading',
  Created = 'org-created',
}

const TITLES = new Map([
  [Steps.OrgName, { title: 'Create an organization', subtitle: 'Choose your organization name.' }],
  [Steps.UserInfo, { title: 'Enter your personal information', subtitle: 'Fill in your name and email.' }],
  [Steps.Server, { title: 'Choose the server you need', subtitle: 'Choose your desired server type.' }],
  [Steps.Authentication, { title: 'Authentication', subtitle: 'Lastly, choose an authentication method.' }],
  [Steps.Summary, { title: 'Overview of your organization', subtitle: 'Verify that the information is correct.' }],
  [Steps.Creation, {}],
  [Steps.Created, { title: 'Your organization has been created!' }],
]);

async function ensureCurrentStepIs(modal: Locator, step: Steps): Promise<void> {
  const expectedTitle = (TITLES.get(step) as { title?: string; subtitle?: string }).title;
  const expectedSubtitle = (TITLES.get(step) as { title?: string; subtitle?: string }).subtitle;

  if (expectedTitle) {
    const modalTitle = modal.locator('.modal-header__title');
    await expect(modalTitle).toHaveText(expectedTitle);
  }
  if (expectedSubtitle) {
    const modalSubtitle = modal.locator('.modal-header__text');
    await expect(modalSubtitle).toHaveText(expectedSubtitle);
  }

  for (const stepCls in Steps) {
    const block = modal.locator('.modal-content').locator(`.${stepCls}`);
    if (stepCls === step) {
      await expect(block).toBeVisible();
    } else {
      await expect(block).toBeHidden();
    }
  }
}

msTest('Opens the create organization modal', async ({ home }) => {
  await home.locator('#create-organization-button').click();
  await expect(home.locator('.homepage-popover')).toBeVisible();
  await expect(home.locator('.create-organization-modal')).toBeHidden();
  await home.locator('.homepage-popover').getByRole('listitem').nth(1).click();
  await expect(home.locator('.text-input-modal')).toBeVisible();
  await fillInputModal(home, BOOTSTRAP_LINK);
  const modal = home.locator('.create-organization-modal');
  await expect(modal).toBeVisible();
});

msTest('Go through the organization bootstrapping process', async ({ home }) => {
  await home.locator('#create-organization-button').click();
  await expect(home.locator('.homepage-popover')).toBeVisible();
  await expect(home.locator('.create-organization-modal')).toBeHidden();
  await home.locator('.homepage-popover').getByRole('listitem').nth(1).click();
  await fillInputModal(home, BOOTSTRAP_LINK);

  const createOrgModal = home.locator('.create-organization-modal');
  const nextButton = createOrgModal.locator('#next-button');
  const modalContent = createOrgModal.locator('.modal-content');

  await ensureCurrentStepIs(createOrgModal, Steps.OrgName);
  await expect(modalContent.locator('#org-name-input').locator('ion-input').locator('input')).toBeDisabled();
  await expect(modalContent.locator('#org-name-input').locator('ion-input').locator('input')).toHaveValue('MyOrg');
  await expect(nextButton).not.toHaveDisabledAttribute();
  await nextButton.click();

  await ensureCurrentStepIs(createOrgModal, Steps.UserInfo);
  await expect(nextButton).toHaveDisabledAttribute();
  await fillIonInput(modalContent.locator('.user-info').locator('ion-input').nth(0), 'Banjo');
  await expect(nextButton).toHaveDisabledAttribute();
  await fillIonInput(modalContent.locator('.user-info').locator('ion-input').nth(1), 'banjo@rare');
  await expect(nextButton).not.toHaveDisabledAttribute();
  await nextButton.click();

  await ensureCurrentStepIs(createOrgModal, Steps.Authentication);
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

  await ensureCurrentStepIs(createOrgModal, Steps.Summary);
  const summaryItems = modalContent.locator('.summary-list').getByRole('listitem');
  await expect(summaryItems).toHaveCount(5);

  await expect(summaryItems.locator('.summary-item__label')).toHaveText([
    'Organization',
    'Full name',
    'Email',
    'Server choice',
    'Authentication method',
  ]);
  await expect(summaryItems.locator('.summary-item__text')).toHaveText(['MyOrg', 'Banjo', 'banjo@rare', 'localhost', 'Password']);

  await expect(summaryItems.nth(0).locator('.summary-item__button')).toBeHidden();
  await expect(summaryItems.nth(1).locator('.summary-item__button')).toBeVisible();
  await expect(summaryItems.nth(2).locator('.summary-item__button')).toBeVisible();
  await expect(summaryItems.nth(3).locator('.summary-item__button')).toBeHidden();
  await expect(summaryItems.nth(4).locator('.summary-item__button')).toBeVisible();

  await nextButton.click();
  await ensureCurrentStepIs(createOrgModal, Steps.Creation);

  await ensureCurrentStepIs(createOrgModal, Steps.Created);
  await nextButton.click();
  await expect(createOrgModal).toBeHidden();
});

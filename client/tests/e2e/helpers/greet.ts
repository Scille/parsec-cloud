// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page } from '@playwright/test';
import {
  DisplaySize,
  expect,
  fillInputModal,
  fillIonInput,
  getClipboardText,
  MsPage,
  setWriteClipboardPermission,
} from '@tests/e2e/helpers';

interface GreetUserModalData {
  modal: Locator;
  nextButton: Locator;
  title: Locator;
  subtitle: Locator;
  content: Locator;
  closeButton: Locator;
}

export async function initGreetUserModals(
  hostPage: Page,
  guestPage: Page,
  email: string,
  displaySize?: DisplaySize,
): Promise<[GreetUserModalData, GreetUserModalData]> {
  // Invite a new user and retrieve the invitation link
  if (displaySize === DisplaySize.Small) {
    await hostPage.locator('.tab-bar-menu').locator('#add-menu-fab-button').click();
    await hostPage.locator('.list-group-item').nth(0).click();
  } else {
    await hostPage.locator('#activate-users-ms-action-bar').locator('#button-invite-user').click();
  }
  await fillInputModal(hostPage, email);
  await expect(hostPage).toShowToast(`An invitation to join the organization has been sent to ${email}.`, 'Success');
  await hostPage.locator('.topbar').locator('#invitations-button').click();
  const popover = hostPage.locator(displaySize === DisplaySize.Small ? '.invitations-list-modal' : '.invitations-list-popover');
  await setWriteClipboardPermission(hostPage.context(), true);

  const inv = popover.locator('.invitation-list-item').nth(1);
  await inv.hover();
  await inv.locator('.copy-link').click();
  await expect(hostPage).toShowToast('Invitation link has been copied to clipboard.', 'Info');
  const invitationLink = await getClipboardText(hostPage);

  // Use the invitation link in the second tab
  await guestPage.locator('#create-organization-button').click();
  if (displaySize === DisplaySize.Small) {
    await expect(guestPage.locator('.create-join-modal')).toBeVisible();
    await guestPage.locator('.create-join-modal').getByRole('listitem').nth(1).click();
  } else {
    await expect(guestPage.locator('.homepage-popover')).toBeVisible();
    await guestPage.locator('.homepage-popover').getByRole('listitem').nth(1).click();
  }
  await fillInputModal(guestPage, invitationLink);
  const joinModal = guestPage.locator('.join-organization-modal');
  await expect(joinModal).toBeVisible();

  // Start the greet
  await inv.locator('.invitation-actions-buttons').locator('ion-button').nth(1).click();

  const greetModal = hostPage.locator('.greet-organization-modal');

  const greetData = {
    modal: greetModal,
    nextButton: greetModal.locator('#next-button'),
    title: greetModal.locator('.modal-header__title'),
    subtitle: greetModal.locator('.modal-header__text'),
    content: greetModal.locator('.modal-content'),
    closeButton: greetModal.locator('.closeBtn'),
  };
  await expect(greetData.nextButton).toHaveText('Start');
  await expect(greetData.title).toHaveText('Onboard a new user');
  await greetData.nextButton.click();

  const joinData = {
    modal: joinModal,
    nextButton: joinModal.locator('#next-button'),
    title: joinModal.locator('.modal-header__title'),
    subtitle: joinModal.locator('.modal-header__text'),
    content: joinModal.locator('.modal-content'),
    closeButton: joinModal.locator('.closeBtn'),
  };

  // Start the join
  await expect(joinData.nextButton).toHaveText('Continue with Alicey McAliceFace');
  await joinData.nextButton.click();

  return [greetData, joinData];
}

export async function addUser(
  usersPage: MsPage,
  secondPage: MsPage,
  userName: string,
  email: string,
  profile: 'admin' | 'standard' | 'external',
): Promise<void> {
  const currentUserCount = await usersPage.locator('#users-page-user-list').getByRole('listitem').count();

  const [greetData, joinData] = await initGreetUserModals(usersPage, secondPage, email);

  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  await expect(joinData.content.locator('.button-choice')).toHaveCount(4);
  await joinData.content.locator('.button-choice', { hasText: greetCode }).click();

  const joinCode = (await joinData.content.locator('.guest-code').locator('.code').textContent()) ?? '';
  expect(joinCode).toMatch(/^[A-Z0-9]{4}$/);

  await expect(greetData.content.locator('.button-choice')).toHaveCount(4);
  await greetData.content.locator('.button-choice', { hasText: joinCode }).click();

  await expect(greetData.nextButton).toBeHidden();

  await fillIonInput(joinData.content.locator('#get-user-info').locator('ion-input').nth(0), userName);
  await expect(joinData.content.locator('#get-user-info').locator('ion-input').nth(1).locator('input')).toHaveValue(email);
  await joinData.nextButton.click();
  await expect(greetData.nextButton).toBeVisible();

  await expect(greetData.content.locator('.user-info-page').locator('ion-input').nth(0).locator('input')).toHaveValue(userName);
  await expect(greetData.content.locator('.user-info-page').locator('ion-input').nth(1)).toHaveTheClass('input-disabled');
  await expect(greetData.content.locator('.user-info-page').locator('ion-input').nth(1).locator('input')).toHaveValue(email);
  const profileButton = greetData.content.locator('.user-info-page').locator('.filter-button');
  await profileButton.click();
  const profileDropdown = usersPage.locator('.dropdown-popover');
  await expect(profileDropdown.getByRole('listitem').locator('.option-text__label')).toHaveText(['Administrator', 'Member', 'External']);
  await profileDropdown
    .getByRole('listitem')
    .nth(['admin', 'standard', 'external'].findIndex((p) => p === profile))
    .click();
  await greetData.nextButton.click();

  // host is done
  await expect(greetData.title).toHaveText('User has been added successfully!');
  await expect(greetData.nextButton).not.toHaveDisabledAttribute();
  await greetData.nextButton.click();
  await expect(greetData.modal).toBeHidden();
  await expect(usersPage).toShowToast(`${userName} can now access to the organization.`, 'Success');
  await expect(usersPage.locator('#users-page-user-list').getByRole('listitem')).toHaveCount(currentUserCount + 1);

  // Joiner sets password
  const passwordChoice = joinData.content.locator('#get-password').locator('.choose-password');
  await passwordChoice.scrollIntoViewIfNeeded();
  await fillIonInput(passwordChoice.locator('ion-input').nth(0), 'AVeryL0ngP@ssw0rd');
  await expect(joinData.nextButton).toHaveDisabledAttribute();
  await fillIonInput(passwordChoice.locator('ion-input').nth(1), 'AVeryL0ngP@ssw0rd');
  await joinData.nextButton.scrollIntoViewIfNeeded();
  await expect(joinData.nextButton).not.toHaveDisabledAttribute();
  await joinData.nextButton.click();
  await expect(joinData.title).toHaveText(/^You have joined the organization Org\d+!$/);
}

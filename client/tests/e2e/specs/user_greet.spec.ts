// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, fillInputModal, fillIonInput, getClipboardText, msTest, setWriteClipboardPermission } from '@tests/e2e/helpers';

msTest('Greet user process', async ({ usersPage, secondTab }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(240_000);

  // Invite a new user and retrieve the invitation link
  await usersPage.locator('#activate-users-ms-action-bar').locator('#button-invite-user').click();
  // cspell:disable-next-line
  await fillInputModal(usersPage, 'gordon.freeman@blackmesa.nm');
  // cspell:disable-next-line
  await expect(usersPage).toShowToast('An invitation to join the organization has been sent to gordon.freeman@blackmesa.nm.', 'Success');
  await usersPage.locator('.topbar').locator('#invitations-button').click();
  const popover = usersPage.locator('.invitations-list-popover');
  await setWriteClipboardPermission(usersPage.context(), true);
  const inv = popover.locator('.invitation-list-item').nth(1);
  await inv.hover();
  await inv.locator('.copy-link').click();
  await expect(usersPage).toShowToast('Invitation link has been copied to clipboard.', 'Info');
  const invitationLink = await getClipboardText(usersPage);

  // Use the invitation link in the second tab
  await secondTab.locator('#create-organization-button').click();
  await expect(secondTab.locator('.homepage-popover')).toBeVisible();
  await secondTab.locator('.homepage-popover').getByRole('listitem').nth(1).click();
  await fillInputModal(secondTab, invitationLink);
  const joinModal = secondTab.locator('.join-organization-modal');
  await expect(joinModal).toBeVisible();

  // Start the greet
  await inv.locator('.invitation-actions-buttons').locator('ion-button').nth(1).click();
  const greetModal = usersPage.locator('.greet-organization-modal');
  const greetNextButton = greetModal.locator('#next-button');
  const greetTitle = greetModal.locator('.modal-header__title');
  const greetSubtitle = greetModal.locator('.modal-header__text');
  const greetContent = greetModal.locator('.modal-content');
  await expect(greetNextButton).toHaveText('Start');
  await greetNextButton.click();

  // Start the join
  const joinNextButton = joinModal.locator('#next-button');
  const joinTitle = joinModal.locator('.modal-header__title');
  const joinContent = joinModal.locator('.modal-content');
  await expect(joinNextButton).toHaveText('I understand!');
  await joinNextButton.click();

  // Check the provide code page from the greeter and retrieve the code
  await expect(greetModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 0);
  await expect(greetTitle).toHaveText('Share your code');
  await expect(greetSubtitle).toHaveText('Give the code below to the guest.');
  const greetCode = (await greetContent.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the joiner and select the code
  await expect(joinTitle).toHaveText('Get host code');
  await expect(joinModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 0);
  await expect(joinModal.locator('.button-choice')).toHaveCount(4);
  await joinModal.locator('.button-choice', { hasText: greetCode }).click();

  // Check the provide code page from the joiner and retrieve the code
  await expect(joinTitle).toHaveText('Share your code');
  await expect(joinModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 1);
  const joinCode = (await joinContent.locator('.guest-code').locator('.code').textContent()) ?? '';
  expect(joinCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the greeter and select the code
  await expect(greetModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 1);
  await expect(greetTitle).toHaveText('Get guest code');
  await expect(greetSubtitle).toHaveText('Click on the code given to you by the guest.');
  await expect(greetModal.locator('.button-choice')).toHaveCount(4);
  await greetModal.locator('.button-choice', { hasText: joinCode }).click();

  // Greeter waits for joiner to fill out the information
  await expect(greetTitle).toHaveText('Contact details');
  await expect(greetModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 2);
  await expect(greetNextButton).toBeHidden();

  // Fill out the joiner information
  await expect(joinTitle).toHaveText('Your contact details');
  await expect(joinModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 2);
  await expect(joinNextButton).toHaveDisabledAttribute();
  await fillIonInput(joinModal.locator('#get-user-info').locator('ion-input').nth(0), 'Gordon Freeman');
  await expect(joinModal.locator('#get-user-info').locator('ion-input').nth(1).locator('input')).toHaveValue('gordon.freeman@blackmesa.nm');
  await joinNextButton.click();
  await expect(joinNextButton).toBeHidden();
  await expect(joinModal.locator('.spinner-container')).toBeVisible();
  await expect(joinModal.locator('.spinner-container')).toHaveText('(Waiting for the host)');

  // Greeter reviews the information and chose profile
  await expect(greetTitle).toHaveText('Contact details');
  await expect(greetSubtitle).toHaveText('You can update the user name, device and profile.');
  await expect(greetModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 2);
  await expect(greetNextButton).toBeVisible();
  await expect(greetNextButton).toHaveDisabledAttribute();
  await expect(greetContent.locator('.user-info-page').locator('ion-input').nth(0).locator('input')).toHaveValue('Gordon Freeman');
  await expect(greetContent.locator('.user-info-page').locator('ion-input').nth(1)).toHaveTheClass('input-disabled');
  await expect(greetContent.locator('.user-info-page').locator('ion-input').nth(1).locator('input')).toHaveValue(
    'gordon.freeman@blackmesa.nm',
  );
  const profileButton = greetContent.locator('.user-info-page').locator('.filter-button');
  await expect(profileButton).toHaveText('Choose a profile');
  await profileButton.click();
  const profileDropdown = greetModal.page().locator('.dropdown-popover');
  await expect(profileDropdown.getByRole('listitem').locator('.option-text__label')).toHaveText(['Administrator', 'Member', 'External']);
  await profileDropdown.getByRole('listitem').nth(1).click();
  await expect(profileButton).toHaveText('Member');
  await expect(greetNextButton).not.toHaveDisabledAttribute();
  await expect(greetNextButton).toHaveText('Approve');
  await greetNextButton.click();

  // Joiner chose auth
  await expect(joinTitle).toHaveText('Authentication');
  await expect(joinModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 3);
  await expect(joinNextButton).toHaveText('Join the organization');
  await expect(joinNextButton).toHaveDisabledAttribute();

  // Greeter is done
  await expect(greetTitle).toHaveText('User has been added successfully!');
  await expect(greetNextButton).not.toHaveDisabledAttribute();
  await expect(greetNextButton).toBeVisible();
  await expect(greetContent.locator('.final-step').locator('.person-name')).toHaveText('Gordon Freeman');
  await expect(greetContent.locator('.final-step').locator('.user-info__email').locator('.cell')).toHaveText('gordon.freeman@blackmesa.nm');
  await expect(greetContent.locator('.final-step').locator('.user-info__role').locator('.label-profile')).toHaveText('Member');
  await greetNextButton.click();
  await expect(greetModal.page().locator('.greet-organization-modal')).toBeHidden();
  await expect(greetModal.page()).toShowToast('Gordon Freeman can now access to the organization.', 'Success');
  // Also check that the user list gets updated with the new user

  // Joiner sets password
  const authRadio = joinModal.locator('.choose-auth-page').locator('.radio-list-item');
  await expect(authRadio).toHaveCount(2);
  await expect(authRadio.nth(0)).toHaveTheClass('radio-disabled');
  await expect(authRadio.nth(0).locator('.item-radio__label')).toHaveText('Use System Authentication');
  await expect(authRadio.nth(0).locator('.item-radio__text:visible')).toHaveText('Unavailable on web');
  await expect(authRadio.nth(1)).toHaveText('Use Password');
  const passwordChoice = joinModal.locator('#get-password').locator('.choose-password');
  await passwordChoice.scrollIntoViewIfNeeded();
  await fillIonInput(passwordChoice.locator('ion-input').nth(0), 'AVeryL0ngP@ssw0rd');
  await expect(joinNextButton).toHaveDisabledAttribute();
  await fillIonInput(passwordChoice.locator('ion-input').nth(1), 'AVeryL0ngP@ssw0rd');
  await joinNextButton.scrollIntoViewIfNeeded();
  await expect(joinNextButton).not.toHaveDisabledAttribute();
  await joinNextButton.click();
  await expect(joinTitle).toHaveText(/^You have joined the organization Org\d+!$/);
  await expect(joinNextButton).not.toHaveDisabledAttribute();
  await joinNextButton.click();
  await expect(joinModal).toBeHidden();
  await expect(secondTab).toShowToast('You successfully joined the organization.', 'Success');
  // Automatically logged in
  await expect(secondTab.locator('#connected-header')).toContainText('My workspaces');
  await expect(secondTab).toBeWorkspacePage();
  const profile = secondTab.locator('.topbar').locator('.profile-header');
  await expect(profile.locator('.text-content-name')).toHaveText('Gordon Freeman');
});

// msTest('User greet select invalid SAS code', async ({ userGreetModal }) => {
//   const title = userGreetModal.locator('.modal-header__title');
//   const nextButton = userGreetModal.locator('#next-button');
//   await nextButton.click();

//   await expect(title).toHaveText('Get guest code');
//   const choices = userGreetModal.locator('.modal-content').locator('.code:visible');
//   await choices.nth(0).click();

// eslint-disable-next-line max-len
//   await expect(userGreetModal.page()).toShowToast('You did not select the correct code. Please restart the onboarding process.', 'Error');
//   await expect(title).toHaveText('Onboard a new user');
// });

// msTest('User greet select no SAS code', async ({ userGreetModal }) => {
//   const title = userGreetModal.locator('.modal-header__title');
//   const nextButton = userGreetModal.locator('#next-button');
//   await nextButton.click();

//   await expect(title).toHaveText('Get guest code');
//   await userGreetModal.locator('.modal-content').locator('.button-none').click();

//   await expect(userGreetModal.page()).toShowToast(
//     'If you did not see the correct code, this could be a sign of a security issue during the onboarding. Please restart the process.',
//     'Error',
//   );
//   await expect(title).toHaveText('Onboard a new user');
// });

// msTest('Close user greet process', async ({ userGreetModal }) => {
//   const title = userGreetModal.locator('.modal-header__title');
//   const nextButton = userGreetModal.locator('#next-button');
//   const closeButton = userGreetModal.locator('.closeBtn');
//   const modalContent = userGreetModal.locator('.modal-content');

//   await nextButton.click();
//   await expect(title).toHaveText('Get guest code');

//   await closeButton.click();
//   await answerQuestion(userGreetModal.page(), false, {
//     expectedTitleText: 'Cancel the onboarding',
//     expectedQuestionText:
//       'Are you sure you want to cancel the onboarding process? Information will not be saved, you will have to restart.',
//     expectedPositiveText: 'Cancel process',
//     expectedNegativeText: 'Resume',
//   });

//   await modalContent.locator('.code:visible').nth(1).click();

//   await closeButton.click();
//   await answerQuestion(userGreetModal.page(), false, {
//     expectedTitleText: 'Cancel the onboarding',
//     expectedQuestionText:
//       'Are you sure you want to cancel the onboarding process? Information will not be saved, you will have to restart.',
//     expectedPositiveText: 'Cancel process',
//     expectedNegativeText: 'Resume',
//   });

//   await expect(title).toHaveText('Contact details');
//   await closeButton.click();
//   await answerQuestion(userGreetModal.page(), false, {
//     expectedTitleText: 'Cancel the onboarding',
//     expectedQuestionText:
//       'Are you sure you want to cancel the onboarding process? Information will not be saved, you will have to restart.',
//     expectedPositiveText: 'Cancel process',
//     expectedNegativeText: 'Resume',
//   });

//   await modalContent.locator('.user-info-page').locator('.filter-button').click();
//   const profileDropdown = userGreetModal.page().locator('.dropdown-popover');
//   await profileDropdown.getByRole('listitem').nth(1).click();
//   await nextButton.click();

//   await expect(closeButton).toBeHidden();
// });

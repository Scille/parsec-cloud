// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page } from '@playwright/test';
import {
  answerQuestion,
  expect,
  fillInputModal,
  fillIonInput,
  getClipboardText,
  msTest,
  setWriteClipboardPermission,
} from '@tests/e2e/helpers';

interface ModalData {
  modal: Locator;
  nextButton: Locator;
  title: Locator;
  subtitle: Locator;
  content: Locator;
  closeButton: Locator;
}

async function initModals(hostPage: Page, guestPage: Page): Promise<[ModalData, ModalData]> {
  // Invite a new user and retrieve the invitation link
  await hostPage.locator('#activate-users-ms-action-bar').locator('#button-invite-user').click();
  // cspell:disable-next-line
  await fillInputModal(hostPage, 'gordon.freeman@blackmesa.nm');
  // cspell:disable-next-line
  await expect(hostPage).toShowToast('An invitation to join the organization has been sent to gordon.freeman@blackmesa.nm.', 'Success');
  await hostPage.locator('.topbar').locator('#invitations-button').click();
  const popover = hostPage.locator('.invitations-list-popover');
  await setWriteClipboardPermission(hostPage.context(), true);
  const inv = popover.locator('.invitation-list-item').nth(1);
  await inv.hover();
  await inv.locator('.copy-link').click();
  await expect(hostPage).toShowToast('Invitation link has been copied to clipboard.', 'Info');
  const invitationLink = await getClipboardText(hostPage);

  // Use the invitation link in the second tab
  await guestPage.locator('#create-organization-button').click();
  await expect(guestPage.locator('.homepage-popover')).toBeVisible();
  await guestPage.locator('.homepage-popover').getByRole('listitem').nth(1).click();
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
  await expect(joinData.nextButton).toHaveText('I understand!');
  await joinData.nextButton.click();

  return [greetData, joinData];
}

msTest('Greet user whole process', async ({ usersPage, secondTab }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const [greetData, joinData] = await initModals(usersPage, secondTab);

  // Check the provide code page from the host and retrieve the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 0);
  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.subtitle).toHaveText('Give the code below to the guest.');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select the code
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 0);
  await expect(joinData.content.locator('.button-choice')).toHaveCount(4);
  await joinData.content.locator('.button-choice', { hasText: greetCode }).click();

  // Check the provide code page from the guest and retrieve the code
  await expect(joinData.title).toHaveText('Share your code');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 1);
  const joinCode = (await joinData.content.locator('.guest-code').locator('.code').textContent()) ?? '';
  expect(joinCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the host and select the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 1);
  await expect(greetData.title).toHaveText('Get guest code');
  await expect(greetData.subtitle).toHaveText('Click on the code given to you by the guest.');
  await expect(greetData.content.locator('.button-choice')).toHaveCount(4);
  await greetData.content.locator('.button-choice', { hasText: joinCode }).click();

  // Host waits for guest to fill out the information
  await expect(greetData.title).toHaveText('Contact details');
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 2);
  await expect(greetData.nextButton).toBeHidden();

  // Fill out the joiner information
  await expect(joinData.title).toHaveText('Your contact details');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 2);
  await expect(joinData.nextButton).toHaveDisabledAttribute();
  await fillIonInput(joinData.content.locator('#get-user-info').locator('ion-input').nth(0), 'Gordon Freeman');
  await expect(joinData.content.locator('#get-user-info').locator('ion-input').nth(1).locator('input')).toHaveValue(
    'gordon.freeman@blackmesa.nm',
  );
  await joinData.nextButton.click();
  await expect(joinData.nextButton).toBeHidden();
  await expect(joinData.modal.locator('.spinner-container')).toBeVisible();
  await expect(joinData.modal.locator('.spinner-container')).toHaveText('(Waiting for the host)');

  // host reviews the information and chose profile
  await expect(greetData.title).toHaveText('Contact details');
  await expect(greetData.subtitle).toHaveText('You can update the user name, device and profile.');
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 2);
  await expect(greetData.nextButton).toBeVisible();
  await expect(greetData.nextButton).toHaveDisabledAttribute();
  await expect(greetData.content.locator('.user-info-page').locator('ion-input').nth(0).locator('input')).toHaveValue('Gordon Freeman');
  await expect(greetData.content.locator('.user-info-page').locator('ion-input').nth(1)).toHaveTheClass('input-disabled');
  await expect(greetData.content.locator('.user-info-page').locator('ion-input').nth(1).locator('input')).toHaveValue(
    'gordon.freeman@blackmesa.nm',
  );
  const profileButton = greetData.content.locator('.user-info-page').locator('.filter-button');
  await expect(profileButton).toHaveText('Choose a profile');
  await profileButton.click();
  const profileDropdown = usersPage.locator('.dropdown-popover');
  await expect(profileDropdown.getByRole('listitem').locator('.option-text__label')).toHaveText(['Administrator', 'Member', 'External']);
  await profileDropdown.getByRole('listitem').nth(1).click();
  await expect(profileButton).toHaveText('Member');
  await expect(greetData.nextButton).not.toHaveDisabledAttribute();
  await expect(greetData.nextButton).toHaveText('Approve');
  await greetData.nextButton.click();

  // Joiner chose auth
  await expect(joinData.title).toHaveText('Authentication');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 3);
  await expect(joinData.nextButton).toHaveText('Join the organization');
  await expect(joinData.nextButton).toHaveDisabledAttribute();

  // host is done
  await expect(greetData.title).toHaveText('User has been added successfully!');
  await expect(greetData.nextButton).not.toHaveDisabledAttribute();
  await expect(greetData.nextButton).toBeVisible();
  await expect(greetData.content.locator('.final-step').locator('.person-name')).toHaveText('Gordon Freeman');
  await expect(greetData.content.locator('.final-step').locator('.user-info__email').locator('.cell')).toHaveText(
    'gordon.freeman@blackmesa.nm',
  );
  await expect(greetData.content.locator('.final-step').locator('.user-info__role').locator('.label-profile')).toHaveText('Member');
  await greetData.nextButton.click();
  await expect(greetData.modal).toBeHidden();
  await expect(usersPage).toShowToast('Gordon Freeman can now access to the organization.', 'Success');
  // Also check that the user list gets updated with the new user

  // Joiner sets password
  const authRadio = joinData.content.locator('.choose-auth-page').locator('.radio-list-item');
  await expect(authRadio).toHaveCount(2);
  await expect(authRadio.nth(0)).toHaveTheClass('radio-disabled');
  await expect(authRadio.nth(0).locator('.item-radio__label')).toHaveText('Use System Authentication');
  await expect(authRadio.nth(0).locator('.item-radio__text:visible')).toHaveText('Unavailable on web');
  await expect(authRadio.nth(1)).toHaveText('Use Password');
  const passwordChoice = joinData.content.locator('#get-password').locator('.choose-password');
  await passwordChoice.scrollIntoViewIfNeeded();
  await fillIonInput(passwordChoice.locator('ion-input').nth(0), 'AVeryL0ngP@ssw0rd');
  await expect(joinData.nextButton).toHaveDisabledAttribute();
  await fillIonInput(passwordChoice.locator('ion-input').nth(1), 'AVeryL0ngP@ssw0rd');
  await joinData.nextButton.scrollIntoViewIfNeeded();
  await expect(joinData.nextButton).not.toHaveDisabledAttribute();
  await joinData.nextButton.click();
  await expect(joinData.title).toHaveText(/^You have joined the organization Org\d+!$/);
  await expect(joinData.nextButton).not.toHaveDisabledAttribute();
  await joinData.nextButton.click();
  await expect(joinData.modal).toBeHidden();
  await expect(secondTab).toShowToast('You successfully joined the organization.', 'Success');
  // Automatically logged in
  await expect(secondTab.locator('#connected-header')).toContainText('My workspaces');
  await expect(secondTab).toBeWorkspacePage();
  const profile = secondTab.locator('.topbar').locator('.profile-header');
  await expect(profile.locator('.text-content-name')).toHaveText('Gordon Freeman');
});

msTest('Host selects invalid SAS code', async ({ usersPage, secondTab }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const [greetData, joinData] = await initModals(usersPage, secondTab);
  // Check the provide code page from the host and retrieve the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 0);
  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.subtitle).toHaveText('Give the code below to the guest.');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select the code
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 0);
  await expect(joinData.content.locator('.button-choice')).toHaveCount(4);
  await joinData.content.locator('.button-choice', { hasText: greetCode }).click();

  // Check the provide code page from the guest and retrieve the code
  await expect(joinData.title).toHaveText('Share your code');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 1);
  const joinCode = (await joinData.content.locator('.guest-code').locator('.code').textContent()) ?? '';
  expect(joinCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the host and select the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 1);
  await expect(greetData.title).toHaveText('Get guest code');
  await expect(greetData.subtitle).toHaveText('Click on the code given to you by the guest.');
  await expect(greetData.content.locator('.button-choice')).toHaveCount(4);
  await greetData.modal.locator('.button-choice').filter({ hasNotText: joinCode }).nth(0).click();

  await expect(usersPage).toShowToast('You did not select the correct code. Please restart the onboarding process.', 'Error');
  await expect(secondTab).toShowToast('The host has selected the wrong code.', 'Error');

  // Back to the beginning
  await expect(greetData.nextButton).toHaveText('Start');
  await expect(greetData.title).toHaveText('Onboard a new user');

  // Back to the beginning
  await expect(joinData.title).toHaveText('Welcome to Parsec!');
  await expect(joinData.nextButton).toHaveText('I understand!');
});

msTest('Host selects no SAS code', async ({ usersPage, secondTab }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const [greetData, joinData] = await initModals(usersPage, secondTab);
  // Check the provide code page from the host and retrieve the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 0);
  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.subtitle).toHaveText('Give the code below to the guest.');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select the code
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 0);
  await expect(joinData.content.locator('.button-choice')).toHaveCount(4);
  await joinData.content.locator('.button-choice', { hasText: greetCode }).click();

  // Check the provide code page from the guest and retrieve the code
  await expect(joinData.title).toHaveText('Share your code');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 1);
  const joinCode = (await joinData.content.locator('.guest-code').locator('.code').textContent()) ?? '';
  expect(joinCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the host and select the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 1);
  await expect(greetData.title).toHaveText('Get guest code');
  await expect(greetData.subtitle).toHaveText('Click on the code given to you by the guest.');
  await expect(greetData.content.locator('.button-none')).toHaveText('None shown');
  await greetData.content.locator('.button-none').click();

  await expect(usersPage).toShowToast(
    'If you did not see the correct code, this could be a sign of a security issue during the onboarding. Please restart the process.',
    'Error',
  );
  await expect(secondTab).toShowToast('The host has cancelled the process.', 'Error');

  // Back to the beginning
  await expect(greetData.nextButton).toHaveText('Start');
  await expect(greetData.title).toHaveText('Onboard a new user');

  // Back to the beginning
  await expect(joinData.title).toHaveText('Welcome to Parsec!');
  await expect(joinData.nextButton).toHaveText('I understand!');
});

msTest('Host closes greet process', async ({ usersPage, secondTab }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const [greetData, joinData] = await initModals(usersPage, secondTab);

  // Check the provide code page from the host and retrieve the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 0);
  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.subtitle).toHaveText('Give the code below to the guest.');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select the code
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 0);
  await expect(joinData.content.locator('.button-choice')).toHaveCount(4);
  await joinData.content.locator('.button-choice', { hasText: greetCode }).click();

  // Check the provide code page from the guest and retrieve the code
  await expect(joinData.title).toHaveText('Share your code');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 1);
  const joinCode = (await joinData.content.locator('.guest-code').locator('.code').textContent()) ?? '';
  expect(joinCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the host and select the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 1);
  await expect(greetData.title).toHaveText('Get guest code');
  await expect(greetData.subtitle).toHaveText('Click on the code given to you by the guest.');

  // Try to close but cancel
  await greetData.closeButton.click();
  await answerQuestion(usersPage, false, {
    expectedTitleText: 'Cancel the onboarding',
    expectedQuestionText:
      'Are you sure you want to cancel the onboarding process? Information will not be saved, you will have to restart.',
    expectedPositiveText: 'Cancel process',
    expectedNegativeText: 'Resume',
  });

  // Now cancel it for real
  await greetData.closeButton.click();
  await answerQuestion(usersPage, true, {
    expectedTitleText: 'Cancel the onboarding',
    expectedQuestionText:
      'Are you sure you want to cancel the onboarding process? Information will not be saved, you will have to restart.',
    expectedPositiveText: 'Cancel process',
    expectedNegativeText: 'Resume',
  });
  await expect(greetData.modal).toBeHidden();

  // Does not seem to work properly on guest side
  // await expect(secondTab).toShowToast('The host has cancelled the process.', 'Error');
  // await expect(joinTitle).toHaveText('Welcome to Parsec!');
  // await expect(joinNextButton).toHaveText('I understand!');
});

msTest('Guest selects invalid SAS code', async ({ usersPage, secondTab }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const [greetData, joinData] = await initModals(usersPage, secondTab);

  // Check the provide code page from the host and retrieve the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 0);
  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.subtitle).toHaveText('Give the code below to the guest.');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select the code
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 0);
  await expect(joinData.content.locator('.button-choice')).toHaveCount(4);
  await joinData.content.locator('.button-choice').filter({ hasNotText: greetCode }).nth(0).click();

  await expect(secondTab).toShowToast('You did not select the correct code. Please restart the onboarding process.', 'Error');
  await expect(usersPage).toShowToast('The invitee has selected the wrong code.', 'Error');

  // Back to the beginning
  await expect(greetData.nextButton).toHaveText('Start');
  await expect(greetData.title).toHaveText('Onboard a new user');

  // Back to the beginning
  await expect(joinData.title).toHaveText('Welcome to Parsec!');
  await expect(joinData.nextButton).toHaveText('I understand!');
});

msTest('Guest selects no SAS code', async ({ usersPage, secondTab }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const [greetData, joinData] = await initModals(usersPage, secondTab);

  // Check the provide code page from the host and retrieve the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 0);
  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.subtitle).toHaveText('Give the code below to the guest.');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select the code
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 0);
  await expect(joinData.content.locator('.button-none')).toHaveText('None shown');
  await joinData.content.locator('.button-none').click();

  await expect(secondTab).toShowToast(
    'If you did not see the correct code, this could be a sign of a security issue during the onboarding. Please restart the process.',
    'Error',
  );
  await expect(usersPage).toShowToast('The invitee has cancelled the process.', 'Error');

  // Back to the beginning
  await expect(greetData.nextButton).toHaveText('Start');
  await expect(greetData.title).toHaveText('Onboard a new user');

  // Back to the beginning
  await expect(joinData.title).toHaveText('Welcome to Parsec!');
  await expect(joinData.nextButton).toHaveText('I understand!');
});

msTest('Guest closes greet process', async ({ usersPage, secondTab }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const [greetData, joinData] = await initModals(usersPage, secondTab);

  // Check the provide code page from the host and retrieve the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 0);
  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.subtitle).toHaveText('Give the code below to the guest.');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select the code
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 0);
  await expect(joinData.content.locator('.button-choice')).toHaveCount(4);
  await joinData.content.locator('.button-choice', { hasText: greetCode }).click();

  // Check the provide code page from the guest and retrieve the code
  await expect(joinData.title).toHaveText('Share your code');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 1);

  // Try to close but cancel
  await joinData.closeButton.click();
  await answerQuestion(secondTab, false, {
    expectedTitleText: 'Cancel the onboarding',
    expectedQuestionText:
      'Are you sure you want to cancel the onboarding process? Information will not be saved, you will have to restart.',
    expectedPositiveText: 'Cancel process',
    expectedNegativeText: 'Resume',
  });

  // Now cancel it for real
  await joinData.closeButton.click();
  await answerQuestion(secondTab, true, {
    expectedTitleText: 'Cancel the onboarding',
    expectedQuestionText:
      'Are you sure you want to cancel the onboarding process? Information will not be saved, you will have to restart.',
    expectedPositiveText: 'Cancel process',
    expectedNegativeText: 'Resume',
  });
  await expect(joinData.modal).toBeHidden();

  // Does not seem to work properly on host side
  // await expect(usersPage).toShowToast('The host has cancelled the process.', 'Error');
  // await expect(greetData.nextButton).toHaveText('Start');
  // await expect(greetData.title).toHaveText('Onboard a new user');
});

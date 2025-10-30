// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, DisplaySize, expect, fillIonInput, initGreetUserModals, msTest } from '@tests/e2e/helpers';

msTest('Greet user whole process in small display', async ({ usersPage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const secondTab = await usersPage.openNewTab();

  await usersPage.setDisplaySize(DisplaySize.Small);
  await secondTab.setDisplaySize(DisplaySize.Small);

  const [greetData, joinData] = await initGreetUserModals(usersPage, secondTab, 'gordon.freeman@blackmesa.nm');

  // Check the provide code page from the host and retrieve the code
  await expect(greetData.title).toHaveText('Share your code');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select the code
  await expect(secondTab.locator('.modal-header__step')).toHaveText('Step 1 of 4');
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.content.locator('.button-choice')).toHaveCount(4);
  await joinData.content.locator('.button-choice', { hasText: greetCode }).click();

  // Check the provide code page from the guest and retrieve the code
  await expect(usersPage.locator('.modal-header__step')).toHaveText('Step 1 of 4');
  await expect(joinData.title).toHaveText('Share your code');
  const joinCode = (await joinData.content.locator('.guest-code').locator('.code').textContent()) ?? '';
  expect(joinCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the host and select the code
  await expect(usersPage.locator('.modal-header__step')).toHaveText('Step 2 of 4');
  await expect(greetData.title).toHaveText('Get guest code');
  await expect(greetData.content.locator('.button-choice')).toHaveCount(4);
  await greetData.content.locator('.button-choice', { hasText: joinCode }).click();

  // Host waits for guest to fill out the information
  await expect(usersPage.locator('.modal-header__step')).toHaveText('Step 3 of 4');
  await expect(greetData.title).toHaveText('Contact details');
  await expect(greetData.nextButton).toBeHidden();

  // Fill out the joiner information
  await expect(secondTab.locator('.modal-header__step')).toHaveText('Step 3 of 4');
  await expect(joinData.title).toHaveText('Your contact details');
  await expect(joinData.nextButton).toHaveDisabledAttribute();
  await fillIonInput(joinData.content.locator('#get-user-info').locator('ion-input').nth(0), 'Gordon Freeman');
  await expect(joinData.content.locator('#get-user-info').locator('ion-input').nth(1).locator('input')).toHaveValue(
    'gordon.freeman@blackmesa.nm',
  );
  await joinData.nextButton.click();
  await expect(joinData.nextButton).toBeHidden();
  await expect(joinData.modal.locator('.spinner-container')).toBeVisible();
  await expect(joinData.modal.locator('.spinner-container')).toHaveText('(Waiting for the administrator)');

  // host reviews the information and chose profile
  await expect(greetData.title).toHaveText('Contact details');
  await expect(greetData.nextButton).toBeVisible();
  await expect(greetData.nextButton).toHaveDisabledAttribute();
  await expect(greetData.content.locator('.user-info-page').locator('ion-input').nth(0).locator('input')).toHaveValue('Gordon Freeman');
  await expect(greetData.content.locator('.user-info-page').locator('ion-input').nth(1)).toHaveTheClass('input-disabled');
  await expect(greetData.content.locator('.user-info-page').locator('ion-input').nth(1).locator('input')).toHaveValue(
    'gordon.freeman@blackmesa.nm',
  );
  const profileButton = greetData.content.locator('.user-info-page').locator('.dropdown-button');
  await expect(profileButton).toHaveText('Choose a profile');
  await profileButton.click();
  const profileDropdown = usersPage.locator('.sheet-modal');
  await expect(profileDropdown.getByRole('listitem').locator('.option-text__label')).toHaveText(['Administrator', 'Member', 'External']);
  await profileDropdown.getByRole('listitem').nth(1).click();
  await profileDropdown.locator('.buttons').locator('ion-button').nth(1).click();
  await expect(profileButton).toHaveText('Member');
  await expect(greetData.nextButton).toNotHaveDisabledAttribute();
  await expect(greetData.nextButton).toHaveText('Approve');
  await greetData.nextButton.click();

  // Joiner chose auth
  await expect(secondTab.locator('.modal-header__step')).toHaveText('Step 4 of 4');
  await expect(joinData.title).toHaveText('Authentication');
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
  await expect(usersPage).toShowToast('Gordon Freeman can now access the organization.', 'Success');
  await expect(usersPage.locator('#users-page-user-list').getByRole('listitem').locator('.user-mobile-text__name')).toHaveText([
    'Alicey McAliceFace',
    'Boby McBobFace',
    'Gordon Freeman',
    'Malloryy McMalloryFace',
  ]);

  // Joiner sets password
  const authRadio = joinData.content.locator('.choose-auth-page').locator('.radio-list-item:visible');
  await expect(authRadio).toHaveCount(2);
  await expect(authRadio.nth(0)).toHaveTheClass('radio-disabled');
  await expect(authRadio.nth(0).locator('.authentication-card-text__title')).toHaveText('System authentication');
  await expect(authRadio.nth(1)).toHaveText('Password');
  await authRadio.nth(1).click();
  const passwordChoice = joinData.content.locator('#get-password').locator('.choose-password');
  await passwordChoice.scrollIntoViewIfNeeded();
  await fillIonInput(passwordChoice.locator('ion-input').nth(0), 'AVeryL0ngP@ssw0rd');
  await expect(joinData.nextButton).toHaveDisabledAttribute();
  await fillIonInput(passwordChoice.locator('ion-input').nth(1), 'AVeryL0ngP@ssw0rd');
  await joinData.nextButton.scrollIntoViewIfNeeded();
  await expect(joinData.nextButton).not.toHaveDisabledAttribute();
  await joinData.nextButton.click();
  await expect(joinData.nextButton).not.toHaveDisabledAttribute();
  await joinData.nextButton.click();
  await expect(joinData.modal).toBeHidden();
  await expect(secondTab).toShowToast('You successfully joined the organization.', 'Success');
  // Automatically logged in
  await expect(secondTab.locator('#connected-header')).toContainText('My workspaces');
  await expect(secondTab).toBeWorkspacePage();
});

msTest('Greet user whole process in large display', async ({ usersPage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const secondTab = await usersPage.openNewTab();

  const [greetData, joinData] = await initGreetUserModals(usersPage, secondTab, 'gordon.freeman@blackmesa.nm');

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
  await expect(joinData.modal.locator('.spinner-container')).toHaveText('(Waiting for the administrator)');

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
  const profileButton = greetData.content.locator('.user-info-page').locator('.dropdown-button');
  await expect(profileButton).toHaveText('Choose a profile');
  await profileButton.click();
  const profileDropdown = usersPage.locator('.dropdown-popover');
  await expect(profileDropdown.getByRole('listitem').locator('.option-text__label')).toHaveText(['Administrator', 'Member', 'External']);
  await profileDropdown.getByRole('listitem').nth(1).click();
  await expect(profileButton).toHaveText('Member');
  await expect(greetData.nextButton).toNotHaveDisabledAttribute();
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
  await expect(usersPage).toShowToast('Gordon Freeman can now access the organization.', 'Success');
  await expect(usersPage.locator('#users-page-user-list').getByRole('listitem').locator('.person-name')).toHaveText([
    'Alicey McAliceFace',
    'Boby McBobFace',
    'Gordon Freeman',
    'Malloryy McMalloryFace',
  ]);

  // Joiner sets password
  const authRadio = joinData.content.locator('.choose-auth-page').locator('.radio-list-item:visible');
  await expect(authRadio).toHaveCount(2);
  await expect(authRadio.nth(0)).toHaveTheClass('radio-disabled');
  await expect(authRadio.nth(0).locator('.authentication-card-text__title')).toHaveText('System authentication');
  await expect(authRadio.nth(1)).toHaveText('Password');
  await authRadio.nth(1).click();
  const passwordChoice = joinData.content.locator('#get-password').locator('.choose-password');
  await passwordChoice.scrollIntoViewIfNeeded();
  await fillIonInput(passwordChoice.locator('ion-input').nth(0), 'AVeryL0ngP@ssw0rd');
  await expect(joinData.nextButton).toHaveDisabledAttribute();
  await fillIonInput(passwordChoice.locator('ion-input').nth(1), 'AVeryL0ngP@ssw0rd');
  await joinData.nextButton.scrollIntoViewIfNeeded();
  await expect(joinData.nextButton).not.toHaveDisabledAttribute();
  await joinData.nextButton.click();
  await expect(joinData.title).toHaveText(/^You have joined the organization TestbedOrg\d+!$/);
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

msTest('Host selects invalid SAS code', async ({ usersPage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const guestPage = await usersPage.openNewTab();
  const [greetData, joinData] = await initGreetUserModals(usersPage, guestPage, 'gordon.freeman@blackmesa.nm');
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

  await expect(guestPage).toShowToast('The host has selected the wrong code.', 'Error');
  await expect(usersPage).toShowToast('You did not select the correct code. Please restart the onboarding process.', 'Error');

  // Back to the beginning
  await expect(greetData.nextButton).toHaveText('Start');
  await expect(greetData.title).toHaveText('Onboard a new user');

  // Back to the beginning
  await expect(joinData.title).toHaveText('Welcome to Parsec!');
  await expect(joinData.nextButton).toHaveText('Continue with Alicey McAliceFace');
});

msTest('Host selects no SAS code', async ({ usersPage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const secondTab = await usersPage.openNewTab();

  const [greetData, joinData] = await initGreetUserModals(usersPage, secondTab, 'gordon.freeman@blackmesa.nm');
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
  await expect(greetData.content.locator('#noneChoicesButton')).toHaveText("Can't find the code?");
  await greetData.content.locator('#noneChoicesButton').click();

  const questionModal = usersPage.locator('.question-modal');
  await expect(questionModal).toBeVisible();
  await expect(questionModal.locator('.ms-modal-header__title')).toHaveText('No matching code');
  await expect(questionModal.locator('.ms-modal-header__text')).toHaveText(
    "If you can't find the matching code, quit and start the process over. If the problem persists, please contact your administrator.",
  );
  await questionModal.locator('#cancel-button').click();
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 1);
  await greetData.content.locator('#noneChoicesButton').click();
  await questionModal.locator('#next-button').click();

  await expect(secondTab).toShowToast('The host has cancelled the process.', 'Error');

  await expect(usersPage).toShowToast(
    'If you did not see the correct code, this could be a sign of a security issue during the onboarding. Please restart the process.',
    'Error',
  );

  // Back to the beginning
  await expect(greetData.nextButton).toHaveText('Start');
  await expect(greetData.title).toHaveText('Onboard a new user');

  // Back to the beginning
  await expect(joinData.title).toHaveText('Welcome to Parsec!');
  await expect(joinData.nextButton).toHaveText('Continue with Alicey McAliceFace');
});

msTest('Host closes greet process', async ({ usersPage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const secondTab = await usersPage.openNewTab();
  const [greetData, joinData] = await initGreetUserModals(usersPage, secondTab, 'gordon.freeman@blackmesa.nm');

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
  // await expect(joinTitle).toHaveText('Welcome to Parsec');
  // await expect(joinNextButton).toHaveText('I understand!');
});

msTest('Guest selects invalid SAS code', async ({ usersPage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const secondTab = await usersPage.openNewTab();
  const [greetData, joinData] = await initGreetUserModals(usersPage, secondTab, 'gordon.freeman@blackmesa.nm');

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
  await expect(joinData.nextButton).toHaveText('Continue with Alicey McAliceFace');
});

msTest('Guest selects no SAS code', async ({ usersPage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const secondTab = await usersPage.openNewTab();
  const [greetData, joinData] = await initGreetUserModals(usersPage, secondTab, 'gordon.freeman@blackmesa.nm');

  // Check the provide code page from the host and retrieve the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 0);
  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.subtitle).toHaveText('Give the code below to the guest.');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select the code
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 0);
  await expect(joinData.content.locator('#noneChoicesButton')).toHaveText("Can't find the code?");
  await joinData.content.locator('#noneChoicesButton').click();

  const questionModal = secondTab.locator('.question-modal');
  await expect(questionModal).toBeVisible();
  await expect(questionModal.locator('.ms-modal-header__title')).toHaveText('No matching code');
  await expect(questionModal.locator('.ms-modal-header__text')).toHaveText(
    "If you can't find the matching code, quit and start the process over. If the problem persists, please contact your administrator.",
  );
  await questionModal.locator('#cancel-button').click();
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 0);
  await joinData.content.locator('#noneChoicesButton').click();
  await questionModal.locator('#next-button').click();

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
  await expect(joinData.nextButton).toHaveText('Continue with Alicey McAliceFace');
});

msTest('Guest closes greet process', async ({ usersPage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const secondTab = await usersPage.openNewTab();
  const [greetData, joinData] = await initGreetUserModals(usersPage, secondTab, 'gordon.freeman@blackmesa.nm');

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

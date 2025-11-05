// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator } from '@playwright/test';
import {
  answerQuestion,
  DisplaySize,
  expect,
  fillInputModal,
  fillIonInput,
  getClipboardText,
  MsPage,
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

async function initModals(hostPage: MsPage, guestPage: MsPage): Promise<[ModalData, ModalData]> {
  const displaySize = await hostPage.getDisplaySize();
  if (displaySize === DisplaySize.Small) {
    await hostPage.locator('.header-selected-item__back').click();
  }
  await expect(hostPage.locator('.menu-list__item').nth(1)).toHaveText('My devices');
  await hostPage.locator('.menu-list__item').nth(1).click();
  // Invite a new user and retrieve the invitation link
  await hostPage.locator('.profile-content-item').locator('#add-device-button').click();
  const greetModal = hostPage.locator('.greet-organization-modal');
  await expect(greetModal.locator('.modal-header__title')).toHaveText('Create a new device');
  await expect(greetModal.locator('.first-step').locator('.container-textinfo__text').nth(0)).toHaveText(
    'Parsec must be open on both devices during the onboarding process.',
  );
  await expect(greetModal.locator('.first-step').locator('.container-textinfo__text').nth(1)).toHaveText(
    'Before you start, make sure you are using the latest version of Parsec on both devices.',
  );
  await expect(greetModal.locator('#next-button')).toHaveText('Start');
  await expect(greetModal.locator('.closeBtn')).toBeVisible();

  await setWriteClipboardPermission(hostPage.context(), true);

  if (displaySize === DisplaySize.Small) {
    await greetModal.locator('#copy-link-btn-small').click();
  } else {
    await greetModal.locator('#copy-link-btn').click();
  }
  await expect(hostPage).toShowToast('Invitation link has been copied to clipboard.', 'Info');
  const invitationLink = await getClipboardText(hostPage);

  // Use the invitation link in the second tab
  await guestPage.locator('#create-organization-button').click();

  if (displaySize === DisplaySize.Small) {
    await expect(guestPage.locator('.create-join-modal')).toBeVisible();
    await guestPage.locator('.create-join-modal-list__item').nth(1).click();
  } else {
    await expect(guestPage.locator('.homepage-popover')).toBeVisible();
    await guestPage.locator('.homepage-popover').getByRole('listitem').nth(1).click();
  }
  await fillInputModal(guestPage, invitationLink);
  const joinModal = guestPage.locator('.join-organization-modal');
  await expect(joinModal).toBeVisible();

  const greetData = {
    modal: greetModal,
    nextButton: greetModal.locator('#next-button'),
    title: greetModal.locator('.modal-header__title'),
    subtitle: greetModal.locator('.modal-header__text'),
    content: greetModal.locator('.modal-content'),
    closeButton: greetModal.locator('.closeBtn'),
  };
  // Start the greet
  await expect(greetData.title).toHaveText('Create a new device');
  await expect(greetData.nextButton).toHaveText('Start');
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
  await expect(joinData.title).toHaveText('Add a new device');
  await expect(joinData.nextButton).toHaveText('I understand!');
  await joinData.nextButton.click();

  return [greetData, joinData];
}

msTest('Greet device whole process on small display', async ({ myProfilePage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const secondTab = await myProfilePage.openNewTab();

  await myProfilePage.setDisplaySize(DisplaySize.Small);
  await secondTab.setDisplaySize(DisplaySize.Small);

  const [greetData, joinData] = await initModals(myProfilePage, secondTab);

  await expect(greetData.modal.locator('.modal-header-stepper__text')).toHaveText('Greet a new device');
  await expect(joinData.modal.locator('.modal-header-stepper__text')).toHaveText('Add a new device');

  // Check the provide code page from the host and retrieve the code

  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.modal.locator('.modal-header__step')).toHaveText('Step 1 of 3');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select the code
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.modal.locator('.modal-header__step')).toHaveText('Step 1 of 3');
  await expect(joinData.content.locator('.button-choice')).toHaveCount(4);
  await joinData.content.locator('.button-choice', { hasText: greetCode }).click();

  // Check the provide code page from the guest and retrieve the code
  await expect(joinData.title).toHaveText('Share guest code');
  await expect(joinData.modal.locator('.modal-header__step')).toHaveText('Step 2 of 3');
  const joinCode = (await joinData.content.locator('.guest-code').locator('.code').textContent()) ?? '';
  expect(joinCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the host and select the code
  await expect(greetData.title).toHaveText('Get guest code');
  await expect(greetData.modal.locator('.modal-header__step')).toHaveText('Step 2 of 3');

  await expect(greetData.content.locator('.button-choice')).toHaveCount(4);
  await greetData.content.locator('.button-choice', { hasText: joinCode }).click();

  // Host waits for guest to choose auth
  await expect(greetData.modal.locator('.modal-header__step')).toHaveText('Step 3 of 3');
  await expect(greetData.title).toHaveText('Waiting for device information');
  await expect(greetData.nextButton).toBeHidden();

  // Guest choose auth
  await expect(joinData.modal.locator('.modal-header__step')).toHaveText('Step 3 of 3');
  await expect(joinData.title).toHaveText('Authentication');
  await expect(joinData.nextButton).toHaveText('Confirm');
  await expect(joinData.nextButton).toHaveDisabledAttribute();

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
  await expect(joinData.title).toHaveText('Device has been added!');
  await expect(joinData.modal.locator('.final-step').locator('.container-textinfo__text')).toHaveText(
    'The device has been created and added to this organization. You can now use it to connect to Parsec.',
  );
  await expect(joinData.nextButton).toHaveText('Log in');
  await expect(joinData.nextButton).not.toHaveDisabledAttribute();
  await joinData.nextButton.click();
  await expect(joinData.modal).toBeHidden();
  await expect(secondTab).toShowToast('You can now access your organization from your new device.', 'Success');
  // Automatically logged in
  await expect(secondTab.locator('#connected-header')).toContainText('My workspaces');
  await expect(secondTab).toBeWorkspacePage();

  await secondTab.release();

  // host is done
  await expect(greetData.title).toHaveText('New device added');

  await expect(greetData.nextButton).toHaveText('Finish');
  await expect(greetData.nextButton).not.toHaveDisabledAttribute();
  await expect(greetData.nextButton).toBeVisible();
  await expect(greetData.content.locator('.final-step').locator('.device-name')).toHaveText('Web');
  await expect(greetData.content.locator('.final-step').locator('.join-date')).toHaveText('Joined: Today');
  await greetData.nextButton.click();
  await expect(greetData.modal).toBeHidden();
  await expect(myProfilePage).toShowToast('You can connect to this organization from your new device.', 'Success');
});

msTest('Greet device whole process on large display', async ({ myProfilePage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const secondTab = await myProfilePage.openNewTab();

  const [greetData, joinData] = await initModals(myProfilePage, secondTab);

  // Check the provide code page from the host and retrieve the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code'], 0);
  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.subtitle).toHaveText('Click on the code below on the guest device.');

  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select the code
  await expect(joinData.title).toHaveText('Get host code');

  await expect(joinData.subtitle).toHaveText('Click on the code you see on the main device.');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Authentication'], 0);

  await expect(joinData.content.locator('.button-choice')).toHaveCount(4);
  await joinData.content.locator('.button-choice', { hasText: greetCode }).click();

  // Check the provide code page from the guest and retrieve the code
  await expect(joinData.title).toHaveText('Share guest code');
  await expect(joinData.subtitle).toHaveText('On the main device, click on the code you see below.');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Authentication'], 1);

  const joinCode = (await joinData.content.locator('.guest-code').locator('.code').textContent()) ?? '';
  expect(joinCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the host and select the code
  await expect(greetData.title).toHaveText('Get guest code');
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code'], 1);
  await expect(greetData.subtitle).toHaveText('Click on the code that appears on the guest device.');

  await expect(greetData.content.locator('.button-choice')).toHaveCount(4);
  await greetData.content.locator('.button-choice', { hasText: joinCode }).click();

  // Host waits for guest to choose auth
  await expect(greetData.title).toHaveText('Waiting for device information');
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code'], 1);
  await expect(greetData.nextButton).toBeHidden();

  // Guest choose auth
  await expect(joinData.title).toHaveText('Authentication');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Authentication'], 2);

  await expect(joinData.nextButton).toHaveText('Confirm');
  await expect(joinData.nextButton).toHaveDisabledAttribute();

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
  await expect(joinData.title).toHaveText('Device has been added!');
  await expect(joinData.modal.locator('.final-step').locator('.container-textinfo__text')).toHaveText(
    'The device has been created and added to this organization. You can now use it to connect to Parsec.',
  );
  await expect(joinData.nextButton).toHaveText('Log in');
  await expect(joinData.nextButton).not.toHaveDisabledAttribute();
  await joinData.nextButton.click();
  await expect(joinData.modal).toBeHidden();
  await expect(secondTab).toShowToast('You can now access your organization from your new device.', 'Success');
  // Automatically logged in
  await expect(secondTab.locator('#connected-header')).toContainText('My workspaces');
  await expect(secondTab).toBeWorkspacePage();
  const profile = secondTab.locator('.topbar').locator('.profile-header');
  await expect(profile.locator('.text-content-name')).toHaveText('Alicey McAliceFace');

  await secondTab.release();

  // host is done
  await expect(greetData.title).toHaveText('New device added');
  await expect(greetData.subtitle).toHaveText('The device Web has been added to your profile.');
  await expect(greetData.nextButton).toHaveText('Finish');
  await expect(greetData.nextButton).not.toHaveDisabledAttribute();
  await expect(greetData.nextButton).toBeVisible();
  await expect(greetData.content.locator('.final-step').locator('.device-name')).toHaveText('Web');
  await expect(greetData.content.locator('.final-step').locator('.join-date')).toHaveText('Joined: Today');
  await greetData.nextButton.click();
  await expect(greetData.modal).toBeHidden();
  await expect(myProfilePage).toShowToast('You can connect to this organization from your new device.', 'Success');
});

msTest('Host selects invalid SAS code', async ({ myProfilePage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const secondTab = await myProfilePage.openNewTab();
  const [greetData, joinData] = await initModals(myProfilePage, secondTab);

  // Check the provide code page from the host and retrieve the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code'], 0);
  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.subtitle).toHaveText('Click on the code below on the guest device.');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select the code
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.subtitle).toHaveText('Click on the code you see on the main device.');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Authentication'], 0);
  await expect(joinData.content.locator('.button-choice')).toHaveCount(4);
  await joinData.content.locator('.button-choice', { hasText: greetCode }).click();

  // Check the provide code page from the guest and retrieve the code
  await expect(joinData.title).toHaveText('Share guest code');
  await expect(joinData.subtitle).toHaveText('On the main device, click on the code you see below.');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Authentication'], 1);
  const joinCode = (await joinData.content.locator('.guest-code').locator('.code').textContent()) ?? '';
  expect(joinCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the host and select an invalid code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code'], 1);
  await expect(greetData.title).toHaveText('Get guest code');
  await expect(greetData.subtitle).toHaveText('Click on the code that appears on the guest device.');
  await expect(greetData.content.locator('.button-choice')).toHaveCount(4);
  await greetData.content.locator('.button-choice', { hasNotText: joinCode }).nth(0).click();

  await expect(secondTab).toShowToast('The code selected on the other device is incorrect.', 'Error');
  await expect(myProfilePage).toShowToast('You did not select the correct code. Please restart the onboarding process.', 'Error');

  // Back to the beginning
  await expect(greetData.nextButton).toHaveText('Start');
  await expect(greetData.title).toHaveText('Create a new device');

  await expect(joinData.title).toHaveText('Add a new device');
  await expect(joinData.nextButton).toHaveText('I understand!');
});

msTest('Host selects no SAS code', async ({ myProfilePage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const secondTab = await myProfilePage.openNewTab();
  const [greetData, joinData] = await initModals(myProfilePage, secondTab);

  // Check the provide code page from the host and retrieve the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code'], 0);
  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.subtitle).toHaveText('Click on the code below on the guest device.');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select the code
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.subtitle).toHaveText('Click on the code you see on the main device.');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Authentication'], 0);
  await expect(joinData.content.locator('.button-choice')).toHaveCount(4);
  await joinData.content.locator('.button-choice', { hasText: greetCode }).click();

  // Check the provide code page from the guest and retrieve the code
  await expect(joinData.title).toHaveText('Share guest code');
  await expect(joinData.subtitle).toHaveText('On the main device, click on the code you see below.');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Authentication'], 1);
  const joinCode = (await joinData.content.locator('.guest-code').locator('.code').textContent()) ?? '';
  expect(joinCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the host and select none of the codes
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code'], 1);
  await expect(greetData.title).toHaveText('Get guest code');
  await expect(greetData.subtitle).toHaveText('Click on the code that appears on the guest device.');
  await expect(greetData.content.locator('#noneChoicesButton')).toHaveText("Can't find the code?");
  await greetData.content.locator('#noneChoicesButton').click();

  const questionModal = myProfilePage.locator('.question-modal');
  await expect(questionModal).toBeVisible();
  await expect(questionModal.locator('.ms-modal-header__title')).toHaveText('No matching code');
  await expect(questionModal.locator('.ms-modal-header__text')).toHaveText(
    "If you can't find the matching code, quit and start the process over. If the problem persists, please contact your administrator.",
  );
  await questionModal.locator('#cancel-button').click();
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code'], 1);
  await greetData.content.locator('#noneChoicesButton').click();
  await questionModal.locator('#next-button').click();

  await expect(secondTab).toShowToast('The process has been cancelled from the other device.', 'Error');
  await expect(myProfilePage).toShowToast(
    'If you did not see the correct code, this could be a sign of a security issue during the onboarding. Please restart the process.',
    'Error',
  );

  // Back to the beginning
  await expect(greetData.nextButton).toHaveText('Start');
  await expect(greetData.title).toHaveText('Create a new device');

  await expect(joinData.title).toHaveText('Add a new device');
  await expect(joinData.nextButton).toHaveText('I understand!');
});

msTest('Host closes greet process', async ({ myProfilePage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const secondTab = await myProfilePage.openNewTab();

  const [greetData, joinData] = await initModals(myProfilePage, secondTab);

  // Check the provide code page from the host and retrieve the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code'], 0);
  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.subtitle).toHaveText('Click on the code below on the guest device.');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select the code
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.subtitle).toHaveText('Click on the code you see on the main device.');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Authentication'], 0);
  await expect(joinData.content.locator('.button-choice')).toHaveCount(4);
  await joinData.content.locator('.button-choice', { hasText: greetCode }).click();

  // Check the provide code page from the guest and retrieve the code
  await expect(joinData.title).toHaveText('Share guest code');
  await expect(joinData.subtitle).toHaveText('On the main device, click on the code you see below.');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Authentication'], 1);
  const joinCode = (await joinData.content.locator('.guest-code').locator('.code').textContent()) ?? '';
  expect(joinCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the host and try to close but cancel
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code'], 1);
  await expect(greetData.title).toHaveText('Get guest code');
  await expect(greetData.subtitle).toHaveText('Click on the code that appears on the guest device.');
  await greetData.closeButton.click();
  await answerQuestion(myProfilePage, false, {
    expectedTitleText: 'Cancel',
    expectedQuestionText: 'Are you sure you want to cancel the onboarding process?',
    expectedPositiveText: 'Cancel process',
    expectedNegativeText: 'Resume',
  });

  // Now cancel it for real
  await greetData.closeButton.click();
  await answerQuestion(myProfilePage, true, {
    expectedTitleText: 'Cancel',
    expectedQuestionText: 'Are you sure you want to cancel the onboarding process?',
    expectedPositiveText: 'Cancel process',
    expectedNegativeText: 'Resume',
  });
  await expect(greetData.modal).toBeHidden();

  // Check that guest is notified and can restart
  await expect(secondTab).toShowToast('The process has been cancelled from the other device.', 'Error');
  await expect(joinData.title).toHaveText('Add a new device');
  await expect(joinData.nextButton).toHaveText('I understand!');
});

msTest('Guest selects invalid SAS code', async ({ myProfilePage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const secondTab = await myProfilePage.openNewTab();

  const [greetData, joinData] = await initModals(myProfilePage, secondTab);

  // Check the provide code page from the host and retrieve the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code'], 0);
  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.subtitle).toHaveText('Click on the code below on the guest device.');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select an invalid code
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.subtitle).toHaveText('Click on the code you see on the main device.');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Authentication'], 0);
  await expect(joinData.content.locator('.button-choice')).toHaveCount(4);
  await joinData.content.locator('.button-choice', { hasNotText: greetCode }).nth(0).click();

  await expect(secondTab).toShowToast('You did not select the correct code. Please restart the onboarding process.', 'Error');
  await expect(myProfilePage).toShowToast('The code selected on the other device is incorrect.', 'Error');

  // Back to the beginning
  await expect(greetData.nextButton).toHaveText('Start');
  await expect(greetData.title).toHaveText('Create a new device');

  await expect(joinData.title).toHaveText('Add a new device');
  await expect(joinData.nextButton).toHaveText('I understand!');
});

msTest('Guest selects no SAS code', async ({ myProfilePage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const secondTab = await myProfilePage.openNewTab();

  const [greetData, joinData] = await initModals(myProfilePage, secondTab);

  // Check the provide code page from the host and retrieve the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code'], 0);
  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.subtitle).toHaveText('Click on the code below on the guest device.');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and select none of the codes
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.subtitle).toHaveText('Click on the code you see on the main device.');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Authentication'], 0);
  await joinData.content.locator('.button-outline').nth(4).click();

  const questionModal = secondTab.locator('.question-modal');
  await expect(questionModal).toBeVisible();
  await expect(questionModal.locator('.ms-modal-header__title')).toHaveText('No matching code');
  await expect(questionModal.locator('.ms-modal-header__text')).toHaveText(
    "If you can't find the matching code, quit and start the process over. If the problem persists, please contact your administrator.",
  );
  await questionModal.locator('#cancel-button').click();
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Authentication'], 0);
  await joinData.content.locator('.button-outline').nth(4).click();
  await questionModal.locator('#next-button').click();

  await expect(secondTab).toShowToast(
    'If you did not see the correct code, this could be a sign of a security issue during the onboarding. Please restart the process.',
    'Error',
  );
  await expect(myProfilePage).toShowToast('The process has been cancelled from the other device.', 'Error');

  // Back to the beginning
  await expect(greetData.nextButton).toHaveText('Start');
  await expect(greetData.title).toHaveText('Create a new device');

  await expect(joinData.title).toHaveText('Add a new device');
  await expect(joinData.nextButton).toHaveText('I understand!');
});

msTest('Guest closes greet process', async ({ myProfilePage }) => {
  // Very slow test since it syncs the greet and join
  msTest.setTimeout(120_000);

  const secondTab = await myProfilePage.openNewTab();

  const [greetData, joinData] = await initModals(myProfilePage, secondTab);

  // Check the provide code page from the host and retrieve the code
  await expect(greetData.modal).toHaveWizardStepper(['Host code', 'Guest code'], 0);
  await expect(greetData.title).toHaveText('Share your code');
  await expect(greetData.subtitle).toHaveText('Click on the code below on the guest device.');
  const greetCode = (await greetData.content.locator('.host-code').locator('.code').textContent()) ?? '';
  expect(greetCode).toMatch(/^[A-Z0-9]{4}$/);

  // Check the enter code page from the guest and try to close but cancel
  await expect(joinData.title).toHaveText('Get host code');
  await expect(joinData.subtitle).toHaveText('Click on the code you see on the main device.');
  await expect(joinData.modal).toHaveWizardStepper(['Host code', 'Guest code', 'Authentication'], 0);
  await joinData.closeButton.click();
  await answerQuestion(secondTab, false, {
    expectedTitleText: 'Cancel the process',
    expectedQuestionText: 'Are you sure you want to cancel the process? Information will not be saved, you will have to restart.',
    expectedPositiveText: 'Cancel process',
    expectedNegativeText: 'Resume',
  });

  // Now cancel it for real
  await joinData.closeButton.click();
  await answerQuestion(secondTab, true, {
    expectedTitleText: 'Cancel the process',
    expectedQuestionText: 'Are you sure you want to cancel the process? Information will not be saved, you will have to restart.',
    expectedPositiveText: 'Cancel process',
    expectedNegativeText: 'Resume',
  });
  await expect(joinData.modal).toBeHidden();

  // Check that host is notified and can restart
  await expect(myProfilePage).toShowToast('The process has been cancelled from the other device.', 'Error');
  await expect(greetData.title).toHaveText('Create a new device');
  await expect(greetData.nextButton).toHaveText('Start');
});

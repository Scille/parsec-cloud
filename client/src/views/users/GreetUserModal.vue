<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal-stepper">
    <ms-wizard-stepper
      v-show="pageStep > GreetUserStep.WaitForGuest && pageStep < GreetUserStep.Summary"
      :current-index="getStepperIndex()"
      :titles="[$t('UsersPage.greet.steps.hostCode'), $t('UsersPage.greet.steps.guestCode'), $t('UsersPage.greet.steps.contactDetails')]"
    />
    <ion-buttons
      slot="end"
      class="closeBtn-container"
    >
      <ion-button
        slot="icon-only"
        @click="cancelModal()"
        class="closeBtn"
      >
        <ion-icon
          :icon="close"
          size="large"
          class="closeBtn__icon"
        />
      </ion-button>
    </ion-buttons>
    <div class="modal">
      <ion-header class="modal-header">
        <ion-title class="modal-header__title title-h2">
          {{ getTitleAndSubtitle()[0] }}
        </ion-title>
        <ion-text class="modal-header__text body">
          {{ getTitleAndSubtitle()[1] }}
        </ion-text>
      </ion-header>
      <div class="modal-content inner-content">
        <!-- waiting step -->
        <div
          v-show="pageStep === GreetUserStep.WaitForGuest"
          class="step"
        >
          <ms-informative-text>
            {{ $t('UsersPage.greet.subtitles.waitForGuest1') }}
          </ms-informative-text>
          <ms-informative-text>
            {{ $t('UsersPage.greet.subtitles.waitForGuest2') }}
          </ms-informative-text>
        </div>

        <!-- give code step -->
        <div
          v-show="pageStep === GreetUserStep.ProvideHostSasCode"
          class="step"
        >
          <sas-code-provide :code="greeter.hostSASCode" />
        </div>

        <!-- choose code step -->
        <div
          v-show="pageStep === GreetUserStep.GetGuestSasCode"
          class="step"
        >
          <sas-code-choice
            :choices="greeter.SASCodeChoices"
            @select="selectGuestSas"
          />
        </div>

        <!-- Waiting guest info step -->
        <div
          v-show="pageStep === GreetUserStep.WaitForGuestInfo"
          class="step"
        >
          <ms-informative-text>
            {{ $t('UsersPage.greet.subtitles.getUserInfo') }}
          </ms-informative-text>
        </div>

        <!-- Check guest info step -->
        <div
          v-show="pageStep === GreetUserStep.CheckGuestInfo"
          class="step user-info-page"
        >
          <user-information
            :default-email="''"
            :default-name="''"
            :default-device="''"
            :email-enabled="false"
            ref="guestInfoPage"
            @on-enter-keyup="nextStep()"
            @field-update="updateCanGoForward"
          />
          <ms-dropdown
            class="dropdown"
            :title="$t('UsersPage.greet.profileDropdownTitle')"
            :label="$t('UsersPage.greet.profileDropdownPlaceholder')"
            :options="profileOptions"
            @change="setUserProfile($event.option.key)"
          />
        </div>

        <!-- Final Step -->
        <div
          v-show="pageStep === GreetUserStep.Summary"
          v-if="guestInfoPage"
          class="final-step"
        >
          <user-avatar-name
            class="avatar"
            :user-name="guestInfoPage?.fullName"
            :user-avatar="guestInfoPage?.fullName"
          />
          <div class="user-info">
            <div class="user-info__email">
              <ion-text class="body">{{ $t('UsersPage.success.email') }}</ion-text>
              <ion-text class="cell">{{ guestInfoPage?.email }}</ion-text>
            </div>
            <div class="user-info__role">
              <ion-text class="body">{{ $t('UsersPage.success.profile') }}</ion-text>
              <tag-profile :profile="profile ? profile : UserProfile.Outsider" />
            </div>
          </div>
        </div>
      </div>
      <ion-footer class="modal-footer">
        <ion-buttons
          slot="primary"
          class="modal-footer-buttons"
        >
          <ion-button
            fill="solid"
            size="default"
            id="next-button"
            v-show="nextButtonIsVisible"
            @click="nextStep()"
            :disabled="!canGoForward"
          >
            <span>
              {{ getNextButtonText() }}
            </span>
          </ion-button>
          <div
            v-show="waitingForGuest"
            class="spinner-container"
          >
            <ion-label class="label-waiting" />
            <ion-spinner
              name="crescent"
              color="primary"
            />
          </div>
        </ion-buttons>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonButton,
  IonButtons,
  IonFooter,
  IonHeader,
  IonIcon,
  IonLabel,
  IonPage,
  IonSpinner,
  IonText,
  IonTitle,
  modalController,
} from '@ionic/vue';

import { Answer, MsDropdown, MsInformativeText, MsModalResult, MsOptions, MsWizardStepper, askQuestion } from '@/components/core';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import TagProfile from '@/components/users/TagProfile.vue';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import UserInformation from '@/components/users/UserInformation.vue';
import { UserGreet, UserInvitation, UserProfile } from '@/parsec';
import { Notification, NotificationKey, NotificationLevel, NotificationManager } from '@/services/notificationManager';
import { translate } from '@/services/translation';
import { close } from 'ionicons/icons';
import { Ref, computed, inject, onMounted, onUnmounted, ref, watch } from 'vue';

enum GreetUserStep {
  WaitForGuest = 1,
  ProvideHostSasCode = 2,
  GetGuestSasCode = 3,
  WaitForGuestInfo = 4,
  CheckGuestInfo = 5,
  Summary = 6,
}

const pageStep = ref(GreetUserStep.WaitForGuest);
const props = defineProps<{
  invitation: UserInvitation;
}>();

const profile: Ref<UserProfile | null> = ref(null);
const guestInfoPage: Ref<typeof UserInformation | null> = ref(null);
const canGoForward = ref(false);
const waitingForGuest = ref(true);
const greeter = ref(new UserGreet());
// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const notificationManager: NotificationManager = inject(NotificationKey)!;

const profileOptions: MsOptions = new MsOptions([
  {
    key: UserProfile.Admin,
    label: translate('UsersPage.profile.admin.label'),
    description: translate('UsersPage.profile.admin.description'),
  },
  {
    key: UserProfile.Standard,
    label: translate('UsersPage.profile.standard.label'),
    description: translate('UsersPage.profile.standard.description'),
  },
  {
    key: UserProfile.Outsider,
    label: translate('UsersPage.profile.outsider.label'),
    description: translate('UsersPage.profile.outsider.description'),
  },
]);

function getTitleAndSubtitle(): [string, string] {
  if (pageStep.value === GreetUserStep.WaitForGuest) {
    return [translate('UsersPage.greet.titles.waitForGuest'), ''];
  } else if (pageStep.value === GreetUserStep.ProvideHostSasCode) {
    return [translate('UsersPage.greet.titles.provideHostCode'), translate('UsersPage.greet.subtitles.provideHostCode')];
  } else if (pageStep.value === GreetUserStep.GetGuestSasCode) {
    return [translate('UsersPage.greet.titles.getGuestCode'), translate('UsersPage.greet.subtitles.getGuestCode')];
  } else if (pageStep.value === GreetUserStep.WaitForGuestInfo) {
    return [translate('UsersPage.greet.titles.contactDetails'), ''];
  } else if (pageStep.value === GreetUserStep.CheckGuestInfo) {
    return [translate('UsersPage.greet.titles.contactDetails'), translate('UsersPage.greet.subtitles.checkUserInfo')];
  } else if (pageStep.value === GreetUserStep.Summary) {
    return [translate('UsersPage.greet.titles.summary'), ''];
  }
  return ['', ''];
}

function setUserProfile(role: string): void {
  profile.value = role as UserProfile;
}

const unwatchProfile = watch(profile, async () => {
  await updateCanGoForward();
});

async function updateCanGoForward(): Promise<void> {
  if (pageStep.value === GreetUserStep.WaitForGuest && waitingForGuest.value === true) {
    canGoForward.value = false;
  } else if (pageStep.value === GreetUserStep.CheckGuestInfo) {
    canGoForward.value = guestInfoPage.value && profile.value && (await guestInfoPage.value.areFieldsCorrect()) && profile.value !== null;
  } else {
    canGoForward.value = true;
  }
}

function getNextButtonText(): string {
  if (pageStep.value === GreetUserStep.WaitForGuest) {
    return translate('UsersPage.greet.actions.start');
  } else if (pageStep.value === GreetUserStep.Summary) {
    return translate('UsersPage.greet.actions.finish');
  } else if (pageStep.value === GreetUserStep.CheckGuestInfo) {
    return translate('UsersPage.greet.actions.approve');
  }
  return '';
}

async function selectGuestSas(code: string | null): Promise<void> {
  if (!code) {
    await showErrorAndRestart(translate('UsersPage.greet.errors.noneCodeSelected'));
    return;
  }
  if (code === greeter.value.correctSASCode) {
    const result = await greeter.value.signifyTrust();
    if (result.ok) {
      await nextStep();
    } else {
      await showErrorAndRestart(translate('UsersPage.greet.errors.unexpected', { reason: result.error.tag }));
    }
  } else {
    await showErrorAndRestart(translate('UsersPage.greet.errors.invalidCodeSelected'));
  }
}

async function restartProcess(): Promise<void> {
  await greeter.value.abort();
  await startProcess();
}

async function startProcess(): Promise<void> {
  pageStep.value = GreetUserStep.WaitForGuest;
  waitingForGuest.value = true;
  const result = await greeter.value.startGreet(props.invitation.token);
  if (!result.ok) {
    notificationManager.showToast(
      new Notification({
        title: translate('UsersPage.greet.errors.startFailed.title'),
        message: translate('UsersPage.greet.errors.startFailed.message'),
        level: NotificationLevel.Error,
      }),
    );
    await cancelModal();
    return;
  }
  const waitResult = await greeter.value.initialWaitGuest();
  if (!waitResult.ok) {
    notificationManager.showToast(
      new Notification({
        title: translate('UsersPage.greet.errors.startFailed.title'),
        message: translate('UsersPage.greet.errors.startFailed.message'),
        level: NotificationLevel.Error,
      }),
    );
    await cancelModal();
    return;
  }
  waitingForGuest.value = false;
  await updateCanGoForward();
}

function getStepperIndex(): number {
  if (pageStep.value <= GreetUserStep.ProvideHostSasCode) {
    return 0;
  } else if (pageStep.value === GreetUserStep.GetGuestSasCode) {
    return 1;
  } else {
    return 2;
  }
}

const nextButtonIsVisible = computed(() => {
  if (pageStep.value === GreetUserStep.WaitForGuest && waitingForGuest.value) {
    return false;
  } else if (pageStep.value === GreetUserStep.ProvideHostSasCode) {
    return false;
  } else if (pageStep.value === GreetUserStep.GetGuestSasCode) {
    return false;
  } else if (pageStep.value === GreetUserStep.WaitForGuestInfo) {
    return false;
  }
  return true;
});

async function cancelModal(): Promise<boolean> {
  if (pageStep.value === GreetUserStep.Summary || pageStep.value === GreetUserStep.WaitForGuest) {
    return await modalController.dismiss(null, MsModalResult.Cancel);
  }
  const answer = await askQuestion(translate('UsersPage.greet.cancelConfirm'), translate('UsersPage.greet.cancelConfirmSubtitle'), {
    yesIsDangerous: true,
    keepMainModalHiddenOnYes: true,
    yesText: translate('UsersPage.greet.cancelYes'),
    noText: translate('UsersPage.greet.cancelNo'),
  });

  if (answer === Answer.Yes) {
    await greeter.value.abort();
    return await modalController.dismiss(null, MsModalResult.Cancel);
  }
  return false;
}

async function showErrorAndRestart(message: string): Promise<void> {
  notificationManager.showToast(
    new Notification({
      title: translate('UsersPage.greet.errors.createUserFailed.title'),
      message: message,
      level: NotificationLevel.Error,
    }),
  );
  await restartProcess();
}

async function nextStep(): Promise<void> {
  await updateCanGoForward();
  if (!canGoForward.value) {
    return;
  }
  if (pageStep.value === GreetUserStep.Summary) {
    notificationManager.showToast(
      new Notification({
        title: translate('UsersPage.greet.success.title'),
        message: translate('UsersPage.greet.success.message', {
          user: guestInfoPage.value?.fullName,
        }),
        level: NotificationLevel.Success,
      }),
    );
    await modalController.dismiss({}, MsModalResult.Confirm);
    return;
  } else if (pageStep.value === GreetUserStep.CheckGuestInfo && guestInfoPage.value && profile.value) {
    const result = await greeter.value.createUser(
      { label: guestInfoPage.value.fullName, email: guestInfoPage.value.email },
      guestInfoPage.value.deviceName,
      profile.value,
    );
    if (!result.ok) {
      await showErrorAndRestart(translate('UsersPage.greet.errors.createUserFailed'));
      return;
    }
  }

  pageStep.value = pageStep.value + 1;

  await updateCanGoForward();

  if (pageStep.value === GreetUserStep.ProvideHostSasCode) {
    waitingForGuest.value = true;
    const result = await greeter.value.waitGuestTrust();
    waitingForGuest.value = false;
    if (result.ok) {
      await nextStep();
    } else {
      await showErrorAndRestart(translate('UsersPage.greet.errors.unexpected', { reason: result.error.tag }));
    }
  } else if (pageStep.value === GreetUserStep.WaitForGuestInfo) {
    waitingForGuest.value = true;
    const result = await greeter.value.getClaimRequests();
    waitingForGuest.value = false;
    if (result.ok && guestInfoPage.value) {
      guestInfoPage.value.fullName = greeter.value.requestedHumanHandle?.label;
      guestInfoPage.value.email = greeter.value.requestedHumanHandle?.email;
      guestInfoPage.value.deviceName = greeter.value.requestedDeviceLabel;
      await nextStep();
    } else {
      await showErrorAndRestart(translate('UsersPage.greet.errors.retrieveUserInfoFailed'));
    }
  }
}

onMounted(async () => {
  await startProcess();
});

onUnmounted(async () => {
  unwatchProfile();
});
</script>

<style scoped lang="scss">
.final-step {
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  color: var(--parsec-color-light-secondary-text);
  gap: 1rem;

  .avatar {
    padding-left: 0.725rem;
    border-left: 2px solid var(--parsec-color-light-secondary-disabled);
  }

  .user-info {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    background: var(--parsec-color-light-secondary-background);
    padding: 1rem;
    border-radius: var(--parsec-radius-6);

    & > [class^='user-info'] {
      display: flex;
      align-items: center;
      gap: 1rem;

      .body {
        color: var(--parsec-color-light-secondary-grey);
      }
    }
  }
}
</style>

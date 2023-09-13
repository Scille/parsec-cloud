<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal-stepper">
    <ms-wizard-stepper
      v-show="pageStep > GreetUserStep.WaitForGuest && pageStep < GreetUserStep.Summary"
      :current-index="getStepperIndex()"
      :titles="[
        $t('UsersPage.greet.steps.hostCode'),
        $t('UsersPage.greet.steps.guestCode'),
        $t('UsersPage.greet.steps.contactDetails'),
      ]"
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
    <div
      class="modal"
    >
      <ion-header class="modal-header">
        <ion-title
          class="modal-header__title title-h2"
        >
          {{ getTitleAndSubtitle()[0] }}
        </ion-title>
        <ion-text
          class="modal-header__text body"
        >
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
          <sas-code-provide
            :code="'ABCD'"
          />
        </div>

        <!-- choose code step -->
        <div
          v-show="pageStep === GreetUserStep.GetGuestSasCode"
          class="step"
        >
          <sas-code-choice
            :choices="['ABCD', 'EFGH', 'IJKL', 'MNOP']"
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
            :default-email="invitation.claimerEmail"
            :default-name="'Name Entered By The Guest'"
            :default-device="'Device-given-by-the-guest'"
            :email-enabled="false"
            ref="guestInfoPage"
          />
          <ion-item class="input-container">
            <ion-select
              label="Profile"
              label-placement="stacked"
              ref="profileSelect"
              :placeholder="t('UsersPage.greet.profilePlaceholder')"
              v-model="profile"
            >
              <ion-select-option :value="Profile.Admin">
                {{ $t('UsersPage.profile.admin') }}
              </ion-select-option>
              <ion-select-option :value="Profile.Standard">
                {{ $t('UsersPage.profile.standard') }}
              </ion-select-option>
              <ion-select-option :value="Profile.Outsider">
                {{ $t('UsersPage.profile.outsider') }}
              </ion-select-option>
            </ion-select>
          </ion-item>
        </div>

        <!-- Final Step -->
        <div
          v-show="pageStep === GreetUserStep.Summary"
          v-if="userInfo"
          class="step final-step"
        >
          <div class="final-step__name">
            <user-avatar-name
              :user-name="userInfo.name"
              :user-avatar="userInfo.name"
            />
          </div>
          <div>
            {{ userInfo.email }}
          </div>
          <div class="user-profile">
            <tag-profile
              :profile="userInfo.profile"
            />
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
            <ion-label
              class="label-waiting"
            >
              {{ }}
            </ion-label>
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
  IonTitle,
  IonText,
  IonPage,
  IonHeader,
  IonButton,
  IonButtons,
  IonFooter,
  IonIcon,
  IonLabel,
  IonSpinner,
  IonSelect,
  IonSelectOption,
  IonItem,
  modalController,
} from '@ionic/vue';

import {
  close,
  caretForward,
} from 'ionicons/icons';
import { ref, Ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import MsWizardStepper from '@/components/core/ms-stepper/MsWizardStepper.vue';
import MsInformativeText from '@/components/core/ms-text/MsInformativeText.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import UserInformation from '@/components/users/UserInformation.vue';
import TagProfile from '@/components/users/TagProfile.vue';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { MsModalResult } from '@/components/core/ms-types';
import { Profile } from '@/common/mocks';
import * as Parsec from '@/common/parsec';

enum GreetUserStep {
  WaitForGuest = 1,
  ProvideHostSasCode = 2,
  GetGuestSasCode = 3,
  WaitForGuestInfo = 4,
  CheckGuestInfo = 5,
  Summary = 6,
}

// Used to simulate host interaction
const OTHER_USER_WAITING_TIME = 500;

const { t } = useI18n();

const pageStep = ref(GreetUserStep.WaitForGuest);

defineProps<{
  invitation: Parsec.UserInvitation
}>();

const profile: Ref<Profile | null> = ref(null);
const guestInfoPage = ref(null);
const waitingForGuest = ref(true);
const userInfo :Ref<UserInfo | null> = ref(null);

interface UserInfo {
  name: string,
  email: string,
  device: string,
  profile: Profile,
}

function getTitleAndSubtitle(): [string, string] {
  if (pageStep.value === GreetUserStep.WaitForGuest) {
    return [t('UsersPage.greet.titles.waitForGuest'), ''];
  } else if (pageStep.value === GreetUserStep.ProvideHostSasCode) {
    return [t('UsersPage.greet.titles.provideHostCode'), t('UsersPage.greet.subtitles.provideHostCode')];
  } else if (pageStep.value === GreetUserStep.GetGuestSasCode) {
    return [t('UsersPage.greet.titles.getGuestCode'), t('UsersPage.greet.subtitles.getGuestCode')];
  } else if (pageStep.value === GreetUserStep.WaitForGuestInfo) {
    return [t('UsersPage.greet.titles.contactDetails'), ''];
  } else if (pageStep.value === GreetUserStep.CheckGuestInfo) {
    return [t('UsersPage.greet.titles.contactDetails'), t('UsersPage.greet.subtitles.checkUserInfo')];
  } else if (pageStep.value === GreetUserStep.Summary) {
    return [t('UsersPage.greet.titles.summary'), t('UsersPage.greet.subtitles.summary', {name: userInfo.value ? userInfo.value.name : ''})];
  }
  return ['', ''];
}

function getNextButtonText(): string {
  if (pageStep.value === GreetUserStep.WaitForGuest) {
    return t('UsersPage.greet.actions.start');
  } else if (pageStep.value === GreetUserStep.Summary) {
    return t('UsersPage.greet.actions.finish');
  } else if (pageStep.value === GreetUserStep.CheckGuestInfo) {
    return t('UsersPage.greet.actions.approve');
  }
  return '';
}

function selectGuestSas(_code: string | null): void {
  nextStep();
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

const canGoForward = computed(() => {
  if (pageStep.value === GreetUserStep.CheckGuestInfo && guestInfoPage.value && profile.value) {
    return (guestInfoPage.value as any).areFieldsCorrect() && profile.value !== null;
  } else if (pageStep.value === GreetUserStep.WaitForGuest && waitingForGuest.value === false) {
    return true;
  } else if (pageStep.value === GreetUserStep.Summary) {
    return true;
  }
  return false;
});

function cancelModal(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Cancel);
}

function nextStep(): void {
  if (pageStep.value === GreetUserStep.Summary) {
    modalController.dismiss({}, MsModalResult.Confirm);
    return;
  } else if (pageStep.value === GreetUserStep.CheckGuestInfo && guestInfoPage.value && profile.value) {
    userInfo.value = {
      name: (guestInfoPage.value as typeof UserInformation).fullName,
      email: (guestInfoPage.value as typeof UserInformation).email,
      device: (guestInfoPage.value as typeof UserInformation).deviceName,
      profile: profile.value,
    };
  }

  pageStep.value = pageStep.value + 1;

  if (pageStep.value === GreetUserStep.ProvideHostSasCode) {
    waitingForGuest.value = true;
    window.setTimeout(guestHasEnteredCode, OTHER_USER_WAITING_TIME);
  } else if (pageStep.value === GreetUserStep.WaitForGuestInfo) {
    waitingForGuest.value = true;
    window.setTimeout(guestHasProvidedInfo, OTHER_USER_WAITING_TIME);
  }
}

function guestHasProvidedInfo(): void {
  waitingForGuest.value = false;
  nextStep();
}

function guestHasEnteredCode(): void {
  waitingForGuest.value = false;
  nextStep();
}

onMounted(() => {
  waitForGuest();
});

function guestArrived(): void {
  waitingForGuest.value = false;
}

function waitForGuest(): void {
  // Simulate guest starting the process
  window.setTimeout(guestArrived, OTHER_USER_WAITING_TIME);
}
</script>

<style scoped lang="scss">
  .final-step {
    width: 100%;
    display: flex;
    flex-direction: row;
    align-items: center;
    background: var(--parsec-color-light-secondary-background);
    padding: .75rem 1rem;
    border-radius: var(--parsec-radius-6);
    justify-content: space-between;
    color: var(--parsec-color-light-secondary-text);

    &__icon {
      font-size: 1.5rem;
      color: var(--parsec-color-light-primary-500);
    }

    &__name {
      font-weight: 500;
    }
  }
</style>

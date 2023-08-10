<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
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
        <div
          v-show="pageStep === GreetUserStep.WaitForGuest"
          class="step"
        >
          <ms-informative-text
            :icon="caretForward"
            :text="$t('UsersPage.greet.subtitles.waitForGuest1')"
          />
          <ms-informative-text
            :icon="caretForward"
            :text="$t('UsersPage.greet.subtitles.waitForGuest2')"
          />
        </div>
        <div
          v-show="pageStep === GreetUserStep.ProvideHostSasCode"
          class="step"
        >
          <sas-code-provide
            :code="'ABCD'"
          />
        </div>
        <div
          v-show="pageStep === GreetUserStep.GetGuestSasCode"
          class="step"
        >
          <sas-code-choice
            :choices="['ABCD', 'EFGH', 'IJKL', 'MNOP']"
            @select="selectGuestSas"
          />
        </div>
        <div
          v-show="pageStep === GreetUserStep.WaitForGuestInfo"
          class="step"
        >
          <ms-informative-text
            :text="$t('UsersPage.greet.subtitles.getUserInfo')"
            :icon="caretForward"
          />
        </div>
        <div
          v-show="pageStep === GreetUserStep.CheckGuestInfo"
          class="step user-info-page"
        >
          <user-information
            :default-email="invitation.email"
            :default-name="'Name Entered By The Guest'"
            :default-device="'Device-given-by-the-guest'"
            :email-enabled="false"
            ref="guestInfoPage"
          />
          <ion-item>
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
        <div
          v-show="pageStep === GreetUserStep.Summary"
          v-if="userInfo"
          class="step"
        >
          <div>
            {{ userInfo.name }}
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
import { ModalResultCode } from '@/common/constants';
import { MockInvitation, Profile } from '@/common/mocks';

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
  invitation: MockInvitation
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
  return modalController.dismiss(null, ModalResultCode.Cancel);
}

function nextStep(): void {
  if (pageStep.value === GreetUserStep.Summary) {
    modalController.dismiss({}, ModalResultCode.Confirm);
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
.modal {
  padding: 3.5rem;
  justify-content: start;
}

.closeBtn-container {
    position: absolute;
    top: 2rem;
    right: 2rem;
  }

.closeBtn-container, .closeBtn {
  margin: 0;
  --padding-start: 0;
  --padding-end: 0;
}

.closeBtn {
  width: fit-content;
  height: fit-content;
  --border-radius: var(--parsec-radius-4);
  --background-hover: var(--parsec-color-light-primary-50);
  border-radius: var(--parsec-radius-4);

  &__icon {
    padding: 4px;
    color: var(--parsec-color-light-primary-500);

    &:hover {
      --background-hover: var(--parsec-color-light-primary-50);
    }
  }

  &:active {
    border-radius: var(--parsec-radius-4);
    background: var(--parsec-color-light-primary-100);
  }
}

.modal-header {
  margin-bottom: 2rem;

  &__title {
    padding: 0;
    margin-bottom: 1.5rem;
    color: var(--parsec-color-light-primary-600);
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  &__text {
    color: var(--parsec-color-light-secondary-grey);
  }
}

.modal-content {
  --background: transparent;

  .step {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
}

.modal-footer {
  margin-top: 2.5rem;

  &::before {
    background: transparent;
  }

  &-buttons {
    display: flex;
    justify-content: end;
    gap: 1rem;
  }
}

.label-waiting {
  color: var(--parsec-color-light-secondary-grey);
  font-style: italic;
  padding-left: 2em;
  padding-right: 2em;
}

.spinner-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
}
</style>

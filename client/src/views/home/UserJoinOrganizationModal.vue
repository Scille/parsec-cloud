<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ms-wizard-stepper
      v-if="pageStep > UserJoinOrganizationStep.WaitForHost"
      :current-index="pageStep - 2"
      :titles="[
        $t('JoinOrganization.stepTitles.GetHostCode'),
        $t('JoinOrganization.stepTitles.ProvideGuestCode'),
        $t('JoinOrganization.stepTitles.ContactDetails'),
        $t('JoinOrganization.stepTitles.Password'),
        $t('JoinOrganization.stepTitles.Validation')
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
    <!-- v-if pageStep === WaitForHost so add class wizardTrue -->
    <div
      class="modal"
      :class="{
        wizardTrue: pageStep > 1
      }"
    >
      <ion-header class="modal-header">
        <ion-title
          v-if="titles.get(pageStep)?.title !== ''"
          class="modal-header__title title-h2"
        >
          {{ titles.get(pageStep)?.title }}
        </ion-title>
        <ion-text
          v-if="titles.get(pageStep)?.subtitle !== ''"
          class="modal-header__text body"
        >
          {{ titles.get(pageStep)?.subtitle }}
        </ion-text>
      </ion-header>
      <!-- modal content: create component for each part-->
      <div class="modal-content inner-content">
        <!-- part 1 (wait for host)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.WaitForHost"
          class="step orga-name"
        >
          <div class="orga-name-content">
            <ms-informative-text
              :icon="caretForward"
              :text="$t('JoinOrganization.instructions.start.first')"
            />
            <ms-informative-text
              :icon="caretForward"
              :text="$t('JoinOrganization.instructions.start.second')"
            />
          </div>
        </div>

        <!-- part 2 (host code)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.GetHostSasCode"
          class="step"
        >
          <sas-code-choice
            ref="hostCodePage"
            :choices="['ABCD', 'EFGH', 'IJKL', 'MNOP']"
            @select="selectHostSas($event)"
          />
        </div>

        <!-- part 3 (guest code)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.ProvideGuestCode"
          class="step guest-code"
        >
          <sas-code-provide
            ref="guestCodePage"
            :code="'ABCD'"
          />
        </div>

        <!-- part 4 (user info)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.GetUserInfo"
          class="step"
          id="get-user-info"
        >
          <user-information
            ref="userInfoPage"
            :email-enabled="false"
            :device-enabled="!waitingForHost"
            :name-enabled="!waitingForHost"
          />
        </div>
        <!-- part 5 (get password)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.GetPassword"
          class="step"
          id="get-password"
        >
          <ms-choose-password-input
            ref="passwordPage"
          />
        </div>
        <!-- part 6 (finish the process)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.Finish"
          class="step"
        >
          <ms-informative-text
            :icon="caretForward"
            :text="$t('JoinOrganization.instructions.finish.first')"
          />
          <ms-informative-text
            :icon="caretForward"
            :text="$t('JoinOrganization.instructions.finish.second')"
          />
        </div>
      </div>
      <!-- the buttons must be only enabled if all fields are filled in -->
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
            v-show="waitingForHost"
            class="spinner-container"
          >
            <ion-label
              class="label-waiting"
            >
              {{ $t('JoinOrganization.waitingForHost') }}
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
  modalController,
} from '@ionic/vue';

import {
  close,
  caretForward,
} from 'ionicons/icons';
import { ref, Ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import MsWizardStepper from '@/components/core/ms-stepper/MsWizardStepper.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import UserInformation from '@/components/users/UserInformation.vue';
import MsChoosePasswordInput from '@/components/core/ms-input/MsChoosePasswordInput.vue';
import MsInformativeText from '@/components/core/ms-text/MsInformativeText.vue';
import { AvailableDevice } from '@/plugins/libparsec/definitions';
import { MsModalResult } from '@/components/core/ms-modal/MsModal.vue';

enum UserJoinOrganizationStep {
  WaitForHost = 1,
  GetHostSasCode = 2,
  ProvideGuestCode = 3,
  GetUserInfo = 4,
  GetPassword = 5,
  Finish = 6,
}

// Used to simulate host interaction
const OTHER_USER_WAITING_TIME = 500;

const { t } = useI18n();

const pageStep = ref(UserJoinOrganizationStep.WaitForHost);
const waitPage = ref();
const hostCodePage = ref();
const guestCodePage = ref();
const userInfoPage = ref();
const passwordPage = ref();

let newDevice: AvailableDevice | null = null;

const props = defineProps<{
  invitationLink: string
}>();

interface ClaimUserLink {
  backendAddr: string,
  token: string,
  org: string
}

// Will be done in Rust
function parseInvitationLink(link: string): ClaimUserLink {
  return {
    backendAddr: link,
    token: 'aaa',
    org: 'My Org',
  };
}

const claimUserLink = parseInvitationLink(props.invitationLink);
const waitingForHost = ref(true);

interface Title {
  title: string,
  subtitle: string,
}

const titles = new Map<UserJoinOrganizationStep, Title>([
  [
    UserJoinOrganizationStep.WaitForHost,
    {title: t('JoinOrganization.titles.waitForHost'), subtitle: t('JoinOrganization.subtitles.waitForHost')},
  ],
  [
    UserJoinOrganizationStep.GetHostSasCode,
    {title: t('JoinOrganization.titles.getHostCode'), subtitle: t('JoinOrganization.subtitles.getHostCode')},
  ],
  [
    UserJoinOrganizationStep.ProvideGuestCode,
    {title: t('JoinOrganization.titles.provideGuestCode'), subtitle: t('JoinOrganization.subtitles.provideGuestCode')},
  ],
  [
    UserJoinOrganizationStep.GetUserInfo,
    {title: t('JoinOrganization.titles.getUserInfo'), subtitle: t('JoinOrganization.subtitles.getUserInfo')},
  ],
  [
    UserJoinOrganizationStep.GetPassword,
    {title: t('JoinOrganization.titles.getPassword'), subtitle: t('JoinOrganization.subtitles.getPassword')},
  ],
  [
    UserJoinOrganizationStep.Finish,
    {title: t('JoinOrganization.titles.finish', {org: ''}), subtitle: t('JoinOrganization.subtitles.finish')},
  ],
]);

function selectHostSas(_code: string | null): void {
  nextStep();
}

function getNextButtonText(): string {
  if (pageStep.value === UserJoinOrganizationStep.GetUserInfo) {
    return t('JoinOrganization.validateUserInfo');
  } else if (pageStep.value === UserJoinOrganizationStep.GetPassword) {
    return t('JoinOrganization.createDevice');
  } else if (pageStep.value === UserJoinOrganizationStep.Finish) {
    return t('JoinOrganization.logIn');
  } else if (pageStep.value === UserJoinOrganizationStep.WaitForHost) {
    return t('JoinOrganization.understand');
  }

  return '';
}

const nextButtonIsVisible = computed(() => {
  return (
    pageStep.value === UserJoinOrganizationStep.WaitForHost && !waitingForHost.value
    || pageStep.value === UserJoinOrganizationStep.GetUserInfo && !waitingForHost.value
    || pageStep.value === UserJoinOrganizationStep.GetPassword
    || pageStep.value === UserJoinOrganizationStep.Finish
  );
});

const canGoForward = computed(() => {
  const currentPage = getCurrentPage();

  if (
    currentPage && currentPage.value
    && (pageStep.value === UserJoinOrganizationStep.GetUserInfo
    || pageStep.value === UserJoinOrganizationStep.GetPassword)
  ) {
    return currentPage.value.areFieldsCorrect();
  }
  return true;
});

function getCurrentPage(): Ref<any> {
  switch(pageStep.value) {
    case UserJoinOrganizationStep.WaitForHost: {
      return waitPage;
    }
    case UserJoinOrganizationStep.GetHostSasCode: {
      return hostCodePage;
    }
    case UserJoinOrganizationStep.ProvideGuestCode: {
      return guestCodePage;
    }
    case UserJoinOrganizationStep.GetUserInfo: {
      return userInfoPage;
    }
    case UserJoinOrganizationStep.GetPassword: {
      return passwordPage;
    }
    default: {
      return ref(null);
    }
  }
}

function cancelModal(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Cancel);
}

function mockCreateDevice(): AvailableDevice {
  const humanHandle = `${userInfoPage.value.fullName} <${userInfoPage.value.email}>`;
  const deviceName = userInfoPage.value.deviceName;
  console.log(
    `Creating device ${claimUserLink?.org}, user ${humanHandle}, device ${deviceName}, backend ${props.invitationLink}`,
  );
  const device: AvailableDevice = {
    organizationId: claimUserLink?.org || '',
    humanHandle: humanHandle,
    deviceLabel: deviceName,
    keyFilePath: 'key_file_path',
    deviceId: 'device1@device1',
    slug: 'slug1',
    ty: {tag: 'Password'},
  };

  return device;
}

function mockSaveDeviceWithPassword(_device: AvailableDevice | null, _password: string): void {
  console.log(`Saving device with password ${passwordPage.value.password}`);
}

function nextStep(): void {
  if (pageStep.value === UserJoinOrganizationStep.GetPassword) {
    mockSaveDeviceWithPassword(newDevice, passwordPage.value.password);
  } else if (pageStep.value === UserJoinOrganizationStep.GetUserInfo) {
    waitingForHost.value = true;
    window.setTimeout(hostHasValidated, OTHER_USER_WAITING_TIME);
    return;
  } else if (pageStep.value === UserJoinOrganizationStep.Finish) {
    modalController.dismiss({ device: newDevice, password: passwordPage.value.password }, MsModalResult.Confirm);
    return;
  }

  pageStep.value = pageStep.value + 1;

  if (pageStep.value === UserJoinOrganizationStep.ProvideGuestCode) {
    waitingForHost.value = true;
    window.setTimeout(hostHasEnteredCode, OTHER_USER_WAITING_TIME);
  }
  if (pageStep.value === UserJoinOrganizationStep.WaitForHost) {
    waitingForHost.value = true;
    claimerRetrieveInfo();
  }
}

function hostHasEnteredCode(): void {
  waitingForHost.value = false;
  nextStep();
}

function hostHasValidated(): void {
  waitingForHost.value = false;
  newDevice = mockCreateDevice();
  pageStep.value = pageStep.value + 1;
}

onMounted(() => {
  claimerRetrieveInfo();
});

function claimerInfoRetrieved(): void {
  userInfoPage.value.email = 'gordon.freeman@blackmesa.nm';
  waitingForHost.value = false;
}

function claimerRetrieveInfo(): void {
  // Simulate host starting the process
  window.setTimeout(claimerInfoRetrieved, OTHER_USER_WAITING_TIME);
}
</script>

<style lang="scss" scoped>
.modal {
  padding: 3.5rem;
  justify-content: start;

  &.wizardTrue {
    padding-top: 2.5rem;
  }
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

.orga-name {
  display: flex;
  flex-direction: column;

  &-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
}

.label-waiting {
  color: var(--parsec-color-light-secondary-grey);
  font-style: italic;
  padding-left: 2em;
  padding-right: 2em;
}

.guest-code {
  margin: 4.7rem auto;
}

.spinner-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
}
</style>

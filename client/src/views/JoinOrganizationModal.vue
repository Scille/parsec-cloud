<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <wizard-stepper
      v-if="pageStep > JoinOrganizationStep.WaitForHost"
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
    <div class="modal wizardTrue">
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
        <!-- part 1 (orga name)-->
        <div
          v-show="pageStep === JoinOrganizationStep.JoinLink"
          class="step orga-name"
        >
          <organization-link-page
            ref="joinPage"
          />
        </div>

        <!-- part 2 (wait for host)-->
        <div
          v-show="pageStep === JoinOrganizationStep.WaitForHost"
          class="step orga-name"
        >
          <wait-for-host-page
            ref="waitPage"
            :org-name="'My Org'"
          />
        </div>

        <!-- part 3 (host code)-->
        <div
          v-show="pageStep === JoinOrganizationStep.GetHostSasCode"
          class="step"
        >
          <sas-code-choice
            ref="hostCodePage"
            :choices="['ABCD', 'EFGH', 'IJKL', 'MNOP']"
            @select="selectHostSas($event)"
          />
        </div>

        <!-- part 4 (guest code)-->
        <div
          v-show="pageStep === JoinOrganizationStep.ProvideGuestCode"
          class="step guest-code"
        >
          <sas-code-provide
            ref="guestCodePage"
            :code="'ABCD'"
          />
        </div>

        <!-- part 5 (user info)-->
        <div
          v-show="pageStep === JoinOrganizationStep.GetUserInfo"
          class="step"
          id="get-user-info"
        >
          <user-information-page
            ref="userInfoPage"
            :email-enabled="false"
            :device-enabled="!waitingForHost"
            :name-enabled="!waitingForHost"
          />
        </div>
        <!-- part 6 (get password)-->
        <div
          v-show="pageStep === JoinOrganizationStep.GetPassword"
          class="step"
          id="get-password"
        >
          <choose-password
            ref="passwordPage"
          />
        </div>
        <!-- part 7 (finish the process)-->
        <div
          v-show="pageStep === JoinOrganizationStep.Finish"
          class="step"
        >
          <finish-process
            ref="finishPage"
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
  modalController
} from '@ionic/vue';

import {
  close
} from 'ionicons/icons';
import { ref, Ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import WizardStepper from '@/components/WizardStepper.vue';
import WaitForHostPage from '@/components/JoinOrganization/WaitForHostPage.vue';
import OrganizationLinkPage from '@/components/JoinOrganization/OrganizationLinkPage.vue';
import SasCodeProvide from '@/components/SasCodeProvide.vue';
import SasCodeChoice from '@/components/SasCodeChoice.vue';
import UserInformationPage from '@/components/UserInformationPage.vue';
import ChoosePassword from '@/components/ChoosePassword.vue';
import FinishProcess from '@/components/JoinOrganization/FinishProcess.vue';
import { AvailableDevice } from '@/plugins/libparsec/definitions';
import { ModalResultCode } from '@/common/constants';

enum JoinOrganizationStep {
  JoinLink = 0,
  WaitForHost = 1,
  GetHostSasCode = 2,
  ProvideGuestCode = 3,
  GetUserInfo = 4,
  GetPassword = 5,
  Finish = 6
}

// Used to simulate host interaction
const OTHER_USER_WAITING_TIME = 500;

const { t } = useI18n();

const pageStep = ref(JoinOrganizationStep.JoinLink);
const joinPage = ref();
const waitPage = ref();
const hostCodePage = ref();
const guestCodePage = ref();
const userInfoPage = ref();
const passwordPage = ref();

let newDevice: AvailableDevice | null = null;

const props = defineProps<{
  invitationLink?: string
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
    org: 'My Org'
  };
}

let claimUserLink: ClaimUserLink | null = null;

const waitingForHost = ref(false);

interface Title {
  title: string,
  subtitle: string
}

const titles = new Map<JoinOrganizationStep, Title>([
  [
    JoinOrganizationStep.JoinLink,
    {title: t('JoinOrganization.titles.joinOrga'), subtitle: t('JoinOrganization.subtitles.joinOrga')}
  ],
  [
    JoinOrganizationStep.WaitForHost,
    {title: t('JoinOrganization.titles.waitForHost'), subtitle: t('JoinOrganization.subtitles.waitForHost')}
  ],
  [
    JoinOrganizationStep.GetHostSasCode,
    {title: t('JoinOrganization.titles.getHostCode'), subtitle: t('JoinOrganization.subtitles.getHostCode')}
  ],
  [
    JoinOrganizationStep.ProvideGuestCode,
    {title: t('JoinOrganization.titles.provideGuestCode'), subtitle: t('JoinOrganization.subtitles.provideGuestCode')}
  ],
  [
    JoinOrganizationStep.GetUserInfo,
    {title: t('JoinOrganization.titles.getUserInfo'), subtitle: t('JoinOrganization.subtitles.getUserInfo')}
  ],
  [
    JoinOrganizationStep.GetPassword,
    {title: t('JoinOrganization.titles.getPassword'), subtitle: t('JoinOrganization.subtitles.getPassword')}
  ],
  [
    JoinOrganizationStep.Finish,
    {title: t('JoinOrganization.titles.finish', {org: ''}), subtitle: t('JoinOrganization.subtitles.finish')}
  ]
]);

function selectHostSas(_code: string | null): void {
  nextStep();
}

function getNextButtonText(): string {
  if (pageStep.value === JoinOrganizationStep.GetUserInfo) {
    return t('JoinOrganization.validateUserInfo');
  } else if (pageStep.value === JoinOrganizationStep.GetPassword) {
    return t('JoinOrganization.createDevice');
  } else if (pageStep.value === JoinOrganizationStep.Finish) {
    return t('JoinOrganization.logIn');
  } else if (pageStep.value === JoinOrganizationStep.WaitForHost) {
    return t('JoinOrganization.understand');
  } else if (pageStep.value === JoinOrganizationStep.JoinLink) {
    return t('JoinOrganization.start');
  }

  return '';
}

const nextButtonIsVisible = computed(() => {
  return (
    pageStep.value === JoinOrganizationStep.JoinLink
    || pageStep.value === JoinOrganizationStep.WaitForHost && !waitingForHost.value
    || pageStep.value === JoinOrganizationStep.GetUserInfo && !waitingForHost.value
    || pageStep.value === JoinOrganizationStep.GetPassword
    || pageStep.value === JoinOrganizationStep.Finish
  );
});

const canGoForward = computed(() => {
  const currentPage = getCurrentPage();

  if (
    currentPage && currentPage.value
    && (pageStep.value === JoinOrganizationStep.JoinLink
    || pageStep.value === JoinOrganizationStep.GetUserInfo
    || pageStep.value === JoinOrganizationStep.GetPassword)
  ) {
    return currentPage.value.areFieldsCorrect();
  }
  return true;
});

function getCurrentPage(): Ref<any> {
  switch(pageStep.value) {
    case JoinOrganizationStep.JoinLink: {
      return joinPage;
    }
    case JoinOrganizationStep.WaitForHost: {
      return waitPage;
    }
    case JoinOrganizationStep.GetHostSasCode: {
      return hostCodePage;
    }
    case JoinOrganizationStep.ProvideGuestCode: {
      return guestCodePage;
    }
    case JoinOrganizationStep.GetUserInfo: {
      return userInfoPage;
    }
    case JoinOrganizationStep.GetPassword: {
      return passwordPage;
    }
    default: {
      return ref(null);
    }
  }
}

function cancelModal(): Promise<boolean> {
  return modalController.dismiss(null, ModalResultCode.Cancel);
}

function mockCreateDevice(): AvailableDevice {
  const humanHandle = `${userInfoPage.value.fullName} <${userInfoPage.value.email}>`;
  const deviceName = userInfoPage.value.deviceName;
  console.log(
    `Creating device ${claimUserLink?.org}, user ${humanHandle}, device ${deviceName}, backend ${props.invitationLink}`
  );
  const device: AvailableDevice = {
    organizationId: claimUserLink?.org || '',
    humanHandle: humanHandle,
    deviceLabel: deviceName,
    keyFilePath: 'key_file_path',
    deviceId: 'device1@device1',
    slug: 'slug1',
    ty: {tag: 'Password'}
  };

  return device;
}

function mockSaveDeviceWithPassword(_device: AvailableDevice | null, _password: string): void {
  console.log(`Saving device with password ${passwordPage.value.password}`);
}

function nextStep(): void {
  if (pageStep.value === JoinOrganizationStep.GetPassword) {
    mockSaveDeviceWithPassword(newDevice, passwordPage.value.password);
  } else if (pageStep.value === JoinOrganizationStep.GetUserInfo) {
    waitingForHost.value = true;
    window.setTimeout(hostHasValidated, OTHER_USER_WAITING_TIME);
    return;
  } else if (pageStep.value === JoinOrganizationStep.Finish) {
    modalController.dismiss({ device: newDevice, password: passwordPage.value.password }, ModalResultCode.Confirm);
    return;
  } else if (pageStep.value === JoinOrganizationStep.JoinLink) {
    claimUserLink = parseInvitationLink(getCurrentPage().value.joinLink);
  }

  pageStep.value = pageStep.value + 1;

  if (pageStep.value === JoinOrganizationStep.ProvideGuestCode) {
    waitingForHost.value = true;
    window.setTimeout(hostHasEnteredCode, OTHER_USER_WAITING_TIME);
  }
  if (pageStep.value === JoinOrganizationStep.WaitForHost) {
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
  if (props.invitationLink) {
    claimUserLink = parseInvitationLink(props.invitationLink);
    pageStep.value = JoinOrganizationStep.WaitForHost;
    waitingForHost.value = true;
    claimerRetrieveInfo();
  }
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

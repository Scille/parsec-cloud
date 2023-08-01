<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ion-buttons
      slot="end"
      class="closeBtn-container"
    >
      <ion-button
        slot="icon-only"
        @click="cancelModal()"
        class="closeBtn"
        v-show="canClose()"
      >
        <ion-icon
          :icon="close"
          size="large"
          class="closeBtn__icon"
        />
      </ion-button>
    </ion-buttons>
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
        v-show="pageStep === CreateOrganizationStep.OrgNameStep"
        class="step orga-name"
      >
        <ms-input
          :label="$t('CreateOrganization.organizationName')"
          :placeholder="$t('CreateOrganization.organizationNamePlaceholder')"
          name="organization"
          v-model="orgName"
          id="org-name-input"
        />
      </div>

      <!-- part 2 (user info)-->
      <div
        v-show="pageStep === CreateOrganizationStep.UserInfoStep"
        class="step"
      >
        <user-information
          ref="userInfoPage"
        />
      </div>

      <!-- part 3 (server)-->
      <div
        class="step orga-server"
        v-show="pageStep === CreateOrganizationStep.ServerStep"
      >
        <choose-server
          ref="serverPage"
        />
      </div>

      <!-- part 4 (password)-->
      <div
        class="step orga-password"
        v-show="pageStep === CreateOrganizationStep.PasswordStep"
      >
        <ms-choose-password-input
          ref="passwordPage"
        />
      </div>

      <!-- part 5 (loading)-->
      <div
        class="step orga-loading"
        v-show="pageStep === CreateOrganizationStep.SpinnerStep"
      >
        <ms-spinner
          :title="$t('CreateOrganization.loading')"
          ref="spinnerPage"
        />
      </div>

      <!-- part 6 (loading) -->
      <div
        class="step orga-created"
        v-show="pageStep === CreateOrganizationStep.FinishStep"
      >
        <ms-informative-text
          :icon="caretForward"
          :text="$t('CreateOrganization.organizationCreated')"
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
          fill="clear"
          size="default"
          id="previous-button"
          @click="previousStep()"
          v-show="canGoBackward()"
        >
          {{ $t('CreateOrganization.previous') }}
          <ion-icon
            slot="start"
            :icon="chevronBack"
            size="small"
          />
        </ion-button>
        <ion-button
          fill="solid"
          size="default"
          id="next-button"
          v-show="shouldShowNextStep()"
          @click="nextStep()"
          :disabled="!canGoForward"
        >
          <span v-show="pageStep !== CreateOrganizationStep.FinishStep">
            {{ $t('CreateOrganization.next') }}
          </span>
          <span v-show="pageStep === CreateOrganizationStep.FinishStep">
            {{ $t('CreateOrganization.done') }}
          </span>
          <ion-icon
            slot="start"
            :icon="chevronForward"
            size="small"
          />
        </ion-button>
      </ion-buttons>
    </ion-footer>
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
  modalController,
} from '@ionic/vue';

import {
  chevronForward,
  chevronBack,
  caretForward,
  close,
} from 'ionicons/icons';
import { ref, Ref, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import MsInformativeText from '@/components/core/ms-text/MsInformativeText.vue';
import MsChoosePasswordInput from '@/components/core/ms-input/MsChoosePasswordInput.vue';
import ChooseServer from '@/components/organizations/ChooseServer.vue';
import UserInformation from '@/components/users/UserInformation.vue';
import MsSpinner from '@/components/core/ms-spinner/MsSpinner.vue';
import MsInput from '@/components/core/ms-input/MsInput.vue';
import { AvailableDevice } from '@/plugins/libparsec/definitions';
import { ModalResultCode } from '@/common/constants';
import { organizationValidator, Validity } from '@/common/validators';

enum CreateOrganizationStep {
  OrgNameStep = 1,
  UserInfoStep = 2,
  ServerStep = 3,
  PasswordStep = 4,
  SpinnerStep = 5,
  FinishStep = 6,
}

const { t } = useI18n();

const DEFAULT_SAAS_ADDR = 'parsec://saas.parsec.cloud/';

const pageStep = ref(CreateOrganizationStep.OrgNameStep);
const serverPage = ref();
const userInfoPage = ref();
const passwordPage = ref();
const spinnerPage = ref();
const orgName = ref('');
const device: Ref<AvailableDevice | null> = ref(null);

interface Title {
  title: string,
  subtitle?: string,
}

const titles = new Map<CreateOrganizationStep, Title>([[
  CreateOrganizationStep.OrgNameStep, {
    title: t('CreateOrganization.title.create'),
    subtitle: t('CreateOrganization.subtitles.nameYourOrg'),
  }], [
  CreateOrganizationStep.UserInfoStep, {
    title: t('CreateOrganization.title.personalDetails'),
    subtitle: t('CreateOrganization.subtitles.personalDetails'),
  }], [
  CreateOrganizationStep.ServerStep, {
    title: t('CreateOrganization.title.server'),
    subtitle: t('CreateOrganization.subtitles.server'),
  }], [
  CreateOrganizationStep.PasswordStep, {
    title: t('CreateOrganization.title.password'),
    subtitle: t('CreateOrganization.subtitles.password'),
  }], [
  CreateOrganizationStep.FinishStep, {
    title: t('CreateOrganization.title.done'),
  }],
]);

function canGoBackward(): boolean {
  return ![
    CreateOrganizationStep.OrgNameStep, CreateOrganizationStep.SpinnerStep, CreateOrganizationStep.FinishStep,
  ].includes(pageStep.value);
}

function canClose(): boolean {
  return ![
    CreateOrganizationStep.SpinnerStep, CreateOrganizationStep.FinishStep,
  ].includes(pageStep.value);
}

const canGoForward = computed(() => {
  const currentPage = getCurrentPage();

  if (pageStep.value === CreateOrganizationStep.FinishStep) {
    return true;
  }
  if (pageStep.value === CreateOrganizationStep.OrgNameStep) {
    return organizationValidator(orgName.value) === Validity.Valid;
  }
  if (!currentPage.value) {
    return false;
  }
  return (
    pageStep.value !== CreateOrganizationStep.SpinnerStep
    && currentPage.value.areFieldsCorrect()
  );
});

function shouldShowNextStep(): boolean {
  return pageStep.value !== CreateOrganizationStep.SpinnerStep;
}

function getCurrentPage(): Ref<any> {
  switch(pageStep.value) {
    case CreateOrganizationStep.UserInfoStep: {
      return userInfoPage;
    }
    case CreateOrganizationStep.ServerStep: {
      return serverPage;
    }
    case CreateOrganizationStep.PasswordStep: {
      return passwordPage;
    }
    case CreateOrganizationStep.SpinnerStep: {
      return spinnerPage;
    }
    default: {
      return ref(null);
    }
  }
}

function cancelModal(): Promise<boolean> {
  return modalController.dismiss(null, ModalResultCode.Cancel);
}

function createOrg(orgName: string, userName: string, userEmail: string, deviceName: string, backendAddr: string, password: string): void {
  console.log(
    `Creating org ${orgName}, user ${userName} <${userEmail}>, device ${deviceName}, password ${password}, backend ${backendAddr}`,
  );
  device.value = {
    organizationId: orgName,
    humanHandle: userName,
    deviceLabel: deviceName,
    keyFilePath: 'key_file_path',
    deviceId: 'device1@device1',
    slug: 'slug1',
    ty: {tag: 'Password'},
  };
  // Simulate connection to the backend
  window.setTimeout(nextStep, 2000);
}

function nextStep(): void {
  if (pageStep.value === CreateOrganizationStep.FinishStep) {
    modalController.dismiss({ device: device.value, password: passwordPage.value.password }, ModalResultCode.Confirm);
    return;
  } else {
    pageStep.value = pageStep.value + 1;
  }
  if (pageStep.value === CreateOrganizationStep.SpinnerStep) {
    const addr = serverPage.value.mode === serverPage.value.ServerMode.SaaS ? DEFAULT_SAAS_ADDR : serverPage.value.backendAddr;
    createOrg(
      orgName.value,
      userInfoPage.value.fullName,
      userInfoPage.value.email,
      userInfoPage.value.deviceName,
      addr,
      passwordPage.value.password,
    );
  }
}

function previousStep(): void {
  pageStep.value = pageStep.value - 1;
}
</script>

<style lang="scss" scoped>
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

.orga-server {
  display: flex;
  flex-direction: column;
}

.orga-password {
  display: flex;
  flex-direction: column;
}

.orga-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 1rem;
}
</style>

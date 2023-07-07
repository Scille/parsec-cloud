<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- the modal must be decomposed in view and components -->
  <!-- View: ionHeader; ionButton(closeBtn); ionFooter -->
  <!-- Components: 1 part = 1 component -->

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
        class="modal-header__title title-h2"
      >
        {{ $t('CreateOrganization.pageTitle') }}
      </ion-title>
      <ion-text
        class="modal-header__text body"
      >
        {{ pageStep }}
      </ion-text>
    </ion-header>
    <!-- modal content: create component for each part-->
    <div class="modal-content inner-content">
      <!-- part 1 (orga name)-->
      <div
        v-show="pageStep === CreateOrganizationStep.OrgNameStep"
        class="step orga-name"
      >
        <organization-name-page
          ref="orgPage"
        />
      </div>

      <!-- part 2 (user info)-->
      <div
        v-show="pageStep === CreateOrganizationStep.UserInfoStep"
        class="step"
      >
        <user-information-page
          ref="userInfoPage"
        />
      </div>

      <!-- part 3 (server)-->
      <div
        class="step orga-server"
        v-show="pageStep === CreateOrganizationStep.ServerStep"
      >
        <choose-server-page
          ref="serverPage"
        />
      </div>

      <!-- part 4 (password)-->
      <div
        class="step orga-password"
        v-show="pageStep === CreateOrganizationStep.PasswordStep"
      >
        <choose-password
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
        <informative-text
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
  modalController
} from '@ionic/vue';

import {
  chevronForward,
  chevronBack,
  caretForward,
  close
} from 'ionicons/icons';
import { ref, Ref, computed } from 'vue';
import InformativeText from '@/components/InformativeText.vue';
import ChoosePassword from '@/components/ChoosePassword.vue';
import OrganizationNamePage from '@/components/CreateOrganization/OrganizationNamePage.vue';
import ChooseServerPage from '@/components/CreateOrganization/ChooseServerPage.vue';
import UserInformationPage from '@/components/UserInformationPage.vue';
import MsSpinner from '@/components/MsSpinner.vue';
import { AvailableDevice } from '@/plugins/libparsec/definitions';

enum CreateOrganizationStep {
  OrgNameStep = 1,
  UserInfoStep = 2,
  ServerStep = 3,
  PasswordStep = 4,
  SpinnerStep = 5,
  FinishStep = 6
}

const DEFAULT_SAAS_ADDR = 'parsec://saas.parsec.cloud/';

const pageStep = ref(CreateOrganizationStep.OrgNameStep);
const serverPage = ref();
const orgPage = ref();
const userInfoPage = ref();
const passwordPage = ref();
const spinnerPage = ref();

const device: Ref<AvailableDevice | null> = ref(null);

function canGoBackward(): boolean {
  return ![
    CreateOrganizationStep.OrgNameStep, CreateOrganizationStep.SpinnerStep, CreateOrganizationStep.FinishStep
  ].includes(pageStep.value);
}

function canClose(): boolean {
  return ![
    CreateOrganizationStep.SpinnerStep, CreateOrganizationStep.FinishStep
  ].includes(pageStep.value);
}

const canGoForward = computed(() => {
  const currentPage = getCurrentPage();

  if (pageStep.value === CreateOrganizationStep.FinishStep) {
    return true;
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
    case CreateOrganizationStep.OrgNameStep: {
      return orgPage;
    }
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
  return modalController.dismiss(null, 'cancel');
}

function createOrg(orgName: string, userName: string, userEmail: string, deviceName: string, backendAddr: string, password: string): void {
  console.log(
    `Creating org ${orgName}, user ${userName} <${userEmail}>, device ${deviceName}, password ${password}, backend ${backendAddr}`
  );
  device.value = {
    organizationId: orgName,
    humanHandle: userName,
    deviceLabel: deviceName,
    keyFilePath: 'key_file_path',
    deviceId: 'device_id',
    slug: 'slug1',
    ty: {tag: 'Password'}
  };
  // Simulate connection to the backend
  window.setTimeout(nextStep, 2000);
}

function nextStep(): void {
  if (pageStep.value === CreateOrganizationStep.FinishStep) {
    modalController.dismiss({ device: device.value, password: passwordPage.value.password }, 'confirm');
    return;
  } else {
    pageStep.value = pageStep.value + 1;
  }
  if (pageStep.value === CreateOrganizationStep.SpinnerStep) {
    const addr = serverPage.value.mode === serverPage.value.ServerMode.SaaS ? DEFAULT_SAAS_ADDR : serverPage.value.backendAddr;
    createOrg(
      orgPage.value.orgName,
      userInfoPage.value.fullName,
      userInfoPage.value.email,
      userInfoPage.value.deviceName,
      addr,
      passwordPage.value.password
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

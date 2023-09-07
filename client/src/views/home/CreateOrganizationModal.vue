<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal-stepper">
    <div class="modal">
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
        <!-- part 1 (org name)-->
        <div
          v-show="pageStep === CreateOrganizationStep.OrgNameStep"
          class="step org-name"
        >
          <ms-input
            :label="$t('CreateOrganization.organizationName')"
            :placeholder="$t('CreateOrganization.organizationNamePlaceholder')"
            name="organization"
            id="org-name-input"
            v-model="orgName"
          />
        </div>

        <!-- part 2 (user info)-->
        <div
          v-show="pageStep === CreateOrganizationStep.UserInfoStep"
          class="step user-info"
        >
          <user-information ref="userInfo" />
        </div>

        <!-- part 3 (server)-->
        <div
          class="step org-server"
          v-show="pageStep === CreateOrganizationStep.ServerStep"
        >
          <choose-server ref="serverChoice" />
        </div>

        <!-- part 4 (password)-->
        <div
          class="step org-password"
          v-show="pageStep === CreateOrganizationStep.PasswordStep"
        >
          <ms-choose-password-input ref="passwordChoice" />
        </div>

        <!-- part 5 (summary) -->
        <div
          class="step org-summary"
          v-show="pageStep === CreateOrganizationStep.SummaryStep"
        >
          <summary-step
            v-if="orgInfo"
            ref="summaryInfo"
            :organization="orgInfo.orgName"
            :fullname="orgInfo.userName"
            :email="orgInfo.email"
            :device-name="orgInfo.deviceName"
            :server-mode="orgInfo.serverMode"
            :server-addr="orgInfo.serverAddr"
            @update-request="onUpdateRequested"
          />
        </div>
        <!-- part 6 (loading)-->
        <div
          class="step org-loading"
          v-show="pageStep === CreateOrganizationStep.SpinnerStep"
        >
          <ms-spinner :title="$t('CreateOrganization.loading')" />
        </div>

        <!-- part 7 (loading) -->
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
            {{ $t('CreateOrganization.button.previous') }}
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
            <span>
              {{ getNextButtonText() }}
            </span>
            <ion-icon
              v-show="pageStep !== CreateOrganizationStep.SummaryStep"
              slot="start"
              :icon="chevronForward"
              size="small"
            />
            <ion-icon
              v-show="pageStep === CreateOrganizationStep.SummaryStep"
              slot="start"
              :icon="checkmarkDone"
              size="small"
            />
          </ion-button>
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
  modalController,
} from '@ionic/vue';

import {
  chevronForward,
  chevronBack,
  caretForward,
  checkmarkDone,
  close,
} from 'ionicons/icons';
import { ref, Ref, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import MsInformativeText from '@/components/core/ms-text/MsInformativeText.vue';
import MsChoosePasswordInput from '@/components/core/ms-input/MsChoosePasswordInput.vue';
import UserInformation from '@/components/users/UserInformation.vue';
import ChooseServer from '@/components/organizations/ChooseServer.vue';
import { ServerMode } from '@/components/organizations/ChooseServer.vue';
import SummaryStep from '@/views/home/SummaryStep.vue';
import { OrgInfo } from '@/views/home/SummaryStep.vue';
import MsSpinner from '@/components/core/ms-spinner/MsSpinner.vue';
import MsInput from '@/components/core/ms-input/MsInput.vue';
import { AvailableDevice, DeviceFileType } from '@/plugins/libparsec/definitions';
import { MsModalResult } from '@/components/core/ms-modal/MsModal.vue';
import { organizationValidator, Validity } from '@/common/validators';

enum CreateOrganizationStep {
  OrgNameStep = 1,
  UserInfoStep = 2,
  ServerStep = 3,
  PasswordStep = 4,
  SummaryStep = 5,
  SpinnerStep = 6,
  FinishStep = 7,
}

const { t } = useI18n();

const DEFAULT_SAAS_ADDR = 'parsec://saas.parsec.cloud/';

const pageStep = ref(CreateOrganizationStep.OrgNameStep);
const orgName = ref('');
const userInfo = ref();
const serverChoice = ref();
const passwordChoice = ref();
const summaryInfo = ref();

const device: Ref<AvailableDevice | null> = ref(null);
const orgInfo: Ref<null | OrgInfoValues> = ref(null);

interface Title {
  title: string,
  subtitle?: string,
}

interface OrgInfoValues {
  orgName: string,
  userName: string,
  email: string,
  deviceName: string,
  serverMode: ServerMode,
  serverAddr: string,
}

const titles = new Map<CreateOrganizationStep, Title>([[
  CreateOrganizationStep.OrgNameStep, {
    title: t('CreateOrganization.title.create'),
    subtitle: t('CreateOrganization.subtitle.nameYourOrg'),
  }], [
  CreateOrganizationStep.UserInfoStep, {
    title: t('CreateOrganization.title.personalDetails'),
    subtitle: t('CreateOrganization.subtitle.personalDetails'),
  }], [
  CreateOrganizationStep.ServerStep, {
    title: t('CreateOrganization.title.server'),
    subtitle: t('CreateOrganization.subtitle.server'),
  }], [
  CreateOrganizationStep.PasswordStep, {
    title: t('CreateOrganization.title.password'),
    subtitle: t('CreateOrganization.subtitle.password'),
  }], [
  CreateOrganizationStep.SummaryStep, {
    title: t('CreateOrganization.title.overview'),
    subtitle: t('CreateOrganization.subtitle.overview'),
  }], [
  CreateOrganizationStep.FinishStep, {
    title: t('CreateOrganization.title.done'),
  }],
]);

function getNextButtonText(): string {
  if (pageStep.value === CreateOrganizationStep.SummaryStep) {
    return t('CreateOrganization.button.create');
  } else if (pageStep.value === CreateOrganizationStep.FinishStep) {
    return t('CreateOrganization.button.done');
  } else {
    return t('CreateOrganization.button.next');
  }
}

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
  const currentPage = getCurrentStep();

  if (pageStep.value === CreateOrganizationStep.FinishStep) {
    return true;
  }
  if (pageStep.value === CreateOrganizationStep.OrgNameStep) {
    return organizationValidator(orgName.value) === Validity.Valid;
  }
  if (pageStep.value === CreateOrganizationStep.UserInfoStep
    || pageStep.value === CreateOrganizationStep.PasswordStep
    || pageStep.value === CreateOrganizationStep.ServerStep
  ) {
    return currentPage.value.areFieldsCorrect();
  }
  if (!currentPage.value) {
    return false;
  }
  return true;
});

function shouldShowNextStep(): boolean {
  return pageStep.value !== CreateOrganizationStep.SpinnerStep;
}

function getCurrentStep(): Ref<any> {
  switch (pageStep.value) {
    case CreateOrganizationStep.OrgNameStep: {
      return orgName;
    }
    case CreateOrganizationStep.UserInfoStep: {
      return userInfo;
    }
    case CreateOrganizationStep.ServerStep: {
      return serverChoice;
    }
    case CreateOrganizationStep.PasswordStep: {
      return passwordChoice;
    }
    case CreateOrganizationStep.SummaryStep: {
      return summaryInfo;
    }
    default: {
      return ref(null);
    }
  }
}

function cancelModal(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Cancel);
}

function createOrg(orgName: string, userName: string, userEmail: string, deviceName: string, backendAddr: string, password: string): void {
  console.log(
    `Creating org ${orgName}, user ${userName} <${userEmail}>, device ${deviceName}, password ${password}, backend ${backendAddr}`,
  );
  device.value = {
    organizationId: orgName,
    humanHandle: { label: userName, email: userEmail },
    deviceLabel: deviceName,
    keyFilePath: 'key_file_path',
    deviceId: 'device1@device1',
    slug: 'slug1',
    ty: DeviceFileType.Password,
  };
  // Simulate connection to the backend
  window.setTimeout(nextStep, 2000);
}

function nextStep(): void {
  if (pageStep.value === CreateOrganizationStep.FinishStep) {
    modalController.dismiss({ device: device.value, password: passwordChoice.value.password }, MsModalResult.Confirm);
    return;
  } else {
    pageStep.value = pageStep.value + 1;
  }
  if (pageStep.value === CreateOrganizationStep.SpinnerStep) {
    const addr = serverChoice.value.mode === serverChoice.value.ServerMode.SaaS ? DEFAULT_SAAS_ADDR : serverChoice.value.backendAddr;
    createOrg(
      orgName.value,
      userInfo.value.fullName,
      userInfo.value.email,
      userInfo.value.deviceName,
      addr,
      passwordChoice.value.password,
    );
  }

  if (pageStep.value === CreateOrganizationStep.SummaryStep) {
    orgInfo.value = {
      orgName: orgName.value,
      userName: userInfo.value.fullName,
      email: userInfo.value.email,
      deviceName: userInfo.value.deviceName,
      serverMode: serverChoice.value.mode,
      serverAddr: serverChoice.value.mode === serverChoice.value.ServerMode.SaaS ? DEFAULT_SAAS_ADDR : serverChoice.value.backendAddr,
    };
  }
}

function previousStep(): void {
  pageStep.value = pageStep.value - 1;
}

function onUpdateRequested(info: OrgInfo): void {
  if (info === OrgInfo.Organization) {
    pageStep.value = CreateOrganizationStep.OrgNameStep;
  } else if (info === OrgInfo.UserInfo) {
    pageStep.value = CreateOrganizationStep.UserInfoStep;
  } else if (info === OrgInfo.ServerMode) {
    pageStep.value = CreateOrganizationStep.ServerStep;
  }
}
</script>

<style lang="scss" scoped>
.org-name {
  display: flex;
  flex-direction: column;
}

.org-server {
  display: flex;
  flex-direction: column;
}

.org-password {
  display: flex;
  flex-direction: column;
}

.org-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 1rem;
}
</style>

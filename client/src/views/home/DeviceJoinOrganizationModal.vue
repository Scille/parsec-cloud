<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal-stepper">
    <ms-wizard-stepper
      v-show="pageStep > DeviceJoinOrganizationStep.Information && pageStep < DeviceJoinOrganizationStep.Finish"
      :current-index="pageStep - 1"
      :titles="[
        $t('ClaimDeviceModal.stepper.GetHostCode'),
        $t('ClaimDeviceModal.stepper.ProvideGuestCode'),
        $t('ClaimDeviceModal.stepper.password'),
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
      :class="{ wizardTrue: pageStep > DeviceJoinOrganizationStep.Information && pageStep != DeviceJoinOrganizationStep.Finish }"
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
        <!-- part 0 (manage by JoinByLink component)-->
        <!-- part 1 (information message join)-->
        <div
          v-show="pageStep === DeviceJoinOrganizationStep.Information"
          class="step orga-name"
        >
          <information-join-device />
        </div>

        <!-- part 2 (host code)-->
        <div
          v-show="pageStep === DeviceJoinOrganizationStep.GetHostSasCode"
          class="step"
        >
          <sas-code-choice
            :choices="['ABCD', 'EFGH', 'IJKL', 'MNOP']"
            @select="selectHostSas($event)"
          />
        </div>

        <!-- part 3 (guest code)-->
        <div
          v-show="pageStep === DeviceJoinOrganizationStep.ProvideGuestCode"
          class="step guest-code"
        >
          <sas-code-provide :code="'ABCD'" />
        </div>

        <!-- part 4 (get password)-->
        <div
          v-show="pageStep === DeviceJoinOrganizationStep.Password"
          class="step"
          id="get-password"
        >
          <ms-choose-password-input ref="passwordPage" />
        </div>
        <!-- part 5 (finish the process)-->
        <div
          v-show="pageStep === DeviceJoinOrganizationStep.Finish"
          class="step"
        >
          <ms-informative-text
            :icon="checkmarkCircle"
            :text="$t('ClaimDeviceModal.subtitles.done')"
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
            :disabled="!canGoForward"
            @click="nextStep()"
          >
            <span>
              {{ getNextButtonText() }}
            </span>
          </ion-button>
          <div
            v-show="waitingForHost"
            class="spinner-container"
          >
            <ion-label class="label-waiting">
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
  checkmarkCircle,
} from 'ionicons/icons';
import { ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import MsWizardStepper from '@/components/core/ms-stepper/MsWizardStepper.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import InformationJoinDevice from '@/views/home/InformationJoinDeviceStep.vue';
import MsInformativeText from '@/components/core/ms-text/MsInformativeText.vue';
import { AvailableDevice, DeviceFileType } from '@/plugins/libparsec';
import { MsModalResult } from '@/components/core/ms-modal/MsModal.vue';
import MsChoosePasswordInput from '@/components/core/ms-input/MsChoosePasswordInput.vue';
import { asyncComputed } from '@/common/asyncComputed';

enum DeviceJoinOrganizationStep {
  Information = 0,
  GetHostSasCode = 1,
  ProvideGuestCode = 2,
  Password = 3,
  Finish = 4,
}

const OTHER_USER_WAITING_TIME = 300;

const { t } = useI18n();

const pageStep = ref(DeviceJoinOrganizationStep.Information);
const passwordPage = ref();

let newDevice: AvailableDevice | null = null;

const props = defineProps<{
  invitationLink: string
}>();

interface ClaimDeviceLink {
  backendAddr: string,
  token: string,
  org: string
}

// Will be done in Rust
function parseInvitationLink(link: string): ClaimDeviceLink {
  return {
    backendAddr: link,
    token: 'aaa',
    org: 'My Org',
  };
}

const claimDeviceLink = parseInvitationLink(props.invitationLink);
const waitingForHost = ref(false);

interface Title {
  title: string,
  subtitle?: string,
}

const titles = new Map<DeviceJoinOrganizationStep, Title>([[
  DeviceJoinOrganizationStep.Information, {
    title: t('ClaimDeviceModal.titles.claimDevice'),
  }], [
  DeviceJoinOrganizationStep.GetHostSasCode, {
    title: t('ClaimDeviceModal.titles.getCode'),
    subtitle: t('ClaimDeviceModal.subtitles.getCode'),
  }], [
  DeviceJoinOrganizationStep.ProvideGuestCode, {
    title: t('ClaimDeviceModal.titles.provideCode'),
    subtitle: t('ClaimDeviceModal.subtitles.provideCode'),
  }], [
  DeviceJoinOrganizationStep.Password, {
    title: t('ClaimDeviceModal.titles.password'), subtitle: t('ClaimDeviceModal.subtitles.password'),
  }], [
  DeviceJoinOrganizationStep.Finish, {
    title: t('ClaimDeviceModal.titles.done', { org: '' }),
  }],
]);

function selectHostSas(_code: string | null): void {
  nextStep();
}

function getNextButtonText(): string {
  if (pageStep.value === DeviceJoinOrganizationStep.Information) {
    return t('ClaimDeviceModal.buttons.understand');
  } else if (pageStep.value === DeviceJoinOrganizationStep.Password) {
    return t('ClaimDeviceModal.buttons.password');
  } else if (pageStep.value === DeviceJoinOrganizationStep.Finish) {
    return t('ClaimDeviceModal.buttons.login');
  }

  return '';
}

const nextButtonIsVisible = computed(() => {
  return (
    pageStep.value === DeviceJoinOrganizationStep.Information
    || pageStep.value === DeviceJoinOrganizationStep.Password
    || pageStep.value === DeviceJoinOrganizationStep.Finish
  );
});
const canGoForward = asyncComputed(async () => {
  if (pageStep.value === DeviceJoinOrganizationStep.Password) {
    return await passwordPage.value.areFieldsCorrect();
  }
  return true;
});

function cancelModal(): Promise<boolean> {
  if (pageStep.value === DeviceJoinOrganizationStep.Finish) {
    return modalController.dismiss({ device: newDevice, password: passwordPage.value.password }, MsModalResult.Confirm);
  } else {
    return modalController.dismiss(null, MsModalResult.Cancel);
  }
}

function mockCreateDevice(): AvailableDevice {
  const humanHandle = {
    label: 'Louis Dark',
    email: 'louis@dark.co',
  };
  const deviceName = 'myDevice';
  console.log(
    `Creating device ${claimDeviceLink?.org}, user ${humanHandle}, device ${deviceName}, backend ${props.invitationLink}`,
  );
  const device: AvailableDevice = {
    organizationId: claimDeviceLink?.org || '',
    humanHandle: humanHandle,
    deviceLabel: deviceName,
    keyFilePath: 'key_file_path',
    deviceId: 'device1@device1',
    slug: 'slug1',
    ty: DeviceFileType.Password,
  };

  return device;
}

function mockSaveDeviceWithPassword(_device: AvailableDevice | null, _password: string): void {
  console.log(`Saving device with password ${passwordPage.value.password}`);
}

function nextStep(): void {
  if (pageStep.value === DeviceJoinOrganizationStep.Password) {
    newDevice = mockCreateDevice();
    mockSaveDeviceWithPassword(newDevice, passwordPage.value.password);
  } else if (pageStep.value === DeviceJoinOrganizationStep.Finish) {
    modalController.dismiss({ device: newDevice, password: passwordPage.value.password }, MsModalResult.Confirm);
    return;
  }

  pageStep.value = pageStep.value + 1;

  if (pageStep.value === DeviceJoinOrganizationStep.ProvideGuestCode) {
    waitingForHost.value = true;
    window.setTimeout(hostHasEnteredCode, OTHER_USER_WAITING_TIME);
  }
}

function hostHasEnteredCode(): void {
  waitingForHost.value = false;
  nextStep();
}

onMounted(() => {
  claimerRetrieveInfo();
});

function claimerInfoRetrieved(): void {
  waitingForHost.value = false;
}

function claimerRetrieveInfo(): void {
  // Simulate host starting the process
  window.setTimeout(claimerInfoRetrieved, OTHER_USER_WAITING_TIME);
}
</script>

<style scoped lang="scss">
.orga-name {
  display: flex;
  flex-direction: column;
}

.guest-code {
  margin: 4.7rem auto;
}
</style>

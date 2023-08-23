<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ms-modal-stepper
      :stepper="{
        show: pageStep > DeviceJoinOrganizationStep.Information && pageStep < DeviceJoinOrganizationStep.Finish,
        currentIndex: pageStep - 1,
        titles: titleStepper,
      }"
      :spinner="{
        show: waitingForHost,
        label: $t('JoinOrganization.waitingForHost'),
      }"
      :title="title"
      :subtitle="subtitle"
      :close-button="{
        show: true,
        onClick: cancelModal,
      }"
      :confirm-button="{
        label: button,
        disabled: !canGoForward,
        onClick: nextStep,
        show: nextButtonIsVisible
      }"
    >
      <div
        class="modal"
        :class="{wizardTrue: pageStep > DeviceJoinOrganizationStep.Information && pageStep != DeviceJoinOrganizationStep.Finish}"
      >
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
            <sas-code-provide
              :code="'ABCD'"
            />
          </div>

          <!-- part 4 (get password)-->
          <div
            v-show="pageStep === DeviceJoinOrganizationStep.Password"
            class="step"
            id="get-password"
          >
            <ms-choose-password-input
              ref="passwordPage"
            />
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
      </div>
    </ms-modal-stepper>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonPage,
  modalController,
} from '@ionic/vue';

import {
  checkmarkCircle,
} from 'ionicons/icons';
import { ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import InformationJoinDevice from '@/views/home/InformationJoinDeviceStep.vue';
import MsInformativeText from '@/components/core/ms-text/MsInformativeText.vue';
import { AvailableDevice } from '@/plugins/libparsec/definitions';
import { ModalResultCode } from '@/common/constants';
import MsChoosePasswordInput from '@/components/core/ms-input/MsChoosePasswordInput.vue';
import MsModalStepper from '@/components/core/ms-modal/MsModalStepper.vue';

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

const titleStepper = [
  t('ClaimDeviceModal.stepper.GetHostCode'),
  t('ClaimDeviceModal.stepper.password'),
  t('ClaimDeviceModal.stepper.done'),
];
const title = computed(() => textHeaderFooter.get(pageStep.value)?.title || '');
const subtitle = computed(() => textHeaderFooter.get(pageStep.value)?.subtitle);
const button = computed(() => textHeaderFooter.get(pageStep.value)?.button || '');

interface TextHeaderFooter {
  title: string,
  subtitle?: string,
  button?: string
}

const textHeaderFooter = new Map<DeviceJoinOrganizationStep, TextHeaderFooter>([[
  DeviceJoinOrganizationStep.Information, {
    title: t('ClaimDeviceModal.titles.claimDevice'),
    button: t('ClaimDeviceModal.buttons.understand'),
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
    title: t('ClaimDeviceModal.titles.password'),
    subtitle: t('ClaimDeviceModal.subtitles.password'),
    button: t('ClaimDeviceModal.buttons.password'),
  }], [
  DeviceJoinOrganizationStep.Finish, {
    title: t('ClaimDeviceModal.titles.done', {org: ''}),
    button: t('ClaimDeviceModal.buttons.login'),
  }],
]);

function selectHostSas(_code: string | null): void {
  nextStep();
}

const nextButtonIsVisible = computed(() => {
  if (
    pageStep.value === DeviceJoinOrganizationStep.Information
    || pageStep.value === DeviceJoinOrganizationStep.Password
    || pageStep.value === DeviceJoinOrganizationStep.Finish
  ) {
    return true;
  } else {
    return false;
  }
});

const canGoForward = computed(() => {
  if (pageStep.value === DeviceJoinOrganizationStep.Password) {
    return passwordPage.value.areFieldsCorrect();
  }
  return true;
});

function cancelModal(): Promise<boolean> {
  if (pageStep.value === DeviceJoinOrganizationStep.Finish) {
    return modalController.dismiss({ device: newDevice, password: passwordPage.value.password }, ModalResultCode.Confirm);
  } else {
    return modalController.dismiss(null, ModalResultCode.Cancel);
  }
}

function mockCreateDevice(): AvailableDevice {
  const humanHandle = 'Louis Dark';
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
    ty: {tag: 'Password'},
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
    modalController.dismiss({ device: newDevice, password: passwordPage.value.password }, ModalResultCode.Confirm);
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
.modal .wizardTrue {
  padding-top: 2.5rem;
}

.modal-content {
  --background: transparent;

  .step {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
}

.orga-name {
  display: flex;
  flex-direction: column;
}

.guest-code {
  margin: 4.7rem auto;
}
</style>

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
          class="modal-header__title title-h2"
        >
          {{ getTitleAndSubtitle().title }}
        </ion-title>
        <ion-text
          v-if="getTitleAndSubtitle().subtitle"
          class="modal-header__text body"
        >
          {{ getTitleAndSubtitle().subtitle }}
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
            :choices="claimer.SASCodeChoices"
            @select="selectHostSas($event)"
          />
        </div>

        <!-- part 3 (guest code)-->
        <div
          v-show="pageStep === DeviceJoinOrganizationStep.ProvideGuestCode"
          class="step guest-code"
        >
          <sas-code-provide
            :code="claimer.guestSASCode"
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
            @on-enter-keyup="nextStep()"
          />
          <ms-input
            :label="$t('CreateOrganization.deviceNameInputLabel')"
            :placeholder="$t('CreateOrganization.deviceNamePlaceholder')"
            v-model="deviceName"
            name="deviceName"
            @on-enter-keyup="nextStep()"
          />
        </div>
        <!-- part 5 (finish the process)-->
        <div
          v-show="pageStep === DeviceJoinOrganizationStep.Finish"
          class="step"
        >
          <ms-informative-text
            :icon="checkmarkCircle"
          >
            {{ $t('ClaimDeviceModal.subtitles.done') }}
          </ms-informative-text>
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
import { ref, computed, onMounted, inject } from 'vue';
import { useI18n } from 'vue-i18n';
import MsWizardStepper from '@/components/core/ms-stepper/MsWizardStepper.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import InformationJoinDevice from '@/views/home/InformationJoinDeviceStep.vue';
import MsInformativeText from '@/components/core/ms-text/MsInformativeText.vue';
import { MsModalResult } from '@/components/core/ms-types';
import MsInput from '@/components/core/ms-input/MsInput.vue';
import MsChoosePasswordInput from '@/components/core/ms-input/MsChoosePasswordInput.vue';
import { asyncComputed } from '@/common/asyncComputed';
import { DeviceClaim, parseBackendAddr, ParsedBackendAddrInvitationDevice, ParsedBackendAddrTag } from '@/parsec';
import { Notification, NotificationCenter, NotificationLevel } from '@/services/notificationCenter';
import { NotificationKey } from '@/common/injectionKeys';
import { Validity, deviceNameValidator } from '@/common/validators';
import { askQuestion, Answer } from '@/components/core/ms-modal/MsQuestionModal.vue';

const notificationCenter  = inject(NotificationKey) as NotificationCenter;

enum DeviceJoinOrganizationStep {
  Information = 0,
  GetHostSasCode = 1,
  ProvideGuestCode = 2,
  Password = 3,
  Finish = 4,
}

const { t } = useI18n();

const pageStep = ref(DeviceJoinOrganizationStep.Information);
const deviceName = ref('');
const passwordPage = ref();
let backendAddr: ParsedBackendAddrInvitationDevice | null = null;
const claimer = ref(new DeviceClaim());

const props = defineProps<{
  invitationLink: string
}>();

const waitingForHost = ref(true);

interface Title {
  title: string,
  subtitle?: string,
}

function getTitleAndSubtitle(): Title {
  switch (pageStep.value) {
    case DeviceJoinOrganizationStep.Information: {
      return {
        title: t('ClaimDeviceModal.titles.claimDevice'),
      };
    }
    case DeviceJoinOrganizationStep.GetHostSasCode: {
      return {
        title: t('ClaimDeviceModal.titles.getCode'),
        subtitle: t('ClaimDeviceModal.subtitles.getCode'),
      };
    }
    case DeviceJoinOrganizationStep.ProvideGuestCode: {
      return {
        title: t('ClaimDeviceModal.titles.provideCode'),
        subtitle: t('ClaimDeviceModal.subtitles.provideCode'),
      };
    }
    case DeviceJoinOrganizationStep.Password: {
      return {
        title: t('ClaimDeviceModal.titles.password'),
        subtitle: t('ClaimDeviceModal.subtitles.password'),
      };
    }
    case DeviceJoinOrganizationStep.Finish: {
      return {
        title: t('ClaimDeviceModal.titles.done', { org: backendAddr?.organizationId || '' }),
      };
    }
  }
}

async function selectHostSas(selectedCode: string | null): Promise<void> {
  if (!selectedCode) {
    await showErrorAndRestart(t('ClaimDeviceModal.errors.noneCodeSelected'));
    return;
  }
  if (selectedCode === claimer.value.correctSASCode) {
    const result = await claimer.value.signifyTrust();
    if (result.ok) {
      nextStep();
    } else {
      await showErrorAndRestart(t('ClaimDeviceModal.errors.unexpected'));
    }
  } else {
    await showErrorAndRestart(t('ClaimDeviceModal.errors.invalidCodeSelected'));
  }
}

async function showErrorAndRestart(message: string): Promise<void> {
  notificationCenter.showToast(new Notification({
    message: message,
    level: NotificationLevel.Error,
  }));
  await restartProcess();
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
    pageStep.value === DeviceJoinOrganizationStep.Information && !waitingForHost.value
    || pageStep.value === DeviceJoinOrganizationStep.Password && !waitingForHost.value
    || pageStep.value === DeviceJoinOrganizationStep.Finish
  );
});

const canGoForward = asyncComputed(async () => {
  if (pageStep.value === DeviceJoinOrganizationStep.Password) {
    const validDeviceName = await deviceNameValidator(deviceName.value);
    return await passwordPage.value.areFieldsCorrect() && validDeviceName === Validity.Valid;
  }
  return true;
});

async function cancelModal(): Promise<boolean> {
  const answer = await askQuestion(
    t('ClaimDeviceModal.cancelConfirm'),
    t('ClaimDeviceModal.cancelConfirmSubtitle'),
    false,
  );

  if (answer === Answer.Yes) {
    await claimer.value.abort();
    return modalController.dismiss(null, MsModalResult.Cancel);
  }
  return false;
}

async function nextStep(): Promise<void> {
  if (!canGoForward.value) {
    return;
  }
  if (pageStep.value === DeviceJoinOrganizationStep.Password) {
    waitingForHost.value = true;
    const doClaimResult = await claimer.value.doClaim(
      deviceName.value,
    );
    if (doClaimResult.ok) {
      waitingForHost.value = false;
      const result = await claimer.value.finalize(passwordPage.value.password);
      if (!result.ok) {
        notificationCenter.showToast(new Notification({
          message: t('ClaimDeviceModal.errors.saveDeviceFailed'),
          level: NotificationLevel.Error,
        }));
        return;
      }
    } else {
      await showErrorAndRestart(t('ClaimDeviceModal.errors.sendDeviceInfoFailed'));
      return;
    }
  } else if (pageStep.value === DeviceJoinOrganizationStep.Finish) {
    const notification = new Notification({
      message: t('ClaimDeviceModal.successMessage'),
      level: NotificationLevel.Success,
    });
    notificationCenter.showToast(notification, {trace: true});
    await modalController.dismiss({ device: claimer.value.device, password: passwordPage.value.password }, MsModalResult.Confirm);
    return;
  }

  pageStep.value = pageStep.value + 1;

  if (pageStep.value === DeviceJoinOrganizationStep.ProvideGuestCode) {
    waitingForHost.value = true;
    const result = await claimer.value.waitHostTrust();
    if (result.ok) {
      waitingForHost.value = false;
      pageStep.value += 1;
    } else {
      await showErrorAndRestart(t('ClaimDeviceModal.errors.unexpected', {reason: result.error.tag}));
    }
  }
}

async function startProcess(): Promise<void> {
  pageStep.value = DeviceJoinOrganizationStep.Information;
  deviceName.value = '';
  waitingForHost.value = true;

  const retrieveResult = await claimer.value.retrieveInfo(props.invitationLink);

  if (!retrieveResult.ok) {
    await notificationCenter.showModal(new Notification({
      message: t('ClaimDeviceModal.errors.startFailed'),
      level: NotificationLevel.Error,
    }));
    await cancelModal();
    return;
  }

  const waitResult = await claimer.value.initialWaitHost();
  if (!waitResult.ok) {
    await notificationCenter.showModal(new Notification({
      message: t('ClaimDeviceModal.errors.startFailed'),
      level: NotificationLevel.Error,
    }));
    await cancelModal();
    return;
  }

  waitingForHost.value = false;
}

async function restartProcess(): Promise<void> {
  if (pageStep.value !== DeviceJoinOrganizationStep.Information) {
    await claimer.value.abort();
  }
  await startProcess();
}

onMounted(async () => {
  const addrResult = await parseBackendAddr(props.invitationLink);

  if (addrResult.ok && addrResult.value.tag === ParsedBackendAddrTag.InvitationDevice) {
    backendAddr = addrResult.value as ParsedBackendAddrInvitationDevice;
  }
  await startProcess();
});
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

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
import { DeviceClaim } from '@/parsec';
import { Notification, NotificationCenter, NotificationLevel } from '@/services/notificationCenter';
import { NotificationKey } from '@/common/injectionKeys';
import { Validity, deviceNameValidator } from '@/common/validators';

const notificationCenter: NotificationCenter = inject(NotificationKey)!;

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

const claimer = ref(new DeviceClaim());

const props = defineProps<{
  invitationLink: string
}>();

const waitingForHost = ref(true);

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

async function selectHostSas(selectedCode: string | null): Promise<void> {
  if (selectedCode && selectedCode === claimer.value.correctSASCode) {
    console.log('Good choice selected, next step');
    const result = await claimer.value.signifyTrust();
    if (result.ok) {
      nextStep();
    } else {
      console.log('Signify trust failed', result.error);
    }
  } else {
    if (!selectedCode) {
      console.log('None selected, back to beginning');
    } else {
      console.log('Invalid selected, back to beginning');
    }
    await claimer.value.abort();
    pageStep.value = DeviceJoinOrganizationStep.Information;
  }
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
  await claimer.value.abort();
  return modalController.dismiss(null, MsModalResult.Cancel);
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
        console.log('Failed to finalize', result.error);
      }
    } else {
      console.log('Do claim failed', doClaimResult.error);
      return;
    }
  } else if (pageStep.value === DeviceJoinOrganizationStep.Finish) {
    const notification = new Notification({
      message: t('JoinOrganization.successMessage'),
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
      console.log('Wait peer trust failed', result.error);
    }
  }
}

onMounted(async () => {
  const retrieveResult = await claimer.value.retrieveInfo(props.invitationLink);

  if (retrieveResult.ok) {
    const waitResult = await claimer.value.initialWaitHost();
    if (waitResult.ok) {
      waitingForHost.value = false;
    } else {
      console.log('Wait host failed', waitResult.error);
    }
  } else {
    console.log('Failed to retrieve info', retrieveResult.error);
  }
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

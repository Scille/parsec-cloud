<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal-stepper">
    <ms-wizard-stepper
      v-show="pageStep > GreetDeviceStep.WaitForGuest && pageStep < GreetDeviceStep.Summary"
      :current-index="getStepperIndex()"
      :titles="[$t('DevicesPage.greet.steps.hostCode'), $t('DevicesPage.greet.steps.guestCode')]"
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
    <div class="modal">
      <ion-header class="modal-header">
        <ion-title class="modal-header__title title-h2">
          {{ getTitleAndSubtitle()[0] }}
        </ion-title>
        <ion-text class="modal-header__text body">
          {{ getTitleAndSubtitle()[1] }}
        </ion-text>
      </ion-header>
      <div class="modal-content inner-content">
        <!-- waiting step -->
        <div
          v-show="pageStep === GreetDeviceStep.WaitForGuest"
          class="step"
        >
          <ms-informative-text>
            {{ $t('DevicesPage.greet.subtitles.waitForGuest1') }}
          </ms-informative-text>
          <ms-informative-text>
            {{ $t('DevicesPage.greet.subtitles.waitForGuest2') }}
          </ms-informative-text>
          <ion-buttons slot="start">
            <ion-button
              fill="solid"
              size="default"
              id="copy-link"
              @click="copyLink"
            >
              <span>
                {{ $t('DevicesPage.greet.copyLink') }}
              </span>
            </ion-button>
          </ion-buttons>
        </div>

        <!-- give code step -->
        <div
          v-show="pageStep === GreetDeviceStep.ProvideHostSasCode"
          class="step"
        >
          <sas-code-provide :code="greeter.hostSASCode" />
        </div>

        <!-- choose code step -->
        <div
          v-show="pageStep === GreetDeviceStep.GetGuestSasCode"
          class="step"
        >
          <sas-code-choice
            :choices="greeter.SASCodeChoices"
            @select="selectGuestSas"
          />
        </div>

        <!-- Waiting guest info step -->
        <div
          v-show="pageStep === GreetDeviceStep.WaitForGuestInfo"
          class="step"
        >
          <ms-informative-text>
            {{ $t('DevicesPage.greet.subtitles.getDeviceInfo') }}
          </ms-informative-text>
        </div>

        <!-- Final Step -->
        <div
          v-show="pageStep === GreetDeviceStep.Summary"
          class="step final-step"
        >
          {{
            $t('DevicesPage.greet.subtitles.summary', {
              label: greeter.requestedDeviceLabel,
            })
          }}
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
            <ion-label class="label-waiting" />
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
  IonButton,
  IonButtons,
  IonFooter,
  IonHeader,
  IonIcon,
  IonLabel,
  IonPage,
  IonSpinner,
  IonText,
  IonTitle,
  modalController,
} from '@ionic/vue';

import { Answer, MsInformativeText, MsModalResult, MsWizardStepper, askQuestion } from '@/components/core';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import { DeviceGreet } from '@/parsec';
import { Notification, NotificationKey, NotificationLevel, NotificationManager } from '@/services/notificationManager';
import { close } from 'ionicons/icons';
import { computed, inject, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';

enum GreetDeviceStep {
  WaitForGuest = 1,
  ProvideHostSasCode = 2,
  GetGuestSasCode = 3,
  WaitForGuestInfo = 4,
  Summary = 5,
}

const { t } = useI18n();

const pageStep = ref(GreetDeviceStep.WaitForGuest);
const canGoForward = ref(false);
const waitingForGuest = ref(true);
const greeter = ref(new DeviceGreet());
// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const notificationManager: NotificationManager = inject(NotificationKey)!;

function getTitleAndSubtitle(): [string, string] {
  if (pageStep.value === GreetDeviceStep.WaitForGuest) {
    return [t('DevicesPage.greet.titles.waitForGuest'), ''];
  } else if (pageStep.value === GreetDeviceStep.ProvideHostSasCode) {
    return [t('DevicesPage.greet.titles.provideHostCode'), t('DevicesPage.greet.subtitles.provideHostCode')];
  } else if (pageStep.value === GreetDeviceStep.GetGuestSasCode) {
    return [t('DevicesPage.greet.titles.getGuestCode'), t('DevicesPage.greet.subtitles.getGuestCode')];
  } else if (pageStep.value === GreetDeviceStep.WaitForGuestInfo) {
    return [t('DevicesPage.greet.titles.deviceDetails'), ''];
  } else if (pageStep.value === GreetDeviceStep.Summary) {
    return [t('DevicesPage.greet.titles.summary'), ''];
  }
  return ['', ''];
}

async function updateCanGoForward(): Promise<void> {
  if (pageStep.value === GreetDeviceStep.WaitForGuest && waitingForGuest.value === true) {
    canGoForward.value = false;
  } else {
    canGoForward.value = true;
  }
}

function getNextButtonText(): string {
  if (pageStep.value === GreetDeviceStep.WaitForGuest) {
    return t('DevicesPage.greet.actions.start');
  } else if (pageStep.value === GreetDeviceStep.Summary) {
    return t('DevicesPage.greet.actions.finish');
  }
  return '';
}

async function selectGuestSas(code: string | null): Promise<void> {
  if (!code) {
    await showErrorAndRestart(t('DevicesPage.greet.errors.noneCodeSelected'));
    return;
  }
  if (code === greeter.value.correctSASCode) {
    const result = await greeter.value.signifyTrust();
    if (result.ok) {
      await nextStep();
    } else {
      await showErrorAndRestart(t('DevicesPage.greet.errors.unexpected', { reason: result.error.tag }));
    }
  } else {
    await showErrorAndRestart(t('DevicesPage.greet.errors.invalidCodeSelected'));
  }
}

async function restartProcess(): Promise<void> {
  await greeter.value.abort();
  await startProcess();
}

async function startProcess(): Promise<void> {
  pageStep.value = GreetDeviceStep.WaitForGuest;
  waitingForGuest.value = true;
  const result = await greeter.value.startGreet();
  if (!result.ok) {
    notificationManager.showToast(
      new Notification({
        message: t('DevicesPage.greet.errors.startFailed'),
        level: NotificationLevel.Error,
      }),
    );
    await cancelModal();
    return;
  }
  const waitResult = await greeter.value.initialWaitGuest();
  if (!waitResult.ok) {
    notificationManager.showToast(
      new Notification({
        message: t('DevicesPage.greet.errors.startFailed'),
        level: NotificationLevel.Error,
      }),
    );
    await cancelModal();
    return;
  }
  waitingForGuest.value = false;
  await updateCanGoForward();
}

function getStepperIndex(): number {
  if (pageStep.value <= GreetDeviceStep.ProvideHostSasCode) {
    return 0;
  }
  return 1;
}

const nextButtonIsVisible = computed(() => {
  if (pageStep.value === GreetDeviceStep.WaitForGuest && waitingForGuest.value) {
    return false;
  } else if (pageStep.value === GreetDeviceStep.ProvideHostSasCode) {
    return false;
  } else if (pageStep.value === GreetDeviceStep.GetGuestSasCode) {
    return false;
  } else if (pageStep.value === GreetDeviceStep.WaitForGuestInfo) {
    return false;
  }
  return true;
});

async function cancelModal(): Promise<boolean> {
  if (pageStep.value === GreetDeviceStep.WaitForGuest || pageStep.value === GreetDeviceStep.Summary) {
    return await modalController.dismiss(null, MsModalResult.Cancel);
  }
  const answer = await askQuestion(t('DevicesPage.greet.cancelConfirm'), t('DevicesPage.greet.cancelConfirmSubtitle'), {
    keepMainModalHiddenOnYes: true,
    yesIsDangerous: true,
    yesText: t('DevicesPage.greet.cancelYes'),
    noText: t('DevicesPage.greet.cancelNo'),
  });

  if (answer === Answer.Yes) {
    await greeter.value.abort();
    return await modalController.dismiss(null, MsModalResult.Cancel);
  }
  return false;
}

async function showErrorAndRestart(message: string): Promise<void> {
  notificationManager.showToast(
    new Notification({
      message: message,
      level: NotificationLevel.Error,
    }),
  );
  await restartProcess();
}

async function nextStep(): Promise<void> {
  await updateCanGoForward();
  if (!canGoForward.value) {
    return;
  }
  if (pageStep.value === GreetDeviceStep.Summary) {
    notificationManager.showToast(
      new Notification({
        message: t('DevicesPage.greet.success', {
          label: greeter.value.requestedDeviceLabel,
        }),
        level: NotificationLevel.Success,
      }),
    );
    await modalController.dismiss({}, MsModalResult.Confirm);
    return;
  }

  pageStep.value = pageStep.value + 1;

  await updateCanGoForward();

  if (pageStep.value === GreetDeviceStep.ProvideHostSasCode) {
    waitingForGuest.value = true;
    const result = await greeter.value.waitGuestTrust();
    waitingForGuest.value = false;
    if (result.ok) {
      await nextStep();
    } else {
      await showErrorAndRestart(t('DevicesPage.greet.errors.unexpected', { reason: result.error.tag }));
    }
  } else if (pageStep.value === GreetDeviceStep.WaitForGuestInfo) {
    waitingForGuest.value = true;
    const result = await greeter.value.getClaimRequests();
    waitingForGuest.value = false;
    if (result.ok) {
      const createResult = await greeter.value.createDevice();
      if (!createResult.ok) {
        await showErrorAndRestart(t('DevicesPage.greet.errors.createDeviceFailed'));
        return;
      }
      await nextStep();
    } else {
      await showErrorAndRestart(t('DevicesPage.greet.errors.retrieveDeviceInfoFailed'));
    }
  }
}

async function copyLink(): Promise<void> {
  await navigator.clipboard.writeText(greeter.value.invitationLink);
  notificationManager.showToast(
    new Notification({
      message: t('DevicesPage.greet.linkCopiedToClipboard'),
      level: NotificationLevel.Info,
    }),
  );
}

onMounted(async () => {
  await greeter.value.createInvitation(true);
  await startProcess();
});
</script>

<style scoped lang="scss">
.final-step {
  width: 100%;
  display: flex;
  flex-direction: row;
  align-items: center;
  background: var(--parsec-color-light-secondary-background);
  padding: 0.75rem 1rem;
  border-radius: var(--parsec-radius-6);
  justify-content: space-between;
  color: var(--parsec-color-light-secondary-text);

  &__icon {
    font-size: 1.5rem;
    color: var(--parsec-color-light-primary-500);
  }

  &__name {
    font-weight: 500;
  }
}
</style>

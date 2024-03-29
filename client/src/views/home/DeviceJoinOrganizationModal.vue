<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page
    class="modal-stepper"
    :class="DeviceJoinOrganizationStep[pageStep]"
  >
    <ms-wizard-stepper
      v-show="pageStep > DeviceJoinOrganizationStep.Information && pageStep < DeviceJoinOrganizationStep.Finish"
      :current-index="pageStep - 1"
      :titles="[
        $t('ClaimDeviceModal.stepper.GetHostCode'),
        $t('ClaimDeviceModal.stepper.ProvideGuestCode'),
        $t('ClaimDeviceModal.stepper.Authentication'),
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
      :class="{
        wizardTrue: pageStep > DeviceJoinOrganizationStep.Information && pageStep != DeviceJoinOrganizationStep.Finish,
      }"
    >
      <ion-header class="modal-header">
        <ion-title class="modal-header__title title-h2">
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
          <sas-code-provide :code="claimer.guestSASCode" />
        </div>

        <!-- part 4 (get password)-->
        <div
          v-show="pageStep === DeviceJoinOrganizationStep.Authentication"
          class="step"
          id="get-password"
        >
          <choose-authentication ref="authChoice" />
        </div>
        <!-- part 5 (finish the process)-->
        <div
          v-show="pageStep === DeviceJoinOrganizationStep.Finish"
          class="step"
        >
          <ms-informative-text :icon="checkmarkCircle">
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

import { asyncComputed } from '@/common/asyncComputed';
import { getDefaultDeviceName } from '@/common/device';
import { Answer, MsInformativeText, MsModalResult, MsWizardStepper, askQuestion } from '@/components/core';
import ChooseAuthentication from '@/components/devices/ChooseAuthentication.vue';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import {
  AccessStrategy,
  ClaimerRetrieveInfoErrorTag,
  DeviceClaim,
  DeviceSaveStrategy,
  DeviceSaveStrategyPassword,
  DeviceSaveStrategyTag,
  ParsedParsecAddrInvitationDevice,
  ParsedParsecAddrTag,
  parseBackendAddr,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { translate } from '@/services/translation';
import InformationJoinDevice from '@/views/home/InformationJoinDeviceStep.vue';
import { checkmarkCircle, close } from 'ionicons/icons';
import { computed, inject, onMounted, ref } from 'vue';

const informationManager = inject(InformationManagerKey) as InformationManager;

enum DeviceJoinOrganizationStep {
  Information = 0,
  GetHostSasCode = 1,
  ProvideGuestCode = 2,
  Authentication = 3,
  Finish = 4,
}

const pageStep = ref(DeviceJoinOrganizationStep.Information);
let backendAddr: ParsedParsecAddrInvitationDevice | null = null;
const claimer = ref(new DeviceClaim());
const authChoice = ref();
const cancelled = ref(false);

const props = defineProps<{
  invitationLink: string;
}>();

const waitingForHost = ref(true);

interface Title {
  title: string;
  subtitle?: string;
}

function getTitleAndSubtitle(): Title {
  switch (pageStep.value) {
    case DeviceJoinOrganizationStep.Information: {
      return {
        title: translate('ClaimDeviceModal.titles.claimDevice'),
      };
    }
    case DeviceJoinOrganizationStep.GetHostSasCode: {
      return {
        title: translate('ClaimDeviceModal.titles.getCode'),
        subtitle: translate('ClaimDeviceModal.subtitles.getCode'),
      };
    }
    case DeviceJoinOrganizationStep.ProvideGuestCode: {
      return {
        title: translate('ClaimDeviceModal.titles.provideCode'),
        subtitle: translate('ClaimDeviceModal.subtitles.provideCode'),
      };
    }
    case DeviceJoinOrganizationStep.Authentication: {
      return {
        title: translate('ClaimDeviceModal.titles.authentication'),
        subtitle: translate('ClaimDeviceModal.subtitles.authentication'),
      };
    }
    case DeviceJoinOrganizationStep.Finish: {
      return {
        title: translate('ClaimDeviceModal.titles.done', {
          org: backendAddr?.organizationId || '',
        }),
      };
    }
  }
}

async function selectHostSas(selectedCode: string | null): Promise<void> {
  if (!selectedCode) {
    await showErrorAndRestart(translate('ClaimDeviceModal.errors.noneCodeSelected'));
    return;
  }
  if (selectedCode === claimer.value.correctSASCode) {
    const result = await claimer.value.signifyTrust();
    if (result.ok) {
      nextStep();
    } else {
      await showErrorAndRestart(translate('ClaimDeviceModal.errors.unexpected'));
    }
  } else {
    await showErrorAndRestart(translate('ClaimDeviceModal.errors.invalidCodeSelected'));
  }
}

async function showErrorAndRestart(message: string): Promise<void> {
  informationManager.present(
    new Information({
      message: message,
      level: InformationLevel.Error,
    }),
    PresentationMode.Toast,
  );
  await restartProcess();
}

function getNextButtonText(): string {
  if (pageStep.value === DeviceJoinOrganizationStep.Information) {
    return translate('ClaimDeviceModal.buttons.understand');
  } else if (pageStep.value === DeviceJoinOrganizationStep.Authentication) {
    return translate('ClaimDeviceModal.buttons.password');
  } else if (pageStep.value === DeviceJoinOrganizationStep.Finish) {
    return translate('ClaimDeviceModal.buttons.login');
  }

  return '';
}

const nextButtonIsVisible = computed(() => {
  return (
    (pageStep.value === DeviceJoinOrganizationStep.Information && !waitingForHost.value) ||
    (pageStep.value === DeviceJoinOrganizationStep.Authentication && !waitingForHost.value) ||
    pageStep.value === DeviceJoinOrganizationStep.Finish
  );
});

const canGoForward = asyncComputed(async () => {
  if (pageStep.value === DeviceJoinOrganizationStep.Authentication) {
    return await authChoice.value.areFieldsCorrect();
  }
  return true;
});

async function cancelModal(): Promise<boolean> {
  const answer = await askQuestion(translate('ClaimDeviceModal.cancelConfirm'), translate('ClaimDeviceModal.cancelConfirmSubtitle'), {
    yesIsDangerous: true,
    keepMainModalHiddenOnYes: true,
    yesText: translate('ClaimDeviceModal.cancelYes'),
    noText: translate('ClaimDeviceModal.cancelNo'),
  });

  if (answer === Answer.Yes) {
    cancelled.value = true;
    await claimer.value.abort();
    return modalController.dismiss(null, MsModalResult.Cancel);
  }
  return false;
}

async function nextStep(): Promise<void> {
  if (!canGoForward.value) {
    return;
  }
  if (pageStep.value === DeviceJoinOrganizationStep.Authentication) {
    const deviceName = await getDefaultDeviceName();
    waitingForHost.value = true;
    const doClaimResult = await claimer.value.doClaim(deviceName);
    if (doClaimResult.ok) {
      waitingForHost.value = false;
      const strategy = authChoice.value.getSaveStrategy();
      const result = await claimer.value.finalize(strategy);
      if (!result.ok) {
        informationManager.present(
          new Information({
            message: translate('ClaimDeviceModal.errors.saveDeviceFailed'),
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
        return;
      }
    } else {
      await showErrorAndRestart(translate('ClaimDeviceModal.errors.sendDeviceInfoFailed'));
      return;
    }
  } else if (pageStep.value === DeviceJoinOrganizationStep.Finish) {
    if (!claimer.value.device) {
      return;
    }
    const notification = new Information({
      message: translate('ClaimDeviceModal.successMessage'),
      level: InformationLevel.Success,
    });
    informationManager.present(notification, PresentationMode.Toast | PresentationMode.Console);
    const saveStrategy: DeviceSaveStrategy = authChoice.value.getSaveStrategy();
    const accessStrategy =
      saveStrategy.tag === DeviceSaveStrategyTag.Keyring
        ? AccessStrategy.useKeyring(claimer.value.device)
        : AccessStrategy.usePassword(claimer.value.device, (saveStrategy as DeviceSaveStrategyPassword).password);
    await modalController.dismiss({ device: claimer.value.device, access: accessStrategy }, MsModalResult.Confirm);
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
      await showErrorAndRestart(translate('ClaimDeviceModal.errors.unexpected', { reason: result.error.tag }));
    }
  }
}

async function startProcess(): Promise<void> {
  pageStep.value = DeviceJoinOrganizationStep.Information;
  waitingForHost.value = true;

  const retrieveResult = await claimer.value.retrieveInfo(props.invitationLink);

  if (!retrieveResult.ok) {
    await claimer.value.abort();
    await modalController.dismiss(null, MsModalResult.Cancel);
    let message = translate('JoinOrganization.errors.unexpected', { reason: retrieveResult.error.tag });
    switch (retrieveResult.error.tag) {
      case ClaimerRetrieveInfoErrorTag.AlreadyUsed:
        message = translate('JoinOrganization.errors.tokenAlreadyUsed');
        break;
      case ClaimerRetrieveInfoErrorTag.NotFound:
        message = translate('JoinOrganization.errors.invitationNotFound');
        break;
      case ClaimerRetrieveInfoErrorTag.Offline:
        message = translate('JoinOrganization.errors.offline');
        break;
    }
    await informationManager.present(
      new Information({
        message: message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
    return;
  }

  const waitResult = await claimer.value.initialWaitHost();
  if (!waitResult.ok && !cancelled.value) {
    await claimer.value.abort();
    await modalController.dismiss(null, MsModalResult.Cancel);
    await informationManager.present(
      new Information({
        message: translate('ClaimDeviceModal.errors.unexpected', { reason: waitResult.error.tag }),
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
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

  if (addrResult.ok && addrResult.value.tag === ParsedParsecAddrTag.InvitationDevice) {
    backendAddr = addrResult.value as ParsedParsecAddrInvitationDevice;
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

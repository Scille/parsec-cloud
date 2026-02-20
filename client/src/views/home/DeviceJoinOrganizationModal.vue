<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page
    class="modal-stepper"
    :class="DeviceJoinOrganizationStep[pageStep]"
  >
    <ms-wizard-stepper
      v-show="pageStep > DeviceJoinOrganizationStep.Information && pageStep < DeviceJoinOrganizationStep.Finish && isLargeDisplay"
      :current-index="pageStep - 1"
      :titles="[
        'ClaimDeviceModal.stepper.GetHostCode',
        'ClaimDeviceModal.stepper.ProvideGuestCode',
        'ClaimDeviceModal.stepper.Authentication',
      ]"
    />
    <ion-button
      slot="icon-only"
      @click="cancelModal()"
      v-show="pageStep !== DeviceJoinOrganizationStep.Finish && isLargeDisplay"
      class="closeBtn"
    >
      <ion-icon
        :icon="close"
        class="closeBtn__icon"
      />
    </ion-button>
    <div
      class="modal"
      :class="{
        wizardTrue: pageStep > DeviceJoinOrganizationStep.Information && pageStep != DeviceJoinOrganizationStep.Finish && isLargeDisplay,
      }"
    >
      <ion-header
        class="modal-header"
        v-if="isLargeDisplay"
      >
        <ion-title class="modal-header__title title-h3">
          {{ $msTranslate(steps[pageStep]?.title) }}
        </ion-title>
        <ion-text
          v-if="steps[pageStep]?.subtitle"
          class="modal-header__text body"
        >
          {{ $msTranslate(steps[pageStep]?.subtitle) }}
        </ion-text>
      </ion-header>
      <template v-else>
        <small-display-modal-header
          v-if="pageStep === DeviceJoinOrganizationStep.Information || pageStep === DeviceJoinOrganizationStep.Finish"
          @close-clicked="cancelModal()"
          :hide-close-button="pageStep === DeviceJoinOrganizationStep.Finish"
          :title="steps[pageStep]?.title"
        />
        <small-display-step-modal-header
          v-else
          @close-clicked="cancelModal()"
          :title="'DevicesPage.addDevice'"
          :icon="phonePortrait"
          :steps="steps.slice(1, steps.length - 1)"
          :current-step="pageStep - 1"
        />
      </template>
      <!-- modal content: create component for each part-->
      <div class="modal-content inner-content">
        <!-- part 0 (manage by JoinByLink component)-->
        <!-- part 1 (information message join)-->
        <div
          v-show="pageStep === DeviceJoinOrganizationStep.Information"
          class="step organization-name"
        >
          <information-join-device />
          <ms-report-text :theme="MsReportTheme.Info">
            {{ $msTranslate('DevicesPage.greet.subtitles.waitForGuestVersionInfo') }}
          </ms-report-text>
        </div>

        <!-- part 2 (host code)-->
        <div
          v-show="pageStep === DeviceJoinOrganizationStep.GetHostSasCode"
          class="step"
        >
          <ms-report-text :theme="MsReportTheme.Info">
            {{ $msTranslate('SasCodeChoice.securityInfo') }}
          </ms-report-text>
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

        <!-- part 4 (get auth)-->
        <div
          v-show="pageStep === DeviceJoinOrganizationStep.Authentication"
          class="step"
          id="get-password"
        >
          <choose-authentication
            ref="authChoice"
            :server-config="serverConfig"
          />
        </div>
        <!-- part 5 (finish the process)-->
        <div
          v-show="pageStep === DeviceJoinOrganizationStep.Finish"
          class="step final-step"
        >
          <ms-informative-text :icon="checkmarkCircle">
            {{ $msTranslate('ClaimDeviceModal.subtitles.done') }}
          </ms-informative-text>
        </div>
      </div>
      <!-- the buttons must be only enabled if all fields are filled in -->
      <ion-footer class="modal-footer">
        <div class="modal-footer-buttons">
          <ion-button
            fill="solid"
            size="default"
            id="next-button"
            v-show="nextButtonIsVisible"
            :disabled="!canGoForward"
            @click="nextStep()"
          >
            <span>
              {{ $msTranslate(getNextButtonText()) }}
            </span>
          </ion-button>
          <div
            v-show="waitingForHost"
            class="spinner-container"
          >
            <ms-spinner title="JoinOrganization.waitingForHost" />
          </div>
        </div>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { getDefaultDeviceName } from '@/common/device';
import ChooseAuthentication from '@/components/devices/ChooseAuthentication.vue';
import SmallDisplayModalHeader from '@/components/header/SmallDisplayModalHeader.vue';
import SmallDisplayStepModalHeader from '@/components/header/SmallDisplayStepModalHeader.vue';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import {
  CancelledGreetingAttemptReason,
  ClaimerRetrieveInfoErrorTag,
  ClaimInProgressErrorTag,
  constructAccessStrategy,
  DeviceClaim,
  DeviceSaveStrategy,
  forgeServerAddr,
  getServerConfig,
  OrganizationID,
  ParsedParsecAddrTag,
  parseParsecAddr,
  ServerConfig,
} from '@/parsec';
import { ParsecInvitationAddr, ParsecInvitationRedirectionURL } from '@/plugins/libparsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import InformationJoinDevice from '@/views/home/InformationJoinDeviceStep.vue';
import { IonButton, IonFooter, IonHeader, IonIcon, IonPage, IonText, IonTitle, modalController } from '@ionic/vue';
import { checkmarkCircle, close, phonePortrait } from 'ionicons/icons';
import {
  Answer,
  askQuestion,
  asyncComputed,
  MsInformativeText,
  MsModalResult,
  MsReportText,
  MsReportTheme,
  MsSpinner,
  MsWizardStepper,
  Translatable,
  useWindowSize,
} from 'megashark-lib';
import { computed, onMounted, ref, useTemplateRef } from 'vue';

enum DeviceJoinOrganizationStep {
  Information = 0,
  GetHostSasCode = 1,
  ProvideGuestCode = 2,
  Authentication = 3,
  Finish = 4,
}

const { isLargeDisplay } = useWindowSize();
const pageStep = ref(DeviceJoinOrganizationStep.Information);
const organizationName = ref<OrganizationID>('');
const claimer = ref(new DeviceClaim());
const authChoiceRef = useTemplateRef<InstanceType<typeof ChooseAuthentication>>('authChoice');
const cancelled = ref(false);
const serverConfig = ref<ServerConfig | undefined>(undefined);

const props = defineProps<{
  invitationLink: ParsecInvitationAddr | ParsecInvitationRedirectionURL;
  informationManager: InformationManager;
}>();

const waitingForHost = ref(true);

const steps = computed(() => [
  { title: 'ClaimDeviceModal.titles.claimDevice' },
  {
    title: 'ClaimDeviceModal.titles.getCode',
    subtitle: 'ClaimDeviceModal.subtitles.getCode',
  },
  {
    title: 'ClaimDeviceModal.titles.provideCode',
    subtitle: 'ClaimDeviceModal.subtitles.provideCode',
  },
  {
    title: 'ClaimDeviceModal.titles.authentication',
    subtitle: 'ClaimDeviceModal.subtitles.authentication',
  },
  {
    title: {
      key: 'ClaimDeviceModal.titles.done',
      data: {
        org: organizationName.value,
      },
    },
  },
]);

async function selectHostSas(selectedCode: string | null): Promise<void> {
  if (!selectedCode) {
    await showErrorAndRestart('ClaimDeviceModal.errors.noneCodeSelected');
    return;
  }
  if (selectedCode === claimer.value.correctSASCode) {
    const result = await claimer.value.signifyTrust();
    if (result.ok) {
      nextStep();
    } else {
      if (result.error.tag === ClaimInProgressErrorTag.GreetingAttemptCancelled) {
        switch (result.error.reason) {
          case CancelledGreetingAttemptReason.ManuallyCancelled:
            await showErrorAndRestart('ClaimDeviceModal.errors.greeter.manuallyCancelled');
            break;
          default:
            await showErrorAndRestart('ClaimDeviceModal.errors.greeter.default');
            break;
        }
      } else {
        await showErrorAndRestart('ClaimDeviceModal.errors.unexpected');
      }
    }
  } else {
    await claimer.value.denyTrust();
    await showErrorAndRestart('ClaimDeviceModal.errors.invalidCodeSelected');
  }
}

async function showErrorAndRestart(message: Translatable): Promise<void> {
  props.informationManager.present(
    new Information({
      message: message,
      level: InformationLevel.Error,
    }),
    PresentationMode.Toast,
  );
  await restartProcess();
}

function getNextButtonText(): Translatable {
  if (pageStep.value === DeviceJoinOrganizationStep.Information) {
    return 'ClaimDeviceModal.buttons.understand';
  } else if (pageStep.value === DeviceJoinOrganizationStep.Authentication) {
    return 'ClaimDeviceModal.buttons.confirm';
  } else if (pageStep.value === DeviceJoinOrganizationStep.Finish) {
    return 'ClaimDeviceModal.buttons.login';
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
    return await authChoiceRef.value?.areFieldsCorrect();
  }
  return true;
});

async function cancelModal(): Promise<boolean> {
  const answer = await askQuestion('ClaimDeviceModal.cancelConfirm', 'ClaimDeviceModal.cancelConfirmSubtitle', {
    yesIsDangerous: true,
    keepMainModalHiddenOnYes: true,
    yesText: 'ClaimDeviceModal.cancelYes',
    noText: 'ClaimDeviceModal.cancelNo',
    backdropDismiss: false,
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
      const strategy = authChoiceRef.value?.getSaveStrategy();
      if (!strategy) return;
      const result = await claimer.value.finalize(strategy);
      if (!result.ok) {
        props.informationManager.present(
          new Information({
            message: 'ClaimDeviceModal.errors.saveDeviceFailed',
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
        return;
      }
    } else {
      await showErrorAndRestart('ClaimDeviceModal.errors.sendDeviceInfoFailed');
      return;
    }
  } else if (pageStep.value === DeviceJoinOrganizationStep.Finish) {
    if (!claimer.value.device) {
      return;
    }
    const notification = new Information({
      message: 'ClaimDeviceModal.successMessage',
      level: InformationLevel.Success,
    });
    props.informationManager.present(notification, PresentationMode.Toast | PresentationMode.Console);
    const saveStrategy: DeviceSaveStrategy | undefined = authChoiceRef.value?.getSaveStrategy();
    if (!saveStrategy) return;
    const accessStrategy = constructAccessStrategy(claimer.value.device, saveStrategy.primaryProtection, saveStrategy.totpProtection);
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
      let message: Translatable = '';
      switch (result.error.tag) {
        case ClaimInProgressErrorTag.GreetingAttemptCancelled:
          switch (result.error.reason) {
            case CancelledGreetingAttemptReason.InvalidSasCode:
              message = 'ClaimDeviceModal.errors.greeter.invalidSasCode';
              break;
            case CancelledGreetingAttemptReason.ManuallyCancelled:
              message = 'ClaimDeviceModal.errors.greeter.manuallyCancelled';
              break;
            case CancelledGreetingAttemptReason.AutomaticallyCancelled:
              message = 'ClaimDeviceModal.errors.greeter.automaticallyCancelled';
              break;
            default:
              message = 'ClaimDeviceModal.errors.greeter.default';
              break;
          }
          break;
        default:
          message = { key: 'ClaimDeviceModal.errors.unexpected', data: { reason: result.error.tag } };
          break;
      }
      await showErrorAndRestart(message);
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
    let message: Translatable = '';
    switch (retrieveResult.error.tag) {
      case ClaimerRetrieveInfoErrorTag.AlreadyUsedOrDeleted:
        message = 'JoinOrganization.errors.tokenAlreadyUsed';
        break;
      case ClaimerRetrieveInfoErrorTag.NotFound:
        message = 'JoinOrganization.errors.invitationNotFound';
        break;
      case ClaimerRetrieveInfoErrorTag.Offline:
        message = 'JoinOrganization.errors.offline';
        break;
      default:
        message = { key: 'JoinOrganization.errors.unexpected', data: { reason: retrieveResult.error.tag } };
    }
    await props.informationManager.present(
      new Information({
        message: message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }

  const waitResult = await claimer.value.initialWaitHost();
  if (!waitResult.ok && !cancelled.value) {
    await claimer.value.abort();
    await modalController.dismiss(null, MsModalResult.Cancel);
    let message: Translatable = '';
    switch (waitResult.error.tag) {
      case ClaimInProgressErrorTag.GreetingAttemptCancelled:
        message = 'ClaimDeviceModal.errors.greeter.default';
        break;
      default:
        message = { key: 'ClaimDeviceModal.errors.unexpected', data: { reason: waitResult.error.tag } };
        break;
    }

    await props.informationManager.present(
      new Information({
        message: message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
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
  const addrResult = await parseParsecAddr(props.invitationLink);

  if (addrResult.ok) {
    if (addrResult.value.tag !== ParsedParsecAddrTag.InvitationDevice) {
      window.electronAPI.log('error', 'Not a device invitation link');
      return;
    }
    organizationName.value = addrResult.value.organizationId;
    const configResult = await getServerConfig(await forgeServerAddr(addrResult.value));
    if (configResult.ok) {
      serverConfig.value = configResult.value;
    }
  } else {
    window.electronAPI.log('error', `Failed to parse invitation link: ${addrResult.error.tag} (${addrResult.error.error})`);
    return;
  }
  await startProcess();
});
</script>

<style scoped lang="scss">
.organization-name {
  display: flex;
  flex-direction: column;
}

.guest-code {
  margin: auto;
}
</style>

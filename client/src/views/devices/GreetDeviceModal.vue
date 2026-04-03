<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page
    class="modal-stepper"
    :class="GreetDeviceStep[pageStep]"
  >
    <ms-wizard-stepper
      v-show="pageStep > GreetDeviceStep.WaitForGuest && pageStep < GreetDeviceStep.Summary && isLargeDisplay"
      :current-index="getStepperIndex()"
      :titles="['DevicesPage.greet.steps.hostCode', 'DevicesPage.greet.steps.guestCode']"
    />
    <ion-button
      slot="icon-only"
      @click="cancelModal()"
      class="closeBtn"
      v-if="pageStep !== GreetDeviceStep.Summary && isLargeDisplay"
    >
      <ion-icon
        :icon="close"
        class="closeBtn__icon"
      />
    </ion-button>
    <div class="modal">
      <ion-header
        class="modal-header"
        v-if="isLargeDisplay"
      >
        <ion-title class="modal-header__title title-h3">
          {{ $msTranslate(steps[pageStep]?.title) }}
        </ion-title>
        <ion-text
          class="modal-header__text body"
          v-if="steps[pageStep]?.subtitle"
        >
          {{ $msTranslate(steps[pageStep]?.subtitle) }}
        </ion-text>
      </ion-header>
      <template v-else>
        <small-display-modal-header
          v-if="pageStep === GreetDeviceStep.WaitForGuest || pageStep === GreetDeviceStep.Summary"
          @close-clicked="cancelModal()"
          :hide-close-button="pageStep === GreetDeviceStep.Summary"
          :title="steps[pageStep]?.title"
        />
        <small-display-step-modal-header
          v-else
          @close-clicked="cancelModal()"
          :title="'DevicesPage.greetDevice'"
          :icon="phonePortrait"
          :steps="steps.slice(1, steps.length - 1)"
          :current-step="pageStep - 1"
        />
      </template>
      <div class="modal-content inner-content">
        <!-- waiting step -->
        <div
          v-show="pageStep === GreetDeviceStep.WaitForGuest"
          class="first-step"
        >
          <ms-report-text :theme="MsReportTheme.Info">
            {{ $msTranslate('DevicesPage.greet.step1.info') }}
          </ms-report-text>

          <div class="step-info">
            <ion-text class="step-info__title title-h4">{{ $msTranslate('DevicesPage.greet.step1.title') }}</ion-text>
            <ion-text class="step-info__subtitle body-lg">{{ $msTranslate('DevicesPage.greet.step1.description') }}</ion-text>
          </div>

          <div class="first-step-content">
            <!-- qr code, link and button -->
            <div class="step-link">
              <figure class="step-link-qrcode">
                <!-- prettier-ignore -->
                <q-r-code-vue3
                  :value="greeter.invitationLink"
                  :key="greeter.invitationLink"
                  :image="(ResourcesManager.instance().get(Resources.LogoIcon, LogoIconGradient) as string)"
                  :image-options="{ hideBackgroundDots: true, imageSize: 1, margin: 1 }"
                  :qr-options="{ errorCorrectionLevel: 'L' }"
                  :dots-options="{
                    type: 'dots',
                    color: '#0058CC',
                  }"
                  :background-options="{ color: '#ffffff' }"
                  :corners-square-options="{ type: 'extra-rounded', color: '#0058CC' }"
                  :corners-dot-options="{ type: 'dot', color: '#0058CC' }"
                />
              </figure>
              <div class="divider">
                <ion-text class="title-h4">
                  {{ $msTranslate('FoldersPage.importModal.or') }}
                </ion-text>
              </div>
              <!-- right element: invite link, copy button, email button -->
              <div class="step-link-copy">
                <ion-text class="step-link-copy-text form-input">{{ greeter.invitationLink }}</ion-text>
                <div class="step-link-copy-buttons">
                  <ion-button
                    id="copy-link-btn"
                    @click="copyLink"
                    :disabled="linkCopiedToClipboard !== undefined"
                    class="button-item"
                  >
                    <ion-icon
                      class="button-icon"
                      :icon="linkCopiedToClipboard ? checkmarkCircle : copy"
                    />
                    <span v-show="linkCopiedToClipboard === undefined">
                      {{ $msTranslate('DevicesPage.greet.actions.copyLink') }}
                    </span>
                    <span v-show="linkCopiedToClipboard === true">
                      {{ $msTranslate('DevicesPage.greet.actions.copied') }}
                    </span>
                  </ion-button>
                  <ion-button
                    class="button-item"
                    :class="isEmailSent && elapsedCount === 0 ? 'button-clicked' : ''"
                    @click="sendEmail"
                    :disabled="isEmailSent && elapsedCount > 0"
                  >
                    <ion-icon
                      class="button-icon"
                      v-if="!(isEmailSent && elapsedCount > 0)"
                      :icon="mail"
                    />
                    <span v-show="!isEmailSent">
                      {{ $msTranslate('DevicesPage.greet.actions.sendEmail') }}
                    </span>
                    <span v-show="isEmailSent && elapsedCount > 0">
                      {{ $msTranslate({ key: 'DevicesPage.greet.actions.waitBeforeResending', data: { seconds: elapsedCount } }) }}
                    </span>
                    <span v-show="isEmailSent && elapsedCount === 0">
                      {{ $msTranslate('DevicesPage.greet.actions.reSendEmail') }}
                    </span>
                  </ion-button>
                </div>
              </div>

              <ion-text
                v-if="clipboardNotAvailable"
                class="step-link-copy-error body-sm"
              >
                {{ $msTranslate('DevicesPage.greet.linkNotCopiedToClipboard') }}
              </ion-text>
            </div>
          </div>
        </div>

        <!-- give code step -->
        <div
          v-show="pageStep === GreetDeviceStep.ProvideHostSasCode"
          class="step host-code"
        >
          <sas-code-provide :code="greeter.hostSASCode" />
        </div>

        <!-- choose code step -->
        <div
          v-show="pageStep === GreetDeviceStep.GetGuestSasCode"
          class="step"
        >
          <ms-report-text :theme="MsReportTheme.Info">
            {{ $msTranslate('SasCodeChoice.securityInfo') }}
          </ms-report-text>
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
            {{ $msTranslate('DevicesPage.greet.subtitles.getDeviceInfo') }}
          </ms-informative-text>
        </div>

        <!-- Final Step -->
        <div
          v-show="pageStep === GreetDeviceStep.Summary"
          class="step final-step"
        >
          <device-card
            :device="{
              deviceLabel: greeter.requestedDeviceLabel,
              createdOn: DateTime.now(),
              id: '',
              purpose: DevicePurpose.Standard,
              createdBy: null,
            }"
            :is-current="false"
            :show-id="false"
          />
        </div>
      </div>
      <ion-footer class="modal-footer">
        <div class="modal-footer-buttons">
          <ion-button
            fill="solid"
            size="default"
            id="next-button"
            v-show="nextButtonIsVisible"
            @click="nextStep()"
            :disabled="!canGoForward"
          >
            <span>
              {{ $msTranslate(getNextButtonText()) }}
            </span>
          </ion-button>
          <div
            v-show="waitingForGuest"
            class="spinner-container"
          >
            <ms-spinner
              title="DevicesPage.greet.waiting"
              :size="20"
            />
          </div>
        </div>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import LogoIconGradient from '@/assets/images/logo-icon-gradient.svg';
import DeviceCard from '@/components/devices/DeviceCard.vue';
import SmallDisplayModalHeader from '@/components/header/SmallDisplayModalHeader.vue';
import SmallDisplayStepModalHeader from '@/components/header/SmallDisplayStepModalHeader.vue';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import { CancelledGreetingAttemptReason, DeviceGreet, DevicePurpose, GreetInProgressErrorTag } from '@/parsec';
import { ParsecInvitationAddr, ParsecInvitationRedirectionURL } from '@/plugins/libparsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonButton, IonFooter, IonHeader, IonIcon, IonPage, IonText, IonTitle, modalController } from '@ionic/vue';
import { checkmarkCircle, close, copy, mail, phonePortrait } from 'ionicons/icons';
import { DateTime } from 'luxon';
import {
  Answer,
  Clipboard,
  MsInformativeText,
  MsModalResult,
  MsReportText,
  MsReportTheme,
  MsSpinner,
  MsWizardStepper,
  Translatable,
  askQuestion,
  startCounter,
  useWindowSize,
} from 'megashark-lib';
import QRCodeVue3 from 'qrcode-vue3';
import { computed, onMounted, ref } from 'vue';

enum GreetDeviceStep {
  WaitForGuest = 0,
  ProvideHostSasCode = 1,
  GetGuestSasCode = 2,
  WaitForGuestInfo = 3,
  Summary = 4,
}

const props = defineProps<{
  informationManager: InformationManager;
  invitationLink: ParsecInvitationAddr | ParsecInvitationRedirectionURL;
  token: string;
}>();

const { isLargeDisplay } = useWindowSize();
const pageStep = ref(GreetDeviceStep.WaitForGuest);
const canGoForward = ref(false);
const waitingForGuest = ref(true);
const isEmailSent = ref(false);
const elapsedCount = ref(0);
const greeter = ref(new DeviceGreet());
const linkCopiedToClipboard = ref<boolean | undefined>(undefined);
const clipboardNotAvailable = ref(false);
const cancelled = ref(false);

const steps = computed(() => [
  { title: 'DevicesPage.greet.titles.waitForGuest' },
  {
    title: 'DevicesPage.greet.titles.provideHostCode',
    subtitle: 'DevicesPage.greet.subtitles.provideHostCode',
  },
  {
    title: 'DevicesPage.greet.titles.getGuestCode',
    subtitle: 'DevicesPage.greet.subtitles.getGuestCode',
  },
  { title: 'DevicesPage.greet.titles.deviceDetails' },
  {
    title: 'DevicesPage.greet.titles.summary',
    subtitle: {
      key: 'DevicesPage.greet.subtitles.summary',
      data: {
        label: greeter.value.requestedDeviceLabel,
      },
    },
  },
]);

async function updateCanGoForward(): Promise<void> {
  if (pageStep.value === GreetDeviceStep.WaitForGuest && waitingForGuest.value === true) {
    canGoForward.value = false;
  } else {
    canGoForward.value = true;
  }
}

function getNextButtonText(): string {
  if (pageStep.value === GreetDeviceStep.WaitForGuest) {
    return 'DevicesPage.greet.actions.start';
  } else if (pageStep.value === GreetDeviceStep.Summary) {
    return 'DevicesPage.greet.actions.finish';
  }
  return '';
}

async function selectGuestSas(code: string | null): Promise<void> {
  if (!code) {
    await showErrorAndRestart('DevicesPage.greet.errors.noneCodeSelected');
    return;
  }
  if (code === greeter.value.correctSASCode) {
    const result = await greeter.value.signifyTrust();
    if (result.ok) {
      await nextStep();
    } else {
      await showErrorAndRestart({ key: 'DevicesPage.greet.errors.unexpected', data: { reason: result.error.tag } });
      if (result.error.tag === GreetInProgressErrorTag.GreetingAttemptCancelled) {
        switch (result.error.reason) {
          case CancelledGreetingAttemptReason.ManuallyCancelled:
            await showErrorAndRestart('DevicesPage.greet.errors.claimer.manuallyCancelled');
            break;
          default:
            await showErrorAndRestart('DevicesPage.greet.errors.claimer.default');
            break;
        }
      } else {
        await showErrorAndRestart({ key: 'DevicesPage.greet.errors.unexpected', data: { reason: result.error.tag } });
      }
    }
  } else {
    await greeter.value.denyTrust();
    await showErrorAndRestart('DevicesPage.greet.errors.invalidCodeSelected');
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
    props.informationManager.present(
      new Information({
        message: 'DevicesPage.greet.errors.startFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    await cancelModal();
    return;
  }
  const waitResult = await greeter.value.initialWaitGuest();
  if (!waitResult.ok && !cancelled.value) {
    let message: Translatable = '';
    let level: InformationLevel;
    switch (waitResult.error.tag) {
      case GreetInProgressErrorTag.Cancelled:
        message = 'DevicesPage.greet.cancelled';
        level = InformationLevel.Info;
        break;
      default:
        message = 'DevicesPage.greet.errors.startFailed';
        level = InformationLevel.Error;
        break;
    }
    props.informationManager.present(
      new Information({
        message: message,
        level: level,
      }),
      PresentationMode.Toast,
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
  if (cancelled.value) {
    return false;
  }
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
  cancelled.value = true;
  if (pageStep.value === GreetDeviceStep.Summary) {
    return await modalController.dismiss(null, MsModalResult.Cancel);
  }
  if (pageStep.value === GreetDeviceStep.WaitForGuest) {
    await greeter.value.abort();
    return await modalController.dismiss(null, MsModalResult.Cancel);
  }
  const answer = await askQuestion('DevicesPage.greet.titles.cancelGreet', 'DevicesPage.greet.subtitles.cancelGreet', {
    keepMainModalHiddenOnYes: true,
    yesIsDangerous: true,
    yesText: 'DevicesPage.greet.actions.cancelGreet.yes',
    noText: 'DevicesPage.greet.actions.cancelGreet.no',
    backdropDismiss: false,
  });

  if (answer === Answer.Yes) {
    await greeter.value.abort();
    return await modalController.dismiss(null, MsModalResult.Cancel);
  }
  return false;
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

async function nextStep(): Promise<void> {
  await updateCanGoForward();
  if (!canGoForward.value) {
    return;
  }
  if (pageStep.value === GreetDeviceStep.Summary) {
    props.informationManager.present(
      new Information({
        message: 'DevicesPage.greet.success',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
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
      if (result.error.tag === GreetInProgressErrorTag.GreetingAttemptCancelled) {
        switch (result.error.reason) {
          case CancelledGreetingAttemptReason.InvalidSasCode:
            await showErrorAndRestart('DevicesPage.greet.errors.claimer.invalidSasCode');
            break;
          case CancelledGreetingAttemptReason.ManuallyCancelled:
            await showErrorAndRestart('DevicesPage.greet.errors.claimer.manuallyCancelled');
            break;
          case CancelledGreetingAttemptReason.AutomaticallyCancelled:
            await showErrorAndRestart('DevicesPage.greet.errors.claimer.automaticallyCancelled');
            break;
          default:
            await showErrorAndRestart('DevicesPage.greet.errors.claimer.default');
            break;
        }
      } else {
        await showErrorAndRestart({ key: 'DevicesPage.greet.errors.unexpected', data: { reason: result.error.tag } });
      }
    }
  } else if (pageStep.value === GreetDeviceStep.WaitForGuestInfo) {
    waitingForGuest.value = true;
    const result = await greeter.value.getClaimRequests();
    waitingForGuest.value = false;
    if (result.ok) {
      const createResult = await greeter.value.createDevice();
      if (!createResult.ok) {
        await showErrorAndRestart('DevicesPage.greet.errors.createDeviceFailed');
        return;
      }
      await nextStep();
    } else {
      await showErrorAndRestart('DevicesPage.greet.errors.retrieveDeviceInfoFailed');
    }
  }
}

async function copyLink(): Promise<void> {
  linkCopiedToClipboard.value = await Clipboard.writeText(greeter.value.invitationLink);

  if (!linkCopiedToClipboard.value) {
    clipboardNotAvailable.value = true;

    linkCopiedToClipboard.value = undefined;
  } else {
    clipboardNotAvailable.value = false;
    setTimeout(() => {
      linkCopiedToClipboard.value = undefined;
    }, 2000);
    props.informationManager.present(
      new Information({
        message: 'DevicesPage.greet.linkCopiedToClipboard',
        level: InformationLevel.Info,
      }),
      PresentationMode.Toast,
    );
  }
}

async function sendEmail(): Promise<void> {
  async function updateCounter(elapsed: number): Promise<void> {
    if (elapsed === 5000) {
      elapsedCount.value = 0;
    } else {
      elapsedCount.value = Math.round((5000 - elapsed) / 1000);
    }
  }

  if (await greeter.value.sendEmail()) {
    isEmailSent.value = true;
    startCounter(5000, 1000, updateCounter);
    props.informationManager.present(
      new Information({
        message: 'DevicesPage.greet.emailSentToast',
        level: InformationLevel.Info,
      }),
      PresentationMode.Toast,
    );
  } else {
    props.informationManager.present(
      new Information({
        message: 'DevicesPage.greet.errors.emailFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

onMounted(async () => {
  greeter.value.setInvitationInformation(props.invitationLink, props.token);
  await startProcess();
});
</script>

<style scoped lang="scss">
.first-step {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  width: 100%;

  @include ms.responsive-breakpoint('sm') {
    padding-inline: 1rem;
  }

  .step-info {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    color: var(--parsec-color-light-secondary-text);
  }

  .step-link {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    background: var(--parsec-color-light-secondary-background);
    border: 1px solid var(--parsec-color-light-secondary-medium);
    padding: 1rem;
    border-radius: var(--parsec-radius-12);
    //should be replaced by a shadow token --parsec-shadow-filter when it will be available
    box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.04);

    &-qrcode {
      margin: 0;
      width: 11rem;
      height: 11rem;
    }

    .divider {
      display: flex;
      flex-direction: row;
      align-items: center;
      gap: 0.5rem;

      ion-text {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        color: var(--parsec-color-light-secondary-light);
        text-transform: uppercase;

        &::before,
        &::after {
          content: '';
          margin: auto;
          display: flex;
          background: var(--parsec-color-light-secondary-light);
        }

        &::before {
          height: 1px;
          width: 3rem;
        }

        &::after {
          height: 1px;
          width: 3rem;
        }
      }
    }

    &-copy {
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      gap: 1rem;
      width: 100%;
      color: var(--parsec-color-light-secondary-soft-text);
      background-color: var(--parsec-color-light-secondary-white);
      border: 1px solid var(--parsec-color-light-secondary-medium);
      border-radius: var(--parsec-radius-12);
      padding: 0.75rem 0.5rem 0.75rem 0.75rem;
      align-items: center;
      overflow: hidden;
      min-height: 2.5rem;

      &-text {
        text-wrap: wrap;
        width: 100%;
      }

      &-buttons {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 1rem;
        width: 100%;
      }

      .button-item {
        border-radius: var(--parsec-radius-8);
        background: var(--parsec-color-light-primary-50);
        color: var(--parsec-color-light-primary-600);
        cursor: pointer;
        flex-grow: 1;

        &.button-clicked {
          background: transparent;
          border: 1px solid var(--parsec-color-light-primary-100);
        }

        &::part(native) {
          padding: 0.75rem 1rem;
          --background: none;
          --background-hover: none;
          --border-radius: none;
        }

        &:hover {
          background: var(--parsec-color-light-primary-100);
        }

        .button-icon {
          color: var(--parsec-color-light-primary-600);
          font-size: 1rem;
          margin-right: 0.5rem;
          flex-shrink: 0;
        }
      }

      &-error {
        color: var(--parsec-color-light-danger-500);
      }
    }
  }
}

.final-step {
  display: flex;
  flex-direction: row;
  align-items: center;
  border-radius: var(--parsec-radius-6);
  justify-content: space-between;
  color: var(--parsec-color-light-secondary-text);
  width: 20rem;

  @include ms.responsive-breakpoint('sm') {
    width: 100%;
  }
}

.spinner {
  padding-bottom: 0.5rem;
}
</style>

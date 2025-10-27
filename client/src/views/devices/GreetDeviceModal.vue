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
          <ms-informative-text>
            {{ $msTranslate('DevicesPage.greet.subtitles.waitForGuest') }}
          </ms-informative-text>
          <div class="first-step-content">
            <!-- title -->
            <ion-text class="content-title">
              <span class="content-title__blue">
                {{ $msTranslate('DevicesPage.greet.subtitles.scanOrShare1') }}
              </span>
              <span class="content-title__grey">
                {{ $msTranslate('DevicesPage.greet.subtitles.scanOrShare2') }}
              </span>
              <span class="content-title__blue">
                {{ $msTranslate('DevicesPage.greet.subtitles.scanOrShare3') }}
              </span>
            </ion-text>
            <!-- qr code, link and button -->
            <div class="content-sharing">
              <!-- left element: qr code -->
              <figure class="qrcode">
                <!-- #4294FF is light-primary-500 -->
                <!-- prettier-ignore -->
                <q-r-code-vue3
                  :value="greeter.invitationLink"
                  :key="greeter.invitationLink"
                  :image="(ResourcesManager.instance().get(Resources.LogoIcon, LogoIconGradient) as string)"
                  :image-options="{ hideBackgroundDots: true, imageSize: 1, margin: 1 }"
                  :qr-options="{ errorCorrectionLevel: 'L' }"
                  :dots-options="{
                    type: 'dots',
                    color: '#4294FF',
                  }"
                  :background-options="{ color: '#ffffff' }"
                  :corners-square-options="{ type: 'extra-rounded', color: '#4294FF' }"
                  :corners-dot-options="{ type: 'dot', color: '#4294FF' }"
                />
              </figure>
              <div class="divider">
                <ion-text class="title-h4">
                  {{ $msTranslate('FoldersPage.importModal.or') }}
                </ion-text>
              </div>
              <!-- right element: invite link, copy button, email button -->
              <div class="right-side">
                <div
                  class="link-content"
                  v-if="isLargeDisplay"
                >
                  <div
                    class="link-content-card"
                    v-if="!linkCopiedToClipboard"
                  >
                    <ion-text class="link-content-card__text body">
                      {{ greeter.invitationLink }}
                    </ion-text>
                    <ion-button
                      fill="clear"
                      size="small"
                      id="copy-link-btn"
                      @click="copyLink"
                    >
                      <ion-icon
                        class="icon-copy"
                        :icon="copy"
                      />
                    </ion-button>
                  </div>
                  <ion-text
                    v-else
                    class="link-content__text body copied"
                  >
                    {{ $msTranslate('DevicesPage.greet.subtitles.copiedToClipboard') }}
                  </ion-text>
                </div>
                <div
                  class="link-content-small"
                  v-else
                >
                  <ion-button
                    id="copy-link-btn-small"
                    :class="{ copied: linkCopiedToClipboard }"
                    @click="copyLink"
                  >
                    <ion-icon
                      class="icon-copy"
                      :icon="copy"
                    />
                    <span v-if="!linkCopiedToClipboard">{{ $msTranslate('DevicesPage.greet.actions.copyLink') }}</span>
                    <span v-else>{{ $msTranslate('DevicesPage.greet.subtitles.copiedToClipboard') }}</span>
                  </ion-button>
                </div>
                <div class="email">
                  <ion-button
                    class="email-button"
                    @click="sendEmail"
                    fill="outline"
                    v-show="!isEmailSent"
                  >
                    {{ $msTranslate('DevicesPage.greet.actions.sendEmail') }}
                  </ion-button>
                  <ion-text
                    v-show="isEmailSent"
                    class="small-text subtitles-sm"
                  >
                    {{ $msTranslate('DevicesPage.greet.emailSent') }}
                    <ion-icon
                      class="email-button__icon"
                      :icon="checkmarkCircle"
                    />
                  </ion-text>
                </div>
              </div>
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
            <ion-text class="subtitles-normal">
              {{ $msTranslate('DevicesPage.greet.waiting') }}
            </ion-text>
            <ms-spinner class="spinner" />
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
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonButton, IonFooter, IonHeader, IonIcon, IonPage, IonText, IonTitle, modalController } from '@ionic/vue';
import { checkmarkCircle, close, copy, phonePortrait } from 'ionicons/icons';
import { DateTime } from 'luxon';
import {
  Answer,
  Clipboard,
  MsInformativeText,
  MsModalResult,
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
  invitationLink: string;
  token: string;
}>();

const { isLargeDisplay } = useWindowSize();
const pageStep = ref(GreetDeviceStep.WaitForGuest);
const canGoForward = ref(false);
const waitingForGuest = ref(true);
const isEmailSent = ref(false);
const elapsedCount = ref(0);
const greeter = ref(new DeviceGreet());
const linkCopiedToClipboard = ref(false);
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
  if (!(await Clipboard.writeText(greeter.value.invitationLink))) {
    props.informationManager.present(
      new Information({
        message: 'DevicesPage.greet.linkNotCopiedToClipboard',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    linkCopiedToClipboard.value = true;
    setTimeout(() => {
      linkCopiedToClipboard.value = false;
    }, 5000);
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
  gap: 2rem;
  flex-shrink: 0;

  @include ms.responsive-breakpoint('sm') {
    padding-inline: 1.5rem;
  }
}

.first-step-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: 0 auto;
  width: 100%;
  max-width: 37.5rem;
  background-color: var(--parsec-color-light-secondary-background);
  border-radius: var(--parsec-radius-6);
  padding: 2rem 1rem;

  @include ms.responsive-breakpoint('sm') {
    padding: 2rem;
    max-width: 20rem;
    border-radius: var(--parsec-radius-12);
    box-shadow: var(--parsec-shadow-soft);
  }

  .content-title {
    text-align: center;

    &__blue {
      color: var(--parsec-color-light-primary-600);
    }

    &__grey {
      color: var(--parsec-color-light-secondary-grey);
    }
  }

  .content-sharing {
    display: flex;
    align-items: center;
    margin-top: 1.5rem;
    gap: 1.5rem;
    width: 100%;

    @include ms.responsive-breakpoint('sm') {
      flex-direction: column;
    }

    .qrcode {
      display: flex;
      width: 7.5rem;
      padding: 0.5rem;
      background: var(--parsec-color-light-secondary-white);
      position: relative;
      margin: 0;
    }

    .divider {
      display: flex;
      flex-direction: column;
      align-items: center;

      ion-text {
        color: var(--parsec-color-light-secondary-light);
        text-transform: uppercase;

        @include ms.responsive-breakpoint('sm') {
          display: flex;
          flex-direction: row;
          align-items: center;
          gap: 0.5rem;
        }

        &::before {
          content: '';
          margin: auto;
          display: flex;
          margin-bottom: 1rem;
          background: var(--parsec-color-light-secondary-light);
          width: 1.5px;
          height: 3rem;

          @include ms.responsive-breakpoint('sm') {
            margin: 0;
            width: 3rem;
            height: 1.5px;
          }
        }

        &::after {
          content: '';
          margin: auto;
          display: flex;
          margin-top: 1rem;
          background: var(--parsec-color-light-secondary-light);
          width: 1.5px;
          height: 3rem;

          @include ms.responsive-breakpoint('sm') {
            margin: 0;
            width: 3rem;
            height: 1.5px;
          }
        }
      }
    }

    .right-side {
      display: flex;
      flex-direction: column;
      gap: 1.5rem;
      width: 20rem;

      @include ms.responsive-breakpoint('sm') {
        max-width: 30rem;
        width: 100%;
      }

      .small-text {
        color: var(--parsec-color-light-success-700);
        display: flex;
        align-items: center;
        gap: 0.5rem;
        position: absolute;
      }
    }

    .link-content {
      margin: 0;
      background-color: var(--parsec-color-light-secondary-white);
      border-radius: var(--parsec-radius-6);
      border: 1px solid var(--parsec-color-light-secondary-disabled);
      padding: 0.25rem 0.5rem;

      @include ms.responsive-breakpoint('sm') {
        width: 100%;
        overflow: hidden;
        display: flex;
        flex-direction: column;
      }

      &-card {
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: space-between;

        @include ms.responsive-breakpoint('sm') {
          width: 100%;
          overflow: hidden;
          display: flex;
          flex-direction: column;
        }

        &__text {
          margin: 0;
          overflow: hidden;
          color: var(--parsec-color-light-secondary-text);
          white-space: nowrap;
          text-overflow: ellipsis;

          &.copied {
            color: var(--parsec-color-light-success-700);
          }
        }

        #copy-link-btn {
          color: var(--parsec-color-light-secondary-text);
          margin: 0;

          &::part(native) {
            padding: 0.5rem;
            border-radius: var(--parsec-radius-6);
          }

          &:hover {
            color: var(--parsec-color-light-primary-600);
          }
        }
      }

      .icon-checkmark {
        position: relative;
        color: var(--parsec-color-light-success-700);
        padding: 0.5rem;
      }

      @include ms.responsive-breakpoint('sm') {
        &-small {
          #copy-link-btn-small {
            width: 100%;

            &::part(native) {
              background: var(--parsec-color-light-secondary-text);
              color: var(--parsec-color-light-secondary-white);
              --background-hover: var(--parsec-color-light-secondary-contrast);
            }

            &.copied {
              &::part(native) {
                background: var(--parsec-color-light-secondary-background);
                border: none;
                color: var(--parsec-color-light-secondary-text);
              }
            }
          }

          ion-icon {
            margin-right: 0.5rem;
            font-size: 1rem;
          }
        }
      }
    }

    .email {
      height: 1rem;
      display: flex;
      flex-direction: column;
      gap: 1rem;
      position: relative;

      @include ms.responsive-breakpoint('sm') {
        height: initial;
        min-height: 2rem;
      }

      &-button {
        display: flex;
        width: fit-content;
        position: relative;
        margin: 0;
        color: var(--parsec-color-light-secondary-text);
        --background-hover: var(--parsec-color-light-secondary-medium);

        @include ms.responsive-breakpoint('sm') {
          width: 100%;
        }

        &::part(native) {
          border: 1px solid var(--parsec-color-light-secondary-text);
        }

        &:hover {
          &::part(native) {
            border: 1px solid var(--parsec-color-light-secondary-contrast);
          }
        }

        &__icon {
          font-size: 1rem;
        }
      }

      .small-text {
        @include ms.responsive-breakpoint('sm') {
          width: 100%;
          justify-content: center;
        }
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

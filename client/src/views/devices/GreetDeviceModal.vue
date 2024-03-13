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
          {{ getTitleAndSubtitle().title }}
        </ion-title>
        <ion-text
          class="modal-header__text body"
          v-show="getTitleAndSubtitle().subtitle"
        >
          {{ getTitleAndSubtitle().subtitle }}
        </ion-text>
      </ion-header>
      <div class="modal-content inner-content">
        <!-- waiting step -->
        <div
          v-show="pageStep === GreetDeviceStep.WaitForGuest"
          class="first-step"
        >
          <ms-informative-text>
            {{ $t('DevicesPage.greet.subtitles.waitForGuest') }}
          </ms-informative-text>
          <div class="first-step-content">
            <!-- title -->
            <ion-text class="content-title">
              <span class="content-title__blue">
                {{ $t('DevicesPage.greet.subtitles.scanOrShare1') }}
              </span>
              <span class="content-title__grey">
                {{ $t('DevicesPage.greet.subtitles.scanOrShare2') }}
              </span>
              <span class="content-title__blue">
                {{ $t('DevicesPage.greet.subtitles.scanOrShare3') }}
              </span>
            </ion-text>
            <!-- qr code, link and button -->
            <div class="content-sharing">
              <!-- left element: qr code -->
              <figure class="qrcode">
                <!-- #4294FF is light-primary-500 -->
                <q-r-code-vue3
                  :value="greeter.invitationLink"
                  :key="greeter.invitationLink"
                  :image="LogoIconGradient"
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
                  {{ $t('FoldersPage.importModal.or') }}
                </ion-text>
              </div>
              <!-- right element: invite link, copy button, email button -->
              <div class="right-side">
                <ion-card class="card">
                  <ion-card-content class="card-content">
                    <ion-text
                      v-if="!linkCopiedToClipboard"
                      class="card-content__text body"
                    >
                      {{ greeter.invitationLink }}
                    </ion-text>
                    <ion-text
                      v-else
                      class="card-content__text body copied"
                    >
                      {{ $t('DevicesPage.greet.subtitles.copiedToClipboard') }}
                    </ion-text>
                    <ion-button
                      fill="clear"
                      size="small"
                      id="copy-link-btn"
                      @click="copyLink"
                      v-if="!linkCopiedToClipboard"
                    >
                      <ion-icon
                        class="icon-copy"
                        :icon="copy"
                      />
                    </ion-button>
                    <ion-icon
                      v-else
                      class="icon-checkmark"
                      :icon="checkmarkCircle"
                    />
                  </ion-card-content>
                </ion-card>
                <div class="email">
                  <ion-button
                    class="email-button"
                    @click="sendEmail"
                    fill="outline"
                  >
                    <span v-if="!isEmailSent">
                      {{ $t('DevicesPage.greet.actions.sendEmail') }}
                    </span>
                    <span v-if="isEmailSent">
                      {{ $t('DevicesPage.greet.actions.reSendEmail') }}
                    </span>
                  </ion-button>
                  <ion-text
                    v-if="isEmailSent"
                    class="small-text subtitles-sm"
                  >
                    {{ $t('DevicesPage.greet.emailSent') }}
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
          <device-card
            :label="greeter.requestedDeviceLabel"
            :is-current="false"
          />
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
import LogoIconGradient from '@/assets/images/logo-icon-gradient.svg';
import { Answer, MsInformativeText, MsModalResult, MsWizardStepper, askQuestion } from '@/components/core';
import DeviceCard from '@/components/devices/DeviceCard.vue';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import { DeviceGreet } from '@/parsec';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { translate } from '@/services/translation';
import {
  IonButton,
  IonButtons,
  IonCard,
  IonCardContent,
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
import { checkmarkCircle, close, copy } from 'ionicons/icons';
import QRCodeVue3 from 'qrcode-vue3';
import { computed, inject, onMounted, ref } from 'vue';

enum GreetDeviceStep {
  WaitForGuest = 1,
  ProvideHostSasCode = 2,
  GetGuestSasCode = 3,
  WaitForGuestInfo = 4,
  Summary = 5,
}

const pageStep = ref(GreetDeviceStep.WaitForGuest);
const canGoForward = ref(false);
const waitingForGuest = ref(true);
const isEmailSent = ref(false);
const greeter = ref(new DeviceGreet());
const informationManager: InformationManager = inject(InformationManagerKey)!;
const linkCopiedToClipboard = ref(false);

interface GreetDeviceTitle {
  title: string;
  subtitle?: string;
}

function getTitleAndSubtitle(): GreetDeviceTitle {
  if (pageStep.value === GreetDeviceStep.WaitForGuest) {
    return { title: translate('DevicesPage.greet.titles.waitForGuest') };
  } else if (pageStep.value === GreetDeviceStep.ProvideHostSasCode) {
    return {
      title: translate('DevicesPage.greet.titles.provideHostCode'),
      subtitle: translate('DevicesPage.greet.subtitles.provideHostCode'),
    };
  } else if (pageStep.value === GreetDeviceStep.GetGuestSasCode) {
    return { title: translate('DevicesPage.greet.titles.getGuestCode'), subtitle: translate('DevicesPage.greet.subtitles.getGuestCode') };
  } else if (pageStep.value === GreetDeviceStep.WaitForGuestInfo) {
    return { title: translate('DevicesPage.greet.titles.deviceDetails') };
  } else if (pageStep.value === GreetDeviceStep.Summary) {
    return {
      title: translate('DevicesPage.greet.titles.summary'),
      subtitle: translate('DevicesPage.greet.subtitles.summary', {
        label: greeter.value.requestedDeviceLabel,
      }),
    };
  }
  return { title: '' };
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
    return translate('DevicesPage.greet.actions.start');
  } else if (pageStep.value === GreetDeviceStep.Summary) {
    return translate('DevicesPage.greet.actions.finish');
  }
  return '';
}

async function selectGuestSas(code: string | null): Promise<void> {
  if (!code) {
    await showErrorAndRestart(translate('DevicesPage.greet.errors.noneCodeSelected'));
    return;
  }
  if (code === greeter.value.correctSASCode) {
    const result = await greeter.value.signifyTrust();
    if (result.ok) {
      await nextStep();
    } else {
      await showErrorAndRestart(translate('DevicesPage.greet.errors.unexpected', { reason: result.error.tag }));
    }
  } else {
    await showErrorAndRestart(translate('DevicesPage.greet.errors.invalidCodeSelected'));
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
    informationManager.present(
      new Information({
        message: translate('DevicesPage.greet.errors.startFailed'),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    await cancelModal();
    return;
  }
  const waitResult = await greeter.value.initialWaitGuest();
  if (!waitResult.ok) {
    informationManager.present(
      new Information({
        message: translate('DevicesPage.greet.errors.startFailed'),
        level: InformationLevel.Error,
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
  const answer = await askQuestion(
    translate('DevicesPage.greet.titles.cancelGreet'),
    translate('DevicesPage.greet.subtitles.cancelGreet'),
    {
      keepMainModalHiddenOnYes: true,
      yesIsDangerous: true,
      yesText: translate('DevicesPage.greet.actions.cancelGreet.yes'),
      noText: translate('DevicesPage.greet.actions.cancelGreet.no'),
    },
  );

  if (answer === Answer.Yes) {
    await greeter.value.abort();
    return await modalController.dismiss(null, MsModalResult.Cancel);
  }
  return false;
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

async function nextStep(): Promise<void> {
  await updateCanGoForward();
  if (!canGoForward.value) {
    return;
  }
  if (pageStep.value === GreetDeviceStep.Summary) {
    informationManager.present(
      new Information({
        message: translate('DevicesPage.greet.success'),
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
      await showErrorAndRestart(translate('DevicesPage.greet.errors.unexpected', { reason: result.error.tag }));
    }
  } else if (pageStep.value === GreetDeviceStep.WaitForGuestInfo) {
    waitingForGuest.value = true;
    const result = await greeter.value.getClaimRequests();
    waitingForGuest.value = false;
    if (result.ok) {
      const createResult = await greeter.value.createDevice();
      if (!createResult.ok) {
        await showErrorAndRestart(translate('DevicesPage.greet.errors.createDeviceFailed'));
        return;
      }
      await nextStep();
    } else {
      await showErrorAndRestart(translate('DevicesPage.greet.errors.retrieveDeviceInfoFailed'));
    }
  }
}

async function copyLink(): Promise<void> {
  await navigator.clipboard.writeText(greeter.value.invitationLink);
  linkCopiedToClipboard.value = true;
  setTimeout(() => {
    linkCopiedToClipboard.value = false;
  }, 5000);
  informationManager.present(
    new Information({
      message: translate('DevicesPage.greet.linkCopiedToClipboard'),
      level: InformationLevel.Info,
    }),
    PresentationMode.Toast,
  );
}

async function sendEmail(): Promise<void> {
  if (await greeter.value.sendEmail()) {
    isEmailSent.value = true;
  } else {
    informationManager.present(
      new Information({
        message: translate('DevicesPage.greet.errors.emailFailed'),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

onMounted(async () => {
  await greeter.value.createInvitation(true);
  await startProcess();
});
</script>

<style scoped lang="scss">
.first-step {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  flex-shrink: 0;
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
  padding: 2rem 0;

  .content-title {
    text-align: center;
    /* Titles/H3 */
    font-family: Montserrat;
    font-size: 1.125rem;
    font-style: normal;
    font-weight: 600;
    line-height: 120%;

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

    .qrcode {
      display: flex;
      width: 7.5rem;
      padding: 0.3rem;
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

        &::before {
          content: '';
          margin: auto;
          display: flex;
          margin-bottom: 1rem;
          background: var(--parsec-color-light-secondary-light);
          width: 1.5px;
          height: 3rem;
        }
        &::after {
          content: '';
          margin: auto;
          display: flex;
          margin-top: 1rem;
          background: var(--parsec-color-light-secondary-light);
          width: 1.5px;
          height: 3rem;
        }
      }
    }

    .right-side {
      display: flex;
      flex-direction: column;
      gap: 1.5rem;
      width: 20rem;

      .card {
        margin: 0;
        background-color: var(--parsec-color-light-secondary-white);
        border-radius: var(--parsec-radius-6);
        border: 1px solid var(--parsec-color-light-secondary-disabled);
        box-shadow: none;

        .card-content {
          display: flex;
          flex-direction: row;
          align-items: center;
          padding: 0.5rem;
          position: relative;

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

          ion-icon[class^='icon-'] {
            display: flex;
            width: 1rem;
            height: 1rem;
            margin: 0;
          }

          .icon-checkmark {
            position: relative;
            color: var(--parsec-color-light-success-700);
            padding: 0.5rem;
          }
        }
      }

      .email {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        position: relative;
      }

      .email-button {
        display: flex;
        width: fit-content;
        position: relative;
        margin: 0;
        color: var(--parsec-color-light-secondary-text);

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
        color: var(--parsec-color-light-success-700);
        display: flex;
        align-items: center;
        gap: 0.5rem;
        position: absolute;
        bottom: -2rem;
      }
    }
  }
}

.final-step {
  width: 100%;
  display: flex;
  flex-direction: row;
  align-items: center;
  border-radius: var(--parsec-radius-6);
  justify-content: space-between;
  color: var(--parsec-color-light-secondary-text);
  width: 20rem;
}
</style>

<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="mfa-activation-modal">
    <ms-modal
      :title="currentStep === Steps.PromptAuthentication ? 'MyProfilePage.titleCurrentPassword' : 'Authentication.mfa.title'"
      :close-button="{ visible: currentStep !== Steps.Finalization }"
      :confirm-button="{
        disabled: !nextButton.enabled || querying,
        label: nextButton.label,
        onClick: onNextButtonClicked,
        queryingSpinner: querying,
      }"
      :cancel-button="cancelButton"
      :class="currentStep"
    >
      <div class="modal-content">
        <div
          v-if="currentStep === Steps.PromptAuthentication && params.mode === 'setup'"
          class="modal-current-authentication"
        >
          <prompt-current-authentication
            :device="params.device"
            :server-config="serverConfig"
            @authentication-selected="primaryProtection = $event"
            @enter-pressed="onNextButtonClicked"
          />
        </div>
        <div
          v-if="currentStep === Steps.Information"
          class="modal-content-information"
        >
          <ms-spinner v-if="!url && !error" />
          <div
            v-if="url"
            class="step-container"
          >
            <ms-report-text :theme="MsReportTheme.Info">
              {{ $msTranslate('Authentication.mfa.info') }}
            </ms-report-text>

            <div class="step-info">
              <ion-text class="step-info__title title-h4">{{ $msTranslate('Authentication.mfa.step1.title') }}</ion-text>
              <ion-text class="step-info__title body-lg">{{ $msTranslate('Authentication.mfa.step1.description') }}</ion-text>
            </div>

            <div class="step-info">
              <ion-text class="step-info__title title-h4">{{ $msTranslate('Authentication.mfa.step2.title') }}</ion-text>
              <ion-text class="step-info__title body-lg">{{ $msTranslate('Authentication.mfa.step2.description') }}</ion-text>
            </div>

            <div class="step-code">
              <figure class="step-code-qrcode">
                <q-r-code-vue3
                  :value="url"
                  :image="ResourcesManager.instance().get(Resources.LogoIcon, LogoIconGradient) as string"
                  :image-options="{ hideBackgroundDots: true, imageSize: 1, margin: 4 }"
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
                  {{ $msTranslate('Authentication.mfa.or') }}
                </ion-text>
              </div>
              <div class="step-code-copy">
                <ion-text class="step-code-copy-text form-input">{{ code }}</ion-text>
                <ion-button
                  @click="copyCode"
                  :disabled="codeCopied !== undefined"
                  class="step-code-copy-button"
                >
                  <ion-icon
                    class="copy-icon"
                    :icon="codeCopied ? checkmarkCircle : copy"
                  />
                  <span
                    v-show="codeCopied === undefined"
                    v-if="isLargeDisplay"
                  >
                    {{ $msTranslate('Authentication.mfa.step2.buttonCopy') }}
                  </span>
                  <span
                    v-show="codeCopied === true"
                    v-if="isLargeDisplay"
                  >
                    {{ $msTranslate('Authentication.mfa.step2.buttonCopied') }}
                  </span>
                </ion-button>
                <ion-text
                  v-if="codeCopied === false"
                  class="step-code-copy-error body-sm"
                >
                  {{ $msTranslate('Authentication.mfa.step2.copyError') }}
                </ion-text>
              </div>
            </div>
          </div>
        </div>
        <div
          v-if="currentStep === Steps.EnterCode"
          class="modal-content-enter-code"
        >
          <div class="authentication-code">
            <ion-text class="authentication-code__title title-h4">
              {{ $msTranslate('Authentication.mfa.digitCode.label') }}
            </ion-text>
          </div>
          <ms-input
            v-model="verifyCode"
            placeholder="Authentication.mfa.digitCode.placeholder"
          />
        </div>
        <div
          v-if="currentStep === Steps.Finalization"
          class="modal-content-finalization"
        >
          <ms-report-text :theme="MsReportTheme.Success">
            {{ $msTranslate('Authentication.mfa.mfaSuccess.info') }}
          </ms-report-text>
          <ion-text class="finalization-title body">{{ $msTranslate('Authentication.mfa.mfaSuccess.description') }}</ion-text>
        </div>
        <ms-report-text
          v-if="error"
          :theme="MsReportTheme.Error"
        >
          {{ $msTranslate(error) }}
        </ms-report-text>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import LogoIconGradient from '@/assets/images/logo-icon-gradient.svg';
import {
  AvailableDevice,
  constructAccessStrategy,
  constructSaveStrategy,
  DevicePrimaryProtectionStrategy,
  generateTotpUrl,
  getServerConfig,
  getTotpOpaqueKey,
  getTotpStatus,
  isAuthenticationValid,
  ParsecTOTPResetAddr,
  ParsedParsecAddrTag,
  parseParsecAddr,
  ServerConfig,
  totpConfirmReset,
  totpResetStatus,
  TOTPSetupStatusTag,
  updateDeviceChangeAuthentication,
  verifyTotpSetup,
} from '@/parsec';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import PromptCurrentAuthentication from '@/views/users/PromptCurrentAuthentication.vue';
import { IonButton, IonIcon, IonPage, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle, copy } from 'ionicons/icons';
import { Clipboard, MsInput, MsModal, MsModalResult, MsReportText, MsReportTheme, MsSpinner, useWindowSize } from 'megashark-lib';
import QRCodeVue3 from 'qrcode-vue3';
import { computed, onMounted, ref, toRaw } from 'vue';

enum Steps {
  PromptAuthentication = 'prompt-authentication',
  Information = 'information',
  EnterCode = 'enter-code',
  Finalization = 'finalization',
}

const props = defineProps<{
  params: { mode: 'setup'; device: AvailableDevice } | { mode: 'reset'; link: ParsecTOTPResetAddr };
}>();

const { isLargeDisplay } = useWindowSize();
const url = ref('');
const code = ref('');
const verifyCode = ref('');
const codeCopied = ref<boolean | undefined>(undefined);
const currentStep = ref<Steps>(props.params.mode === 'setup' ? Steps.PromptAuthentication : Steps.Information);
const error = ref('');
const serverConfig = ref<ServerConfig | undefined>(undefined);
const primaryProtection = ref<DevicePrimaryProtectionStrategy | undefined>(undefined);
const querying = ref(false);

const nextButton = computed(() => {
  if (currentStep.value === Steps.PromptAuthentication) {
    return { enabled: primaryProtection.value !== undefined, label: 'MyProfilePage.nextButton' };
  } else if (currentStep.value === Steps.Information) {
    return { enabled: true, label: 'Authentication.mfa.buttons.next' };
  } else if (currentStep.value === Steps.EnterCode) {
    return { enabled: verifyCode.value.length > 0, label: 'Authentication.mfa.buttons.next' };
  } else {
    return { enabled: true, label: 'Authentication.mfa.buttons.done' };
  }
});

const cancelButton = computed(() => {
  if (currentStep.value === Steps.PromptAuthentication) {
    return {
      label: 'MyProfilePage.cancelButton',
      disabled: false,
      onClick: cancel,
    };
  } else if (currentStep.value === Steps.EnterCode) {
    return {
      label: 'Authentication.mfa.buttons.back',
      disabled: false,
      onClick: onPreviousClicked,
    };
  }
  return undefined;
});

onMounted(async () => {
  console.log(props.params);
  if (props.params.mode === 'setup') {
    const statusResult = await getTotpStatus();

    if (!statusResult.ok || statusResult.value.tag !== TOTPSetupStatusTag.Stalled) {
      error.value = 'Authentication.mfa.error.failedToConfigure';
      return;
    }

    const serverConfigResult = await getServerConfig(props.params.device.serverAddr);

    if (serverConfigResult.ok) {
      serverConfig.value = serverConfigResult.value;
    }

    if (primaryProtection.value) {
      currentStep.value = Steps.Information;
    } else {
      currentStep.value = Steps.PromptAuthentication;
    }
    code.value = statusResult.value.base32TotpSecret;
    url.value = await generateTotpUrl(code.value, props.params.device.organizationId);
  } else {
    const parseResult = await parseParsecAddr(props.params.link);
    if (!parseResult.ok || parseResult.value.tag !== ParsedParsecAddrTag.TOTPReset) {
      window.electronAPI.log('warn', 'Invalid totp reset link');
      return;
    }
    const statusResult = await totpResetStatus(props.params.link);
    if (!statusResult.ok || statusResult.value.tag !== TOTPSetupStatusTag.Stalled) {
      error.value = 'Authentication.mfa.error.failedToConfigure';
      return;
    }

    code.value = statusResult.value.base32TotpSecret;
    url.value = await generateTotpUrl(code.value, parseResult.value.organizationId);
  }
});

async function copyCode(): Promise<void> {
  codeCopied.value = await Clipboard.writeText(code.value);
  setTimeout(() => {
    codeCopied.value = undefined;
  }, 2000);
}

async function setupTotp(): Promise<void> {
  if (props.params.mode !== 'setup') {
    window.electronAPI.log('warn', 'Mode is not setup.');
    return;
  }
  if (!primaryProtection.value) {
    window.electronAPI.log('warn', 'Primary protection not provided.');
    return;
  }
  const result = await verifyTotpSetup(verifyCode.value);
  if (!result.ok) {
    error.value = 'Authentication.mfa.error.invalidCode';
    return;
  }
  const totpOpaqueKeyResult = await getTotpOpaqueKey();
  if (!totpOpaqueKeyResult.ok) {
    error.value = 'Authentication.mfa.error.failedToRetrieveKey';
    return;
  }
  const saveStrategy = constructSaveStrategy(toRaw(primaryProtection.value), [totpOpaqueKeyResult.value[0], totpOpaqueKeyResult.value[1]]);
  const accessStrategy = constructAccessStrategy(props.params.device, toRaw(primaryProtection.value));
  const updateAuthResult = await updateDeviceChangeAuthentication(accessStrategy, saveStrategy);
  if (!updateAuthResult.ok) {
    error.value = 'Authentication.mfa.error.failedToConfigure';
    return;
  }
  currentStep.value = Steps.Finalization;
}

async function resetTotp(): Promise<void> {
  if (props.params.mode !== 'reset') {
    window.electronAPI.log('warn', 'Mode is not reset.');
    return;
  }
  if (!verifyCode.value.length) {
    error.value = 'Authentication.mfa.error.emptyCode';
    return;
  }
  const result = await totpConfirmReset(props.params.link, verifyCode.value);
  if (!result.ok) {
    error.value = 'Authentication.mfa.error.failedToReset';
    return;
  }
  currentStep.value = Steps.Finalization;
}

async function onNextButtonClicked(): Promise<boolean> {
  if (currentStep.value === Steps.PromptAuthentication && primaryProtection.value && props.params.mode === 'setup') {
    const access = constructAccessStrategy(props.params.device, toRaw(primaryProtection.value));
    const result = await isAuthenticationValid(props.params.device, access);

    if (result) {
      error.value = '';
      currentStep.value = Steps.Information;
    } else {
      error.value = 'MyProfilePage.errors.wrongAuthentication';
      return false;
    }
  } else if (currentStep.value === Steps.Information) {
    currentStep.value = Steps.EnterCode;
  } else if (currentStep.value === Steps.EnterCode) {
    try {
      querying.value = true;
      if (props.params.mode === 'setup') {
        await setupTotp();
      } else {
        await resetTotp();
      }
    } finally {
      querying.value = false;
    }
  } else if (currentStep.value === Steps.Finalization) {
    await modalController.dismiss(undefined, MsModalResult.Confirm);
  }
  return true;
}

async function cancel(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Cancel);
}

async function onPreviousClicked(): Promise<boolean> {
  if (currentStep.value === Steps.EnterCode) {
    currentStep.value = Steps.Information;
    return true;
  }
  return false;
}
</script>

<style scoped lang="scss">
.modal-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.step-container {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  .step-info {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    color: var(--parsec-color-light-secondary-text);
  }

  .step-code {
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
      justify-content: space-between;
      gap: 0.5rem;
      width: 100%;
      color: var(--parsec-color-light-secondary-text);
      background-color: var(--parsec-color-light-secondary-white);
      border: 1px solid var(--parsec-color-light-secondary-medium);
      border-radius: var(--parsec-radius-12);
      padding: 0.5rem 0.5rem 0.5rem 0.75rem;
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 0.5rem;
      overflow: hidden;
      width: 100%;
      min-height: 2.5rem;

      &-text {
        flex-grow: 1;
      }

      &-button {
        border-radius: var(--parsec-radius-8);
        background: var(--parsec-color-light-primary-30);
        color: var(--parsec-color-light-primary-600);
        transition: all 0.2s;
        padding: 0.5rem 0.625rem;
        cursor: pointer;

        &::part(native) {
          padding: 0;
          --background: none;
          --background-hover: none;
          --border-radius: none;
        }

        &:hover {
          background: var(--parsec-color-light-primary-50);
        }

        .copy-icon {
          color: var(--parsec-color-light-primary-600);
          font-size: 1rem;
          margin-right: 0.5rem;

          @include ms.responsive-breakpoint('sm') {
            margin-right: 0;
          }
        }
      }
    }
  }
}

.modal-content-enter-code {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.authentication-code {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  color: var(--parsec-color-light-secondary-text);

  &__title {
    color: var(--parsec-color-light-secondary-text);
  }

  &__description {
    color: var(--parsec-color-light-secondary-text);
  }
}

.modal-content-finalization {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  color: var(--parsec-color-light-secondary-text);
}
</style>

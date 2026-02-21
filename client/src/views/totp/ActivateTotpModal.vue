<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="mfa-activation-modal">
    <ms-modal
      title="Authentication.mfaModal.title"
      :close-button="{ visible: currentStep !== Steps.Finalization }"
      :confirm-button="{
        disabled: !nextButton.enabled || querying,
        label: nextButton.label,
        onClick: onNextButtonClicked,
        queryingSpinner: querying,
      }"
      :cancel-button="cancelButton"
    >
      <div class="modal-content">
        <div
          v-if="currentStep === Steps.PromptAuthentication"
        >
          <prompt-current-authentication
            :device="device"
            :server-config="serverConfig"
            @authentication-selected="primaryProtection = $event"
          />
        </div>
        <div
          v-if="currentStep === Steps.Information"
          class="modal-content-information"
        >
          <ms-spinner v-if="!url && !error" />
          <ms-report-text
            v-if="error"
            :theme="MsReportTheme.Warning"
          >
            {{ $msTranslate(error) }}
          </ms-report-text>
          <div
            v-if="url"
            class="step-container"
          >
            <ms-report-text :theme="MsReportTheme.Info">
              {{ $msTranslate('Authentication.mfaModal.info') }}
            </ms-report-text>

            <div class="step-info">
              <ion-text class="step-info__title title-h4">{{ $msTranslate('Authentication.mfaModal.step1.title') }}</ion-text>
              <ion-text class="step-info__title body-lg">{{ $msTranslate('Authentication.mfaModal.step1.description') }}</ion-text>
            </div>

            <div class="step-info">
              <ion-text class="step-info__title title-h4">{{ $msTranslate('Authentication.mfaModal.step2.title') }}</ion-text>
              <ion-text class="step-info__title body-lg">{{ $msTranslate('Authentication.mfaModal.step2.description') }}</ion-text>
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
                  {{ $msTranslate('Authentication.mfaModal.or') }}
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
                  <span v-show="codeCopied === undefined">{{ $msTranslate('Authentication.mfaModal.step2.buttonCopy') }}</span>
                  <span v-show="codeCopied === true">{{ $msTranslate('Authentication.mfaModal.step2.buttonCopied') }}</span>
                </ion-button>
                <ion-text
                  v-if="codeCopied === false"
                  class="step-code-copy-error body-sm"
                >
                  {{ $msTranslate('Authentication.mfaModal.step2.copyError') }}
                </ion-text>
              </div>
            </div>

            <div class="step-info">
              <ion-text class="step-info__title title-h4">{{ $msTranslate('Authentication.mfaModal.step3.title') }}</ion-text>
              <ion-text class="step-info__title body-lg">{{ $msTranslate('Authentication.mfaModal.step3.description') }}</ion-text>
            </div>
          </div>
        </div>
        <div
          v-if="currentStep === Steps.EnterCode"
          class="modal-content-enter-code"
        >
          <div class="authentication-code">
            <ion-text class="authentication-code__title title-h4">
              {{ $msTranslate('Authentication.mfaModal.digitCode.label') }}
            </ion-text>
            <ion-text class="authentication-code__description body">
              {{ $msTranslate('Authentication.mfaModal.digitCode.description') }}
            </ion-text>
          </div>
          <ms-input
            v-model="verifyCode"
            placeholder="Authentication.mfaModal.digitCode.placeholder"
          />

          <ms-report-text
            v-if="error"
            :theme="MsReportTheme.Warning"
          >
            {{ $msTranslate(error) }}
          </ms-report-text>
        </div>
        <div
          v-if="currentStep === Steps.Finalization"
          class="modal-content-finalization"
        >
          <ms-report-text :theme="MsReportTheme.Success">
            {{ $msTranslate('Authentication.mfaModal.mfaSuccess.info') }}
          </ms-report-text>
          <ion-text class="finalization-title body">{{ $msTranslate('Authentication.mfaModal.mfaSuccess.description') }}</ion-text>
        </div>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import LogoIconGradient from '@/assets/images/logo-icon-gradient.svg';
import { AvailableDevice, constructAccessStrategy, constructSaveStrategy, DevicePrimaryProtectionStrategy, generateTotpUrl, getServerConfig, getTotpOpaqueKey, getTotpStatus, ServerConfig, TOTPSetupStatusTag, updateDeviceChangeAuthentication, verifyTotpSetup } from '@/parsec';
import { Resources, ResourcesManager } from '@/services/resourcesManager';
import { IonButton, IonIcon, IonPage, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle, copy } from 'ionicons/icons';
import { Clipboard, MsInput, MsModal, MsModalResult, MsReportText, MsReportTheme, MsSpinner } from 'megashark-lib';
import QRCodeVue3 from 'qrcode-vue3';
import { computed, onMounted, ref, toRaw } from 'vue';
import PromptCurrentAuthentication from '@/views/users/PromptCurrentAuthentication.vue';

enum Steps {
  PromptAuthentication = 'prompt-authentication',
  Information = 'information',
  EnterCode = 'enter-code',
  Finalization = 'finalization',
}

const props = defineProps<{
  device: AvailableDevice;
  protection?: DevicePrimaryProtectionStrategy;
}>();

const url = ref('');
const code = ref('');
const verifyCode = ref('');
const codeCopied = ref<boolean | undefined>(undefined);
const currentStep = ref<Steps>(Steps.PromptAuthentication);
const error = ref('');
const serverConfig = ref<ServerConfig | undefined>(undefined);
const primaryProtection = ref<DevicePrimaryProtectionStrategy | undefined>(props.protection);
const querying = ref(false);

const nextButton = computed(() => {
  if (currentStep.value === Steps.PromptAuthentication) {
    return { enabled: primaryProtection.value !== undefined, label: 'Authentication.mfaModal.buttons.next' };
  } else if (currentStep.value === Steps.Information) {
    return { enabled: true, label: 'Authentication.mfaModal.buttons.next' };
  } else if (currentStep.value === Steps.EnterCode) {
    return { enabled: verifyCode.value.length > 0, label: 'Authentication.mfaModal.buttons.next' };
  } else {
    return { enabled: true, label: 'Authentication.mfaModal.buttons.done' };
  }
});

const cancelButton = computed(() => {
  if (currentStep.value === Steps.EnterCode) {
    return {
      label: 'Authentication.mfaModal.buttons.previous',
      disabled: false,
      onClick: onPreviousClicked,
    };
  }
  return undefined;
});

onMounted(async () => {
  const statusResult = await getTotpStatus();

  if (!statusResult.ok || statusResult.value.tag !== TOTPSetupStatusTag.Stalled) {
    error.value = 'TOTP NOT REQUIRED';
    return;
  }

  const serverConfigResult = await getServerConfig(props.device.serverAddr);

  if (serverConfigResult.ok) {
    serverConfig.value = serverConfigResult.value;
  }

  if (primaryProtection.value) {
    currentStep.value = Steps.Information;
  } else {
    currentStep.value = Steps.PromptAuthentication;
  }
  code.value = statusResult.value.base32TotpSecret;
  url.value = await generateTotpUrl(code.value, props.device.organizationId);
});

async function copyCode(): Promise<void> {
  codeCopied.value = await Clipboard.writeText(code.value);
  setTimeout(() => {
    codeCopied.value = undefined;
  }, 2000);
}

async function onNextButtonClicked(): Promise<boolean> {
  if (currentStep.value === Steps.PromptAuthentication && primaryProtection.value) {
    currentStep.value = Steps.Information;
  } else if (currentStep.value === Steps.Information) {
    currentStep.value = Steps.EnterCode;
  } else if (currentStep.value === Steps.EnterCode && primaryProtection.value) {
    try {
      querying.value = true;
      if (!verifyCode.value.length) {
        error.value = 'Authentication.mfaModal.error.emptyCode';
        return false;
      }
      console.log('VERIFYING CODE');
      const result = await verifyTotpSetup(verifyCode.value);
      if (!result.ok) {
        error.value = 'Authentication.mfaModal.error.invalidCode';
        return false;
      }
      console.log('GET OPAQUE KEY');
      const totpOpaqueKeyResult = await getTotpOpaqueKey();
      if (!totpOpaqueKeyResult.ok) {
        error.value = 'FAILED TO GET TOTP KEY';
        return false;
      }
      console.log('CHANGE AUTH');
      const saveStrategy = constructSaveStrategy(toRaw(primaryProtection.value), [totpOpaqueKeyResult.value[0], totpOpaqueKeyResult.value[1]]);
      const accessStrategy = constructAccessStrategy(props.device, toRaw(primaryProtection.value));
      console.log(saveStrategy);
      console.log(accessStrategy);
      const updateAuthResult = await updateDeviceChangeAuthentication(accessStrategy, saveStrategy);
      if (!updateAuthResult.ok) {
        error.value = 'FAILED TO SET UP TOTP FOR THIS DEVICE';
        return false;
      }
      console.log('NEXT STEP');
      currentStep.value = Steps.Finalization;
    } finally {
      querying.value = false;
    }
  } else if (currentStep.value === Steps.Finalization) {
    await modalController.dismiss(undefined, MsModalResult.Confirm);
  }
  return true;
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
      width: 8.125rem;
      height: 8.125rem;
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
      gap: 0.5rem;
      width: 100%;

      &-text {
        color: var(--parsec-color-light-secondary-text);
        background-color: var(--parsec-color-light-secondary-white);
        border: 1px solid var(--parsec-color-light-secondary-medium);
        border-radius: var(--parsec-radius-6);
        padding: 0.5rem 0.75rem;
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: 0.5rem;
        overflow: hidden;
        width: 100%;
        min-height: 2.5rem;
      }

      &-button {
        width: 100%;
        border: 1px solid var(--parsec-color-light-secondary-medium);
        box-shadow: var(--parsec-shadow-input);
        border-radius: var(--parsec-radius-8);
        color: var(--parsec-color-light-secondary-text);
        transition: all 0.2s;
        padding: 0.75rem 1rem;
        cursor: pointer;

        &::part(native) {
          padding: 0;
          --background: none;
          --background-hover: none;
        }

        &:hover {
          border-color: var(--parsec-color-light-secondary-grey);
          box-shadow: var(--parsec-shadow-input-hover);
        }

        .copy-icon {
          color: var(--parsec-color-light-secondary-text);
          font-size: 1rem;
          margin-right: 0.5rem;
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

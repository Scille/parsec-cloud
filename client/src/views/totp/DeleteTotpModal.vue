<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="delete-totp-modal">
    <ms-modal
      title="Authentication.mfa.delete.title"
      :close-button="{ visible: true }"
      :confirm-button="{
        disabled: !canConfirm || querying,
        label: confirmButtonLabel,
        onClick: onConfirmButtonClicked,
        queryingSpinner: querying,
      }"
      :cancel-button="cancelButton"
    >
      <div class="modal-content">
        <ms-report-text
          v-if="currentStep === Steps.PromptAuthentication"
          :theme="MsReportTheme.Warning"
        >
          {{ $msTranslate('Authentication.mfa.delete.warning') }}
        </ms-report-text>

        <div
          v-if="currentStep === Steps.PromptAuthentication"
          class="authentication-section"
        >
          <prompt-current-authentication
            :device="device"
            :server-config="serverConfig"
            @authentication-selected="primaryProtection = $event"
            @enter-pressed="onConfirmButtonClicked"
          />
        </div>

        <div
          v-if="currentStep === Steps.EnterCode"
          class="totp-section"
        >
          <ion-text class="section-title title-h4">
            {{ $msTranslate('Authentication.mfa.delete.totpCodeTitle') }}
          </ion-text>
          <ms-input
            v-model="verifyCode"
            placeholder="Authentication.mfa.digitCode.placeholder"
          />
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
import {
  AvailableDevice,
  constructAccessStrategy,
  constructSaveStrategy,
  DevicePrimaryProtectionStrategy,
  fetchTotpOpaqueKey,
  getServerConfig,
  getTotpStatus,
  ServerConfig,
  TOTPSetupStatusTag,
  updateDeviceChangeAuthentication,
} from '@/parsec';
import PromptCurrentAuthentication from '@/views/users/PromptCurrentAuthentication.vue';
import { IonPage, IonText, modalController } from '@ionic/vue';
import { MsInput, MsModal, MsModalResult, MsReportText, MsReportTheme } from 'megashark-lib';
import { computed, onMounted, ref, toRaw } from 'vue';

enum Steps {
  PromptAuthentication = 'prompt-authentication',
  EnterCode = 'enter-code',
}

const props = defineProps<{
  device: AvailableDevice;
}>();

const currentStep = ref<Steps>(Steps.PromptAuthentication);
const primaryProtection = ref<DevicePrimaryProtectionStrategy | undefined>(undefined);
const verifyCode = ref('');
const error = ref('');
const querying = ref(false);
const serverConfig = ref<ServerConfig | undefined>(undefined);

const canConfirm = computed(() => {
  if (currentStep.value === Steps.PromptAuthentication) {
    return primaryProtection.value !== undefined;
  } else {
    return verifyCode.value.length > 0;
  }
});

const confirmButtonLabel = computed(() => {
  if (currentStep.value === Steps.PromptAuthentication) {
    return 'Authentication.mfa.buttons.next';
  } else {
    return 'Authentication.mfa.delete.confirmButton';
  }
});

const cancelButton = computed(() => {
  if (currentStep.value === Steps.EnterCode) {
    return {
      label: 'Authentication.mfa.buttons.back',
      disabled: false,
      onClick: onPreviousClicked,
    };
  }
  return {
    label: 'Authentication.mfa.delete.cancelButton',
    disabled: false,
  };
});

onMounted(async () => {
  const statusResult = await getTotpStatus();

  if (!statusResult.ok || statusResult.value.tag !== TOTPSetupStatusTag.AlreadySetup || !props.device.totpOpaqueKeyId) {
    error.value = 'Authentication.mfa.delete.error.notSetup';
    return;
  }

  const serverConfigResult = await getServerConfig(props.device.serverAddr);

  if (serverConfigResult.ok) {
    serverConfig.value = serverConfigResult.value;
  }
});

async function onConfirmButtonClicked(): Promise<boolean> {
  if (!primaryProtection.value) {
    return false;
  }
  if (!props.device.totpOpaqueKeyId) {
    return false;
  }

  if (currentStep.value === Steps.PromptAuthentication && primaryProtection.value) {
    currentStep.value = Steps.EnterCode;
  } else if (currentStep.value === Steps.EnterCode && primaryProtection.value && verifyCode.value) {
    try {
      querying.value = true;
      const totpResult = await fetchTotpOpaqueKey(
        props.device.serverAddr,
        props.device.organizationId,
        props.device.userId,
        props.device.totpOpaqueKeyId,
        verifyCode.value,
      );
      if (!totpResult.ok) {
        error.value = 'Authentication.mfa.failedToRetrieveKey';
        return false;
      }
      const accessStrategy = constructAccessStrategy(props.device, toRaw(primaryProtection.value), [
        props.device.totpOpaqueKeyId,
        totpResult.value,
      ]);
      const saveStrategy = constructSaveStrategy(toRaw(primaryProtection.value));
      const updateAuthResult = await updateDeviceChangeAuthentication(accessStrategy, saveStrategy);
      if (!updateAuthResult.ok) {
        error.value = 'Authentication.mfa.error.failedToDelete';
        return false;
      }
      await modalController.dismiss(undefined, MsModalResult.Confirm);
    } finally {
      querying.value = false;
    }
  }
  return true;
}

async function onPreviousClicked(): Promise<boolean> {
  if (currentStep.value === Steps.EnterCode) {
    error.value = '';
    verifyCode.value = '';
    currentStep.value = Steps.PromptAuthentication;
    return true;
  }
  return false;
}
</script>

<style scoped lang="scss">
.modal-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.authentication-section,
.totp-section {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.section-title {
  color: var(--parsec-color-light-secondary-text);
}
</style>

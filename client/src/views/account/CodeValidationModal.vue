<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="HomePage.profile.account.deleteAccount.codeModal.title"
    subtitle="HomePage.profile.account.deleteAccount.codeModal.description"
    :close-button="{ visible: true }"
    class="code-validation-modal"
    :cancel-button="{
      label: 'lib.components.msModal.cancelButtonLabel',
      disabled: loading,
      onClick: () => modalController.dismiss(),
    }"
    :confirm-button="{
      label: 'HomePage.profile.account.deleteAccount.codeModal.button',
      disabled: code.length !== 6 || loading,
      onClick: onConfirm,
      theme: MsReportTheme.Error,
      queryingSpinner: loading,
    }"
  >
    <div class="code-validation">
      <ms-code-validation-input
        ref="codeValidationInputRef"
        :code-length="6"
        :allowed-input="AllowedInput.UpperAlphaNumeric"
        @code-complete="onCodeComplete"
        class="code-validation-input"
      />
      <span
        class="form-error subtitles-sm"
        v-show="error"
      >
        {{ $msTranslate(error) }}
      </span>
    </div>
    <ion-text
      button
      @click="resendCode"
      :disabled="resendDisabled"
      class="send-code subtitles-sm"
    >
      {{ $msTranslate('HomePage.profile.account.deleteAccount.codeModal.resendCode') }}
    </ion-text>
  </ms-modal>
</template>

<script setup lang="ts">
import { ParsecAccount } from '@/parsec';
import { IonText, modalController } from '@ionic/vue';
import { AllowedInput, MsCodeValidationInput, MsModal, MsModalResult, MsReportTheme } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const error = ref('');
const resendDisabled = ref(false);
const codeValidationInputRef = ref();
const loading = ref(false);
const code = ref<Array<string>>([]);

onMounted(async () => {
  setTimeout(async () => {
    await codeValidationInputRef.value.setFocus();
  }, 500);
});

async function resendCode(): Promise<void> {
  resendDisabled.value = true;
  const result = await ParsecAccount.requestAccountDeletion();
  if (!result.ok) {
    window.electronAPI.log('error', `Failed to request account deletion: ${result.error.tag} (${result.error.error})`);
    error.value = 'HomePage.profile.account.deleteAccount.error.reSentCode';
  }
  setTimeout(() => {
    resendDisabled.value = false;
  }, 5000);
}

async function onConfirm(): Promise<boolean> {
  if (code.value.length !== 6) {
    return false;
  }
  loading.value = true;
  try {
    const result = await ParsecAccount.confirmAccountDeletion(code.value);
    if (!result.ok) {
      window.electronAPI.log('error', `Failed to confirm account deletion: ${result.error.tag} (${result.error.error})`);
      error.value = 'HomePage.profile.account.deleteAccount.error.delete';
      return false;
    } else {
      return await modalController.dismiss({ code: code.value }, MsModalResult.Confirm);
    }
  } finally {
    loading.value = false;
  }
}

async function onCodeComplete(completeCode: Array<string>): Promise<void> {
  code.value = completeCode;
}
</script>

<style scoped lang="scss">
.code-validation-input {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 1rem;

  @include ms.responsive-breakpoint(sm) {
    margin-top: 1rem;
  }
}

.code-validation {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.send-code {
  position: absolute;
  bottom: 2.25rem;
  left: 2rem;
  z-index: 2;
  color: var(--parsec-color-light-secondary-hard-grey);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  background: var(--parsec-color-light-secondary-background);
  border-radius: var(--parsec-radius-6);
  padding: 0.5rem 1.25rem;
  width: fit-content;
  text-align: center;

  &:hover {
    cursor: pointer;
    background: var(--parsec-color-light-secondary-medium);
  }

  @include ms.responsive-breakpoint(sm) {
    position: initial;
    margin-top: 1rem;
  }

  &[disabled='true'] {
    pointer-events: none;
    color: var(--parsec-color-light-secondary-light);
  }
}
</style>

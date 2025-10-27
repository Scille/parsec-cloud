<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="HomePage.profile.authentication.updateAuthMethod.title"
    :close-button="{ visible: true }"
    :confirm-button="{
      disabled: !validAuth || querying,
      label: 'HomePage.profile.authentication.updateAuthMethod.updatePassword',
      onClick: onPasswordChosen,
      queryingSpinner: querying,
    }"
  >
    <ms-report-text
      v-show="error"
      :theme="MsReportTheme.Error"
      class="error"
    >
      {{ $msTranslate(error) }}
    </ms-report-text>
    <ms-choose-password-input
      ref="passwordInput"
      @on-enter-keyup="onPasswordChosen"
    />
  </ms-modal>
</template>

<script setup lang="ts">
import { ParsecAccount } from '@/parsec';
import { modalController } from '@ionic/vue';
import { asyncComputed, MsChoosePasswordInput, MsModal, MsModalResult, MsReportText, MsReportTheme } from 'megashark-lib';
import { ref, useTemplateRef } from 'vue';

const error = ref('');
const passwordInputRef = useTemplateRef<typeof MsChoosePasswordInput>('passwordInput');
const querying = ref(false);
const validAuth = asyncComputed(async () => {
  return passwordInputRef.value && (await passwordInputRef.value.areFieldsCorrect());
});

async function onPasswordChosen(): Promise<boolean> {
  error.value = '';
  if (!validAuth.value || !passwordInputRef.value) {
    return false;
  }
  querying.value = true;
  const result = await ParsecAccount.updatePassword(passwordInputRef.value.password);
  querying.value = false;
  if (!result.ok) {
    error.value = 'HomePage.profile.authentication.updateAuthMethod.error';

    return false;
  } else {
    return await modalController.dismiss(undefined, MsModalResult.Confirm);
  }
}
</script>

<style scoped lang="scss">
.error {
  margin-bottom: 1rem;
}
</style>

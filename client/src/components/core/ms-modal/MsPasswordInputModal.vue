<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="title"
    :subtitle="subtitle"
    :close-button="{ visible: true }"
    :confirm-button="{
      label: okButtonText || $t('PasswordInputModal.ok'),
      disabled: password.length === 0,
      onClick: confirm,
    }"
  >
    <ms-password-input
      :label="inputLabel || ''"
      ref="passwordInputRef"
      v-model="password"
      @on-enter-keyup="confirm()"
    />
  </ms-modal>
</template>

<script setup lang="ts">
import { MsPasswordInput } from '@/components/core/ms-input';
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import { GetPasswordOptions, MsModalResult } from '@/components/core/ms-modal/types';
import { modalController } from '@ionic/vue';
import { onMounted, ref } from 'vue';

defineProps<GetPasswordOptions>();

const password = ref('');
const passwordInputRef = ref();

onMounted(() => {
  passwordInputRef.value.setFocus();
});

async function confirm(): Promise<boolean> {
  if (password.value.length === 0) {
    return false;
  }
  return await modalController.dismiss(password.value, MsModalResult.Confirm);
}
</script>

<style scoped lang="scss"></style>

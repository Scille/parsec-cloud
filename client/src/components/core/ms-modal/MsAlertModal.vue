<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="title"
    :subtitle="message"
    :theme="theme"
    :close-button="{ visible: false }"
    :confirm-button="{
      disabled: false,
      label: $t('Notification.nextButton'),
      onClick: confirm,
    }"
    @on-enter-keyup="confirm()"
  />
</template>

<script setup lang="ts">
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import { modalController } from '@ionic/vue';
import { MsAlertModalConfig, MsModalResult } from '@/components/core/ms-modal/types';

defineProps<MsAlertModalConfig>();

async function confirm(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped></style>

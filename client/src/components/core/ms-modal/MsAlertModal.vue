<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="title"
    :subtitle="message"
    :theme="theme"
    :close-button-enabled="false"
    :confirm-button="{disabled: false, label: $t('Notification.nextButton'), onClick: confirm}"
    @on-enter-keyup="confirm()"
  />
</template>

<script lang="ts">
export interface MsAlertModalConfig {
  title?: string,
  theme: MsTheme,
  message: string,
}
</script>

<script setup lang="ts">
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import { modalController } from '@ionic/vue';
import { MsTheme, MsModalResult } from '@/components/core/ms-types';

defineProps<MsAlertModalConfig>();

function confirm(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped>
</style>

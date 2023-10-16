<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="title"
    :subtitle="message"
    :theme="theme"
    :close-button="{visible: false}"
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

async function confirm(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped>
.notification-modal {
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.notification-modal-ontop {
  // eslint-disable-next-line vue-scoped-css/no-unused-selector
  .modal {
    background-color: green;
  }
}
</style>

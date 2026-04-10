<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="OrganizationRecovery.modal.title"
    subtitle="ExportRecoveryDevicePage.subtitles.aboutRecoveryFiles"
    :close-button="{ visible: true }"
    :cancel-button="{ disabled: false }"
    :confirm-button="{
      disabled: !(downloaded && confirmed),
      label: 'ExportRecoveryDevicePage.actions.backToDevices',
      onClick: onConfirm,
    }"
  >
    <export-recovery-device
      :organization-id="organizationId"
      :information-manager="informationManager"
      @downloaded="downloaded = true"
      @confirmed="confirmed = $event"
    />
  </ms-modal>
</template>

<script setup lang="ts">
import { OrganizationID } from '@/parsec';
import { InformationManager } from '@/services/informationManager';
import ExportRecoveryDevice from '@/views/profile/ExportRecoveryDevice.vue';
import { modalController } from '@ionic/vue';
import { MsModal, MsModalResult } from 'megashark-lib';
import { ref } from 'vue';

defineProps<{
  organizationId: OrganizationID;
  informationManager: InformationManager;
}>();

const downloaded = ref(false);
const confirmed = ref(false);

async function onConfirm(): Promise<boolean> {
  return modalController.dismiss(undefined, MsModalResult.Confirm);
}
</script>

<style></style>

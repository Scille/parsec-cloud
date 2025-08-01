<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="openFallback.title"
    subtitle="openFallback.subtitle"
  >
    <div>
      <ion-buttons>
        <ion-button
          v-if="viewerOption"
          @click="onViewerClick"
        >
          {{ $msTranslate('openFallback.actions.viewer') }}
        </ion-button>
        <ion-button
          v-if="isWeb()"
          @click="onDownloadClick"
        >
          {{ $msTranslate('openFallback.actions.download') }}
        </ion-button>
        <ion-button
          v-if="isDesktop()"
          @click="onOpenClick"
        >
          {{ $msTranslate('openFallback.actions.open') }}
        </ion-button>
        <ion-button
          @click="onClose"
          fill="clear"
        >
          {{ $msTranslate('openFallback.actions.cancel') }}
        </ion-button>
      </ion-buttons>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { IonButton, IonButtons, modalController } from '@ionic/vue';
import { isDesktop, isWeb } from '@/parsec';
import { OpenFallbackChoice } from '@/components/files';
import { MsModal, MsModalResult } from 'megashark-lib';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { inject } from 'vue';
import { openDownloadConfirmationModal } from '@/views/files';

defineProps<{
  viewerOption?: boolean;
}>();

const storageManager: StorageManager = inject(StorageManagerKey)!;

async function onViewerClick(): Promise<void> {
  await modalController.dismiss(OpenFallbackChoice.View);
}

async function onDownloadClick(): Promise<void> {
  const result = await openDownloadConfirmationModal(storageManager);
  if (result === MsModalResult.Confirm) {
    await modalController.dismiss(OpenFallbackChoice.Download);
  }
}

async function onOpenClick(): Promise<void> {
  await modalController.dismiss(OpenFallbackChoice.Open);
}

async function onClose(): Promise<void> {
  await modalController.dismiss();
}
</script>

<style lang="scss" scoped></style>

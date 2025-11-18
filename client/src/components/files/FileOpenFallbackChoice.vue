<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="title ?? 'openFallback.title'"
    :subtitle="subtitle ?? 'openFallback.subtitle.download'"
    :close-button="{ visible: true }"
  >
    <div class="open-fallback-buttons">
      <ion-button
        v-if="!viewerOption"
        @click="onClose"
        fill="clear"
        class="button-default open-fallback-buttons__item"
      >
        {{ $msTranslate('openFallback.actions.cancel') }}
      </ion-button>
      <ion-button
        v-if="viewerOption"
        @click="onViewerClick"
        fill="outline"
        class="button-default open-fallback-buttons__item"
      >
        {{ $msTranslate('openFallback.actions.viewer') }}
      </ion-button>
      <ion-button
        v-if="isWeb()"
        @click="onDownloadClick"
        class="button-default open-fallback-buttons__item"
      >
        {{ $msTranslate('openFallback.actions.download') }}
      </ion-button>
      <ion-button
        v-if="isDesktop()"
        @click="onOpenClick"
        class="button-default open-fallback-buttons__item"
      >
        {{ $msTranslate('openFallback.actions.open') }}
      </ion-button>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { OpenFallbackChoice } from '@/components/files';
import { isDesktop, isWeb } from '@/parsec';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { openDownloadConfirmationModal } from '@/views/files';
import { IonButton, modalController } from '@ionic/vue';
import { MsModal, MsModalResult, Translatable } from 'megashark-lib';
import { inject } from 'vue';

defineProps<{
  viewerOption?: boolean;
  title?: Translatable;
  subtitle?: Translatable;
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

<style lang="scss" scoped>
.open-fallback-buttons {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;

  &__item::part(native) {
    padding: 0.75rem 1.125rem;
  }
}
</style>

<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="container">
    <div class="import-drag-drop">
      <ms-image :image="FileImportImage" />
      <ion-text class="import-drag-drop__title title-h3">
        {{ $t('FoldersPage.importModal.dragAndDrop') }}
      </ion-text>
    </div>
    <div class="divider">
      <ion-text class="title-h3">
        {{ $t('FoldersPage.importModal.or') }}
      </ion-text>
    </div>
    <div class="import-button">
      <input
        type="file"
        multiple
        hidden
        ref="hiddenInput"
      />
      <ion-button
        @click="importButtonClick"
        size="default"
        class="button"
      >
        <ion-icon
          :icon="ellipsisHorizontalCircle"
          slot="start"
          id="browse-icon"
        />
        {{ $t('FoldersPage.importModal.browse') }}
      </ion-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { FileImport as FileImportImage, MsImage } from '@/components/core';
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { ellipsisHorizontalCircle } from 'ionicons/icons';
import { onMounted, onUnmounted, ref } from 'vue';
const emits = defineEmits<{
  (e: 'filesImport', entries: File[]): void;
}>();

const hiddenInput = ref();

onMounted(() => {
  hiddenInput.value.addEventListener('change', onInputChange);
});

onUnmounted(() => {
  document.body.removeEventListener('change', onInputChange);
});

function onInputChange(_event: any): void {
  // Would love to use `hiddenInput.value.webkitEntries` instead but it returns
  // an empty list (may be browser dependant).
  // So we have to use `.files` instead, which is a worst API.
  if (hiddenInput.value.files.length > 0) {
    emits('filesImport', hiddenInput.value.files);
  }
}

function importButtonClick(): void {
  hiddenInput.value.click();
}
</script>

<style scoped lang="scss">
.container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  gap: 2rem;
}

.import-drag-drop {
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  width: 18.5rem;

  &__title {
    color: var(--parsec-color-light-primary-700);
  }
}

.divider {
  display: flex;
  flex-direction: column;
  align-items: center;

  ion-text {
    color: var(--parsec-color-light-secondary-light);
    text-transform: uppercase;

    &::before {
      content: '';
      margin: auto;
      display: flex;
      margin-bottom: 1rem;
      background: var(--parsec-color-light-secondary-light);
      width: 1.5px;
      height: 3rem;
    }
    &::after {
      content: '';
      margin: auto;
      display: flex;
      margin-top: 1rem;
      background: var(--parsec-color-light-secondary-light);
      width: 1.5px;
      height: 3rem;
    }
  }
}

#browse-icon {
  margin-right: 0.625rem;
}
</style>

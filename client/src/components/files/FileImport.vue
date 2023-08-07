<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div class="container">
    <div class="import-drag-drop">
      <ion-img
        src="../src/assets/images/image_import.svg"
      />
      <ion-text class="import-drag-drop__title title-h3">
        {{ $t('FoldersPage.importModal.dragAndDrop') }}
      </ion-text>
      <ion-text class="import-drag-drop__subtitle title-h4">
        {{ $t('FoldersPage.importModal.maxSize') }}
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
      >
      <ion-button
        @click="importButtonClick"
      >
        <ion-icon
          :icon="ellipsisHorizontalCircle"
          slot="start"
        />
        {{ $t('FoldersPage.importModal.browse') }}
      </ion-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  ellipsisHorizontalCircle,
} from 'ionicons/icons';
import {
  IonImg,
  IonButton,
  IonText,
} from '@ionic/vue';
import { ref, onMounted, onUnmounted } from 'vue';

defineEmits<{
  (e: 'filesImport', entries: FileSystemEntry[]): void
}>();

const hiddenInput = ref();

onMounted(() => {
  hiddenInput.value.addEventListener('change', onInputChange);
});

onUnmounted(() => {
  document.body.removeEventListener('change', onInputChange);
});

function onInputChange(_event: any): void {
  console.log(hiddenInput.value.webkitEntries);
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

  &__subtitle {
    color: var(--parsec-color-light-secondary-grey);
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
</style>

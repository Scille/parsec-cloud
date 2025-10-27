<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="import-container">
    <ion-item
      class="option"
      @click="onOptionClick(ImportType.Files)"
    >
      <ms-image
        :image="ImportMultipleFiles"
        class="option__icon"
      />
      <ion-icon
        class="arrow-up"
        :icon="arrowUp"
      />
      <ion-text class="body item-label">
        {{ $msTranslate('FoldersPage.ImportFile.importFilesAction') }}
      </ion-text>
    </ion-item>
    <ion-item
      class="option"
      @click="onOptionClick(ImportType.Folder)"
    >
      <ms-image
        :image="Folder"
        class="option__icon"
      />
      <ion-icon
        class="arrow-up"
        :icon="arrowUp"
      />
      <ion-text class="body item-label">
        {{ $msTranslate('FoldersPage.ImportFile.importFolderAction') }}
      </ion-text>
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import { ImportType } from '@/components/files/types';
import { popoverController } from '@ionic/core';
import { IonIcon, IonItem, IonList, IonText } from '@ionic/vue';
import { arrowUp } from 'ionicons/icons';
import { Folder, ImportMultipleFiles, MsImage, MsModalResult } from 'megashark-lib';

async function onOptionClick(type: ImportType): Promise<void> {
  await popoverController.dismiss(
    {
      type: type,
    },
    MsModalResult.Confirm,
  );
}
</script>

<style lang="scss" scoped>
.import-container {
  padding: 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.option {
  --background-hover: none;
  --color: var(--parsec-color-light-secondary-hard-grey);
  padding: 0.375rem 0.75rem;
  --background: none;
  border-radius: var(--parsec-radius-6);
  --min-height: 0;
  --inner-padding-end: 0;
  position: relative;
  z-index: 2;
  pointer-events: auto;

  &:hover:not(.item-disabled) {
    background: var(--parsec-color-light-primary-50);
    --background-hover: var(--parsec-color-light-primary-50);
    cursor: pointer;
  }

  &::part(native) {
    padding: 0;
  }

  .option__icon {
    margin-right: 0.5rem;
    width: 1.5rem;
  }

  .arrow-up {
    width: 0.625rem;
    height: 0.625rem;
    border-radius: var(--parsec-radius-6);
    bottom: 0;
    left: 1rem;
    padding: 1px;
    position: absolute;
    color: var(--parsec-color-light-primary-600);
    background: var(--parsec-color-light-secondary-white);
  }

  .item-label {
    margin: 0;
  }
}
</style>

<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="import-container">
    <ion-item
      class="option"
      @click="onOptionClick(ImportType.Files)"
    >
      <ion-label class="body item-label">
        {{ $msTranslate('FoldersPage.ImportFile.importFilesAction') }}
      </ion-label>
    </ion-item>
    <ion-item
      class="option"
      @click="onOptionClick(ImportType.Folder)"
    >
      <ion-label class="body item-label">
        {{ $msTranslate('FoldersPage.ImportFile.importFolderAction') }}
      </ion-label>
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import { MsModalResult } from 'megashark-lib';
import { ImportType } from '@/components/files/types';
import { popoverController } from '@ionic/core';
import { IonItem, IonLabel, IonList } from '@ionic/vue';

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
  --color: var(--parsec-color-light-secondary-grey);
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

  .item-label {
    margin: 0;
  }
}
</style>

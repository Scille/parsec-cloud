<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="import-container">
    <ion-item
      class="option"
      @click="onOptionClick(FileContentType.Document)"
    >
      <ms-image
        :image="File.Word"
        class="option__icon"
      />
      <ion-text class="button-medium item-label">
        {{ $msTranslate('FoldersPage.createFile.createDocument') }}
      </ion-text>
    </ion-item>
    <ion-item
      class="option"
      @click="onOptionClick(FileContentType.Spreadsheet)"
    >
      <ms-image
        :image="File.Excel"
        class="option__icon"
      />
      <ion-text class="button-medium item-label">
        {{ $msTranslate('FoldersPage.createFile.createSpreadsheet') }}
      </ion-text>
    </ion-item>
    <ion-item
      class="option"
      @click="onOptionClick(FileContentType.Presentation)"
    >
      <ms-image
        :image="File.Powerpoint"
        class="option__icon"
      />
      <ion-text class="button-medium item-label">
        {{ $msTranslate('FoldersPage.createFile.createPresentation') }}
      </ion-text>
    </ion-item>
    <ion-item
      class="option"
      @click="onOptionClick(FileContentType.Text)"
    >
      <ms-image
        :image="File.Text"
        class="option__icon"
      />
      <ion-text class="button-medium item-label">
        {{ $msTranslate('FoldersPage.createFile.createText') }}
      </ion-text>
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import { FileContentType } from '@/common/fileTypes';
import { popoverController } from '@ionic/core';
import { IonItem, IonList, IonText } from '@ionic/vue';
import { File, MsImage, MsModalResult } from 'megashark-lib';

async function onOptionClick(type: FileContentType): Promise<void> {
  await popoverController.dismiss(
    {
      fileType: type,
    },
    MsModalResult.Confirm,
  );
}
</script>

<style lang="scss" scoped></style>

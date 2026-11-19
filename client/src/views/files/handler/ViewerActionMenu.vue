<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-content id="viewer-action-menu">
    <ion-list class="menu-list">
      <ion-item-group class="list-group">
        <ion-item
          button
          id="file-handler-details"
          @click="onClick(FileHandlerAction.Details)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="informationCircle"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('fileViewers.details') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          id="file-handler-copy-link"
          @click="onClick(FileHandlerAction.CopyLink)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="link"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('fileViewers.copyLink') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          id="file-handler-open-editor"
          @click="onClick(FileHandlerAction.Edit)"
          class="ion-no-padding list-group-item"
          v-if="canEdit"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="create"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('fileViewers.openInEditor') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          id="file-handler-download"
          @click="onClick(FileHandlerAction.Download)"
          class="ion-no-padding list-group-item"
          v-if="isWeb()"
        >
          <ms-image
            :image="DownloadIcon"
            class="list-group-item__icon"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('fileViewers.download') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          id="file-handler-convert-to-pdf"
          @click="onClick(FileHandlerAction.ConvertToPdf)"
          class="ion-no-padding list-group-item"
          v-if="canConvertToPdf"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="documentText"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('fileViewers.convertToPdf') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          id="file-handler-open-with-system"
          @click="onClick(FileHandlerAction.OpenWithSystem)"
          class="ion-no-padding list-group-item"
          v-if="canOpenWithSystem"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="open"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('fileViewers.openWithDefault') }}
          </ion-text>
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-content>
</template>

<script setup lang="ts">
import { isWeb } from '@/parsec';
import { FileHandlerAction } from '@/views/files';
import { IonContent, IonIcon, IonItem, IonItemGroup, IonList, IonText, popoverController } from '@ionic/vue';
import { create, documentText, informationCircle, link, open } from 'ionicons/icons';
import { DownloadIcon, MsImage } from 'megashark-lib';

defineProps<{
  canEdit: boolean;
  canOpenWithSystem: boolean;
  canConvertToPdf: boolean;
}>();

function onClick(action: FileHandlerAction): Promise<boolean> {
  return popoverController.dismiss({ action: action });
}
</script>

<style lang="scss" scoped></style>

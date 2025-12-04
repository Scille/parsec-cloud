<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page id="viewer-action-menu-modal">
    <ion-content class="viewer-action-menu-modal-content">
      <ion-list class="menu-list menu-list-small list-group">
        <ion-item
          button
          @click="onClick(FileHandlerAction.Details)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="informationCircle"
          />
          <ion-text class="button-large list-group-item__label-small">
            {{ $msTranslate('fileViewers.details') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          @click="onClick(FileHandlerAction.Edit)"
          class="ion-no-padding list-group-item"
          v-if="canEdit"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="create"
          />
          <ion-text class="button-large list-group-item__label-small">
            {{ $msTranslate('fileViewers.openInEditor') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          @click="onClick(FileHandlerAction.CopyLink)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="link"
          />
          <ion-text class="button-large list-group-item__label-small">
            {{ $msTranslate('fileViewers.copyLink') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          @click="onClick(FileHandlerAction.Download)"
          class="ion-no-padding list-group-item"
          v-if="isWeb()"
        >
          <ms-image
            :image="DownloadIcon"
            class="item-icon"
          />
          <ion-text class="button-large list-group-item__label-small">
            {{ $msTranslate('fileViewers.download') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          @click="onClick(FileHandlerAction.OpenWithSystem)"
          class="ion-no-padding list-group-item"
          v-if="canOpenWithSystem"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="open"
          />
          <ion-text class="button-large list-group-item__label-small">
            {{ $msTranslate('fileViewers.openWithDefault') }}
          </ion-text>
        </ion-item>
      </ion-list>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { isWeb } from '@/parsec';
import { FileHandlerAction } from '@/views/files';
import { IonContent, IonIcon, IonItem, IonList, IonPage, IonText, modalController } from '@ionic/vue';
import { create, informationCircle, link, open } from 'ionicons/icons';
import { DownloadIcon, MsImage } from 'megashark-lib';

defineProps<{
  canEdit: boolean;
  canOpenWithSystem: boolean;
}>();

function onClick(action: FileHandlerAction): Promise<boolean> {
  return modalController.dismiss({ action: action });
}
</script>

<style lang="scss" scoped>
.item-icon {
  width: 1.125rem;
  height: 1.125rem;
  max-width: 1.125rem;
  max-height: 1.125rem;
  --fill-color: var(--parsec-color-light-secondary-hard-grey);
}
</style>

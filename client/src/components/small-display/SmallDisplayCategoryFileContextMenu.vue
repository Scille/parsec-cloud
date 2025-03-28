<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page id="file-context-sheet-modal">
    <ion-content class="content">
      <small-display-context-menu-buttons
        :left-button="leftButton"
        :right-button="rightButton"
      />
      <ion-list class="menu-list menu-list-small list">
        <ion-item-group class="list-group">
          <ion-item
            button
            v-show="!allSelected"
            @click="onButtonClick(FileAction.SelectAll)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon :icon="checkmarkCircle" />
            <ion-label class="body list-group-item__label-small">
              {{ $msTranslate('FoldersPage.fileContextMenu.actionSelectAll') }}
            </ion-label>
          </ion-item>
          <ion-item
            button
            v-show="someSelected"
            @click="onButtonClick(FileAction.UnselectAll)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon :icon="checkmarkCircleOutline" />
            <ion-label class="body list-group-item__label-small">
              {{ $msTranslate('FoldersPage.fileContextMenu.actionUnselectAll') }}
            </ion-label>
          </ion-item>
          <ion-item
            button
            v-show="!multipleSelected"
            @click="onButtonClick(FileAction.CopyLink)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon :icon="link" />
            <ion-label class="body list-group-item__label-small">
              {{ $msTranslate('FoldersPage.fileContextMenu.actionCopyLink') }}
            </ion-label>
          </ion-item>
        </ion-item-group>
      </ion-list>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { IonContent, IonIcon, IonItem, IonItemGroup, IonLabel, IonList, IonPage } from '@ionic/vue';
import { FileAction } from '@/views/files';
import { modalController } from '@ionic/vue';
import SmallDisplayContextMenuButtons from '@/components/small-display/SmallDisplayContextMenuButtons.vue';
import { checkbox, checkmarkCircle, checkmarkCircleOutline, link, shareSocial } from 'ionicons/icons';

defineProps<{
  multipleSelected?: boolean;
  someSelected?: boolean;
  allSelected?: boolean;
}>();

const leftButton = {
  icon: checkbox,
  text: 'FoldersPage.fileContextMenu.actionSelect',
  callback: (): Promise<boolean> => onButtonClick(FileAction.Select),
};

const rightButton = {
  icon: shareSocial,
  text: 'FoldersPage.fileContextMenu.actionShare',
  callback: (): Promise<boolean> => onButtonClick(FileAction.Share),
};

async function onButtonClick(action: FileAction): Promise<boolean> {
  return await modalController.dismiss({ action: action });
}
</script>

<style lang="scss" scoped>
.content {
  display: flex;
  flex-direction: column;
  height: 20rem;
}
</style>

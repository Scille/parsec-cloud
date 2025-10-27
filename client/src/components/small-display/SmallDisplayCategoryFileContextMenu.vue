<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page id="file-context-sheet-modal">
    <ion-content class="content">
      <small-display-context-menu-buttons
        :left-button="disableSelect ? undefined : leftButton"
        :right-button="rightButton"
      />
      <ion-list
        class="menu-list menu-list-small list"
        v-if="!disableSelect"
      >
        <ion-item-group class="list-group">
          <ion-item
            button
            @click="onButtonClick(FolderGlobalAction.SelectAll)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="checkmarkCircle"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('FoldersPage.fileContextMenu.actionSelectAll') }}
            </ion-text>
          </ion-item>
        </ion-item-group>
      </ion-list>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import SmallDisplayContextMenuButtons from '@/components/small-display/SmallDisplayContextMenuButtons.vue';
import { FolderGlobalAction } from '@/views/files';
import { IonContent, IonIcon, IonItem, IonItemGroup, IonList, IonPage, IonText, modalController } from '@ionic/vue';
import { checkbox, checkmarkCircle, shareSocial } from 'ionicons/icons';

defineProps<{
  disableSelect: boolean;
}>();

const leftButton = {
  icon: checkbox,
  text: 'FoldersPage.fileContextMenu.actionSelect',
  callback: (): Promise<boolean> => onButtonClick(FolderGlobalAction.ToggleSelect),
};

const rightButton = {
  icon: shareSocial,
  text: 'FoldersPage.fileContextMenu.actionShare',
  callback: (): Promise<boolean> => onButtonClick(FolderGlobalAction.Share),
};

async function onButtonClick(action: FolderGlobalAction): Promise<boolean> {
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

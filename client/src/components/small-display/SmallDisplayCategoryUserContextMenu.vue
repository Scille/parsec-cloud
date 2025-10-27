<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page id="user-context-sheet-modal">
    <ion-content class="content">
      <small-display-context-menu-buttons
        :left-button="leftButton"
        :right-button="rightButton"
      />
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import SmallDisplayContextMenuButtons from '@/components/small-display/SmallDisplayContextMenuButtons.vue';
import { UserAction } from '@/views/users/types';
import { IonContent, IonPage, modalController } from '@ionic/vue';
import { checkbox, checkmarkCircle } from 'ionicons/icons';

const leftButton = {
  icon: checkbox,
  text: 'UsersPage.userContextMenu.actionSelect',
  callback: (): Promise<boolean> => onButtonClick(UserAction.ToggleSelect),
};

const rightButton = {
  icon: checkmarkCircle,
  text: 'UsersPage.userContextMenu.actionSelectAll',
  callback: (): Promise<boolean> => onButtonClick(UserAction.SelectAll),
};

async function onButtonClick(action: UserAction): Promise<boolean> {
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

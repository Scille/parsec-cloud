<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page id="user-context-sheet-modal">
    <ion-content class="content">
      <small-display-context-menu-buttons
        :left-button="selectable ? leftButton : undefined"
        :right-button="rightButton"
      />
      <ion-list
        v-if="someSelected"
        class="menu-list menu-list-small list"
      >
        <ion-item-group class="list-group">
          <ion-item
            button
            @click="onButtonClick(UserAction.UnselectAll)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon :icon="checkmarkCircleOutline" />
            <ion-label class="body list-group-item__label-small">
              {{ $msTranslate('FoldersPage.fileContextMenu.actionUnselectAll') }}
            </ion-label>
          </ion-item>
        </ion-item-group>
      </ion-list>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { IonContent, IonIcon, IonItem, IonItemGroup, IonLabel, IonList, IonPage } from '@ionic/vue';
import { modalController } from '@ionic/vue';
import { UserAction } from '@/views/users/UserContextMenu.vue';
import SmallDisplayContextMenuButtons from '@/components/small-display/SmallDisplayContextMenuButtons.vue';
import { checkbox, checkmarkCircle, checkmarkCircleOutline } from 'ionicons/icons';

defineProps<{
  selectable?: boolean;
  someSelected?: boolean;
}>();

const leftButton = {
  icon: checkbox,
  text: 'UsersPage.userContextMenu.actionSelect',
  callback: (): Promise<boolean> => onButtonClick(UserAction.Select),
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

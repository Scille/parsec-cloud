<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<!-- this modal is temporary, it will be replaced by a modal from the megashark-lib -->

<template>
  <ion-page id="fab-modal">
    <ion-list class="add-menu-list menu-list menu-list-small list">
      <ion-item-group
        class="list-group"
        v-for="(actionGroup, index) in actions"
        :key="index"
      >
        <ion-item
          class="list-group-item ion-no-padding"
          v-for="action in actionGroup"
          :key="action.action"
          @click="onActionClicked(action)"
        >
          <ion-icon
            v-if="action.icon"
            class="item-icon"
            :icon="action.icon"
          />
          <ion-label class="button-large list-group-item__label-small">
            {{ $msTranslate(action.label) }}
          </ion-label>
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonIcon, IonItem, IonList, IonItemGroup, IonLabel, modalController } from '@ionic/vue';
import { MenuAction } from '@/views/menu/types';
import { MsModalResult } from 'megashark-lib';

defineProps<{
  actions: Array<Array<MenuAction>>;
}>();

async function onActionClicked(action: MenuAction): Promise<void> {
  await modalController.dismiss({ action: action }, MsModalResult.Confirm);
}
</script>

<style scoped lang="scss"></style>

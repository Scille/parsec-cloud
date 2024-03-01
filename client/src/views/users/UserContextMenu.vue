<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-content id="user-context-menu">
    <ion-list class="menu-list">
      <ion-item-group
        class="list-group"
        v-if="!isRevoked && clientIsAdmin"
      >
        <ion-item class="list-group-title caption-caption">
          <ion-label class="list-group-title__label">
            {{ $t('UsersPage.userContextMenu.titleRemove') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          @click="onClick(UserAction.Revoke)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon :icon="personRemove" />
          <ion-label class="body list-group-item__label">
            {{ $t('UsersPage.userContextMenu.actionRevoke') }}
          </ion-label>
        </ion-item>
      </ion-item-group>

      <ion-item-group class="menu-list-group">
        <ion-item class="list-group-title caption-caption">
          <ion-label class="list-group-title__label">
            {{ $t('UsersPage.userContextMenu.titleDetails') }}
          </ion-label>
        </ion-item>
        <ion-item
          button
          @click="onClick(UserAction.Details)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon :icon="informationCircle" />
          <ion-label class="body list-group-item__label">
            {{ $t('UsersPage.userContextMenu.actionDetails') }}
          </ion-label>
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-content>
</template>

<script lang="ts">
export enum UserAction {
  Revoke,
  Details,
}
</script>

<script setup lang="ts">
import { IonContent, IonIcon, IonItem, IonItemGroup, IonLabel, IonList, popoverController } from '@ionic/vue';
import { informationCircle, personRemove } from 'ionicons/icons';

defineProps<{
  isRevoked: boolean;
  clientIsAdmin?: boolean;
}>();

function onClick(action: UserAction): Promise<boolean> {
  return popoverController.dismiss({ action: action });
}
</script>

<style lang="scss" scoped></style>

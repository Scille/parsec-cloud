<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page id="user-context-sheet-modal">
    <ion-content class="context-sheet-modal-content">
      <ion-list class="menu-list menu-list-small">
        <ion-item-group class="list-group">
          <ion-item
            v-if="canRevoke"
            button
            @click="onClick(UserAction.Revoke)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="personRemove"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate({ key: 'UsersPage.userContextMenu.actionRevoke', count: multipleSelected ? 2 : 1 }) }}
            </ion-text>
          </ion-item>
          <ion-item
            v-if="!multipleSelected"
            button
            @click="onClick(UserAction.AssignRoles)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="returnUpForward"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('UsersPage.userContextMenu.actionAssignRoles') }}
            </ion-text>
          </ion-item>
          <ion-item
            v-if="canUpdateProfile"
            button
            @click="onClick(UserAction.UpdateProfile)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="repeat"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate({ key: 'UsersPage.userContextMenu.actionUpdateProfile', count: multipleSelected ? 2 : 1 }) }}
            </ion-text>
          </ion-item>
        </ion-item-group>

        <ion-item-group
          class="list-group"
          v-if="!multipleSelected"
        >
          <ion-item
            button
            @click="onClick(UserAction.Details)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="informationCircle"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('UsersPage.userContextMenu.actionDetails') }}
            </ion-text>
          </ion-item>
        </ion-item-group>
      </ion-list>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { UserAction } from '@/views/users/types';
import { IonContent, IonIcon, IonItem, IonItemGroup, IonList, IonPage, IonText, modalController } from '@ionic/vue';
import { informationCircle, personRemove, repeat, returnUpForward } from 'ionicons/icons';

defineProps<{
  multipleSelected?: boolean;
  canUpdateProfile?: boolean;
  canRevoke?: boolean;
}>();

async function onClick(action: UserAction): Promise<boolean> {
  return modalController.dismiss({ action: action });
}
</script>

<style lang="scss" scoped></style>

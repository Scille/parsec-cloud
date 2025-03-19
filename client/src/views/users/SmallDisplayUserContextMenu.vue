<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page id="user-context-menu">
    <ion-list class="menu-list menu-list-small">
      <ion-item-group class="list-group">
        <ion-item
          v-if="!user.isRevoked() && clientIsAdmin"
          button
          @click="onClick(UserAction.Revoke)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon :icon="personRemove" />
          <ion-label class="body list-group-item__label-small">
            {{ $msTranslate('UsersPage.userContextMenu.actionRevoke') }}
          </ion-label>
        </ion-item>
        <ion-item
          button
          @click="onClick(UserAction.AssignRoles)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon :icon="returnUpForward" />
          <ion-label class="body list-group-item__label-small">
            {{ $msTranslate('UsersPage.userContextMenu.actionAssignRoles') }}
          </ion-label>
        </ion-item>
        <ion-item
          v-if="!user.isRevoked() && clientIsAdmin && user.currentProfile !== UserProfile.Outsider"
          button
          @click="onClick(UserAction.UpdateProfile)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon :icon="repeat" />
          <ion-label class="body list-group-item__label-small">
            {{ $msTranslate('UsersPage.userContextMenu.actionUpdateProfile') }}
          </ion-label>
        </ion-item>
      </ion-item-group>

      <ion-item-group class="list-group">
        <ion-item
          button
          @click="onClick(UserAction.Details)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon :icon="informationCircle" />
          <ion-label class="body list-group-item__label-small">
            {{ $msTranslate('UsersPage.userContextMenu.actionDetails') }}
          </ion-label>
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-page>
</template>

<script setup lang="ts">
import { UserInfo, UserProfile } from '@/parsec';
import { IonIcon, IonItem, IonItemGroup, IonLabel, IonList, IonPage, modalController } from '@ionic/vue';
import { informationCircle, personRemove, returnUpForward, repeat } from 'ionicons/icons';
import { UserAction } from '@/views/users/UserContextMenu.vue';

defineProps<{
  user: UserInfo;
  clientIsAdmin?: boolean;
}>();

async function onClick(action: UserAction): Promise<boolean> {
  return modalController.dismiss({ action: action });
}
</script>

<style lang="scss" scoped></style>

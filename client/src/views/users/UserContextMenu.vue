<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-content id="user-context-menu">
    <ion-list class="menu-list">
      <ion-item-group class="group">
        <ion-item
          class="group-title caption-caption"
        >
          <ion-label class="group-title__label">
            {{ $t('UsersPage.userContextMenu.titleRemove') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          @click="onClick(UserAction.Revoke)"
          class="group-item"
        >
          <ion-icon :icon="personRemove" />
          <ion-label class="body">
            {{ $t('UsersPage.userContextMenu.actionRevoke') }}
          </ion-label>
        </ion-item>
      </ion-item-group>

      <ion-item-group class="group">
        <ion-item
          class="group-title caption-caption"
        >
          <ion-label class="group-title__label">
            {{ $t('UsersPage.userContextMenu.titleDetails') }}
          </ion-label>
        </ion-item>
        <ion-item
          button
          @click="onClick(UserAction.Details)"
          class="group-item"
        >
          <ion-icon :icon="informationCircle" />
          <ion-label class="body">
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
  Details
}
</script>

<script setup lang="ts" >
import { IonContent, IonItem, IonLabel, IonList, popoverController, IonIcon, IonItemGroup } from '@ionic/vue';
import {
  personRemove,
  informationCircle,
} from 'ionicons/icons';

function onClick(action: UserAction): Promise<boolean> {
  return popoverController.dismiss({'action': action});
}
</script>

<style lang="scss" scoped>
#user-context-menu {
  background-color: var(--parsec-color-light-secondary-inverted-contrast);
}

ion-item {
  --min-height: 1rem;
}

.menu-list {
  margin: 1rem 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.group:not(:last-child) {
  padding-bottom: 1rem;
  border-bottom: solid 1px var(--parsec-color-light-secondary-disabled);
}

.group-title {
  padding: 0 .75rem;
  color: var(--parsec-color-light-secondary-light);
  user-select: none;
  text-transform: uppercase;

  &::part(native) {
    padding-left: .5rem;
  }

  &__label {
    margin: 0 0 0.5rem 0;
  }
}

.group-item {
  padding: 0 .75rem;
  color: var(--parsec-color-light-secondary-grey);
  --border-radius: 4px;

  &::part(native) {
    padding-left: .5rem;
  }

  &:hover {
    --background: var(--parsec-color-light-primary-30);
    --color: var(--parsec-color-light-primary-600);

    ion-icon {
      color: var(--parsec-color-light-primary-600);
    }
  }

  ion-icon {
    font-size: 1.25rem;
    margin-right: .75em;
    color: var(--parsec-color-light-secondary-grey);
  }
}
</style>

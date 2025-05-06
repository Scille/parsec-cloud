<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<!-- this modal is temporary, it will be replaced by a modal from the megashark-lib -->

<template>
  <ion-page id="fab-modal">
    <ion-list class="fab-modal-list">
      <ion-item
        class="fab-modal-list__item ion-no-padding button-medium"
        @click="onClick(AddElementAction.CreateWorkspace)"
      >
        <ion-icon
          class="item-icon"
          :icon="addCircle"
        />
        {{ $msTranslate('SideMenu.tabbar.newWorkspace') }}
      </ion-item>
      <ion-item
        class="fab-modal-list__item ion-no-padding button-medium"
        @click="onClick(AddElementAction.CreateFolder)"
      >
        <ion-icon
          class="item-icon"
          :icon="folderOpen"
        />
        {{ $msTranslate('SideMenu.tabbar.newFolder') }}
      </ion-item>
    </ion-list>
    <ion-list class="fab-modal-list">
      <ion-item
        class="fab-modal-list__item ion-no-padding button-medium"
        @click="onClick(AddElementAction.ImportFolder)"
      >
        <ms-image
          class="item-icon"
          :image="DocumentImport"
        />
        {{ $msTranslate('SideMenu.tabbar.importFolder') }}
      </ion-item>
      <ion-item
        class="fab-modal-list__item ion-no-padding button-medium"
        @click="onClick(AddElementAction.ImportFile)"
      >
        <ms-image
          class="item-icon"
          :image="DocumentImport"
        />
        {{ $msTranslate('SideMenu.tabbar.importFile') }}
      </ion-item>
      <ion-item
        class="fab-modal-list__item ion-no-padding button-medium"
        :disabled="true"
        v-if="false"
      >
        <ion-icon
          class="item-icon"
          :icon="scan"
        />
        {{ $msTranslate('SideMenu.tabbar.scanDocument') }}
      </ion-item>
      <ion-item
        class="fab-modal-list__item ion-no-padding button-medium"
        :disabled="true"
        v-if="false"
      >
        <ion-icon
          class="item-icon"
          :icon="images"
        />
        {{ $msTranslate('SideMenu.tabbar.importPhoto') }}
      </ion-item>
    </ion-list>
    <ion-list class="fab-modal-list">
      <ion-item
        class="fab-modal-list__item ion-no-padding button-medium"
        @click="onClick(AddElementAction.AddUser)"
      >
        <ion-icon
          class="item-icon"
          :icon="personAdd"
        />
        {{ $msTranslate('SideMenu.tabbar.addUser') }}
      </ion-item>
    </ion-list>
  </ion-page>
</template>

<script lang="ts"></script>

<script setup lang="ts">
import { MsImage, DocumentImport } from 'megashark-lib';
import { IonPage, IonIcon, IonItem, IonList, modalController } from '@ionic/vue';
import { personAdd, images, scan, addCircle, folderOpen } from 'ionicons/icons';
import { AddElementAction } from '@/views/menu';

async function onClick(action: AddElementAction): Promise<boolean> {
  return modalController.dismiss({ action: action });
}
</script>

<style scoped lang="scss">
#fab-modal {
  display: flex;
  flex-direction: column;
  padding-top: 1rem;
  background-color: var(--parsec-color-light-background);
}

.fab-modal-list {
  padding: 1rem 1.5rem;

  &:not(:last-child) {
    border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
  }

  &__item {
    display: flex;
    align-items: center;
    padding: 0.75rem 0;
    color: var(--parsec-color-light-secondary-soft-text);

    .item-icon {
      font-size: 1.125rem;
      width: 1.125rem;
      height: 1.125rem;
      color: var(--parsec-color-light-secondary-soft-text);
      --fill-color: var(--parsec-color-light-secondary-soft-text);
      margin-inline-end: 0.75rem;
    }
  }
}
</style>

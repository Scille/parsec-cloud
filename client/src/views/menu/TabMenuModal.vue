<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<!-- this modal is temporary, it will be replaced by a modal from the megashark-lib -->

<template>
  <ion-modal
    class="fab-modal"
    trigger="open-modal"
    :initial-breakpoint="1"
    :breakpoints="[0, 1]"
  >
    <ion-list class="fab-modal-list">
      <ion-item
        class="fab-modal-list__item ion-no-padding button-medium"
        :disabled="true"
      >
        <ion-icon
          class="item-icon"
          :icon="add"
        />
        {{ $msTranslate('SideMenu.tabbar.newWorkspace') }}
      </ion-item>
      <ion-item
        class="fab-modal-list__item ion-no-padding button-medium"
        :disabled="true"
      >
        <ms-image
          class="item-icon"
          :image="DocumentImport"
        />
        {{ $msTranslate('SideMenu.tabbar.newFolder') }}
      </ion-item>
    </ion-list>
    <ion-list class="fab-modal-list">
      <ion-item
        class="fab-modal-list__item ion-no-padding button-medium"
        :disabled="true"
      >
        <ms-image
          class="item-icon"
          :image="DocumentImport"
        />
        {{ $msTranslate('SideMenu.tabbar.importFolder') }}
      </ion-item>
      <ion-item
        class="fab-modal-list__item ion-no-padding button-medium"
        :disabled="true"
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
        :disabled="true"
      >
        <ion-icon
          class="item-icon"
          :icon="personAdd"
        />
        {{ $msTranslate('SideMenu.tabbar.addUser') }}
      </ion-item>
    </ion-list>
  </ion-modal>
</template>

<script setup lang="ts">
import { MsImage, DocumentImport } from 'megashark-lib';
import { IonModal, IonIcon, IonItem, IonList } from '@ionic/vue';
import { add, personAdd, images, scan } from 'ionicons/icons';
</script>

<style scoped lang="scss">
.fab-modal {
  &::part(content) {
    padding-top: 2rem;
  }

  &::part(handle) {
    margin: 1rem auto;
  }

  &-list {
    padding: 1rem 0.75rem;

    &:not(:last-child) {
      border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
    }

    &__item {
      display: flex;
      align-items: center;
      padding: 0.5rem 0;
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
}
</style>

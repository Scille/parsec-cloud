<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<!-- this modal is temporary, it will be replaced by a modal from the megashark-lib -->

<template>
  <ion-page id="fab-modal">
    <ion-list class="add-menu-list menu-list menu-list-small list">
      <ion-item-group class="list-group">
        <ion-item
          class="list-group-item ion-no-padding"
        >
          <ion-icon
            class="item-icon"
            :icon="addCircle"
          />
          <ion-label class="button-large list-group-item__label-small">
            {{ $msTranslate('SideMenu.tabbar.newWorkspace') }}
          </ion-label>
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ion-icon
            class="item-icon"
            :icon="folderOpen"
          />
          <ion-label class="button-large list-group-item__label-small">
            {{ $msTranslate('SideMenu.tabbar.newFolder') }}
          </ion-label>
        </ion-item>
      </ion-item-group>
      <ion-item-group class="list-group">
        <ion-item class="list-group-item ion-no-padding">
          <ms-image
            class="item-icon"
            :image="DocumentImport"
          />
          <ion-label class="button-large list-group-item__label-small">
            {{ $msTranslate('SideMenu.tabbar.importFolder') }}
          </ion-label>
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ms-image
            class="item-icon"
            :image="DocumentImport"
          />
          <ion-label class="button-large list-group-item__label-small">
            {{ $msTranslate('SideMenu.tabbar.importFile') }}
          </ion-label>
        </ion-item>
        <ion-item
          class="list-group-item ion-no-padding"
          :disabled="true"
          v-if="false"
        >
          <ion-icon
            class="item-icon"
            :icon="scan"
          />
          <ion-label class="button-large list-group-item__label-small">
            {{ $msTranslate('SideMenu.tabbar.scanDocument') }}
          </ion-label>
        </ion-item>
        <ion-item
          class="list-group-item ion-no-padding"
          :disabled="true"
          v-if="false"
        >
          <ion-icon
            class="item-icon"
            :icon="images"
          />
          <ion-label class="button-large list-group-item__label-small">
            {{ $msTranslate('SideMenu.tabbar.importPhoto') }}
          </ion-label>
        </ion-item>
      </ion-item-group>
      <ion-item-group class="list-group">
        <ion-item class="list-group-item ion-no-padding">
          <ion-icon
            class="item-icon"
            :icon="personAdd"
          />
          <ion-label class="button-large list-group-item__label-small">
            {{ $msTranslate('SideMenu.tabbar.addUser') }}
          </ion-label>
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-page>
</template>

<script lang="ts"></script>

<script setup lang="ts">
import { MsImage, DocumentImport } from 'megashark-lib';
import { IonPage, IonIcon, IonItem, IonList, IonItemGroup, IonLabel } from '@ionic/vue';
import { personAdd, images, scan, addCircle, folderOpen } from 'ionicons/icons';
</script>

<style scoped lang="scss"></style>

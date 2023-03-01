<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-content>
    <ion-list>
      <ion-item
        button
        lines="none"
        @click="onShareClick()"
      >
        <ion-label>{{ $t('WorkspacesPage.workspaceContextMenu.share') }}</ion-label>
      </ion-item>
      <ion-item
        button
        lines="none"
        @click="onHistoryClick()"
      >
        <ion-label>{{ $t('WorkspacesPage.workspaceContextMenu.history') }}</ion-label>
      </ion-item>
    </ion-list>
  </ion-content>
</template>

<script setup lang = "ts" >
import { IonContent, IonItem, IonLabel, IonList, popoverController } from '@ionic/vue';

function onShareClick(): Promise<boolean> {
  return popoverController.dismiss(null, 'share');
}

function onHistoryClick(): Promise<boolean> {
  return popoverController.dismiss(null, 'history');
}
</script>

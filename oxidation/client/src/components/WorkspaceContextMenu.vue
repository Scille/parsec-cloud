<template>
  <ion-content>
    <ion-list>
      <ion-item
        button
        lines="none"
      >
        <ion-label>{{ $t('WorkspacesPage.workspaceContextMenu.share') }}</ion-label>
      </ion-item>
      <ion-item
        button
        lines="none"
      >
        <ion-label>{{ $t('WorkspacesPage.workspaceContextMenu.history') }}</ion-label>
      </ion-item>
    </ion-list>
  </ion-content>
</template>

<script setup lang = "ts" >
import { IonContent, IonItem, IonLabel, IonList } from '@ionic/vue';
</script>

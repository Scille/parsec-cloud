<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page id="workspace-context-sheet-modal">
    <ion-content class="context-sheet-modal-content">
      <ion-list class="menu-list menu-list-small">
        <ion-item-group class="list-group">
          <ion-item
            button
            @click="onClick(WorkspaceAction.CreateWorkspace)"
            class="ion-no-padding list-group-item item-action"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="addCircle"
            />
            <ion-text class="button-large list-group-item__label">
              {{ $msTranslate('WorkspacesPage.createWorkspace') }}
            </ion-text>
          </ion-item>
        </ion-item-group>
      </ion-list>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { WorkspaceAction } from '@/views/workspaces/types';
import { IonContent, IonIcon, IonItem, IonItemGroup, IonList, IonPage, IonText, modalController } from '@ionic/vue';
import { addCircle } from 'ionicons/icons';

function onClick(action: WorkspaceAction): Promise<boolean> {
  return modalController.dismiss({ action: action });
}
</script>

<style lang="scss" scoped></style>

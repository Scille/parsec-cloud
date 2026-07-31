<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-content id="workspace-context-menu">
    <ion-list class="menu-list">
      <ion-item-group class="list-group">
        <ion-item class="list-group-title button-small">
          <ion-text class="list-group-title__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.titleManage') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          @click="onClick(WorkspaceAction.CreateWorkspace)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="addCircle"
          />
          <ion-text class="button-medium list-group-item__label">
            {{ $msTranslate('WorkspacesPage.createWorkspace') }}
          </ion-text>
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-content>
</template>

<script setup lang="ts">
import { WorkspaceAction } from '@/views/workspaces/types';
import { IonContent, IonIcon, IonItem, IonItemGroup, IonList, IonText, popoverController } from '@ionic/vue';
import { addCircle } from 'ionicons/icons';

function onClick(action: WorkspaceAction): Promise<boolean> {
  return popoverController.dismiss({ action: action });
}
</script>

<style lang="scss" scoped></style>

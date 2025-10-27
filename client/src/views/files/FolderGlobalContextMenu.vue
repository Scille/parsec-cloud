<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-content id="file-context-menu">
    <ion-list class="menu-list">
      <ion-item-group class="list-group">
        <ion-item
          button
          v-if="role !== WorkspaceRole.Reader"
          @click="onClick(FolderGlobalAction.CreateFolder)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="folderOpen"
          />
          <ion-text class="body list-group-item__label">
            {{ $msTranslate('FoldersPage.createFolder') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          v-if="role !== WorkspaceRole.Reader"
          @click="onClick(FolderGlobalAction.ImportFiles)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="cloudUpload"
          />
          <ion-text class="body list-group-item__label">
            {{ $msTranslate('FoldersPage.ImportFile.importFilesAction') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="role !== WorkspaceRole.Reader"
          @click="onClick(FolderGlobalAction.ImportFolder)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="cloudUpload"
          />
          <ion-text class="body list-group-item__label">
            {{ $msTranslate('FoldersPage.ImportFile.importFolderAction') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="isDesktop()"
          @click="onClick(FolderGlobalAction.OpenInExplorer)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="open"
          />
          <ion-text class="body list-group-item__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.actionSeeInExplorer') }}
          </ion-text>
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-content>
</template>

<script setup lang="ts">
import { WorkspaceRole, isDesktop } from '@/parsec';
import { FolderGlobalAction } from '@/views/files/types';
import { IonContent, IonIcon, IonItem, IonItemGroup, IonList, IonText, popoverController } from '@ionic/vue';
import { cloudUpload, folderOpen, open } from 'ionicons/icons';

defineProps<{
  role: WorkspaceRole;
}>();

async function onClick(action: FolderGlobalAction): Promise<boolean> {
  return await popoverController.dismiss({ action: action });
}
</script>

<style lang="scss" scoped></style>

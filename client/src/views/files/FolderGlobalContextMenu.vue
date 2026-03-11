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
          <ion-text class="button-medium list-group-item__label">
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
          <ion-text class="button-medium list-group-item__label">
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
          <ion-text class="button-medium list-group-item__label">
            {{ $msTranslate('FoldersPage.ImportFile.importFolderAction') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="role !== WorkspaceRole.Reader"
          @click="onClick(FolderGlobalAction.CreateFileDocument)"
          class="ion-no-padding list-group-item"
        >
          <ms-image
            class="list-group-item__icon"
            :image="File.Word"
          />
          <ion-text class="button-medium list-group-item__label">
            {{ $msTranslate('FoldersPage.createFile.createDocument') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          v-if="role !== WorkspaceRole.Reader"
          @click="onClick(FolderGlobalAction.CreateFileSpreadsheet)"
          class="ion-no-padding list-group-item"
        >
          <ms-image
            class="list-group-item__icon"
            :image="File.Excel"
          />
          <ion-text class="button-medium list-group-item__label">
            {{ $msTranslate('FoldersPage.createFile.createSpreadsheet') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          v-if="role !== WorkspaceRole.Reader"
          @click="onClick(FolderGlobalAction.CreateFilePresentation)"
          class="ion-no-padding list-group-item"
        >
          <ms-image
            class="list-group-item__icon"
            :image="File.Powerpoint"
          />
          <ion-text class="button-medium list-group-item__label">
            {{ $msTranslate('FoldersPage.createFile.createPresentation') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          v-if="role !== WorkspaceRole.Reader"
          @click="onClick(FolderGlobalAction.CreateFileText)"
          class="ion-no-padding list-group-item"
        >
          <ms-image
            class="list-group-item__icon"
            :image="File.Text"
          />
          <ion-text class="button-medium list-group-item__label">
            {{ $msTranslate('FoldersPage.createFile.createText') }}
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
          <ion-text class="button-medium list-group-item__label">
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
import { File, MsImage } from 'megashark-lib';

defineProps<{
  role: WorkspaceRole;
}>();

async function onClick(action: FolderGlobalAction): Promise<boolean> {
  return await popoverController.dismiss({ action: action });
}
</script>

<style lang="scss" scoped></style>

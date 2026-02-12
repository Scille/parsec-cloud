<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-content id="folder-breadcrumb-context-menu">
    <ion-list class="menu-list">
      <ion-item-group class="list-group">
        <ion-item class="list-group-title button-small">
          <ion-text class="list-group-title__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.titleManageFolder') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          v-if="role !== WorkspaceRole.Reader"
          @click="onClick(FileAction.Rename)"
          class="ion-no-padding list-group-item"
        >
          <ms-image
            class="list-group-item__icon"
            :image="RenameIcon"
          />
          <ion-text class="button-medium list-group-item__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.actionRename') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="isDesktop()"
          @click="onClick(FileAction.SeeInExplorer)"
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

        <ion-item
          button
          @click="onClick(FileAction.ShowHistory)"
          class="ion-no-padding list-group-item"
          v-show="role !== WorkspaceRole.Reader"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="time"
          />
          <ion-text class="button-medium list-group-item__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.actionHistory') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          @click="onClick(FileAction.ShowDetails)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="informationCircle"
          />
          <ion-text class="button-medium list-group-item__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.actionDetails') }}
          </ion-text>
        </ion-item>
      </ion-item-group>
      <ion-item-group class="list-group">
        <ion-item class="list-group-title button-small">
          <ion-text class="list-group-title__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.titleCollaboration') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          @click="onClick(FileAction.CopyLink)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="link"
          />
          <ion-text class="button-medium list-group-item__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.actionCopyLink') }}
          </ion-text>
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-content>
</template>

<script setup lang="ts">
import { isDesktop, WorkspaceRole } from '@/parsec';
import { FileAction } from '@/views/files/types';
import { IonContent, IonIcon, IonItem, IonItemGroup, IonList, IonText, popoverController } from '@ionic/vue';
import { informationCircle, link, open, time } from 'ionicons/icons';
import { MsImage, RenameIcon } from 'megashark-lib';

defineProps<{
  role: WorkspaceRole;
}>();

async function onClick(action: FileAction): Promise<boolean> {
  return await popoverController.dismiss({ action: action });
}
</script>

<style lang="scss" scoped></style>

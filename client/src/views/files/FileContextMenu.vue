<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-content id="file-context-menu">
    <ion-list class="menu-list">
      <ion-item-group class="list-group">
        <ion-item class="list-group-title button-small">
          <ion-text class="list-group-title__label">
            <span v-if="isFile">{{ $msTranslate('FoldersPage.fileContextMenu.titleManageFile') }}</span>
            <span v-else>{{ $msTranslate('FoldersPage.fileContextMenu.titleManageFolder') }}</span>
          </ion-text>
        </ion-item>
        <ion-item
          button
          v-if="!multipleFiles && isFile"
          @click="onClick(FileAction.Preview)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="eye"
          />
          <ion-text class="body list-group-item__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.actionPreview') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          v-if="!multipleFiles && isFile && role !== WorkspaceRole.Reader && isEditable"
          @click="onClick(FileAction.Edit)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="create"
          />
          <ion-text class="body list-group-item__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.actionEdit') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          v-if="!multipleFiles && role !== WorkspaceRole.Reader"
          @click="onClick(FileAction.Rename)"
          class="ion-no-padding list-group-item"
        >
          <ms-image
            class="list-group-item__icon"
            :image="RenameIcon"
          />
          <ion-text class="body list-group-item__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.actionRename') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          v-if="role !== WorkspaceRole.Reader"
          @click="onClick(FileAction.MoveTo)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="arrowRedo"
          />
          <ion-text class="body list-group-item__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.actionMoveTo') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="role !== WorkspaceRole.Reader"
          @click="onClick(FileAction.MakeACopy)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="duplicate"
          />
          <ion-text class="body list-group-item__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.actionMakeACopy') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="!multipleFiles && isFile && isDesktop()"
          @click="onClick(FileAction.Open)"
          class="ion-no-padding list-group-item"
        >
          <ms-image
            class="list-group-item__icon"
            :image="EyeOpenIcon"
          />
          <ion-text class="body list-group-item__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.actionOpen') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="!multipleFiles && isDesktop()"
          @click="onClick(FileAction.SeeInExplorer)"
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

        <ion-item
          button
          @click="onClick(FileAction.ShowHistory)"
          class="ion-no-padding list-group-item"
          v-show="!multipleFiles && role !== WorkspaceRole.Reader"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="time"
          />
          <ion-text class="body list-group-item__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.actionHistory') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="!isDesktop() && isFile"
          @click="onClick(FileAction.Download)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="download"
          />
          <ion-text class="body list-group-item__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.actionDownload') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="!multipleFiles"
          @click="onClick(FileAction.ShowDetails)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="informationCircle"
          />
          <ion-text class="body list-group-item__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.actionDetails') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="role !== WorkspaceRole.Reader"
          @click="onClick(FileAction.Delete)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="trashBin"
          />
          <ion-text class="body list-group-item__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.actionDelete') }}
          </ion-text>
        </ion-item>
      </ion-item-group>
      <ion-item-group
        class="list-group"
        v-if="!multipleFiles"
      >
        <ion-item class="list-group-title button-small">
          <ion-text class="list-group-title__label">
            {{ $msTranslate('FoldersPage.fileContextMenu.titleCollaboration') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          v-if="!multipleFiles"
          @click="onClick(FileAction.CopyLink)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="link"
          />
          <ion-text class="body list-group-item__label">
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
import { arrowRedo, create, download, duplicate, eye, informationCircle, link, open, time, trashBin } from 'ionicons/icons';
import { EyeOpenIcon, MsImage, RenameIcon } from 'megashark-lib';

defineProps<{
  role: WorkspaceRole;
  multipleFiles?: boolean;
  isFile: boolean;
  isEditable?: boolean;
}>();

async function onClick(action: FileAction): Promise<boolean> {
  return await popoverController.dismiss({ action: action });
}
</script>

<style lang="scss" scoped></style>

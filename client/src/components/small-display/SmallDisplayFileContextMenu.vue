<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page id="file-context-sheet-modal">
    <ion-content class="context-sheet-modal-content">
      <ion-list class="menu-list menu-list-small list">
        <ion-item-group class="list-group">
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
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('FoldersPage.fileContextMenu.actionPreview') }}
            </ion-text>
          </ion-item>
          <ion-item
            button
            v-if="!multipleFiles && role !== WorkspaceRole.Reader && isEditable && isFile"
            @click="onClick(FileAction.Edit)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="create"
            />
            <ion-text class="button-large list-group-item__label-small">
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
            <ion-text class="button-large list-group-item__label-small">
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
            <ion-text class="button-large list-group-item__label-small">
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
              :icon="copy"
            />
            <ion-text class="button-large list-group-item__label-small">
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
            <ion-text class="button-large list-group-item__label-small">
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
            <ion-text class="button-large list-group-item__label-small">
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
            <ion-text class="button-large list-group-item__label-small">
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
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('FoldersPage.fileContextMenu.actionDownload') }}
            </ion-text>
          </ion-item>

          <ion-item
            button
            v-show="!multipleFiles"
            @click="onClick(FileAction.CopyLink)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="link"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('FoldersPage.fileContextMenu.actionCopyLink') }}
            </ion-text>
          </ion-item>

          <ion-item
            button
            v-show="!multipleFiles"
            @click="onClick(FileAction.ShowDetails)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="informationCircle"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('FoldersPage.fileContextMenu.actionDetails') }}
            </ion-text>
          </ion-item>
        </ion-item-group>
        <ion-item-group
          class="list-group"
          v-if="role !== WorkspaceRole.Reader"
        >
          <ion-item
            button
            @click="onClick(FileAction.Delete)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="trashBin"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('FoldersPage.fileContextMenu.actionDelete') }}
            </ion-text>
          </ion-item>
        </ion-item-group>
      </ion-list>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { isDesktop, WorkspaceRole } from '@/parsec';
import { FileAction } from '@/views/files/types';
import { IonContent, IonIcon, IonItem, IonItemGroup, IonList, IonPage, IonText, modalController } from '@ionic/vue';
import { arrowRedo, copy, create, download, eye, informationCircle, link, open, time, trashBin } from 'ionicons/icons';
import { EyeOpenIcon, MsImage, RenameIcon } from 'megashark-lib';

defineProps<{
  role: WorkspaceRole;
  multipleFiles?: boolean;
  isFile: boolean;
  isEditable?: boolean;
}>();

async function onClick(action: FileAction): Promise<boolean> {
  return await modalController.dismiss({ action: action });
}
</script>

<style lang="scss" scoped></style>

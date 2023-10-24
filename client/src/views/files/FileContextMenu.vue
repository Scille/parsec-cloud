<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-content id="file-context-menu">
    <ion-list class="menu-list">
      <ion-item-group class="group">
        <ion-item
          class="group-title caption-caption"
        >
          <ion-label class="group-title__label">
            {{ $t('FoldersPage.fileContextMenu.titleManage') }}
          </ion-label>
        </ion-item>
        <ion-item
          button
          @click="onClick(FileAction.Rename)"
          class="group-item"
        >
          <ion-icon :icon="pencil" />
          <ion-label class="body">
            {{ $t('FoldersPage.fileContextMenu.actionRename') }}
          </ion-label>
        </ion-item>
        <ion-item
          button
          @click="onClick(FileAction.MoveTo)"
          class="group-item"
        >
          <ion-icon :icon="arrowRedo" />
          <ion-label class="body">
            {{ $t('FoldersPage.fileContextMenu.actionMoveTo') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          @click="onClick(FileAction.MakeACopy)"
          class="group-item"
        >
          <ion-icon :icon="copy" />
          <ion-label class="body">
            {{ $t('FoldersPage.fileContextMenu.actionMakeACopy') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          @click="onClick(FileAction.Delete)"
          class="group-item"
        >
          <ion-icon :icon="trashBin" />
          <ion-label class="body">
            {{ $t('FoldersPage.fileContextMenu.actionDelete') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          @click="onClick(FileAction.Open)"
          class="group-item"
        >
          <ion-icon :icon="open" />
          <ion-label class="body">
            {{ $t('FoldersPage.fileContextMenu.actionOpen') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          @click="onClick(FileAction.ShowHistory)"
          class="group-item"
        >
          <ion-icon :icon="time" />
          <ion-label class="body">
            {{ $t('FoldersPage.fileContextMenu.actionHistory') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          v-show="!isDesktop()"
          @click="onClick(FileAction.Download)"
          class="group-item"
        >
          <ion-icon :icon="download" />
          <ion-label class="body">
            {{ $t('FoldersPage.fileContextMenu.actionDownload') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          @click="onClick(FileAction.ShowDetails)"
          class="group-item"
        >
          <ion-icon :icon="informationCircle" />
          <ion-label class="body">
            {{ $t('FoldersPage.fileContextMenu.actionDetails') }}
          </ion-label>
        </ion-item>
      </ion-item-group>
      <ion-item-group class="group">
        <ion-item
          class="group-title caption-caption"
        >
          <ion-label class="group-title__label">
            {{ $t('FoldersPage.fileContextMenu.titleCollaboration') }}
          </ion-label>
        </ion-item>
        <ion-item
          button
          @click="onClick(FileAction.CopyLink)"
          class="group-item"
        >
          <ion-icon :icon="link" />
          <ion-label class="body">
            {{ $t('FoldersPage.fileContextMenu.actionCopyLink') }}
          </ion-label>
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-content>
</template>

<script lang="ts">
export enum FileAction {
  Rename,
  MoveTo,
  MakeACopy,
  Delete,
  Open,
  ShowHistory,
  Download,
  ShowDetails,
  CopyLink
}
</script>

<script setup lang="ts" >
import { IonContent, IonItem, IonLabel, IonList, popoverController, IonIcon, IonItemGroup } from '@ionic/vue';
import {
  copy,
  arrowRedo,
  pencil,
  link,
  informationCircle,
  download,
  time,
  open,
  trashBin,
} from 'ionicons/icons';
import { isDesktop } from '@/parsec';

async function onClick(action: FileAction): Promise<boolean> {
  return await popoverController.dismiss({'action': action});
}
</script>

<style lang="scss" scoped>
#file-context-menu {
  background-color: var(--parsec-color-light-secondary-inverted-contrast);
}

ion-item {
  --min-height: 1rem;
}

.menu-list {
  margin: 1rem 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.group:not(:last-child) {
  padding-bottom: 1rem;
  border-bottom: solid 1px var(--parsec-color-light-secondary-disabled);
}

.group-title {
  padding: 0 .75rem;
  color: var(--parsec-color-light-secondary-light);
  user-select: none;
  text-transform: uppercase;

  &::part(native) {
    padding-left: .5rem;
  }

  &__label {
    margin: 0 0 0.5rem 0;
  }
}

.group-item {
  padding: 0 .75rem;
  color: var(--parsec-color-light-secondary-grey);
  --border-radius: 4px;

  &::part(native) {
    padding-left: .5rem;
  }

  &:hover {
    --background: var(--parsec-color-light-primary-30);
    --color: var(--parsec-color-light-primary-600);

    ion-icon {
      color: var(--parsec-color-light-primary-600);
    }
  }

  ion-icon {
    font-size: 1.25rem;
    margin-right: .75em;
    color: var(--parsec-color-light-secondary-grey);
  }
}
</style>

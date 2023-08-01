<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-content id="workspace-context-menu">
    <ion-list class="menu-list">
      <ion-item-group class="group">
        <ion-item
          class="group-title caption-caption"
        >
          <ion-label class="group-title__label">
            {{ $t('WorkspacesPage.workspaceContextMenu.titleOffline') }}
          </ion-label>
        </ion-item>
        <ion-item
          button
          @click="onClick(WorkspaceAction.MakeAvailableOffline)"
          class="group-item"
        >
          <ion-icon :icon="cloudy" />
          <ion-label class="body">
            {{ $t('WorkspacesPage.workspaceContextMenu.actionOffline') }}
          </ion-label>
        </ion-item>
      </ion-item-group>

      <ion-item-group class="group">
        <ion-item
          class="group-title caption-caption"
        >
          <ion-label class="group-title__label">
            {{ $t('WorkspacesPage.workspaceContextMenu.titleManage') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          @click="onClick(WorkspaceAction.Rename)"
          class="group-item"
        >
          <ion-icon :icon="pencil" />
          <ion-label class="body">
            {{ $t('WorkspacesPage.workspaceContextMenu.actionRename') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          @click="onClick(WorkspaceAction.OpenInExplorer)"
          class="group-item"
        >
          <ion-icon :icon="open" />
          <ion-label class="body">
            {{ $t('WorkspacesPage.workspaceContextMenu.actionOpenInExplorer') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          @click="onClick(WorkspaceAction.ShowHistory)"
          class="group-item"
        >
          <ion-icon :icon="time" />
          <ion-label class="body">
            {{ $t('WorkspacesPage.workspaceContextMenu.actionHistory') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          @click="onClick(WorkspaceAction.ShowDetails)"
          class="group-item"
        >
          <ion-icon :icon="informationCircle" />
          <ion-label class="body">
            {{ $t('WorkspacesPage.workspaceContextMenu.actionDetails') }}
          </ion-label>
        </ion-item>
      </ion-item-group>
      <ion-item-group class="group">
        <ion-item
          class="group-title caption-caption"
        >
          <ion-label class="group-title__label">
            {{ $t('WorkspacesPage.workspaceContextMenu.titleCollaboration') }}
          </ion-label>
        </ion-item>
        <ion-item
          button
          @click="onClick(WorkspaceAction.CopyLink)"
          class="group-item"
        >
          <ion-icon :icon="link" />
          <ion-label class="body">
            {{ $t('WorkspacesPage.workspaceContextMenu.actionCopyLink') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          @click="onClick(WorkspaceAction.Share)"
          class="group-item"
        >
          <ion-icon :icon="shareSocial" />
          <ion-label class="body">
            {{ $t('WorkspacesPage.workspaceContextMenu.actionShare') }}
          </ion-label>
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-content>
</template>

<script lang="ts">
export enum WorkspaceAction {
  Rename,
  MakeAvailableOffline,
  CopyLink,
  ShowDetails,
  Share,
  ShowHistory,
  OpenInExplorer,
  Mount,
}
</script>

<script setup lang="ts" >
import { IonContent, IonItem, IonLabel, IonList, popoverController, IonIcon, IonItemGroup } from '@ionic/vue';
import {
  cloudy,
  pencil,
  link,
  informationCircle,
  shareSocial,
  time,
  open,
} from 'ionicons/icons';

function onClick(action: WorkspaceAction): Promise<boolean> {
  return popoverController.dismiss({'action': action});
}
</script>

<style lang="scss" scoped>
#workspace-context-menu {
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

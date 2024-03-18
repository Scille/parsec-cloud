<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-content id="workspace-context-menu">
    <ion-list class="menu-list">
      <ion-item-group
        class="list-group"
        v-show="false"
      >
        <ion-item class="list-group-title caption-caption">
          <ion-label class="list-group-title__label">
            {{ $t('WorkspacesPage.workspaceContextMenu.titleOffline') }}
          </ion-label>
        </ion-item>
        <ion-item
          button
          @click="onClick(WorkspaceAction.MakeAvailableOffline)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon :icon="cloudy" />
          <ion-label class="body list-group-item__label">
            {{ $t('WorkspacesPage.workspaceContextMenu.actionOffline') }}
          </ion-label>
        </ion-item>
      </ion-item-group>

      <ion-item-group
        class="list-group"
        v-show="isDesktop()"
      >
        <ion-item class="list-group-title caption-caption">
          <ion-label class="list-group-title__label">
            {{ $t('WorkspacesPage.workspaceContextMenu.titleManage') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          @click="onClick(WorkspaceAction.Rename)"
          class="ion-no-padding list-group-item"
          v-show="false"
        >
          <ion-icon :icon="pencil" />
          <ion-label class="body list-group-item__label">
            {{ $t('WorkspacesPage.workspaceContextMenu.actionRename') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          v-show="isDesktop()"
          @click="onClick(WorkspaceAction.OpenInExplorer)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon :icon="open" />
          <ion-label class="body list-group-item__label">
            {{ $t('WorkspacesPage.workspaceContextMenu.actionOpenInExplorer') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          @click="onClick(WorkspaceAction.ShowHistory)"
          class="ion-no-padding list-group-item"
          v-show="false"
        >
          <ion-icon :icon="time" />
          <ion-label class="body list-group-item__label">
            {{ $t('WorkspacesPage.workspaceContextMenu.actionHistory') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          v-show="clientProfile !== UserProfile.Outsider && false"
          @click="onClick(WorkspaceAction.ShowDetails)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon :icon="informationCircle" />
          <ion-label class="body list-group-item__label">
            {{ $t('WorkspacesPage.workspaceContextMenu.actionDetails') }}
          </ion-label>
        </ion-item>
      </ion-item-group>
      <ion-item-group class="list-group">
        <ion-item class="list-group-title caption-caption">
          <ion-label class="list-group-title__label">
            {{ $t('WorkspacesPage.workspaceContextMenu.titleCollaboration') }}
          </ion-label>
        </ion-item>
        <ion-item
          button
          @click="onClick(WorkspaceAction.CopyLink)"
          class="ion-no-padding list-group-item"
          v-show="false"
        >
          <ion-icon :icon="link" />
          <ion-label class="body list-group-item__label">
            {{ $t('WorkspacesPage.workspaceContextMenu.actionCopyLink') }}
          </ion-label>
        </ion-item>

        <ion-item
          v-show="clientProfile !== UserProfile.Outsider"
          button
          @click="onClick(WorkspaceAction.Share)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon :icon="shareSocial" />
          <ion-label class="body list-group-item__label">
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

<script setup lang="ts">
import { UserProfile, isDesktop } from '@/parsec';
import { IonContent, IonIcon, IonItem, IonItemGroup, IonLabel, IonList, popoverController } from '@ionic/vue';
import { cloudy, informationCircle, link, open, pencil, shareSocial, time } from 'ionicons/icons';

function onClick(action: WorkspaceAction): Promise<boolean> {
  return popoverController.dismiss({ action: action });
}

defineProps<{
  clientProfile: UserProfile;
}>();
</script>

<style lang="scss" scoped></style>

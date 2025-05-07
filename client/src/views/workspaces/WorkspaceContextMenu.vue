<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-content id="workspace-context-menu">
    <div class="list-title">
      <ion-label class="list-title__text button-medium">
        {{ workspaceName }}
      </ion-label>
      <ion-icon
        v-show="isFavorite"
        class="favorite"
        :icon="star"
      />
    </div>
    <ion-list class="menu-list">
      <ion-item-group
        class="list-group"
        v-show="false"
      >
        <ion-item class="list-group-title button-small">
          <ion-label class="list-group-title__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.titleOffline') }}
          </ion-label>
        </ion-item>
        <ion-item
          button
          @click="onClick(WorkspaceAction.MakeAvailableOffline)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon :icon="cloudy" />
          <ion-label class="body list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionOffline') }}
          </ion-label>
        </ion-item>
      </ion-item-group>

      <ion-item-group
        class="list-group"
        v-show="isDesktop() || clientRole === WorkspaceRole.Owner"
      >
        <ion-item class="list-group-title button-small">
          <ion-label class="list-group-title__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.titleManage') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          v-show="clientRole === WorkspaceRole.Owner"
          @click="onClick(WorkspaceAction.Rename)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon :icon="pencil" />
          <ion-label class="body list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionRename') }}
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
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionOpenInExplorer') }}
          </ion-label>
        </ion-item>

        <ion-item
          button
          @click="onClick(WorkspaceAction.ShowHistory)"
          class="ion-no-padding list-group-item"
          v-show="clientRole === WorkspaceRole.Manager || clientRole === WorkspaceRole.Owner"
        >
          <ion-icon :icon="time" />
          <ion-label class="body list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionHistory') }}
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
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionDetails') }}
          </ion-label>
        </ion-item>
      </ion-item-group>
      <ion-item-group class="list-group">
        <ion-item class="list-group-title button-small">
          <ion-label class="list-group-title__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.titleCollaboration') }}
          </ion-label>
        </ion-item>
        <ion-item
          button
          @click="onClick(WorkspaceAction.CopyLink)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon :icon="link" />
          <ion-label class="body list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionCopyLink') }}
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
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionShare') }}
          </ion-label>
        </ion-item>
      </ion-item-group>
      <ion-item-group class="list-group">
        <ion-item class="list-group-title button-small">
          <ion-label class="list-group-title__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.titleMisc') }}
          </ion-label>
        </ion-item>
        <ion-item
          button
          @click="onClick(WorkspaceAction.Favorite)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon :icon="star" />
          <ion-label class="body list-group-item__label">
            {{
              $msTranslate(
                isFavorite
                  ? 'WorkspacesPage.workspaceContextMenu.actionRemoveFavorite'
                  : 'WorkspacesPage.workspaceContextMenu.actionAddFavorite',
              )
            }}
          </ion-label>
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-content>
</template>

<script setup lang="ts">
import { UserProfile, WorkspaceName, WorkspaceRole, isDesktop } from '@/parsec';
import { IonContent, IonIcon, IonItem, IonItemGroup, IonLabel, IonList, popoverController } from '@ionic/vue';
import { cloudy, informationCircle, link, open, pencil, shareSocial, star, time } from 'ionicons/icons';
import { WorkspaceAction } from '@/views/workspaces/types';

function onClick(action: WorkspaceAction): Promise<boolean> {
  return popoverController.dismiss({ action: action });
}

defineProps<{
  workspaceName: WorkspaceName;
  clientProfile: UserProfile;
  clientRole: WorkspaceRole;
  isFavorite: boolean;
}>();
</script>

<style lang="scss" scoped>
.list-title {
  border-bottom: solid 1px var(--parsec-color-light-secondary-disabled);
  padding: 0.75rem 0.75rem;
  background-color: var(--parsec-color-light-secondary-premiere);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: flex;
  align-items: center;
  justify-content: space-between;

  &__text {
    color: var(--parsec-color-light-secondary-soft-text);
  }

  .favorite {
    display: flex;
    font-size: 1rem;
    color: var(--parsec-color-light-primary-600);
  }
}
</style>

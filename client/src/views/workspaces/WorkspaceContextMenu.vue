<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-content id="workspace-context-menu">
    <div class="list-title">
      <ion-text class="list-title__text">
        {{ workspace.name }}
      </ion-text>
      <ion-icon
        v-show="isFavorite"
        class="favorite"
        :icon="star"
      />
    </div>
    <ion-list class="menu-list">
      <ion-item-group
        class="list-group"
        v-if="false"
      >
        <ion-item class="list-group-title">
          <ion-text class="list-group-title__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.titleOffline') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          @click="onClick(WorkspaceAction.MakeAvailableOffline)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="cloudy"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionOffline') }}
          </ion-text>
        </ion-item>
      </ion-item-group>

      <ion-item-group class="list-group">
        <ion-item class="list-group-title">
          <ion-text class="list-group-title__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.titleManage') }}
          </ion-text>
        </ion-item>

        <ion-item
          v-if="workspace.canSelfPromoteToOwner"
          button
          @click="onClick(WorkspaceAction.TakeOwnership)"
          class="ion-no-padding list-group-item item-action"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="repeat"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionTakeOwnership') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="workspace.selfRole === WorkspaceRole.Owner && !(workspace.isArchived || workspace.isTrashed)"
          @click="onClick(WorkspaceAction.Rename)"
          class="ion-no-padding list-group-item"
        >
          <ms-image
            class="list-group-item__icon"
            :image="RenameIcon"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionRename') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="isDesktop() && !(workspace.isArchived || workspace.isTrashed)"
          @click="onClick(WorkspaceAction.OpenInExplorer)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="open"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionOpenInExplorer') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          @click="onClick(WorkspaceAction.ShowHistory)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="time"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionHistory') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="!isHidden && !(workspace.isArchived || workspace.isTrashed)"
          @click="onClick(WorkspaceAction.UnMount)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="eyeOff"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionHide') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="isHidden && !(workspace.isArchived || workspace.isTrashed)"
          @click="onClick(WorkspaceAction.Mount)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="eye"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionShow') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="workspace.selfRole === WorkspaceRole.Owner && !(workspace.isArchived || workspace.isTrashed)"
          @click="onClick(WorkspaceAction.Archive)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="archive"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionArchive') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          v-if="(workspace.isArchived || workspace.isTrashed) && workspace.selfRole === WorkspaceRole.Owner"
          @click="onClick(WorkspaceAction.Restore)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="reload"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionRestore') }}
          </ion-text>
        </ion-item>

        <ion-item
          button
          @click="onClick(WorkspaceAction.ShowDetails)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="informationCircle"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionDetails') }}
          </ion-text>
        </ion-item>

        <ion-item
          v-if="!workspace.isTrashed && workspace.selfRole === WorkspaceRole.Owner"
          button
          @click="onClick(WorkspaceAction.Trash)"
          class="ion-no-padding list-group-item item-danger"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="trash"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionDelete') }}
          </ion-text>
        </ion-item>
      </ion-item-group>
      <ion-item-group
        v-if="!(workspace.isArchived || workspace.isTrashed)"
        class="list-group"
      >
        <ion-item class="list-group-title">
          <ion-text class="list-group-title__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.titleCollaboration') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          @click="onClick(WorkspaceAction.CopyLink)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="link"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionCopyLink') }}
          </ion-text>
        </ion-item>

        <ion-item
          v-if="clientProfile !== UserProfile.Outsider"
          button
          @click="onClick(WorkspaceAction.Share)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="shareSocial"
          />
          <ion-text class="list-group-item__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionShare') }}
          </ion-text>
        </ion-item>
      </ion-item-group>
      <ion-item-group
        v-if="!(workspace.isArchived || workspace.isTrashed)"
        class="list-group"
      >
        <ion-item class="list-group-title">
          <ion-text class="list-group-title__label">
            {{ $msTranslate('WorkspacesPage.workspaceContextMenu.titleMisc') }}
          </ion-text>
        </ion-item>
        <ion-item
          button
          @click="onClick(WorkspaceAction.Favorite)"
          class="ion-no-padding list-group-item"
        >
          <ion-icon
            class="list-group-item__icon"
            :icon="star"
          />
          <ion-text class="list-group-item__label">
            {{
              $msTranslate(
                isFavorite
                  ? 'WorkspacesPage.workspaceContextMenu.actionRemoveFavorite'
                  : 'WorkspacesPage.workspaceContextMenu.actionAddFavorite',
              )
            }}
          </ion-text>
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-content>
</template>

<script setup lang="ts">
import { UserProfile, WorkspaceInfo, WorkspaceRole, isDesktop } from '@/parsec';
import { WorkspaceAction } from '@/views/workspaces/types';
import { IonContent, IonIcon, IonItem, IonItemGroup, IonList, IonText, popoverController } from '@ionic/vue';
import {
  archive,
  cloudy,
  eye,
  eyeOff,
  informationCircle,
  link,
  open,
  reload,
  repeat,
  shareSocial,
  star,
  time,
  trash,
} from 'ionicons/icons';
import { MsImage, RenameIcon } from 'megashark-lib';

function onClick(action: WorkspaceAction): Promise<boolean> {
  return popoverController.dismiss({ action: action });
}

defineProps<{
  workspace: WorkspaceInfo;
  clientProfile: UserProfile;
  isFavorite: boolean;
  isHidden: boolean;
}>();
</script>

<style lang="scss" scoped>
.list-title {
  border-bottom: solid ms.border('thin') ms.color('border-base-default');
  padding: ms.spacing('padding-2xl');
  background-color: ms.color('surface-base-page-secondary');
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: space-between;

  &__text {
    @include ms.font('label-md-medium');

    color: ms.color('text-neutral-default');
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .favorite {
    display: flex;
    flex-shrink: 0;
    font-size: 1rem;
    color: ms.color('icon-brand-default');
  }
}
</style>

<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page id="workspace-context-sheet-modal">
    <ion-content class="context-sheet-modal-content">
      <ion-list class="menu-list menu-list-small">
        <ion-item-group
          class="list-group"
          v-show="false"
        >
          <ion-item
            button
            @click="onClick(WorkspaceAction.MakeAvailableOffline)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="cloudy"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionOffline') }}
            </ion-text>
          </ion-item>
        </ion-item-group>

        <ion-item-group class="list-group">
          <ion-item
            button
            v-show="workspace.selfRole === WorkspaceRole.Owner && !(workspace.isArchived || workspace.isTrashed)"
            @click="onClick(WorkspaceAction.Rename)"
            class="ion-no-padding list-group-item"
          >
            <ms-image
              class="list-group-item__icon"
              :image="RenameIcon"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionRename') }}
            </ion-text>
          </ion-item>

          <ion-item
            button
            v-show="isDesktop() && !(workspace.isArchived || workspace.isTrashed)"
            @click="onClick(WorkspaceAction.OpenInExplorer)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="open"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionOpenInExplorer') }}
            </ion-text>
          </ion-item>

          <ion-item
            button
            @click="onClick(WorkspaceAction.ShowHistory)"
            class="ion-no-padding list-group-item"
            v-show="workspace.selfRole === WorkspaceRole.Manager || workspace.selfRole === WorkspaceRole.Owner"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="time"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionHistory') }}
            </ion-text>
          </ion-item>

          <ion-item
            button
            v-show="clientProfile !== UserProfile.Outsider && false"
            @click="onClick(WorkspaceAction.ShowDetails)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="informationCircle"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionDetails') }}
            </ion-text>
          </ion-item>

          <ion-item
            button
            v-show="!isHidden && !(workspace.isArchived || workspace.isTrashed)"
            @click="onClick(WorkspaceAction.UnMount)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="eyeOff"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionHide') }}
            </ion-text>
          </ion-item>

          <ion-item
            button
            v-show="isHidden && !(workspace.isArchived || workspace.isTrashed)"
            @click="onClick(WorkspaceAction.Mount)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="eye"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionShow') }}
            </ion-text>
          </ion-item>
          <ion-item
            button
            v-show="workspace.selfRole === WorkspaceRole.Owner && !(workspace.isArchived || workspace.isTrashed)"
            @click="onClick(WorkspaceAction.Archive)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="archive"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionArchive') }}
            </ion-text>
          </ion-item>
          <ion-item
            button
            v-show="(workspace.isArchived || workspace.isTrashed) && workspace.selfRole === WorkspaceRole.Owner"
            @click="onClick(WorkspaceAction.Restore)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="reload"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionRestore') }}
            </ion-text>
          </ion-item>
          <ion-item
            v-show="workspace.isArchived && workspace.selfRole === WorkspaceRole.Owner"
            button
            @click="onClick(WorkspaceAction.Trash)"
            class="ion-no-padding list-group-item item-danger"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="trash"
            />
            <ion-text class="button-large list-group-item__label">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionDelete') }}
            </ion-text>
          </ion-item>
        </ion-item-group>
        <ion-item-group
          v-if="!(workspace.isArchived || workspace.isTrashed)"
          class="list-group"
        >
          <ion-item
            button
            @click="onClick(WorkspaceAction.CopyLink)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="link"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionCopyLink') }}
            </ion-text>
          </ion-item>

          <ion-item
            v-show="clientProfile !== UserProfile.Outsider"
            button
            @click="onClick(WorkspaceAction.Share)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="shareSocial"
            />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionShare') }}
            </ion-text>
          </ion-item>
        </ion-item-group>
        <ion-item-group
          v-if="!(workspace.isArchived || workspace.isTrashed)"
          class="list-group"
        >
          <ion-item
            button
            @click="onClick(WorkspaceAction.Favorite)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon
              class="list-group-item__icon"
              :icon="star"
            />
            <ion-text class="button-large list-group-item__label-small">
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
  </ion-page>
</template>

<script setup lang="ts">
import { UserProfile, WorkspaceInfo, WorkspaceRole, isDesktop } from '@/parsec';
import { WorkspaceAction } from '@/views/workspaces/types';
import { IonContent, IonIcon, IonItem, IonItemGroup, IonList, IonPage, IonText, modalController } from '@ionic/vue';
import { archive, cloudy, eye, eyeOff, informationCircle, link, open, reload, shareSocial, star, time, trash } from 'ionicons/icons';
import { MsImage, RenameIcon } from 'megashark-lib';

function onClick(action: WorkspaceAction): Promise<boolean> {
  return modalController.dismiss({ action: action });
}

defineProps<{
  workspace: WorkspaceInfo;
  clientProfile: UserProfile;
  isFavorite: boolean;
  isHidden: boolean;
}>();
</script>

<style lang="scss" scoped></style>

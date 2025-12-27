<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page id="workspace-context-sheet-modal">
    <ion-content class="context-sheet-modal-content">
      <ion-list class="menu-list menu-list-small">
        <ion-item-group
          class="list-group"
          v-show="isDesktop() || clientRole === WorkspaceRole.Owner"
        >
          <ion-item
            button
            v-show="clientRole === WorkspaceRole.Owner"
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
            v-show="isDesktop()"
            @click="onClick(WorkspaceAction.OpenInExplorer)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon :icon="open" />
            <ion-text class="button-large list-group-item__label-small">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionOpenInExplorer') }}
            </ion-text>
          </ion-item>

          <ion-item
            button
            @click="onClick(WorkspaceAction.ShowHistory)"
            class="ion-no-padding list-group-item"
            v-show="clientRole === WorkspaceRole.Manager || clientRole === WorkspaceRole.Owner"
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

          <ion-item
            button
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
        </ion-item-group>
        <ion-item-group class="list-group">
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
        <ion-item-group class="list-group">
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
import { UserProfile, WorkspaceName, WorkspaceRole, isDesktop } from '@/parsec';
import { WorkspaceAction } from '@/views/workspaces/types';
import { IonContent, IonIcon, IonItem, IonItemGroup, IonList, IonPage, IonText, modalController } from '@ionic/vue';
import { cloudy, informationCircle, link, open, shareSocial, star, time } from 'ionicons/icons';
import { MsImage, RenameIcon } from 'megashark-lib';

function onClick(action: WorkspaceAction): Promise<boolean> {
  return modalController.dismiss({ action: action });
}

defineProps<{
  workspaceName: WorkspaceName;
  clientProfile: UserProfile;
  clientRole: WorkspaceRole;
  isFavorite: boolean;
}>();
</script>

<style lang="scss" scoped></style>

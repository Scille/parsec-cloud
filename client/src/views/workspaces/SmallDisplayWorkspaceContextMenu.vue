<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page id="workspace-context-sheet-modal">
    <ion-content class="content">
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
            <ion-icon :icon="cloudy" />
            <ion-label class="body list-group-item__label-small">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionOffline') }}
            </ion-label>
          </ion-item>
        </ion-item-group>

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
            <ion-icon :icon="pencil" />
            <ion-label class="body list-group-item__label-small">
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
            <ion-label class="body list-group-item__label-small">
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
            <ion-label class="body list-group-item__label-small">
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
            <ion-label class="body list-group-item__label-small">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionDetails') }}
            </ion-label>
          </ion-item>
        </ion-item-group>
        <ion-item-group class="list-group">
          <ion-item
            button
            @click="onClick(WorkspaceAction.CopyLink)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon :icon="link" />
            <ion-label class="body list-group-item__label-small">
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
            <ion-label class="body list-group-item__label-small">
              {{ $msTranslate('WorkspacesPage.workspaceContextMenu.actionShare') }}
            </ion-label>
          </ion-item>
        </ion-item-group>
        <ion-item-group class="list-group">
          <ion-item
            button
            @click="onClick(WorkspaceAction.Favorite)"
            class="ion-no-padding list-group-item"
          >
            <ion-icon :icon="star" />
            <ion-label class="body list-group-item__label-small">
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
  </ion-page>
</template>

<script setup lang="ts">
import { UserProfile, WorkspaceName, WorkspaceRole, isDesktop } from '@/parsec';
import { IonContent, IonIcon, IonItem, IonItemGroup, IonLabel, IonList, IonPage, modalController } from '@ionic/vue';
import { cloudy, informationCircle, link, open, pencil, shareSocial, star, time } from 'ionicons/icons';
import { WorkspaceAction } from '@/views/workspaces/WorkspaceContextMenu.vue';

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

<style lang="scss" scoped>
.content {
  height: 20rem;
}
</style>

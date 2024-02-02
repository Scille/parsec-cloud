<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      class="content-scroll"
    >
      <ms-action-bar id="workspaces-ms-action-bar">
        <!-- contextual menu -->
        <ms-action-bar-button
          v-show="clientProfile != UserProfile.Outsider"
          id="button-new-workspace"
          :button-label="$t('WorkspacesPage.createWorkspace')"
          :icon="addCircle"
          @click="openCreateWorkspaceModal()"
        />
        <div class="right-side">
          <div class="counter">
            <ion-text class="body">
              {{ $t('WorkspacesPage.itemCount', { count: workspaceList.length }, workspaceList.length) }}
            </ion-text>
          </div>
          <ms-sorter
            id="workspace-filter-select"
            :options="msSorterOptions"
            default-option="name"
            :sorter-labels="msSorterLabels"
            @change="onMsSorterChange($event)"
          />
          <ms-grid-list-toggle v-model="displayView" />
        </div>
      </ms-action-bar>
      <!-- workspaces -->
      <div class="workspaces-container scroll">
        <div v-if="filteredWorkspaces.length === 0">
          {{ $t('WorkspacesPage.noWorkspaces') }}
        </div>

        <div v-if="filteredWorkspaces.length > 0 && displayView === DisplayState.List">
          <ion-list class="list">
            <ion-list-header
              class="workspace-list-header"
              lines="full"
            >
              <ion-label class="workspace-list-header__label cell-title label-name">
                {{ $t('WorkspacesPage.listDisplayTitles.name') }}
              </ion-label>
              <ion-label class="workspace-list-header__label cell-title label-role">
                {{ $t('WorkspacesPage.listDisplayTitles.role') }}
              </ion-label>
              <ion-label
                class="workspace-list-header__label cell-title label-users"
                v-show="clientProfile !== UserProfile.Outsider"
              >
                {{ $t('WorkspacesPage.listDisplayTitles.sharedWith') }}
              </ion-label>
              <ion-label class="workspace-list-header__label cell-title label-update">
                {{ $t('WorkspacesPage.listDisplayTitles.lastUpdate') }}
              </ion-label>
              <ion-label class="workspace-list-header__label cell-title label-size">
                {{ $t('WorkspacesPage.listDisplayTitles.size') }}
              </ion-label>
              <ion-label class="workspace-list-header__label cell-title label-space" />
            </ion-list-header>
            <workspace-list-item
              v-for="workspace in filteredWorkspaces"
              :key="workspace.id"
              :workspace="workspace"
              :client-profile="clientProfile"
              @click="onWorkspaceClick"
              @menu-click="openWorkspaceContextMenu"
              @share-click="onWorkspaceShareClick"
            />
          </ion-list>
        </div>
        <div
          v-if="filteredWorkspaces.length > 0 && displayView === DisplayState.Grid"
          class="workspaces-container-grid list"
        >
          <ion-item
            class="workspaces-grid-item"
            v-for="workspace in filteredWorkspaces"
            :key="workspace.id"
          >
            <workspace-card
              :workspace="workspace"
              :client-profile="clientProfile"
              @click="onWorkspaceClick"
              @menu-click="openWorkspaceContextMenu"
              @share-click="onWorkspaceShareClick"
            />
          </ion-item>
        </div>
      </div>
      <ion-fab
        v-if="isPlatform('mobile')"
        vertical="bottom"
        horizontal="end"
        slot="fixed"
      >
        <ion-fab-button @click="openCreateWorkspaceModal()">
          <ion-icon :icon="addCircle" />
        </ion-fab-button>
      </ion-fab>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonContent,
  IonFab,
  IonFabButton,
  IonIcon,
  IonItem,
  IonLabel,
  IonList,
  IonListHeader,
  IonPage,
  IonText,
  isPlatform,
  modalController,
  popoverController,
} from '@ionic/vue';

import { writeTextToClipboard } from '@/common/clipboard';
import { workspaceNameValidator } from '@/common/validators';
import {
  DisplayState,
  MsActionBar,
  MsActionBarButton,
  MsGridListToggle,
  MsOptions,
  MsSorter,
  MsSorterChangeEvent,
  getTextInputFromUser,
} from '@/components/core';
import WorkspaceCard from '@/components/workspaces/WorkspaceCard.vue';
import WorkspaceListItem from '@/components/workspaces/WorkspaceListItem.vue';
import {
  UserProfile,
  WorkspaceInfo,
  getClientProfile,
  createWorkspace as parsecCreateWorkspace,
  getPathLink as parsecGetPathLink,
  listWorkspaces as parsecListWorkspaces,
} from '@/parsec';
import { navigateToWorkspace } from '@/router';
import { Information, InformationKey, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { translate } from '@/services/translation';
import WorkspaceContextMenu, { WorkspaceAction } from '@/views/workspaces/WorkspaceContextMenu.vue';
import WorkspaceSharingModal from '@/views/workspaces/WorkspaceSharingModal.vue';
import { addCircle } from 'ionicons/icons';
import { Ref, computed, inject, onMounted, ref } from 'vue';

const sortBy = ref('name');
const sortByAsc = ref(true);
const workspaceList: Ref<Array<WorkspaceInfo>> = ref([]);
const displayView = ref(DisplayState.Grid);
// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const informationManager: InformationManager = inject(InformationKey)!;
const msSorterLabels = {
  asc: translate('HomePage.organizationList.sortOrderAsc'),
  desc: translate('HomePage.organizationList.sortOrderDesc'),
};
const clientProfile: Ref<UserProfile> = ref(UserProfile.Outsider);

onMounted(async (): Promise<void> => {
  clientProfile.value = await getClientProfile();
  await refreshWorkspacesList();
});

async function refreshWorkspacesList(): Promise<void> {
  const result = await parsecListWorkspaces();
  if (result.ok) {
    workspaceList.value = result.value;
  } else {
    informationManager.present(
      new Information({
        message: translate('WorkspacesPage.listError.message'),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

const filteredWorkspaces = computed(() => {
  return Array.from(workspaceList.value).sort((a: WorkspaceInfo, b: WorkspaceInfo) => {
    if (sortBy.value === 'name') {
      return sortByAsc.value ? a.name.localeCompare(b.name) : b.name.localeCompare(a.name);
    } else if (sortBy.value === 'size') {
      return sortByAsc.value ? a.size - b.size : b.size - a.size;
    } else if (sortBy.value === 'lastUpdated') {
      return sortByAsc.value ? b.lastUpdated.diff(a.lastUpdated).milliseconds : a.lastUpdated.diff(b.lastUpdated).milliseconds;
    }
    return 0;
  });
});

const msSorterOptions: MsOptions = new MsOptions([
  { label: translate('WorkspacesPage.sort.sortByName'), key: 'name' },
  { label: translate('WorkspacesPage.sort.sortBySize'), key: 'size' },
  { label: translate('WorkspacesPage.sort.sortByLastUpdated'), key: 'lastUpdated' },
]);

function onMsSorterChange(event: MsSorterChangeEvent): void {
  sortBy.value = event.option.key;
  sortByAsc.value = event.sortByAsc;
}

async function openCreateWorkspaceModal(): Promise<void> {
  const workspaceName = await getTextInputFromUser({
    title: translate('WorkspacesPage.CreateWorkspaceModal.pageTitle'),
    trim: true,
    validator: workspaceNameValidator,
    inputLabel: translate('WorkspacesPage.CreateWorkspaceModal.label'),
    placeholder: translate('WorkspacesPage.CreateWorkspaceModal.placeholder'),
    okButtonText: translate('WorkspacesPage.CreateWorkspaceModal.create'),
  });

  if (workspaceName) {
    const result = await parsecCreateWorkspace(workspaceName);
    if (result.ok) {
      informationManager.present(
        new Information({
          message: translate('WorkspacesPage.newWorkspaceSuccess.message', {
            workspace: workspaceName,
          }),
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
      await refreshWorkspacesList();
    } else {
      informationManager.present(
        new Information({
          message: translate('WorkspacesPage.newWorkspaceError.message'),
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  }
}

async function onWorkspaceClick(_event: Event, workspace: WorkspaceInfo): Promise<void> {
  await navigateToWorkspace(workspace.id);
}

async function onWorkspaceShareClick(_: Event, workspace: WorkspaceInfo): Promise<void> {
  const modal = await modalController.create({
    component: WorkspaceSharingModal,
    componentProps: {
      workspaceId: workspace.id,
      ownRole: workspace.selfCurrentRole,
    },
    cssClass: 'workspace-sharing-modal',
  });
  await modal.present();
  await modal.onWillDismiss();
  await refreshWorkspacesList();
}

async function openWorkspaceContextMenu(event: Event, workspace: WorkspaceInfo): Promise<void> {
  const popover = await popoverController.create({
    component: WorkspaceContextMenu,
    cssClass: 'workspace-context-menu',
    event: event,
    translucent: true,
    showBackdrop: false,
    dismissOnSelect: true,
    alignment: 'end',
    componentProps: {
      clientProfile: clientProfile.value,
    },
  });
  await popover.present();

  const { data } = await popover.onDidDismiss();
  if (data !== undefined) {
    if (data.action === WorkspaceAction.Share) {
      onWorkspaceShareClick(new Event('ignored'), workspace);
    } else if (data.action === WorkspaceAction.CopyLink) {
      await copyLinkToClipboard(workspace);
    }
  }
}

async function copyLinkToClipboard(workspace: WorkspaceInfo): Promise<void> {
  const result = await parsecGetPathLink(workspace.id, '/');

  if (result.ok) {
    if (!(await writeTextToClipboard(result.value))) {
      informationManager.present(
        new Information({
          message: translate('WorkspacesPage.linkNotCopiedToClipboard.message'),
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.present(
        new Information({
          message: translate('WorkspacesPage.linkCopiedToClipboard.message'),
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    informationManager.present(
      new Information({
        message: translate('WorkspacesPage.getLinkError.message', { reason: result.error.tag }),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}
</script>

<style lang="scss" scoped>
.workspaces-container {
  background-color: white;
}

.workspace-list-header {
  &__label {
    padding: 0 1rem;
    height: 100%;
    display: flex;
    align-items: center;
  }

  .label-name {
    width: 100%;
    max-width: 20vw;
    min-width: 11.25rem;
    white-space: nowrap;
    overflow: hidden;
  }

  .label-role {
    min-width: 11.25rem;
    max-width: 10vw;
    flex-grow: 2;
  }

  .label-users {
    min-width: 14.5rem;
    flex-grow: 0;
  }

  .label-update {
    min-width: 11.25rem;
    flex-grow: 0;
  }

  .label-size {
    min-width: 7.5rem;
  }

  .label-space {
    min-width: 4rem;
    flex-grow: 0;
    margin-left: auto;
    margin-right: 1rem;
  }
}

.workspaces-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5em;
  overflow-y: auto;
}

ion-item::part(native) {
  --padding-start: 0px;
}
</style>

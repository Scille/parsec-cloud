<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      class="content-scroll"
    >
      <ms-action-bar
        id="workspaces-ms-action-bar"
        v-if="isLargeDisplay"
      >
        <!-- contextual menu -->
        <ms-action-bar-button
          v-show="clientProfile != UserProfile.Outsider"
          id="button-new-workspace"
          :button-label="'WorkspacesPage.createWorkspace'"
          :icon="addCircle"
          @click="openCreateWorkspaceModal()"
        />
        <div class="right-side">
          <div class="counter">
            <ion-text class="body">
              {{ $msTranslate({ key: 'WorkspacesPage.itemCount', data: { count: workspaceList.length }, count: workspaceList.length }) }}
            </ion-text>
          </div>
          <ms-search-input
            v-model="filterWorkspaceName"
            placeholder="WorkspacesPage.filterPlaceholder"
            id="search-input-workspace"
          />
          <ms-sorter
            id="workspace-filter-select"
            :options="msSorterOptions"
            default-option="name"
            :sorter-labels="msSorterLabels"
            @change="onMsSorterChange($event)"
          />
          <ms-grid-list-toggle
            v-model="displayView"
            @update:model-value="onDisplayStateChange"
          />
        </div>
      </ms-action-bar>

      <!-- workspaces -->
      <div class="workspaces-container scroll">
        <div
          class="mobile-filters"
          v-if="isSmallDisplay"
        >
          <ms-search-input
            v-model="filterWorkspaceName"
            placeholder="WorkspacesPage.filterPlaceholder"
            id="search-input-workspace"
            class="mobile-filters__search"
          />
          <ms-sorter
            id="workspace-filter-select"
            :options="msSorterOptions"
            default-option="name"
            :sorter-labels="msSorterLabels"
            @change="onMsSorterChange($event)"
            class="mobile-filters__sorter"
          />
        </div>
        <div
          v-show="querying && filteredWorkspaces.length === 0"
          class="no-workspaces-loading"
        >
          <ms-spinner :title="'WorkspacesPage.loading'" />
        </div>

        <div
          v-if="!querying && filteredWorkspaces.length === 0"
          class="no-workspaces body"
        >
          <div class="no-workspaces-content">
            <ms-image
              :image="NoWorkspace"
              class="no-workspaces-content__image"
            />
            <ion-text>
              {{
                workspaceList.length > 0 ? $msTranslate('WorkspacesPage.noMatchingWorkspaces') : $msTranslate('WorkspacesPage.noWorkspaces')
              }}
            </ion-text>
            <ion-button
              v-show="clientProfile != UserProfile.Outsider"
              id="new-workspace"
              fill="outline"
              @click="openCreateWorkspaceModal()"
            >
              <ion-icon :icon="addCircle" />
              {{ $msTranslate('WorkspacesPage.createWorkspace') }}
            </ion-button>
          </div>
        </div>

        <div v-if="filteredWorkspaces.length > 0 && displayView === DisplayState.List">
          <ion-list class="list">
            <ion-list-header
              class="workspace-list-header"
              lines="full"
            >
              <ion-label class="workspace-list-header__label cell-title label-name">
                {{ $msTranslate('WorkspacesPage.listDisplayTitles.name') }}
              </ion-label>
              <ion-label class="workspace-list-header__label cell-title label-role">
                {{ $msTranslate('WorkspacesPage.listDisplayTitles.role') }}
              </ion-label>
              <ion-label
                class="workspace-list-header__label cell-title label-users"
                v-show="clientProfile !== UserProfile.Outsider"
              >
                {{ $msTranslate('WorkspacesPage.listDisplayTitles.sharedWith') }}
              </ion-label>
              <ion-label
                class="workspace-list-header__label cell-title label-update"
                v-show="false"
              >
                {{ $msTranslate('WorkspacesPage.listDisplayTitles.lastUpdate') }}
              </ion-label>
              <ion-label
                class="workspace-list-header__label cell-title label-size"
                v-show="false"
              >
                {{ $msTranslate('WorkspacesPage.listDisplayTitles.size') }}
              </ion-label>
              <ion-label class="workspace-list-header__label cell-title label-space" />
            </ion-list-header>
            <workspace-list-item
              v-for="workspace in filteredWorkspaces"
              :key="workspace.id"
              :workspace="workspace"
              :client-profile="clientProfile"
              :is-favorite="favorites.includes(workspace.id)"
              @click="onWorkspaceClick"
              @favorite-click="onWorkspaceFavoriteClick"
              @menu-click="onOpenWorkspaceContextMenu"
              @share-click="onWorkspaceShareClick"
            />
          </ion-list>
        </div>
        <div
          v-if="filteredWorkspaces.length > 0 && displayView === DisplayState.Grid"
          class="workspaces-container-grid list"
        >
          <workspace-card
            v-for="workspace in filteredWorkspaces"
            :key="workspace.id"
            :workspace="workspace"
            :client-profile="clientProfile"
            :is-favorite="favorites.includes(workspace.id)"
            @click="onWorkspaceClick"
            @favorite-click="onWorkspaceFavoriteClick"
            @menu-click="onOpenWorkspaceContextMenu"
            @share-click="onWorkspaceShareClick"
          />
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { workspaceNameValidator } from '@/common/validators';
import {
  MsOptions,
  getTextFromUser,
  MsImage,
  NoWorkspace,
  DisplayState,
  MsActionBar,
  MsActionBarButton,
  MsGridListToggle,
  MsSorter,
  MsSorterChangeEvent,
  MsSearchInput,
  MsSpinner,
  useWindowSize,
} from 'megashark-lib';
import {
  WORKSPACES_PAGE_DATA_KEY,
  WorkspaceDefaultData,
  WorkspacesPageSavedData,
  openWorkspaceContextMenu,
  toggleFavorite,
  workspaceShareClick,
} from '@/components/workspaces';
import WorkspaceCard from '@/components/workspaces/WorkspaceCard.vue';
import WorkspaceListItem from '@/components/workspaces/WorkspaceListItem.vue';
import {
  ClientInfo,
  getClientInfo as parsecGetClientInfo,
  EntryName,
  ParsecWorkspacePathAddr,
  Path,
  UserProfile,
  WorkspaceID,
  WorkspaceInfo,
  WorkspaceName,
  decryptFileLink,
  entryStat,
  getClientProfile,
  isDesktop,
  parseFileLink,
  createWorkspace as parsecCreateWorkspace,
  getWorkspaceSharing as parsecGetWorkspaceSharing,
  listWorkspaces as parsecListWorkspaces,
  mountWorkspace as parsecMountWorkspace,
} from '@/parsec';
import { Routes, currentRouteIs, getCurrentRouteQuery, navigateTo, navigateToWorkspace, watchRoute } from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { IonButton, IonContent, IonIcon, IonLabel, IonList, IonListHeader, IonPage, IonText } from '@ionic/vue';
import { addCircle } from 'ionicons/icons';
import { Ref, computed, inject, onMounted, onUnmounted, ref } from 'vue';
import { recentDocumentManager } from '@/services/recentDocuments';

enum SortWorkspaceBy {
  Name = 'name',
  Size = 'size',
  LastUpdate = 'lastUpdate',
}

const { isLargeDisplay, isSmallDisplay } = useWindowSize();
const userInfo: Ref<ClientInfo | null> = ref(null);
const sortBy = ref(SortWorkspaceBy.Name);
const sortByAsc = ref(true);
const workspaceList: Ref<Array<WorkspaceInfo>> = ref([]);
const displayView = ref(DisplayState.Grid);
const favorites: Ref<WorkspaceID[]> = ref([]);
const filterWorkspaceName = ref('');
const querying = ref(true);

const informationManager: InformationManager = inject(InformationManagerKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;

let eventCbId: string | null = null;

const routeWatchCancel = watchRoute(async () => {
  if (!currentRouteIs(Routes.Workspaces)) {
    return;
  }
  const query = getCurrentRouteQuery();
  if (query.workspaceName) {
    await createWorkspace(query.workspaceName);
    await navigateTo(Routes.Workspaces, { replace: true, query: {} });
  } else if (query.fileLink) {
    const success = await handleFileLink(query.fileLink);
    if (!success) {
      await navigateTo(Routes.Workspaces, { query: {} });
    }
  }
  await refreshWorkspacesList();
});

const msSorterLabels = {
  asc: 'HomePage.organizationList.sortOrderAsc',
  desc: 'HomePage.organizationList.sortOrderDesc',
};
const clientProfile: Ref<UserProfile> = ref(UserProfile.Outsider);
let hotkeys: HotkeyGroup | null = null;

async function loadFavorites(): Promise<void> {
  favorites.value = (
    await storageManager.retrieveComponentData<WorkspacesPageSavedData>(WORKSPACES_PAGE_DATA_KEY, WorkspaceDefaultData)
  ).favoriteList;
}

onMounted(async (): Promise<void> => {
  displayView.value = (
    await storageManager.retrieveComponentData<WorkspacesPageSavedData>(WORKSPACES_PAGE_DATA_KEY, WorkspaceDefaultData)
  ).displayState;

  await loadFavorites();

  hotkeys = hotkeyManager.newHotkeys();
  hotkeys.add(
    { key: 'n', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Workspaces },
    openCreateWorkspaceModal,
  );
  hotkeys.add(
    { key: 'g', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Workspaces },
    async () => {
      displayView.value = displayView.value === DisplayState.Grid ? DisplayState.List : DisplayState.Grid;
    },
  );

  eventCbId = await eventDistributor.registerCallback(
    Events.WorkspaceFavorite | Events.WorkspaceUpdated | Events.WorkspaceCreated,
    async (event: Events, _data?: EventData) => {
      switch (event) {
        case Events.WorkspaceFavorite:
          await loadFavorites();
          break;
        case Events.WorkspaceUpdated:
        case Events.WorkspaceCreated:
          await refreshWorkspacesList();
          break;
        default:
          console.log(`Unhandled event ${event}`);
          break;
      }
    },
  );

  clientProfile.value = await getClientProfile();
  await refreshWorkspacesList();

  const infoResult = await parsecGetClientInfo();

  if (infoResult.ok) {
    userInfo.value = infoResult.value;
  } else {
    window.electronAPI.log('error', `Failed to retrieve user info ${JSON.stringify(infoResult.error)}`);
  }

  const query = getCurrentRouteQuery();
  if (query.fileLink) {
    const success = await handleFileLink(query.fileLink);
    if (!success) {
      await navigateTo(Routes.Workspaces, { query: {} });
    }
  }
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
  routeWatchCancel();
  if (eventCbId) {
    eventDistributor.removeCallback(eventCbId);
  }
});

async function handleFileLink(fileLink: ParsecWorkspacePathAddr): Promise<boolean> {
  const parseResult = await parseFileLink(fileLink);

  if (!parseResult.ok) {
    informationManager.present(
      new Information({
        message: 'link.invalidFileLink',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return false;
  }

  const workspace = workspaceList.value.find((w) => w.id === parseResult.value.workspaceId);
  if (!workspace) {
    informationManager.present(
      new Information({
        message: 'link.workspaceNotFound',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return false;
  }

  const decryptResult = await decryptFileLink(workspace.handle, fileLink);
  if (!decryptResult.ok) {
    informationManager.present(
      new Information({
        message: 'link.invalidFileLink',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return false;
  }

  const fileInfoResult = await entryStat(workspace.handle, decryptResult.value);
  if (!fileInfoResult.ok) {
    informationManager.present(
      new Information({
        message: 'link.fileNotFound',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return false;
  }

  let selectFile: EntryName | undefined = undefined;
  let path = decryptResult.value;
  if (fileInfoResult.value.isFile()) {
    const fileName = await Path.filename(decryptResult.value);
    if (fileName) {
      selectFile = fileName;
    }
    path = await Path.parent(path);
  }

  recentDocumentManager.addWorkspace(workspace);
  await recentDocumentManager.saveToStorage(storageManager);

  await navigateToWorkspace(workspace.handle, path, selectFile, true);
  return true;
}

async function onDisplayStateChange(): Promise<void> {
  await storageManager.updateComponentData<WorkspacesPageSavedData>(
    WORKSPACES_PAGE_DATA_KEY,
    { displayState: displayView.value },
    WorkspaceDefaultData,
  );
}

async function refreshWorkspacesList(): Promise<void> {
  if (!currentRouteIs(Routes.Workspaces)) {
    return;
  }
  querying.value = true;
  const result = await parsecListWorkspaces();
  if (result.ok) {
    for (const wk of result.value) {
      const sharingResult = await parsecGetWorkspaceSharing(wk.id, false);
      if (sharingResult.ok) {
        wk.sharing = sharingResult.value;
      } else {
        console.warn(`Failed to get sharing for ${wk.currentName}`);
      }
      if (isDesktop() && wk.mountpoints.length === 0) {
        const mountResult = await parsecMountWorkspace(wk.handle);
        if (mountResult.ok) {
          wk.mountpoints.push(mountResult.value);
        } else {
          console.warn(`Failed to mount ${wk.currentName}: ${mountResult.error.error}`);
        }
      }
    }
    workspaceList.value = result.value;
    const recentWorkspaces = recentDocumentManager.getWorkspaces();
    // If a workspace is listed in recent workspaces but not present in the list, remove it
    for (const recentWorkspace of recentWorkspaces) {
      if (workspaceList.value.find((wk) => wk.id === recentWorkspace.id) === undefined) {
        recentDocumentManager.removeWorkspace(recentWorkspace);
      }
    }
    await recentDocumentManager.saveToStorage(storageManager);
  } else {
    informationManager.present(
      new Information({
        message: 'WorkspacesPage.listError',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
  querying.value = false;
}

const filteredWorkspaces = computed(() => {
  const filter = filterWorkspaceName.value.toLocaleLowerCase();
  return Array.from(workspaceList.value)
    .filter((workspace) => workspace.currentName.toLocaleLowerCase().includes(filter))
    .sort((a: WorkspaceInfo, b: WorkspaceInfo) => {
      if (favorites.value.includes(b.id) !== favorites.value.includes(a.id)) {
        return favorites.value.includes(b.id) ? 1 : -1;
      }
      if (sortBy.value === SortWorkspaceBy.Name) {
        return sortByAsc.value ? a.currentName.localeCompare(b.currentName) : b.currentName.localeCompare(a.currentName);
      } else if (sortBy.value === SortWorkspaceBy.Size) {
        return sortByAsc.value ? a.size - b.size : b.size - a.size;
      } else if (sortBy.value === SortWorkspaceBy.LastUpdate) {
        return sortByAsc.value ? b.lastUpdated.diff(a.lastUpdated).milliseconds : a.lastUpdated.diff(b.lastUpdated).milliseconds;
      }
      return 0;
    });
});

const msSorterOptions: MsOptions = new MsOptions([
  { label: 'WorkspacesPage.sort.sortByName', key: SortWorkspaceBy.Name },
  // for now we don't have the data for both size and last update
  // { label: 'WorkspacesPage.sort.sortBySize', key: SortWorkspaceBy.Size },
  // { label: 'WorkspacesPage.sort.sortByLastUpdated', key: SortWorkspaceBy.LastUpdate },
]);

function onMsSorterChange(event: MsSorterChangeEvent): void {
  sortBy.value = event.option.key;
  sortByAsc.value = event.sortByAsc;
}

async function createWorkspace(name: WorkspaceName): Promise<void> {
  const result = await parsecCreateWorkspace(name);
  if (result.ok) {
    informationManager.present(
      new Information({
        message: {
          key: 'WorkspacesPage.newWorkspaceSuccess',
          data: {
            workspace: name,
          },
        },
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  } else {
    informationManager.present(
      new Information({
        message: 'WorkspacesPage.newWorkspaceError',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

async function openCreateWorkspaceModal(): Promise<void> {
  const workspaceName = await getTextFromUser({
    title: 'WorkspacesPage.CreateWorkspaceModal.pageTitle',
    trim: true,
    validator: workspaceNameValidator,
    inputLabel: 'WorkspacesPage.CreateWorkspaceModal.label',
    placeholder: 'WorkspacesPage.CreateWorkspaceModal.placeholder',
    okButtonText: 'WorkspacesPage.CreateWorkspaceModal.create',
  });

  if (workspaceName) {
    await createWorkspace(workspaceName);
  }
}

async function onWorkspaceClick(workspace: WorkspaceInfo): Promise<void> {
  recentDocumentManager.addWorkspace(workspace);
  await recentDocumentManager.saveToStorage(storageManager);
  await navigateToWorkspace(workspace.handle);
}

async function onWorkspaceFavoriteClick(workspace: WorkspaceInfo): Promise<void> {
  await toggleFavorite(workspace, favorites.value, eventDistributor, storageManager);
}

async function onWorkspaceShareClick(workspace: WorkspaceInfo): Promise<void> {
  await workspaceShareClick(workspace, informationManager);
  await refreshWorkspacesList();
}

async function onOpenWorkspaceContextMenu(workspace: WorkspaceInfo, event: Event, onFinished?: () => void): Promise<void> {
  await openWorkspaceContextMenu(event, workspace, favorites.value, eventDistributor, informationManager, storageManager);
  await refreshWorkspacesList();
  if (onFinished) {
    onFinished();
  }
}
</script>

<style lang="scss" scoped>
.no-workspaces {
  max-width: 30rem;
  color: var(--parsec-color-light-secondary-grey);
  display: flex;
  margin: auto;
  height: 100%;
  align-items: center;

  &-content,
  &-loading {
    border-radius: var(--parsec-radius-8);
    display: flex;
    height: fit-content;
    text-align: center;
    flex-direction: column;
    gap: 1rem;
    align-items: center;
    padding: 3rem 1rem;

    #new-workspace {
      display: flex;
      align-items: center;

      ion-icon {
        margin-inline: 0em;
        margin-right: 0.375rem;
      }
    }
  }
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
    max-width: 40vw;
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
    width: 4rem;
    flex-grow: 0;
    margin-left: auto;
  }
}

.workspaces-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5em;
  overflow: visible;
  padding: 2rem 0;

  @include ms.responsive-breakpoint('sm') {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
  }

  @include ms.responsive-breakpoint('xs') {
    grid-template-columns: repeat(1, 1fr);
  }
}
</style>

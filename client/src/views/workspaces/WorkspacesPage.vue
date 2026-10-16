<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      class="content-scroll"
    >
      <ms-action-bar
        id="workspaces-ms-action-bar"
        v-if="isLargeDisplay"
        :buttons="search ? [] : actionBarOptionsWorkspacesPage"
      >
        <div class="right-side">
          <div
            class="counter"
            v-show="!search"
          >
            <ion-text>
              {{
                $msTranslate({
                  key: 'WorkspacesPage.itemCount',
                  data: { count: workspaceList.length },
                  count: workspaceList.length,
                })
              }}
            </ion-text>
          </div>
          <ms-search-input
            placeholder="WorkspacesPage.filterPlaceholder"
            id="search-input-workspace"
            ref="searchInputRef"
            :debounce="300"
            @change="onSearchValueChange"
            v-model="searchPattern"
          />
          <workspace-filter
            v-show="!search"
            v-model:filters="workspaceFilters"
            class="mobile-filters__filter"
          />
          <ms-sorter
            v-show="!search"
            id="workspace-filter-select"
            :options="msSorterOptions"
            default-option="name"
            :sorter-labels="msSorterLabels"
            @change="onMsSorterChange($event)"
          />
          <ms-grid-list-toggle
            v-show="!search"
            v-model="displayView"
            @update:model-value="onDisplayStateChange"
          />
        </div>
      </ms-action-bar>

      <div
        class="workspace-filters-menu"
        v-if="isLargeDisplay && !search"
      >
        <workspace-categories-menu
          :active-menu="workspaceMenuState"
          :show-hidden="someHiddenWorkspaces"
          @update-menu="onMenuUpdate"
        />
      </div>

      <!-- workspaces -->
      <div
        class="main-container workspaces-container scroll"
        @contextmenu.stop="openWorkspaceGlobalContextMenu"
      >
        <div
          class="mobile-filters"
          v-if="isSmallDisplay"
        >
          <div class="mobile-filters-buttons">
            <workspace-filter
              v-show="!search"
              v-model:filters="workspaceFilters"
              class="mobile-filters-buttons__filter"
            />
            <ms-sorter
              id="workspace-filter-select"
              v-show="!search"
              :options="msSorterOptions"
              default-option="name"
              :sorter-labels="msSorterLabels"
              @change="onMsSorterChange($event)"
              class="mobile-filters-buttons__sorter"
            />
          </div>
          <workspace-categories-menu
            v-show="!search"
            :active-menu="workspaceMenuState"
            :show-hidden="someHiddenWorkspaces"
            @update-menu="onMenuUpdate"
          />
          <ms-search-input
            placeholder="WorkspacesPage.filterPlaceholder"
            id="search-input-workspace"
            class="mobile-filters__search"
            :debounce="300"
            @change="onSearchValueChange"
            v-model="searchPattern"
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
          class="no-workspaces"
        >
          <div
            class="no-all-workspaces"
            v-if="workspaceMenuState === WorkspaceMenu.All && filteredWorkspaces.length === 0"
          >
            <ms-image
              :image="isDarkTheme ? NoWorkspaceDark : NoWorkspace"
              class="no-workspaces__image"
            />
            <ion-text v-if="workspaceList.length > 0">
              <ms-rich-text
                v-if="workspaceList.every((wk) => workspaceAttributes.isHidden(wk.id))"
                text="WorkspacesPage.allWorkspacesHidden"
              />
              <span v-else-if="filteredWorkspaces.length === 0"> {{ $msTranslate('WorkspacesPage.noMatchingWorkspaces') }}</span>
            </ion-text>
            <span v-else>
              <span v-if="userInfo?.currentProfile === UserProfile.Outsider">{{
                $msTranslate('WorkspacesPage.noWorkspacesExternal')
              }}</span>
              <span v-else>{{ $msTranslate('WorkspacesPage.noWorkspaces') }}</span>
            </span>
            <ion-button
              v-if="showHiddenWorkspacesButton"
              @click="workspaceMenuState = WorkspaceMenu.Hidden"
              id="show-hidden-workspaces"
            >
              {{ $msTranslate('WorkspacesPage.categoriesMenu.showHiddenWorkspaces') }}
            </ion-button>
            <ion-button
              v-if="workspaceList.length === 0"
              v-show="clientProfile !== UserProfile.Outsider"
              id="new-workspace"
              class="button-large"
              fill="outline"
              @click="actions.openCreateWorkspaceModal()"
            >
              <ion-icon
                class="button-icon-left"
                :icon="addCircle"
              />
              {{ $msTranslate('WorkspacesPage.createWorkspace') }}
            </ion-button>
          </div>
          <div
            class="no-recent-workspaces"
            v-if="workspaceMenuState === WorkspaceMenu.Recent && filteredWorkspaces.length === 0"
          >
            <ms-image
              :image="isDarkTheme ? NoRecentWorkspaceDark : NoRecentWorkspace"
              class="no-workspaces__image"
            />
            <ion-text>
              {{ $msTranslate('WorkspacesPage.categoriesMenu.noRecentWorkspaces') }}
            </ion-text>
          </div>
          <div
            class="no-favorite-workspaces"
            v-if="workspaceMenuState === WorkspaceMenu.Favorites && filteredWorkspaces.length === 0"
          >
            <ms-image
              :image="isDarkTheme ? NoFavoriteWorkspaceDark : NoFavoriteWorkspace"
              class="no-workspaces__image"
            />
            <ion-text>
              {{ $msTranslate('WorkspacesPage.categoriesMenu.noFavoriteWorkspaces') }}
            </ion-text>
          </div>
          <div
            class="no-hidden-workspaces"
            v-if="workspaceMenuState === WorkspaceMenu.Hidden && filteredWorkspaces.length === 0"
          >
            <ms-image
              :image="isDarkTheme ? NoHiddenWorkspaceDark : NoHiddenWorkspace"
              class="no-workspaces__image"
            />
            <ion-text>
              {{ $msTranslate('WorkspacesPage.categoriesMenu.noHiddenWorkspaces') }}
            </ion-text>
          </div>
        </div>
        <template v-if="filteredWorkspaces.length > 0 && !search">
          <div v-if="displayView === DisplayState.List">
            <ion-list
              class="list-container workspaces-container-list"
              id="workspaces-page-workspace-list"
            >
              <workspace-list-item
                v-for="workspace in filteredWorkspaces"
                :key="workspace.id"
                :workspace="workspace"
                :client-profile="clientProfile"
                :is-favorite="workspaceAttributes.isFavorite(workspace.id)"
                :is-hidden="workspaceAttributes.isHidden(workspace.id)"
                @click="onWorkspaceClick"
                @favorite-click="onWorkspaceFavoriteClick"
                @menu-click="onOpenWorkspaceContextMenu"
                @share-click="onWorkspaceShareClick"
                @self-promote-click="onSelfPromoteClick"
              />
            </ion-list>
          </div>
          <div
            v-if="displayView === DisplayState.Grid"
            class="workspaces-container-grid"
          >
            <workspace-card
              v-for="workspace in filteredWorkspaces"
              :key="workspace.id"
              :workspace="workspace"
              :client-profile="clientProfile"
              :is-favorite="workspaceAttributes.isFavorite(workspace.id)"
              :is-hidden="workspaceAttributes.isHidden(workspace.id)"
              @click="onWorkspaceClick"
              @favorite-click="onWorkspaceFavoriteClick"
              @menu-click="onOpenWorkspaceContextMenu"
              @share-click="onWorkspaceShareClick"
              @self-promote-click="onSelfPromoteClick"
            />
          </div>
        </template>
        <template v-if="search">
          <file-search-results
            :pattern="search.pattern"
            :search-results="search.results"
            :active="search.active"
            :multiple-workspaces="true"
            @item-click="onSearchResultClick"
            @menu-item-click="onSearchResultContextMenu"
            @update-pattern="search.pattern = $event"
          />
        </template>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { FileSearchResults } from '@/components/files';
import { FileSearch } from '@/components/files/handler/viewer/types';
import {
  WORKSPACES_PAGE_DATA_KEY,
  WorkspaceDefaultData,
  WorkspaceFilter,
  WorkspacesPageFilters,
  WorkspacesPageSavedData,
  compareWorkspaceRoles,
} from '@/components/workspaces';
import WorkspaceCard from '@/components/workspaces/WorkspaceCard.vue';
import WorkspaceCategoriesMenu from '@/components/workspaces/WorkspaceCategoriesMenu.vue';
import WorkspaceListItem from '@/components/workspaces/WorkspaceListItem.vue';
import {
  ClientInfo,
  EntryName,
  ParsecWorkspacePathAddr,
  Path,
  SearchResult,
  UserProfile,
  WorkspaceInfo,
  WorkspaceRole,
  decryptFileLink,
  entryStat,
  fileSearch,
  getClientProfile,
  getWorkspaceInfo,
  isDesktop,
  parseFileLink,
  getClientInfo as parsecGetClientInfo,
  getWorkspaceSharing as parsecGetWorkspaceSharing,
  listAvailableWorkspaces as parsecListWorkspaces,
  mountWorkspace as parsecMountWorkspace,
} from '@/parsec';
import { Routes, currentRouteIs, getCurrentRouteQuery, navigateTo, navigateToWorkspace, watchRoute } from '@/router';
import { useFileContextMenu, useWorkspaceActions, useWorkspaceContextMenu } from '@/services/contextMenu';
import { EventData, EventDistributor, EventDistributorKey, Events, MenuActionData } from '@/services/eventDistributor';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { recentDocumentManager } from '@/services/recentDocuments';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { useWorkspaceAttributes } from '@/services/workspaceAttributes';
import { FileAction } from '@/views/files';
import { WorkspaceAction, WorkspaceMenu, isWorkspaceAction } from '@/views/workspaces/types';
import { IonButton, IonContent, IonIcon, IonList, IonPage, IonText } from '@ionic/vue';
import { addCircle } from 'ionicons/icons';
import {
  DisplayState,
  MsActionBar,
  MsGridListToggle,
  MsImage,
  MsOptions,
  MsRichText,
  MsSearchInput,
  MsSorter,
  MsSorterChangeEvent,
  MsSpinner,
  NoFavoriteWorkspace,
  NoFavoriteWorkspaceDark,
  NoHiddenWorkspace,
  NoHiddenWorkspaceDark,
  NoRecentWorkspace,
  NoRecentWorkspaceDark,
  NoWorkspace,
  NoWorkspaceDark,
  useTheme,
  useWindowSize,
} from 'megashark-lib';
import { Ref, computed, inject, nextTick, onMounted, onUnmounted, ref } from 'vue';

enum SortWorkspaceBy {
  Name = 'name',
  Role = 'role',
  Size = 'size',
  LastUpdate = 'lastUpdate',
}

const fileContextMenu = useFileContextMenu();
const contextMenu = useWorkspaceContextMenu(false);
const workspaceAttributes = useWorkspaceAttributes();
const actions = useWorkspaceActions();

const { isDarkTheme } = useTheme();
const { isLargeDisplay, isSmallDisplay } = useWindowSize();
const userInfo: Ref<ClientInfo | null> = ref(null);
const sortBy = ref(SortWorkspaceBy.Name);
const sortByAsc = ref(true);
const workspaceList: Ref<Array<WorkspaceInfo>> = ref([]);
const displayView = ref(DisplayState.Grid);
const workspaceFilters = ref<WorkspacesPageFilters>({ owner: true, manager: true, contributor: true, reader: true });
const querying = ref(true);
const searchInputRef = ref();

const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
const search = ref<FileSearch | undefined>(undefined);
let searchAborter: AbortController | undefined = undefined;
const searchPattern = ref('');
const searchInputValue = ref('');
const someHiddenWorkspaces = computed(() => workspaceList.value.some((wk) => workspaceAttributes.isHidden(wk.id)));

let eventCbId: string | null = null;

const workspaceMenuState = ref<WorkspaceMenu>(WorkspaceMenu.All);

const routeWatchCancel = watchRoute(async () => {
  if (!currentRouteIs(Routes.Workspaces)) {
    return;
  }

  const query = getCurrentRouteQuery();
  if (query.fileLink) {
    const success = await handleFileLink(query.fileLink);
    if (!success) {
      await navigateTo(Routes.Workspaces, { query: {} });
    }
  }
  if (query.workspaceMenu && Object.values(WorkspaceMenu).includes(query.workspaceMenu)) {
    workspaceMenuState.value = query.workspaceMenu as WorkspaceMenu;
  }
  await refreshWorkspacesList();
});

const msSorterLabels = {
  asc: 'HomePage.organizationList.sortOrderAsc',
  desc: 'HomePage.organizationList.sortOrderDesc',
};

const clientProfile: Ref<UserProfile> = ref(UserProfile.Outsider);
let hotkeys: HotkeyGroup | null = null;

onMounted(async (): Promise<void> => {
  window.nativeAPI.log('debug', 'Mounted WorkspacePage');
  displayView.value = (
    await storageManager.retrieveComponentData<WorkspacesPageSavedData>(WORKSPACES_PAGE_DATA_KEY, WorkspaceDefaultData)
  ).displayState;

  window.nativeAPI.log('debug', 'Loading favorite workspaces');

  await workspaceAttributes.load();

  hotkeys = hotkeyManager.newHotkeys();
  hotkeys.add(
    { key: 'n', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Workspaces },
    actions.openCreateWorkspaceModal,
  );
  hotkeys.add(
    { key: 'g', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Workspaces },
    async () => {
      displayView.value = displayView.value === DisplayState.Grid ? DisplayState.List : DisplayState.Grid;
    },
  );
  hotkeys.add(
    { key: 'f', modifiers: Modifiers.Ctrl, platforms: Platforms.Web | Platforms.Desktop, disableIfModal: true, route: Routes.Workspaces },
    async () => await searchInputRef.value?.setFocus(),
  );

  eventCbId = await eventDistributor.value.registerCallback(
    [Events.WorkspaceUpdated, Events.WorkspaceCreated, Events.MenuAction, Events.WorkspaceMountpointsSync],
    async (event: Events, data?: EventData) => {
      switch (event) {
        case Events.WorkspaceUpdated:
        case Events.WorkspaceCreated:
        case Events.WorkspaceMountpointsSync:
          await refreshWorkspacesList();
          break;
        case Events.MenuAction:
          if (isWorkspaceAction((data as MenuActionData).action.action)) {
            await performWorkspaceAction((data as MenuActionData).action.action);
          }
          break;
        default:
          window.nativeAPI.log('warn', `Unhandled event ${event}`);
          break;
      }
    },
  );

  window.nativeAPI.log('debug', 'Getting user profile');
  clientProfile.value = await getClientProfile();

  window.nativeAPI.log('debug', 'Refreshing workspace list');
  await refreshWorkspacesList();

  window.nativeAPI.log('debug', 'Getting Parsec client info');
  const infoResult = await parsecGetClientInfo();

  if (infoResult.ok) {
    userInfo.value = infoResult.value;
  } else {
    window.nativeAPI.log('error', `Failed to retrieve user info ${JSON.stringify(infoResult.error)}`);
  }

  const query = getCurrentRouteQuery();
  if (query.fileLink) {
    window.nativeAPI.log('debug', 'Handling file link');
    const success = await handleFileLink(query.fileLink);
    if (!success) {
      window.nativeAPI.log('warn', 'Could not handle file link, going back to workspaces');
      await navigateTo(Routes.Workspaces, { query: {} });
    }
  }
  if (query.workspaceMenu && Object.values(WorkspaceMenu).includes(query.workspaceMenu)) {
    workspaceMenuState.value = query.workspaceMenu as WorkspaceMenu;
  }
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
  routeWatchCancel();
  if (eventCbId) {
    eventDistributor.value.removeCallback(eventCbId);
  }
});

async function handleFileLink(fileLink: ParsecWorkspacePathAddr): Promise<boolean> {
  const parseResult = await parseFileLink(fileLink);

  if (!parseResult.ok) {
    informationManager.value.present(
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
    informationManager.value.present(
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
    informationManager.value.present(
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
    informationManager.value.present(
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

async function openWorkspaceGlobalContextMenu(event: Event): Promise<void> {
  if (userInfo.value?.currentProfile === UserProfile.Outsider) {
    return;
  }
  event.preventDefault();
  await contextMenu.openGlobalContextMenu(event);
}

async function onSelfPromoteClick(workspace: WorkspaceInfo): Promise<void> {
  await actions.takeOwnershipOfWorkspace(workspace);
  await refreshWorkspacesList();
}

async function refreshWorkspacesList(): Promise<void> {
  if (!currentRouteIs(Routes.Workspaces)) {
    return;
  }
  querying.value = true;
  window.nativeAPI.log('debug', 'Starting Parsec list workspaces');
  const result = await parsecListWorkspaces();
  if (result.ok) {
    for (const wk of result.value) {
      window.nativeAPI.log('debug', `Processing workspace: ${wk.name}`);
      const sharingResult = await parsecGetWorkspaceSharing(wk.id, false);
      if (sharingResult.ok) {
        wk.sharing = sharingResult.value;
      } else {
        window.nativeAPI.log('warn', `Failed to get sharing for ${wk.name}`);
      }
      if (isDesktop() && wk.mountpoints.length === 0 && !workspaceAttributes.isHidden(wk.id)) {
        const mountResult = await parsecMountWorkspace(wk.handle);
        if (mountResult.ok) {
          wk.mountpoints.push(mountResult.value);
        } else {
          window.nativeAPI.log('warn', `Failed to mount ${wk.name}: ${mountResult.error.error}`);
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
    if (workspaceMenuState.value === WorkspaceMenu.Hidden && !someHiddenWorkspaces.value) {
      workspaceMenuState.value = WorkspaceMenu.All;
    }
  } else {
    informationManager.value.present(
      new Information({
        message: 'WorkspacesPage.listError',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
  querying.value = false;
}

const showHiddenWorkspacesButton = computed(() => {
  return (
    workspaceList.value.length > 0 &&
    workspaceList.value.every((wk) => workspaceAttributes.isHidden(wk.id)) &&
    filteredWorkspaces.value.length === 0 &&
    workspaceMenuState.value !== WorkspaceMenu.Hidden
  );
});

const filteredWorkspaces = computed(() => {
  return Array.from(workspaceList.value)
    .filter((workspace) => {
      if (isWorkspaceFiltered(workspace.selfRole)) {
        return false;
      }

      switch (workspaceMenuState.value) {
        case WorkspaceMenu.Recent:
          return (
            recentDocumentManager.getWorkspaces().find((workspaceRecent) => workspaceRecent.id === workspace.id) !== undefined &&
            !workspaceAttributes.isHidden(workspace.id)
          );
        case WorkspaceMenu.Favorites:
          return workspaceAttributes.isFavorite(workspace.id) && !workspaceAttributes.isHidden(workspace.id);
        case WorkspaceMenu.Hidden:
          return workspaceAttributes.isHidden(workspace.id);
        case WorkspaceMenu.All:
          return workspaceAttributes.isHidden(workspace.id) === false;
        default:
          break;
      }
      return true;
    })
    .sort((a: WorkspaceInfo, b: WorkspaceInfo) => {
      if (workspaceAttributes.isFavorite(b.id) !== workspaceAttributes.isFavorite(a.id)) {
        return workspaceAttributes.isFavorite(b.id) ? 1 : -1;
      }
      if (sortBy.value === SortWorkspaceBy.Name) {
        return sortByAsc.value ? a.name.localeCompare(b.name) : b.name.localeCompare(a.name);
      } else if (sortBy.value === SortWorkspaceBy.Role) {
        return compareWorkspaceRoles(b.selfRole, a.selfRole) * (sortByAsc.value ? 1 : -1);
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
  { label: 'WorkspacesPage.sort.sortByRole', key: SortWorkspaceBy.Role },
  // for now we don't have the data for both size and last update
  // { label: 'WorkspacesPage.sort.sortBySize', key: SortWorkspaceBy.Size },
  // { label: 'WorkspacesPage.sort.sortByLastUpdated', key: SortWorkspaceBy.LastUpdate },
]);

function onMsSorterChange(event: MsSorterChangeEvent): void {
  sortBy.value = event.option.key;
  sortByAsc.value = event.sortByAsc;
}

async function onWorkspaceClick(workspace: WorkspaceInfo): Promise<void> {
  recentDocumentManager.addWorkspace(workspace);
  await recentDocumentManager.saveToStorage(storageManager);
  await navigateToWorkspace(workspace.handle);
}

async function onWorkspaceFavoriteClick(workspace: WorkspaceInfo): Promise<void> {
  workspaceAttributes.toggleFavorite(workspace.id);
  await workspaceAttributes.save();
}

async function onWorkspaceShareClick(workspace: WorkspaceInfo): Promise<void> {
  await actions.workspaceShareClick(workspace);
  await refreshWorkspacesList();
}

async function performWorkspaceAction(action: WorkspaceAction): Promise<void> {
  if (action === WorkspaceAction.CreateWorkspace) {
    await actions.openCreateWorkspaceModal();
  }
}

async function onOpenWorkspaceContextMenu(workspace: WorkspaceInfo, event: Event, onFinished?: () => void): Promise<void> {
  await contextMenu.openContextMenu(event, workspace);
  if (onFinished) {
    onFinished();
  }
  await refreshWorkspacesList();
}

const actionBarOptionsWorkspacesPage = computed(() => {
  const actionsArray = [];

  if (clientProfile.value !== UserProfile.Outsider) {
    actionsArray.push({
      id: 'button-new-workspace',
      label: 'WorkspacesPage.createWorkspace',
      icon: addCircle,
      onClick: async (): Promise<void> => {
        await actions.openCreateWorkspaceModal();
      },
    });
  }
  return actionsArray;
});

function isWorkspaceFiltered(role: WorkspaceRole): boolean {
  switch (role) {
    case WorkspaceRole.Owner:
      return workspaceFilters.value.owner === false;
    case WorkspaceRole.Manager:
      return workspaceFilters.value.manager === false;
    case WorkspaceRole.Contributor:
      return workspaceFilters.value.contributor === false;
    case WorkspaceRole.Reader:
      return workspaceFilters.value.reader === false;
  }
}

async function onMenuUpdate(menu: WorkspaceMenu): Promise<void> {
  if (workspaceMenuState.value === menu) {
    menu = WorkspaceMenu.All;
  }
  await navigateTo(Routes.Workspaces, { query: { workspaceMenu: menu } });
}

function onSearchValueChange(pattern: string): void {
  searchInputValue.value = pattern;
  startSearch(pattern);
}

async function cancelSearch(): Promise<void> {
  if (searchAborter) {
    searchAborter.abort();
  }
  search.value = undefined;
}

async function startSearch(pattern: string): Promise<void> {
  await cancelSearch();
  if (pattern.length < 3) {
    return;
  }
  search.value = {
    results: [],
    pattern: pattern,
    active: true,
  };
  searchAborter = new AbortController();
  for (const wkInfo of filteredWorkspaces.value) {
    for await (const result of fileSearch(wkInfo.handle, '/', pattern, searchAborter.signal)) {
      if (search.value) {
        search.value.results.push(result);
      }
      await nextTick();
    }
  }
  if (search.value) {
    search.value.active = false;
  }
  searchAborter = undefined;
}

async function onSearchResultClick(entry: SearchResult): Promise<void> {
  searchPattern.value = '';
  await cancelSearch();
  navigateTo(Routes.Documents, {
    query: { documentPath: entry.parent, selectFile: entry.stats.name, workspaceHandle: entry.workspaceHandle },
  });
}

async function onSearchResultContextMenu(event: Event, entry: SearchResult, onFinished?: () => void): Promise<void> {
  if (!userInfo.value) {
    return;
  }
  const wkInfoResult = await getWorkspaceInfo(entry.workspaceHandle);
  if (!wkInfoResult.ok) {
    return;
  }

  const action = await fileContextMenu.openEntryContextMenu(
    event,
    [entry.stats],
    wkInfoResult.value.selfRole === WorkspaceRole.Reader,
    true,
  );

  if (action) {
    await fileContextMenu.dispatchContextMenuAction(action, [entry.stats], wkInfoResult.value, userInfo.value.currentProfile);
    if ([FileAction.Delete, FileAction.Rename, FileAction.MoveTo, FileAction.MakeACopy].includes(action)) {
      if (search.value) {
        await startSearch(searchPattern.value);
      }
    }
  }
  if (onFinished) {
    onFinished();
  }
}
</script>

<style lang="scss" scoped>
.counter {
  @include ms.font('body-md-regular');
}

.content-scroll {
  &::part(background) {
    @include ms.responsive-breakpoint('sm') {
      background: ms.color('surface-base-page');
    }
  }

  &::part(scroll) {
    @include ms.responsive-breakpoint('sm') {
      padding-top: ms.spacing('padding-3xl');
    }
  }
}

.no-workspaces {
  @include ms.font('body-lg-regular');

  max-width: 30rem;
  color: ms.color('text-neutral-default');
  display: flex;
  justify-content: center;
  margin: auto;
  height: 100%;
  align-items: center;

  @include ms.responsive-breakpoint('xs') {
    align-items: start;
    height: fit-content;
  }

  &-loading,
  .no-all-workspaces,
  .no-recent-workspaces,
  .no-favorite-workspaces,
  .no-hidden-workspaces {
    border-radius: ms.radius('lg');
    display: flex;
    height: fit-content;
    text-align: center;
    flex-direction: column;
    justify-content: center;
    gap: ms.spacing('gap-4xl');
    align-items: center;
    padding: ms.spacing('padding-7xl') ms.spacing('padding-3xl');

    #show-hidden-workspaces {
      @include ms.font('label-md-medium');
    }

    #new-workspace {
      @include ms.font('label-lg-medium');

      display: flex;
      align-items: center;
    }
  }

  &__image {
    width: 8rem;
    height: 8rem;

    @include ms.responsive-breakpoint('xs') {
      width: 6rem;
      height: 6rem;
    }
  }

  .no-all-workspaces .no-workspaces__image {
    width: 12.5rem;
    height: 12.5rem;
  }
}

.workspaces-container {
  @include ms.responsive-breakpoint('sm') {
    position: sticky;
    z-index: 10;
    background: ms.color('surface-base-default');
    box-shadow: ms.shadow('strong');
    border-radius: ms.radius('4xl') ms.radius('4xl') 0 0;
  }
}

.workspaces-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: ms.spacing('gap-4xl');
  overflow: visible;
  padding: ms.spacing('padding-2xl') ms.spacing('padding-none') ms.spacing('padding-4xl') ms.spacing('padding-none');

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-4xl') ms.spacing('padding-3xl');
    display: grid;
    grid-template-columns: repeat(2, 1fr);
  }

  @include ms.responsive-breakpoint('xs') {
    grid-template-columns: repeat(1, 1fr);
  }
}

.workspaces-container-list {
  padding: ms.spacing('padding-3xl') ms.spacing('padding-none');
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-lg');
  position: relative;
  z-index: 4;

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-4xl') ms.spacing('padding-3xl');
  }
}

.workspace-filters-menu {
  display: flex;
  gap: ms.spacing('gap-3xl');
  align-items: center;
  padding: ms.spacing('padding-4xl') ms.spacing('padding-5xl') ms.spacing('padding-none');

  @include ms.responsive-breakpoint('lg') {
    width: 100%;
    flex-direction: column;
    padding: ms.spacing('padding-4xl') 1.25rem ms.spacing('padding-none');
  }
}

.hidden-workspaces-checkbox {
  position: relative;
  z-index: 3;
  align-self: stretch;
  border-radius: ms.radius('2xl');
  border: ms.border('thin') solid ms.color('border-base-default');
  background: ms.color('surface-base-default');
  padding: ms.spacing('padding-lg') 0.875rem;
  display: flex;
  align-items: center;
  color: ms.color('text-base-description');
  --checkbox-background-checked: #{ms.color('surface-neutral-default')};
  --checkbox-border-checked: #{ms.color('border-neutral-default')};
  --border-color-checked: #{ms.color('border-neutral-default')};
  transition: all 0.2s ease-in-out;

  &::part(label) {
    margin-left: 0.675rem;
  }

  &.checkbox-checked::part(container) {
    border-color: ms.color('border-neutral-default');
  }

  &:hover {
    box-shadow: ms.shadow('light');
  }

  &:is(.checkbox-checked) {
    box-shadow: ms.shadow('light');
    border-color: ms.color('border-base-default-hover');
    color: ms.color('text-base-body');
  }

  @include ms.responsive-breakpoint('lg') {
    margin-right: auto;
  }
}

.workspace-filters-menu {
  display: flex;
  gap: ms.spacing('gap-3xl');
  align-items: center;
  padding: ms.spacing('padding-4xl') ms.spacing('padding-5xl') ms.spacing('padding-lg');

  @include ms.responsive-breakpoint('lg') {
    width: 100%;
    flex-direction: column;
    padding: ms.spacing('padding-4xl') 1.25rem ms.spacing('padding-none');
  }
}
</style>

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
        :buttons="actionBarOptionsWorkspacesPage"
      >
        <div class="right-side">
          <div class="counter">
            <ion-text class="body">
              {{ $msTranslate({ key: 'WorkspacesPage.itemCount', data: { count: workspaceList.length }, count: workspaceList.length }) }}
            </ion-text>
          </div>
          <ms-search-input
            v-model="searchFilterContent"
            placeholder="WorkspacesPage.filterPlaceholder"
            id="search-input-workspace"
          />
          <workspace-filter
            :filters="workspaceFilters"
            @change="onFilterUpdate"
            class="mobile-filters__filter"
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

      <div
        class="workspace-filters-menu"
        v-if="isLargeDisplay"
      >
        <workspace-categories-menu
          :active-menu="activeCategoryMenu"
          @update-menu="
            (active) => {
              activeCategoryMenu = active;
            }
          "
        />
        <ms-checkbox
          v-model="hiddenWorkspaces"
          @click.stop
          @dblclick.stop
          label-placement="end"
          class="hidden-workspaces-checkbox"
          id="show-hidden-workspaces-large"
        >
          <ion-text class="button-medium">
            <span v-if="windowWidth > WindowSizeBreakpoints.LG">
              {{ $msTranslate('WorkspacesPage.categoriesMenu.showHiddenWorkspaces') }}
            </span>
            <span v-else>{{ $msTranslate('WorkspacesPage.categoriesMenu.showHiddenWorkspacesShort') }}</span>
          </ion-text>
        </ms-checkbox>
      </div>

      <!-- workspaces -->
      <div class="workspaces-container scroll">
        <div
          class="mobile-filters"
          v-if="isSmallDisplay"
        >
          <div class="mobile-filters-buttons">
            <ms-checkbox
              v-model="hiddenWorkspaces"
              @click.stop
              @dblclick.stop
              label-placement="end"
              class="hidden-workspaces-checkbox"
              id="show-hidden-workspaces-small"
            >
              <ion-text class="button-medium">{{ $msTranslate('WorkspacesPage.categoriesMenu.showHiddenWorkspaces') }}</ion-text>
            </ms-checkbox>
            <workspace-filter
              :filters="workspaceFilters"
              @change="onFilterUpdate"
              class="mobile-filters-buttons__filter"
            />
            <ms-sorter
              id="workspace-filter-select"
              :options="msSorterOptions"
              default-option="name"
              :sorter-labels="msSorterLabels"
              @change="onMsSorterChange($event)"
              class="mobile-filters-buttons__sorter"
            />
          </div>
          <workspace-categories-menu
            :active-menu="activeCategoryMenu"
            @update-menu="
              (active) => {
                activeCategoryMenu = active;
              }
            "
          />
          <ms-search-input
            v-model="searchFilterContent"
            placeholder="WorkspacesPage.filterPlaceholder"
            id="search-input-workspace"
            class="mobile-filters__search"
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
              v-show="clientProfile !== UserProfile.Outsider"
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
          <ion-list class="workspaces-container-list list">
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
            :is-favorite="workspaceAttributes.isFavorite(workspace.id)"
            :is-hidden="workspaceAttributes.isHidden(workspace.id)"
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
  WORKSPACES_PAGE_DATA_KEY,
  WorkspaceDefaultData,
  WorkspaceFilter,
  WorkspacesPageSavedData,
  openWorkspaceContextMenu,
  workspaceShareClick,
} from '@/components/workspaces';
import { WorkspacesPageFilters, compareWorkspaceRoles } from '@/components/workspaces/utils';
import WorkspaceCard from '@/components/workspaces/WorkspaceCard.vue';
import WorkspaceCategoriesMenu from '@/components/workspaces/WorkspaceCategoriesMenu.vue';
import WorkspaceListItem from '@/components/workspaces/WorkspaceListItem.vue';
import {
  ClientInfo,
  EntryName,
  ParsecWorkspacePathAddr,
  Path,
  UserProfile,
  WorkspaceInfo,
  WorkspaceName,
  WorkspaceRole,
  decryptFileLink,
  entryStat,
  getClientProfile,
  isDesktop,
  parseFileLink,
  createWorkspace as parsecCreateWorkspace,
  getClientInfo as parsecGetClientInfo,
  getWorkspaceSharing as parsecGetWorkspaceSharing,
  listWorkspaces as parsecListWorkspaces,
  mountWorkspace as parsecMountWorkspace,
} from '@/parsec';
import { Routes, currentRouteIs, getCurrentRouteQuery, navigateTo, navigateToWorkspace, watchRoute } from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events, MenuActionData } from '@/services/eventDistributor';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { recentDocumentManager } from '@/services/recentDocuments';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { useWorkspaceAttributes } from '@/services/workspaceAttributes';
import { WorkspaceAction, WorkspaceMenu, isWorkspaceAction } from '@/views/workspaces/types';
import { IonButton, IonContent, IonIcon, IonList, IonPage, IonText } from '@ionic/vue';
import { addCircle } from 'ionicons/icons';
import {
  Answer,
  DisplayState,
  MsActionBar,
  MsCheckbox,
  MsGridListToggle,
  MsImage,
  MsOptions,
  MsSearchInput,
  MsSorter,
  MsSorterChangeEvent,
  MsSpinner,
  NoWorkspace,
  WindowSizeBreakpoints,
  askQuestion,
  getTextFromUser,
  useWindowSize,
} from 'megashark-lib';
import { Ref, computed, inject, onMounted, onUnmounted, ref } from 'vue';

enum SortWorkspaceBy {
  Name = 'name',
  Role = 'role',
  Size = 'size',
  LastUpdate = 'lastUpdate',
}

const workspaceAttributes = useWorkspaceAttributes();

const { isLargeDisplay, isSmallDisplay, windowWidth } = useWindowSize();
const userInfo: Ref<ClientInfo | null> = ref(null);
const sortBy = ref(SortWorkspaceBy.Name);
const sortByAsc = ref(true);
const workspaceList: Ref<Array<WorkspaceInfo>> = ref([]);
const displayView = ref(DisplayState.Grid);
const searchFilterContent = ref('');
const workspaceFilters = ref<WorkspacesPageFilters>({ owner: true, manager: true, contributor: true, reader: true });
const querying = ref(true);

const hiddenWorkspaces = ref(false);
const activeCategoryMenu = ref<WorkspaceMenu>(WorkspaceMenu.All);
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

onMounted(async (): Promise<void> => {
  window.electronAPI.log('debug', 'Mounted WorkspacePage');
  displayView.value = (
    await storageManager.retrieveComponentData<WorkspacesPageSavedData>(WORKSPACES_PAGE_DATA_KEY, WorkspaceDefaultData)
  ).displayState;

  window.electronAPI.log('debug', 'Loading favorite workspaces');
  await workspaceAttributes.load();

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
    Events.WorkspaceUpdated | Events.WorkspaceCreated | Events.MenuAction,
    async (event: Events, data?: EventData) => {
      switch (event) {
        case Events.WorkspaceUpdated:
        case Events.WorkspaceCreated:
          await refreshWorkspacesList();
          break;
        case Events.MenuAction:
          if (isWorkspaceAction((data as MenuActionData).action.action)) {
            await performWorkspaceAction((data as MenuActionData).action.action);
          }
          break;
        default:
          window.electronAPI.log('warn', `Unhandled event ${event}`);
          break;
      }
    },
  );

  window.electronAPI.log('debug', 'Getting user profile');
  clientProfile.value = await getClientProfile();

  window.electronAPI.log('debug', 'Refreshing workspace list');
  await refreshWorkspacesList();

  window.electronAPI.log('debug', 'Getting Parsec client info');
  const infoResult = await parsecGetClientInfo();

  if (infoResult.ok) {
    userInfo.value = infoResult.value;
  } else {
    window.electronAPI.log('error', `Failed to retrieve user info ${JSON.stringify(infoResult.error)}`);
  }

  const query = getCurrentRouteQuery();
  if (query.fileLink) {
    window.electronAPI.log('debug', 'Handling file link');
    const success = await handleFileLink(query.fileLink);
    if (!success) {
      window.electronAPI.log('warn', 'Could not handle file link, going back to workspaces');
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
  window.electronAPI.log('debug', 'Starting Parsec list workspaces');
  const result = await parsecListWorkspaces();
  const hidden = workspaceAttributes.getHidden();
  if (result.ok) {
    for (const wk of result.value) {
      window.electronAPI.log('debug', `Processing workspace: ${wk.currentName}`);
      const sharingResult = await parsecGetWorkspaceSharing(wk.id, false);
      if (sharingResult.ok) {
        wk.sharing = sharingResult.value;
      } else {
        window.electronAPI.log('warn', `Failed to get sharing for ${wk.currentName}`);
      }
      if (isDesktop() && wk.mountpoints.length === 0 && !hidden.value.includes(wk.id)) {
        const mountResult = await parsecMountWorkspace(wk.handle);
        if (mountResult.ok) {
          wk.mountpoints.push(mountResult.value);
        } else {
          window.electronAPI.log('warn', `Failed to mount ${wk.currentName}: ${mountResult.error.error}`);
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
    window.electronAPI.log('error', 'Error listing workspaces');
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
  const filter = searchFilterContent.value.toLocaleLowerCase();
  return Array.from(workspaceList.value)
    .filter((workspace) => {
      switch (activeCategoryMenu.value) {
        case WorkspaceMenu.Recents:
          return recentDocumentManager
            .getWorkspaces()
            .map((w) => w.id)
            .includes(workspace.id);
        case WorkspaceMenu.Favorites:
          return workspaceAttributes.isFavorite(workspace.id);
        // case WorkspaceMenu.ShareWithMe:
        //   return workspace.currentSelfRole !== WorkspaceRole.Owner;
        default:
          break;
      }

      if (!hiddenWorkspaces.value && workspaceAttributes.isHidden(workspace.id)) {
        return false;
      }

      if (!workspace.currentName.toLocaleLowerCase().includes(filter) || isWorkspaceFiltered(workspace.currentSelfRole)) {
        return false;
      }
      return true;
    })
    .sort((a: WorkspaceInfo, b: WorkspaceInfo) => {
      if (workspaceAttributes.isFavorite(b.id) !== workspaceAttributes.isFavorite(a.id)) {
        return workspaceAttributes.isFavorite(b.id) ? 1 : -1;
      }
      if (sortBy.value === SortWorkspaceBy.Name) {
        return sortByAsc.value ? a.currentName.localeCompare(b.currentName) : b.currentName.localeCompare(a.currentName);
      } else if (sortBy.value === SortWorkspaceBy.Role) {
        return compareWorkspaceRoles(b.currentSelfRole, a.currentSelfRole) * (sortByAsc.value ? 1 : -1);
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

async function createWorkspace(name: WorkspaceName): Promise<void> {
  // Externals shouldn't be able to access here, but putting this as safety
  if (clientProfile.value === UserProfile.Outsider) {
    return;
  }
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
  let workspaceName: string | null = null;
  while (true) {
    workspaceName = await getTextFromUser(
      {
        title: 'WorkspacesPage.CreateWorkspaceModal.pageTitle',
        trim: true,
        validator: workspaceNameValidator,
        inputLabel: 'WorkspacesPage.CreateWorkspaceModal.label',
        placeholder: 'WorkspacesPage.CreateWorkspaceModal.placeholder',
        okButtonText: 'WorkspacesPage.CreateWorkspaceModal.create',
        defaultValue: workspaceName ?? undefined,
      },
      isLargeDisplay.value,
    );

    if (!workspaceName) {
      return;
    }
    const found = workspaceList.value.find((wi) => {
      // eslint thinks workspaceName can be null here, no idea why
      const newName = workspaceName!.toLocaleLowerCase();
      const current = wi.currentName.toLocaleLowerCase();

      // If we find a case-insensitive match or one name contains the other and the name is a bit longer that a few letters,
      // both names are not too far off each other in length (3 characters difference at most)
      // For example, we already have a 'Workspace', and we're trying to create 'Workspace 1'
      if (newName === current) {
        return true;
      }
      return (
        newName.length > 6 && Math.abs(newName.length - current.length) <= 3 && (current.includes(newName) || newName.includes(current))
      );
    });
    if (found) {
      const answer = await askQuestion(
        'WorkspacesPage.CreateWorkspaceModal.sameNameExistsTitle',
        'WorkspacesPage.CreateWorkspaceModal.sameNameExistsQuestion',
        {
          yesText: 'WorkspacesPage.CreateWorkspaceModal.createAnyway',
          noText: 'WorkspacesPage.CreateWorkspaceModal.cancel',
        },
      );
      if (answer === Answer.Yes) {
        break;
      }
    } else {
      break;
    }
  }
  await createWorkspace(workspaceName);
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
  await workspaceShareClick(workspace, informationManager, eventDistributor, isLargeDisplay.value);
  await refreshWorkspacesList();
}

async function performWorkspaceAction(action: WorkspaceAction): Promise<void> {
  if (action === WorkspaceAction.CreateWorkspace) {
    return await openCreateWorkspaceModal();
  }
}

async function onOpenWorkspaceContextMenu(workspace: WorkspaceInfo, event: Event, onFinished?: () => void): Promise<void> {
  await openWorkspaceContextMenu(
    event,
    workspace,
    workspaceAttributes,
    eventDistributor,
    informationManager,
    false,
    isLargeDisplay.value,
    storageManager,
  );
  await refreshWorkspacesList();

  if (onFinished) {
    onFinished();
  }
}

const actionBarOptionsWorkspacesPage = computed(() => {
  const actionsArray = [];

  if (clientProfile.value !== UserProfile.Outsider) {
    actionsArray.push({
      id: 'button-new-workspace',
      label: 'WorkspacesPage.createWorkspace',
      icon: addCircle,
      onClick: async (): Promise<void> => {
        await openCreateWorkspaceModal();
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

async function onFilterUpdate(): Promise<void> {
  await refreshWorkspacesList();
}
</script>

<style lang="scss" scoped>
.content-scroll {
  &::part(background) {
    @include ms.responsive-breakpoint('sm') {
      background: var(--parsec-color-light-secondary-background);
    }
  }

  &::part(scroll) {
    @include ms.responsive-breakpoint('sm') {
      padding-top: 1rem;
    }
  }
}

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

.workspaces-container {
  @include ms.responsive-breakpoint('sm') {
    position: sticky;
    z-index: 10;
    background: var(--parsec-color-light-secondary-white);
    box-shadow: var(--parsec-shadow-strong);
    border-radius: var(--parsec-radius-18) var(--parsec-radius-18) 0 0;
  }
}

.workspaces-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5em;
  overflow: visible;
  padding: 1.5rem 0;

  @include ms.responsive-breakpoint('sm') {
    padding: 1.5rem 1rem;
    display: grid;
    grid-template-columns: repeat(2, 1fr);
  }

  @include ms.responsive-breakpoint('xs') {
    grid-template-columns: repeat(1, 1fr);
  }
}

.workspaces-container-list {
  padding: 1rem 0;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  position: relative;
  z-index: 4;

  @include ms.responsive-breakpoint('sm') {
    padding: 1.5rem 1rem;
  }
}

.workspace-filters-menu {
  display: flex;
  gap: 1rem;
  align-items: center;
  padding: 1.5rem 2rem 0;

  @include ms.responsive-breakpoint('lg') {
    width: 100%;
    flex-direction: column;
    padding: 1.5rem 1.25rem 0;
  }
}

.hidden-workspaces-checkbox {
  position: relative;
  z-index: 10;
  align-self: stretch;
  border-radius: var(--parsec-radius-12);
  border: 1px solid var(--parsec-color-light-secondary-medium);
  background: var(--parsec-color-light-secondary-white);
  padding: 0.5rem 0.875rem;
  display: flex;
  align-items: center;
  color: var(--parsec-color-light-secondary-grey);
  --checkbox-background-checked: var(--parsec-color-light-secondary-text);
  --checkbox-border-checked: var(--parsec-color-light-secondary-text);
  --border-color-checked: var(--parsec-color-light-secondary-text);
  transition: all 0.2s ease-in-out;

  &::part(label) {
    margin-left: 0.675rem;
  }

  &.checkbox-checked::part(container) {
    border-color: var(--parsec-color-light-secondary-text);
  }

  &:hover {
    box-shadow: var(--parsec-shadow-soft);
  }

  &:is(.checkbox-checked) {
    box-shadow: var(--parsec-shadow-soft);
    border-color: var(--parsec-color-light-secondary-light);
    color: var(--parsec-color-light-secondary-grey);
  }

  @include ms.responsive-breakpoint('lg') {
    margin-right: auto;
  }
}
</style>

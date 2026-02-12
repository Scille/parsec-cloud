<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      class="content-scroll folder-content"
    >
      <ms-action-bar
        id="folders-ms-action-bar"
        v-if="isLargeDisplay"
        :buttons="actionBarOptionsFoldersPage"
      >
        <div class="right-side">
          <workspace-role-tag
            :role="ownRole"
            class="workspace-role-tag"
          />

          <div class="counter">
            <ion-text
              class="body"
              v-if="selectedFilesCount === 0"
            >
              {{
                $msTranslate({
                  key: 'FoldersPage.itemCount',
                  data: { count: folders.entriesCount() + files.entriesCount() },
                  count: folders.entriesCount() + files.entriesCount(),
                })
              }}
            </ion-text>
            <ion-text
              class="body item-selected"
              v-if="selectedFilesCount > 0"
            >
              {{ $msTranslate({ key: 'FoldersPage.itemSelectedCount', data: { count: selectedFilesCount }, count: selectedFilesCount }) }}
            </ion-text>
          </div>

          <ms-sorter
            :sort-ascending="currentSortOrder"
            :key="`${currentSortProperty}-${currentSortOrder}`"
            label="FoldersPage.sort.byName"
            :options="msSorterOptions"
            :default-option="currentSortProperty"
            :sorter-labels="msSorterLabels"
            :sort-by-asc="currentSortOrder"
            @change="onSortChange"
          />

          <ms-grid-list-toggle
            v-model="displayView"
            @update:model-value="onDisplayStateChange"
          />
        </div>
      </ms-action-bar>
      <small-display-selection-header
        v-if="workspaceInfo && isSmallDisplay && selectionEnabled"
        :title="getDisplayText()"
        @open-contextual-modal="openGlobalContextMenu"
        @select="selectAll"
        @unselect="unselectAll"
        @cancel-selection="onSelectionCancel"
        :some-selected="selectedFilesCount > 0"
      />
      <div class="folder-container scroll">
        <file-inputs
          ref="fileInputs"
          @files-added="startImportFiles"
        />
        <div
          v-show="querying"
          class="body-lg"
        >
          <div class="no-files-content">
            <ms-spinner class="ms-spinner" />
            <ion-text>
              {{ $msTranslate('FoldersPage.loading') }}
            </ion-text>
          </div>
        </div>

        <div
          v-if="isSmallDisplay"
          class="mobile-filters"
        >
          <div class="mobile-filters-buttons">
            <ms-sorter
              :key="`${currentSortProperty}-${currentSortOrder}`"
              :label="'FoldersPage.sort.byName'"
              :options="msSorterOptions"
              :default-option="currentSortProperty"
              :sorter-labels="msSorterLabels"
              :sort-by-asc="currentSortOrder"
              @change="onSortChange"
            />

            <ms-grid-list-toggle
              v-model="displayView"
              @update:model-value="onDisplayStateChange"
            />
          </div>
        </div>

        <div
          v-if="!querying && itemsToShow === 0"
          class="no-files body-lg"
        >
          <file-drop-zone
            :show-drop-message="true"
            @files-added="startImportFiles"
            @global-menu-click="openGlobalContextMenu"
            :is-reader="ownRole === parsec.WorkspaceRole.Reader"
          >
            <div
              class="error-list-folder"
              v-if="showErrorListPage"
            >
              <ms-image
                :image="ListFolderError"
                class="error-list-folder__image"
              />
              <ms-report-text
                :theme="MsReportTheme.Error"
                class="error-list-folder__text"
              >
                {{ $msTranslate('FoldersPage.errors.failedToListContent') }}
              </ms-report-text>
            </div>
            <div
              class="no-files-content"
              v-else
            >
              <ms-image :image="EmptyFolder" />
              <ion-text>
                {{ $msTranslate('FoldersPage.emptyFolder') }}
              </ion-text>
            </div>
          </file-drop-zone>
        </div>
        <div
          v-else-if="!querying && !showErrorListPage"
          class="grid-list-container"
        >
          <div v-if="displayView === DisplayState.List && userInfo">
            <file-list-display
              ref="fileListDisplay"
              :files="files"
              :folders="folders"
              :own-profile="userInfo.currentProfile"
              :own-role="ownRole"
              :selection-enabled="selectionEnabled && isSmallDisplay"
              :current-sort-order="currentSortOrder"
              :current-sort-property="currentSortProperty"
              :file-operations="fileOperationsCurrentDir"
              @open-item="onEntryClick"
              @menu-click="openEntryContextMenu"
              @files-added="startImportFiles"
              @global-menu-click="openGlobalContextMenu"
              @drop-as-reader="onDropAsReader"
              @sort-change="onSortChange"
            />
          </div>
          <div v-if="displayView === DisplayState.Grid">
            <file-grid-display
              ref="fileGridDisplay"
              :files="files"
              :folders="folders"
              :own-role="ownRole"
              :selection-enabled="selectionEnabled && isSmallDisplay"
              :file-operations="fileOperationsCurrentDir"
              @open-item="onEntryClick"
              @menu-click="openEntryContextMenu"
              @files-added="startImportFiles"
              @global-menu-click="openGlobalContextMenu"
              @drop-as-reader="onDropAsReader"
            />
          </div>
        </div>
      </div>
      <tab-bar-options
        v-if="customTabBar.isVisible.value"
        :actions="tabBarActions"
      />
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { entryNameValidator } from '@/common/validators';
import * as parsec from '@/parsec';
import {
  Answer,
  DisplayState,
  DocumentImport,
  EmptyFolder,
  EyeOpenIcon,
  I18n,
  MsActionBar,
  MsGridListToggle,
  MsImage,
  MsModalResult,
  MsOptions,
  MsReportText,
  MsReportTheme,
  MsSorter,
  MsSorterChangeEvent,
  MsSpinner,
  RenameIcon,
  Translatable,
  askQuestion,
  asyncComputed,
  getTextFromUser,
  useWindowSize,
} from 'megashark-lib';

import ListFolderError from '@/assets/images/list-folder-error.svg?raw';
import {
  EntryCollection,
  EntryModel,
  FileDropZone,
  FileGridDisplay,
  FileImportPopover,
  FileInputs,
  FileListDisplay,
  FileModel,
  FolderDefaultData,
  FolderModel,
  FoldersPageSavedData,
  ImportType,
  SortProperty,
  copyPathLinkToClipboard,
  selectFolder,
} from '@/components/files';
import { EntrySyncStatus, FileOperationCurrentFolder } from '@/components/files/types';
import SmallDisplaySelectionHeader from '@/components/header/SmallDisplaySelectionHeader.vue';
import { WorkspaceRoleTag } from '@/components/workspaces';
import { showWorkspace } from '@/components/workspaces/utils';
import {
  ClientInfo,
  EntryName,
  EntryStatFile,
  FsPath,
  Path,
  WorkspaceCreateFolderErrorTag,
  WorkspaceID,
  WorkspaceRole,
  entryStat,
  getClientInfo,
  isDesktop,
  isWeb,
  listWorkspaces,
} from '@/parsec';
import { Routes, currentRouteIs, getCurrentRouteQuery, getDocumentPath, getWorkspaceHandle, navigateTo, watchRoute } from '@/router';
import { isFileEditable } from '@/services/cryptpad';
import {
  EntrySyncData,
  EventData,
  EventDistributor,
  EventDistributorKey,
  Events,
  MenuActionData,
  OpenContextualMenuData,
  OpenFolderBreadcrumbContextMenuData,
} from '@/services/eventDistributor';
import {
  FileEventRegistrationCanceller,
  FileOperationCopyData,
  FileOperationData,
  FileOperationDataType,
  FileOperationEventData,
  FileOperationEvents,
  FileOperationImportData,
  FileOperationManager,
  FileOperationManagerKey,
  FileOperationMoveData,
  FileOperationRestoreData,
} from '@/services/fileOperation';
import useHeaderControl from '@/services/headerControl';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import usePathOpener, { OpenPathOptions } from '@/services/pathOpener';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { useWorkspaceAttributes } from '@/services/workspaceAttributes';
import {
  FileAction,
  FileDetailsModal,
  FolderGlobalAction,
  openEntryContextMenu as _openEntryContextMenu,
  openFolderBreadcrumbContextMenu as _openFolderBreadcrumbContextMenu,
  openGlobalContextMenu as _openGlobalContextMenu,
  downloadArchive,
  downloadEntry,
  getDuplicatePolicy,
  isFolderGlobalAction,
  openDownloadConfirmationModal,
} from '@/views/files';
import { MenuAction, TabBarOptions, useCustomTabBar } from '@/views/menu';
import { IonContent, IonPage, IonText, modalController, popoverController } from '@ionic/vue';
import { arrowRedo, create, download, duplicate, eye, folderOpen, informationCircle, link, open, time, trashBin } from 'ionicons/icons';
import { Ref, computed, inject, nextTick, onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

const customTabBar = useCustomTabBar();
const workspaceAttributes = useWorkspaceAttributes();

const { isLargeDisplay, isSmallDisplay } = useWindowSize();
const { hideHeader, showHeader } = useHeaderControl();

const msSorterOptions: MsOptions = new MsOptions([
  { label: 'FoldersPage.sort.byName', key: SortProperty.Name },
  { label: 'FoldersPage.sort.byLastUpdate', key: SortProperty.LastUpdate },
  { label: 'FoldersPage.sort.byCreation', key: SortProperty.CreationDate },
  { label: 'FoldersPage.sort.bySize', key: SortProperty.Size },
]);

const msSorterLabels = {
  asc: 'FoldersPage.sort.asc',
  desc: 'FoldersPage.sort.desc',
};

const tabBarActions = computed(() => {
  const selectedEntries = getSelectedEntries();
  const isReadOnly: boolean = workspaceInfo.value ? workspaceInfo.value.currentSelfRole === WorkspaceRole.Reader : true;
  const actions: MenuAction[] = [];
  if (!isReadOnly) {
    if (selectedEntries.length === 1) {
      actions.push({
        label: 'FoldersPage.tabbar.rename',
        action: async () => await renameEntries(getSelectedEntries()),
        image: RenameIcon,
      });
    } else {
      actions.push({ label: 'FoldersPage.tabbar.duplicate', action: async () => await copyEntries(getSelectedEntries()), icon: duplicate });
    }
    actions.push({ label: 'FoldersPage.tabbar.move', action: async () => await moveEntriesTo(getSelectedEntries()), icon: arrowRedo });
    actions.push({
      label: 'FoldersPage.tabbar.delete',
      action: async () => await deleteEntries(getSelectedEntries()),
      icon: trashBin,
      danger: true,
    });
  }
  if (selectedEntries.length === 1 && selectedEntries[0].isFile()) {
    actions.push({
      label: 'FoldersPage.tabbar.preview',
      action: async () => await openEntry(getSelectedEntries()[0], {}),
      icon: eye,
    });
    if (isFileEditable(selectedEntries[0].name) && !isReadOnly) {
      actions.push({
        label: 'FoldersPage.tabbar.edit',
        action: async () => await openEntry(getSelectedEntries()[0], {}),
        icon: create,
      });
    }
  }
  if (selectedEntries.length > folders.value.getSelectedEntries().length && isWeb()) {
    actions.push({ label: 'FoldersPage.tabbar.download', action: async () => await downloadEntries(getSelectedEntries()), icon: download });
  }
  if (selectedEntries.length === 1) {
    if (isDesktop()) {
      if (selectedEntries[0].isFile()) {
        actions.push({
          label: 'FoldersPage.tabbar.seeInExplorer',
          action: async () => await seeInExplorer(getSelectedEntries()),
          image: EyeOpenIcon,
        });
      }
      actions.push({
        label: 'FoldersPage.tabbar.openWithDefault',
        action: async () => await openEntry(getSelectedEntries()[0], { skipViewers: true }),
        icon: open,
      });
    }
    if (isReadOnly) {
      actions.push(
        { label: 'FoldersPage.tabbar.copyLink', action: async () => await copyLink(getSelectedEntries()), icon: link },
        { label: 'FoldersPage.tabbar.details', action: async () => await showDetails(getSelectedEntries()), icon: informationCircle },
      );
    } else {
      actions.push(
        { label: 'FoldersPage.tabbar.duplicate', action: async () => await copyEntries(getSelectedEntries()), icon: duplicate },
        { label: 'FoldersPage.tabbar.copyLink', action: async () => await copyLink(getSelectedEntries()), icon: link },
        { label: 'FoldersPage.tabbar.history', action: async () => await showHistory(getSelectedEntries()), icon: time },
        { label: 'FoldersPage.tabbar.details', action: async () => await showDetails(getSelectedEntries()), icon: informationCircle },
      );
    }
  }
  return actions;
});

const routeWatchCancel = watchRoute(async () => {
  if (!currentRouteIs(Routes.Documents)) {
    return;
  }

  const newPath = getDocumentPath();
  if (newPath === '') {
    return;
  }
  let samePath = true;
  if (newPath !== currentPath.value) {
    currentPath.value = newPath;
    samePath = false;
  }
  const workspaceHandle = getWorkspaceHandle();
  if (workspaceHandle) {
    if (workspaceInfo.value && workspaceHandle !== workspaceInfo.value.handle) {
      samePath = false;
    }
    const infoResult = await parsec.getWorkspaceInfo(workspaceHandle);
    if (infoResult.ok) {
      workspaceInfo.value = infoResult.value;
    }
  }

  const query = getCurrentRouteQuery();

  await listFolder({ selectFile: query.selectFile, sameFolder: samePath });
});

const fileOperationManager: FileOperationManager = inject(FileOperationManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
const informationManager: InformationManager = inject(InformationManagerKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;

const FOLDERS_PAGE_DATA_KEY = 'FoldersPage';

const currentSortProperty = ref<SortProperty>(SortProperty.Name);
const currentSortOrder = ref(true);

const showErrorListPage = ref(false);
const userInfo: Ref<ClientInfo | null> = ref(null);
const fileOperations = ref<Array<FileOperationData>>([]);
const currentPath = ref('/');
const currentFolder: Ref<EntryName> = ref('/');
const folders = ref(new EntryCollection<FolderModel>());
const files = ref(new EntryCollection<FileModel>());
const displayView = ref(DisplayState.List);
const workspaceInfo: Ref<parsec.StartedWorkspaceInfo | null> = ref(null);
// Init at true to avoid blinking while we're mounting the component
// but we're not loading the files yet.
const querying = ref(true);
const fileListDisplayRef = useTemplateRef<InstanceType<typeof FileListDisplay>>('fileListDisplay');
const fileGridDisplayRef = useTemplateRef<InstanceType<typeof FileGridDisplay>>('fileGridDisplay');

const fileInputsRef = useTemplateRef<InstanceType<typeof FileInputs>>('fileInputs');
let eventCbId: string | null = null;

const selectedFilesCount = computed(() => {
  return files.value.selectedCount() + folders.value.selectedCount();
});
const selectionEnabled = ref<boolean>(false);
const manualSelection = ref<boolean>(false);

const pathOpener = usePathOpener();

let hotkeys: HotkeyGroup | null = null;
let fileOpCanceller!: FileEventRegistrationCanceller;

const ownRole = computed(() => {
  return workspaceInfo.value ? workspaceInfo.value.currentSelfRole : parsec.WorkspaceRole.Reader;
});

const itemsToShow = computed(() => {
  return folders.value.entriesCount() + files.value.entriesCount() + fileOperationsCurrentDir.value.length;
});

const fileOperationsCurrentDir = asyncComputed(async () => {
  const entryNames: Array<FileOperationCurrentFolder> = [];

  for (const op of fileOperations.value) {
    // Early break if it's not the same workspace
    if (op.workspaceId !== workspaceInfo.value?.id) {
      continue;
    }
    if (op.type === FileOperationDataType.Import) {
      const importOp = op as FileOperationImportData;
      // Early break if the current path is not included in the destination
      if (!importOp.destination.startsWith(currentPath.value)) {
        continue;
      }
      for (const file of importOp.files) {
        const filePath = await Path.parent(await Path.joinPaths(importOp.destination, (file as any).relativePath));
        if (Path.areSame(filePath, currentPath.value)) {
          entryNames.push({ type: op.type, entryName: file.name });
        }
      }
    } else if (op.type === FileOperationDataType.Restore) {
      const restoreOp = op as FileOperationRestoreData;
      for (const entry of restoreOp.entries) {
        if (Path.areSame(await Path.parent(entry.path), currentPath.value)) {
          entryNames.push({ type: op.type, entryName: entry.name });
        }
      }
    } else if (op.type === FileOperationDataType.Copy || op.type === FileOperationDataType.Move) {
      let opData: FileOperationCopyData | FileOperationMoveData;
      if (op.type === FileOperationDataType.Copy) {
        opData = op as FileOperationCopyData;
      } else {
        opData = op as FileOperationMoveData;
      }
      // Early break if the destination is not the current path
      if (!Path.areSame(opData.destination, currentPath.value)) {
        continue;
      }
      for (const entry of opData.sources) {
        entryNames.push({ type: op.type, entryName: entry.name });
      }
    }
  }
  return entryNames;
});

const headerWatchCancel = watch([isSmallDisplay, selectionEnabled], () => {
  isSmallDisplay.value && selectionEnabled.value ? hideHeader() : showHeader();
});

const tabBarWatchCancel = watch([isSmallDisplay, selectedFilesCount], () => {
  if (isSmallDisplay.value && selectedFilesCount.value >= 1) {
    customTabBar.show();
  } else {
    customTabBar.hide();
  }
});

const selectedCountWatchCancel = watch([selectedFilesCount, manualSelection], () => {
  selectionEnabled.value = selectedFilesCount.value > 0 || manualSelection.value === true;
});

async function defineShortcuts(): Promise<void> {
  hotkeys = hotkeyManager.newHotkeys();
  hotkeys.add(
    { key: 'o', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop | Platforms.Web, disableIfModal: true, route: Routes.Documents },
    async () => {
      await fileInputsRef.value?.importFiles();
    },
  );
  hotkeys.add(
    {
      key: 'o',
      modifiers: Modifiers.Ctrl | Modifiers.Shift,
      platforms: Platforms.Desktop | Platforms.Web,
      disableIfModal: true,
      route: Routes.Documents,
    },
    async () => {
      await fileInputsRef.value?.importFolder();
    },
  );
  hotkeys.add(
    { key: 'enter', modifiers: Modifiers.None, platforms: Platforms.MacOS, disableIfModal: true, route: Routes.Documents },
    async () => await renameEntries(getSelectedEntries()),
  );
  hotkeys.add(
    { key: 'f2', modifiers: Modifiers.None, platforms: Platforms.Windows | Platforms.Linux, disableIfModal: true, route: Routes.Documents },
    async () => await renameEntries(getSelectedEntries()),
  );
  hotkeys.add(
    { key: 'i', modifiers: Modifiers.Ctrl | Modifiers.Shift, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Documents },
    async () => await showDetails(getSelectedEntries()),
  );
  // FIXME: Reactivate `x` and `c` hotkeys when copy/move bindings are available
  // hotkeys.add(
  //   { key: 'c', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Documents },
  //   async () => await copyEntries(getSelectedEntries()),
  // );
  // hotkeys.add(
  //   { key: 'x', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Documents },
  //   async () => await moveEntriesTo(getSelectedEntries()),
  // );
  hotkeys.add(
    { key: 'l', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop | Platforms.Web, disableIfModal: true, route: Routes.Documents },
    async () => await copyLink(getSelectedEntries()),
  );
  hotkeys.add(
    {
      key: 'delete',
      modifiers: Modifiers.None,
      platforms: Platforms.Windows | Platforms.Linux | Platforms.Web,
      disableIfModal: true,
      route: Routes.Documents,
    },
    async () => await deleteEntries(getSelectedEntries()),
  );
  hotkeys.add(
    { key: 'backspace', modifiers: Modifiers.Ctrl, platforms: Platforms.MacOS, disableIfModal: true, route: Routes.Documents },
    async () => await deleteEntries(getSelectedEntries()),
  );
  hotkeys.add(
    { key: 'g', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Documents },
    async () => {
      displayView.value = displayView.value === DisplayState.Grid ? DisplayState.List : DisplayState.Grid;
    },
  );
  hotkeys.add(
    { key: 'a', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true, route: Routes.Documents },
    async () => {
      selectAll();
    },
  );
}

async function handleEvents(event: Events, data?: EventData): Promise<void> {
  if (event === Events.EntryUpdated) {
    await listFolder({ sameFolder: true });
  } else if (event === Events.WorkspaceUpdated && workspaceInfo.value) {
    await updateWorkspaceInfo(workspaceInfo.value.id);
  } else if (event === Events.EntrySynced || event === Events.EntrySyncStarted || event === Events.EntrySyncProgress) {
    const syncedData = data as EntrySyncData;
    if (!workspaceInfo.value || workspaceInfo.value.id !== syncedData.workspaceId) {
      return;
    }
    if (syncedData.way === 'inbound') {
      await listFolder({ sameFolder: true });
    } else {
      let entry: EntryModel | undefined = files.value.getEntries().find((e) => e.id === syncedData.entryId);
      if (!entry) {
        entry = folders.value.getEntries().find((e) => e.id === syncedData.entryId);
      }
      if (entry && workspaceInfo.value) {
        if (event === Events.EntrySynced) {
          entry.needSync = false;
          entry.syncStatus = EntrySyncStatus.Synced;
        } else if (event === Events.EntrySyncProgress || event === Events.EntrySyncStarted) {
          entry.needSync = true;
          entry.syncStatus = EntrySyncStatus.Uploading;
        }
      }
    }
  } else if (event === Events.MenuAction) {
    const menuAction = (data as MenuActionData).action;
    if (isFolderGlobalAction(menuAction.action)) {
      await performFolderAction(menuAction.action);
    }
  } else if (event === Events.OpenContextMenu) {
    await openGlobalContextMenu((data as OpenContextualMenuData).event);
  } else if (event === Events.OpenFolderBreadcrumbContextMenu) {
    await openFolderBreadcrumbContextMenu((data as OpenFolderBreadcrumbContextMenuData).event, ownRole.value);
  }
}

function getDisplayText(): Translatable {
  if (files.value.selectedCount() > 0 || folders.value.selectedCount() > 0) {
    return {
      key: 'FoldersPage.itemSelectedCount',
      data: { count: selectedFilesCount.value },
      count: selectedFilesCount.value,
    };
  } else {
    return currentFolder.value !== '/'
      ? I18n.valueAsTranslatable(currentFolder.value)
      : I18n.valueAsTranslatable(workspaceInfo.value?.currentName);
  }
}

onMounted(async () => {
  const componentData = await storageManager.retrieveComponentData<FoldersPageSavedData>(FOLDERS_PAGE_DATA_KEY, FolderDefaultData);

  displayView.value = componentData.displayState;
  currentSortProperty.value = componentData.sortProperty;
  currentSortOrder.value = componentData.sortAscending;

  await defineShortcuts();

  eventCbId = await eventDistributor.registerCallback(
    Events.EntryUpdated |
      Events.WorkspaceUpdated |
      Events.EntrySynced |
      Events.EntrySyncStarted |
      Events.MenuAction |
      Events.EntrySyncProgress |
      Events.OpenContextMenu |
      Events.OpenFolderBreadcrumbContextMenu,

    handleEvents,
  );

  const workspaceHandle = getWorkspaceHandle();
  if (workspaceHandle) {
    const infoResult = await parsec.getWorkspaceInfo(workspaceHandle);
    if (infoResult.ok) {
      workspaceInfo.value = infoResult.value;
    }
  }

  const clientInfoResult = await getClientInfo();
  if (clientInfoResult.ok) {
    userInfo.value = clientInfoResult.value;
  } else {
    window.electronAPI.log('error', `Failed to retrieve user info ${JSON.stringify(clientInfoResult.error)}`);
  }

  fileOpCanceller = await fileOperationManager.registerCallback(onFileOperationEvent);
  currentPath.value = getDocumentPath();
  const query = getCurrentRouteQuery();
  await listFolder({ selectFile: query.selectFile });
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
  fileOpCanceller.cancel();
  customTabBar.hide();
  selectionEnabled.value = false;
  routeWatchCancel();
  tabBarWatchCancel();
  headerWatchCancel();
  selectedCountWatchCancel();
  if (eventCbId) {
    eventDistributor.removeCallback(eventCbId);
  }
});

async function updateWorkspaceInfo(workspaceId: WorkspaceID): Promise<void> {
  const workspacesResult = await listWorkspaces();

  if (workspacesResult.ok) {
    const wInfo = workspacesResult.value.find((wi) => wi.id === workspaceId);
    if (!wInfo) {
      // Not found, probably means that we've been excluded from the workspace
      await informationManager.present(
        new Information({
          message: {
            key: 'FoldersPage.events.workspaceUnshared',
            data: {
              name: workspaceInfo.value?.currentName,
            },
          },
          level: InformationLevel.Error,
        }),
        PresentationMode.Modal,
      );
      await navigateTo(Routes.Workspaces);
    } else {
      // display a toast if the role has been changed in a significant manner: Reader to something else, or something else to Reader
      if (workspaceInfo.value?.currentSelfRole === WorkspaceRole.Reader && wInfo.currentSelfRole !== WorkspaceRole.Reader) {
        await informationManager.present(
          new Information({
            message: {
              key: 'FoldersPage.events.roleUpdateNoLongerReader',
            },
            level: InformationLevel.Info,
          }),
          PresentationMode.Toast,
        );
      } else if (workspaceInfo.value?.currentSelfRole !== WorkspaceRole.Reader && wInfo.currentSelfRole === WorkspaceRole.Reader) {
        await informationManager.present(
          new Information({
            message: {
              key: 'FoldersPage.events.roleUpdateNowReader',
            },
            level: InformationLevel.Warning,
          }),
          PresentationMode.Toast,
        );
      }
      // Just update the info, the page appearance should update automatically
      if (!workspaceInfo.value) {
        return;
      }
      workspaceInfo.value.currentName = wInfo.currentName;
      workspaceInfo.value.currentSelfRole = wInfo.currentSelfRole;
    }
  } else {
    // Don't really know what to do in this case, just move the user back to workspaces list
    await informationManager.present(
      new Information({
        message: {
          key: 'FoldersPage.errors.failedToListWorkspaces',
          data: {
            reason: workspacesResult.error.tag,
          },
        },
        level: InformationLevel.Info,
      }),
      PresentationMode.Toast,
    );
    await navigateTo(Routes.Workspaces);
  }
}

async function onDisplayStateChange(): Promise<void> {
  await storeComponentData();
}

async function onSortChange(event: MsSorterChangeEvent): Promise<void> {
  currentSortProperty.value = event.option.key;
  currentSortOrder.value = event.sortByAsc;
  folders.value.sort(currentSortProperty.value, currentSortOrder.value);
  files.value.sort(currentSortProperty.value, currentSortOrder.value);
  await storeComponentData();
}

async function storeComponentData(): Promise<void> {
  await storageManager.storeComponentData<FoldersPageSavedData>(FOLDERS_PAGE_DATA_KEY, {
    displayState: displayView.value,
    sortProperty: currentSortProperty.value,
    sortAscending: currentSortOrder.value,
  });
}

async function onFileOperationEvent(
  event: FileOperationEvents,
  operationData?: FileOperationData,
  _eventData?: FileOperationEventData,
): Promise<void> {
  if (!operationData) {
    if (event !== FileOperationEvents.AllFinished) {
      window.electronAPI.log('warn', `Event ${event} received without operation data`);
    }
    return;
  }
  switch (event) {
    case FileOperationEvents.Added:
      fileOperations.value.push(operationData);
      break;
    case FileOperationEvents.Progress:
      break;
    case FileOperationEvents.Cancelled:
    case FileOperationEvents.Failed: {
      const index = fileOperations.value.findIndex((item) => item.id === operationData.id);
      if (index !== -1) {
        fileOperations.value.splice(index, 1);
      }
      break;
    }
    case FileOperationEvents.Finished: {
      const index = fileOperations.value.findIndex((item) => item.id === operationData.id);
      if (index === -1) {
        break;
      }
      fileOperations.value.splice(index, 1);
      await listFolder({ sameFolder: true });
      break;
    }
  }
}

async function startImportFiles(files: Array<File>, destinationFolder?: EntryName): Promise<void> {
  const existing: Array<File> = [];

  if (!workspaceInfo.value) {
    return;
  }

  let destination = currentPath.value;
  if (destinationFolder && destinationFolder !== '/') {
    destination = await Path.join(destination, destinationFolder);
  }
  for (const file of files) {
    if (!(file as any).relativePath) {
      (file as any).relativePath = `/${file.name}`;
    }
    const importPath = await Path.joinPaths(destination, (file as any).relativePath);
    const result = await entryStat(workspaceInfo.value.handle, importPath);
    if (result.ok && result.value.isFile()) {
      existing.push(file);
    }
  }

  const dupPolicy = await getDuplicatePolicy(existing);

  if (!dupPolicy) {
    return;
  }

  await fileOperationManager.import(
    workspaceInfo.value.handle,
    files,
    destinationFolder ? await Path.join(currentPath.value, destinationFolder) : currentPath.value,
    dupPolicy,
  );
}

async function listFolder(options?: { selectFile?: EntryName; sameFolder?: boolean }): Promise<void> {
  if (currentPath.value) {
    currentFolder.value = (await Path.filename(currentPath.value)) ?? '/';
  }

  if (!workspaceInfo.value) {
    return;
  }
  if (!currentRouteIs(Routes.Documents)) {
    return;
  }
  /*
   * If the folder didn't change, we don't need to show a spinner
   * as it will result in blinking.
   */
  if (!options || !options.sameFolder) {
    querying.value = true;
  }
  const result = await parsec.statFolderChildren(workspaceInfo.value.handle, currentPath.value);
  if (result.ok) {
    const newFolders: FolderModel[] = [];
    const newFiles: FileModel[] = [];

    for (const childStat of result.value) {
      const childName = childStat.name;
      if (childStat.isFile()) {
        (childStat as FileModel).isSelected = Boolean(options && options.selectFile && options.selectFile === childName);
        (childStat as FileModel).syncStatus = !childStat.needSync ? EntrySyncStatus.Synced : undefined;
        newFiles.push(childStat as FileModel);
      } else {
        (childStat as FolderModel).isSelected = false;
        (childStat as FolderModel).syncStatus = !childStat.needSync ? EntrySyncStatus.Synced : undefined;
        newFolders.push(childStat as FolderModel);
      }
    }
    folders.value.smartUpdate(newFolders);
    files.value.smartUpdate(newFiles);
    folders.value.sort(currentSortProperty.value, currentSortOrder.value);
    files.value.sort(currentSortProperty.value, currentSortOrder.value);
    showErrorListPage.value = false;
  } else {
    files.value.clear();
    folders.value.clear();
    showErrorListPage.value = true;
    // This happens when the handle becomes invalid (if we're logging out for example) and the app tries to refresh at the same
    // time. Logging out while importing files is a good example of that: logging out will cancel the imports which will trigger
    // a refresh.
  }
  querying.value = false;
  if (options && options.selectFile) {
    // ref are not set if querying is true, so after querying has been set to false, we force an update
    await nextTick();
    if (fileListDisplayRef.value) {
      await fileListDisplayRef.value.scrollToSelected();
    } else if (fileGridDisplayRef.value) {
      await fileGridDisplayRef.value.scrollToSelected();
    }
  }
}

async function onEntryClick(entry: EntryModel, _event: Event): Promise<void> {
  if (!workspaceInfo.value) {
    return;
  }

  if (!entry.isFile()) {
    const newPath = await parsec.Path.join(currentPath.value, entry.name);
    navigateTo(Routes.Documents, {
      query: { documentPath: newPath, workspaceHandle: workspaceInfo.value?.handle },
    });
  } else {
    const config = await storageManager.retrieveConfig();
    await openEntry(entry, { skipViewers: config.skipViewers });
  }
}

async function createFolder(): Promise<void> {
  if (!workspaceInfo.value) {
    return;
  }
  const folderName = await getTextFromUser(
    {
      title: 'FoldersPage.CreateFolderModal.title',
      trim: true,
      validator: entryNameValidator,
      inputLabel: 'FoldersPage.CreateFolderModal.label',
      placeholder: 'FoldersPage.CreateFolderModal.placeholder',
      okButtonText: 'FoldersPage.CreateFolderModal.create',
    },
    isLargeDisplay.value,
  );

  if (!folderName) {
    return;
  }
  const folderPath = await parsec.Path.join(currentPath.value, folderName);
  const result = await parsec.createFolder(workspaceInfo.value.handle, folderPath);
  if (!result.ok) {
    let message: Translatable = { key: 'FoldersPage.errors.createFolderFailed', data: { name: folderName } };
    switch (result.error.tag) {
      case WorkspaceCreateFolderErrorTag.EntryExists: {
        message = { key: 'FoldersPage.errors.createFolderAlreadyExists', data: { name: folderName } };
        break;
      }
      default:
        break;
    }
    informationManager.present(
      new Information({
        message: message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
  await listFolder({ sameFolder: true });
}

async function onImportClicked(event: Event): Promise<void> {
  const popover = await popoverController.create({
    component: FileImportPopover,
    cssClass: 'import-popover',
    event: event,
    alignment: 'end',
    showBackdrop: false,
  });
  await popover.present();
  const result = await popover.onDidDismiss();
  await popover.dismiss();
  if (result.role !== MsModalResult.Confirm) {
    return;
  }
  if (result.data.type === ImportType.Files) {
    await fileInputsRef.value?.importFiles();
  } else if (result.data.type === ImportType.Folder) {
    await fileInputsRef.value?.importFolder();
  }
}

function getSelectedEntries(): EntryModel[] {
  return [...folders.value.getSelectedEntries(), ...files.value.getSelectedEntries()];
}

async function deleteEntries(entries: EntryModel[]): Promise<void> {
  if (entries.length === 0 || !workspaceInfo.value) {
    return;
  } else if (entries.length === 1) {
    const entry = entries[0];
    const title = entry.isFile() ? 'FoldersPage.deleteOneFileQuestionTitle' : 'FoldersPage.deleteOneFolderQuestionTitle';
    const subtitle = entry.isFile()
      ? { key: 'FoldersPage.deleteOneFileQuestionSubtitle', data: { name: entry.name } }
      : { key: 'FoldersPage.deleteOneFolderQuestionSubtitle', data: { name: entry.name } };
    const answer = await askQuestion(title, subtitle, {
      yesIsDangerous: true,
      yesText: entry.isFile() ? 'FoldersPage.deleteOneFileYes' : 'FoldersPage.deleteOneFolderYes',
      noText: entry.isFile() ? 'FoldersPage.deleteOneFileNo' : 'FoldersPage.deleteOneFolderNo',
    });

    if (answer === Answer.No) {
      return;
    }
    const path = await parsec.Path.join(currentPath.value, entry.name);
    const result = entry.isFile()
      ? await parsec.deleteFile(workspaceInfo.value.handle, path)
      : await parsec.deleteFolder(workspaceInfo.value.handle, path);
    if (!result.ok) {
      informationManager.present(
        new Information({
          message: { key: 'FoldersPage.errors.deleteFailed', data: { name: entry.name } },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      await eventDistributor.dispatchEvent(Events.EntryDeleted, {
        workspaceHandle: workspaceInfo.value.handle,
        entryId: entry.id,
        path: entry.path,
      });
    }
  } else {
    const answer = await askQuestion(
      'FoldersPage.deleteMultipleQuestionTitle',
      {
        key: 'FoldersPage.deleteMultipleQuestionSubtitle',
        data: {
          count: entries.length,
        },
      },
      {
        yesIsDangerous: true,
        yesText: { key: 'FoldersPage.deleteMultipleYes', data: { count: entries.length } },
        noText: { key: 'FoldersPage.deleteMultipleNo', data: { count: entries.length } },
      },
    );
    if (answer === Answer.No) {
      return;
    }
    let errorsEncountered = 0;
    for (const entry of entries) {
      const path = await parsec.Path.join(currentPath.value, entry.name);
      const result = entry.isFile()
        ? await parsec.deleteFile(workspaceInfo.value.handle, path)
        : await parsec.deleteFolder(workspaceInfo.value.handle, path);
      if (!result.ok) {
        errorsEncountered += 1;
      } else {
        await eventDistributor.dispatchEvent(Events.EntryDeleted, {
          workspaceHandle: workspaceInfo.value.handle,
          entryId: entry.id,
          path: entry.path,
        });
      }
    }
    if (errorsEncountered > 0) {
      informationManager.present(
        new Information({
          message:
            errorsEncountered === entries.length
              ? 'FoldersPage.errors.deleteMultipleAllFailed'
              : 'FoldersPage.errors.deleteMultipleSomeFailed',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  }
  await onSelectionCancel();
  await listFolder({ sameFolder: true });
}

async function renameEntries(entries: EntryModel[]): Promise<void> {
  if (entries.length !== 1 || !workspaceInfo.value) {
    return;
  }
  const entry = entries[0];
  const ext = parsec.Path.getFileExtension(entry.name);
  const newName = await getTextFromUser(
    {
      title: entry.isFile() ? 'FoldersPage.RenameModal.fileTitle' : 'FoldersPage.RenameModal.folderTitle',
      trim: true,
      validator: entryNameValidator,
      inputLabel: entry.isFile() ? 'FoldersPage.RenameModal.fileLabel' : 'FoldersPage.RenameModal.folderLabel',
      placeholder: entry.isFile() ? 'FoldersPage.RenameModal.filePlaceholder' : 'FoldersPage.RenameModal.folderPlaceholder',
      okButtonText: 'FoldersPage.RenameModal.rename',
      defaultValue: entry.name,
      selectionRange: [0, entry.name.length - (ext.length > 0 ? ext.length + 1 : 0)],
    },
    isLargeDisplay.value,
  );

  if (!newName) {
    return;
  }
  const filePath = entry.path === currentPath.value ? currentPath.value : await parsec.Path.join(currentPath.value, entry.name);
  const isRenamingCurrentFolder = entry.path === currentPath.value;
  const result = await parsec.rename(workspaceInfo.value.handle, filePath, newName);
  if (!result.ok) {
    let message: Translatable = '';
    switch (result.error.tag) {
      case parsec.WorkspaceMoveEntryErrorTag.DestinationExists:
        message = 'FoldersPage.errors.renameFailedAlreadyExists';
        break;
      default:
        message = { key: 'FoldersPage.errors.renameFailed', data: { name: entry.name } };
    }
    informationManager.present(
      new Information({
        message: message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    await eventDistributor.dispatchEvent(Events.EntryRenamed, {
      workspaceHandle: workspaceInfo.value.handle,
      entryId: entry.id,
      oldPath: entry.path,
      newPath: result.value,
      oldName: entry.name,
      newName: newName,
    });

    entry.name = newName;
    entry.path = result.value;

    if (isRenamingCurrentFolder) {
      await navigateTo(Routes.Documents, {
        query: {
          workspaceHandle: workspaceInfo.value.handle,
          documentPath: result.value,
        },
      });
      return;
    }
  }
  await onSelectionCancel();
}

async function copyLink(entries: EntryModel[]): Promise<void> {
  if (entries.length !== 1 || !workspaceInfo.value) {
    return;
  }
  const entry = entries[0];
  const filePath = await parsec.Path.join(currentPath.value, entry.name);
  const workspaceHandle = workspaceInfo.value.handle;

  copyPathLinkToClipboard(filePath, workspaceHandle, informationManager);
}

async function moveEntriesTo(entries: EntryModel[]): Promise<void> {
  if (entries.length === 0 || !workspaceInfo.value) {
    return;
  }
  const excludePaths: Array<FsPath> = [];
  for (const entry of entries) {
    if (!entry.isFile()) {
      excludePaths.push(entry.path);
    }
  }
  const folder = await selectFolder({
    title: { key: 'FoldersPage.moveSelectFolderTitle', data: { count: entries.length }, count: entries.length },
    startingPath: currentPath.value,
    workspaceHandle: workspaceInfo.value.handle,
    excludePaths: excludePaths,
  });
  if (!folder) {
    return;
  }

  const existing: Array<EntryModel> = [];

  for (const entry of entries) {
    const destPath = await Path.join(folder, entry.name);
    const result = await entryStat(workspaceInfo.value.handle, destPath);
    if (result.ok) {
      existing.push(entry);
    }
  }
  const dupPolicy = await getDuplicatePolicy(existing);
  if (!dupPolicy) {
    return;
  }

  await fileOperationManager.move(workspaceInfo.value.handle, entries, folder, dupPolicy);
  selectionEnabled.value = false;
}

async function showDetails(entries: EntryModel[]): Promise<void> {
  if (entries.length !== 1 || !workspaceInfo.value) {
    return;
  }
  const entry = entries[0];
  const modal = await modalController.create({
    component: FileDetailsModal,
    cssClass: 'file-details-modal',
    componentProps: {
      entry: entry,
      ownProfile: userInfo.value ? userInfo.value.currentProfile : undefined,
      workspaceHandle: workspaceInfo.value.handle,
    },
  });
  await modal.present();
  await modal.onWillDismiss();
}

async function copyEntries(entries: EntryModel[]): Promise<void> {
  if (entries.length === 0 || !workspaceInfo.value) {
    return;
  }

  const excludePaths: Array<FsPath> = [];
  for (const entry of entries) {
    if (!entry.isFile()) {
      excludePaths.push(entry.path);
    }
  }
  const folder = await selectFolder({
    title: { key: 'FoldersPage.copySelectFolderTitle', data: { count: entries.length }, count: entries.length },
    startingPath: currentPath.value,
    workspaceHandle: workspaceInfo.value.handle,
    excludePaths: excludePaths,
    allowStartingPath: true,
    okButtonLabel: 'FoldersPage.copyHere',
  });
  if (!folder) {
    return;
  }

  const existing: Array<EntryModel> = [];

  for (const entry of entries) {
    const destPath = await Path.join(folder, entry.name);
    const result = await entryStat(workspaceInfo.value.handle, destPath);
    if (result.ok) {
      existing.push(entry);
    }
  }
  const dupPolicy = await getDuplicatePolicy(existing);
  if (!dupPolicy) {
    return;
  }

  await fileOperationManager.copy(workspaceInfo.value.handle, entries, folder, dupPolicy);
  selectionEnabled.value = false;
}

async function downloadEntries(entries: EntryModel[]): Promise<void> {
  if (!workspaceInfo.value) {
    window.electronAPI.log('error', 'No workspace info when trying to download a file');
    return;
  }
  if (entries.length < 1) {
    return;
  }

  const result = await openDownloadConfirmationModal(storageManager);
  if (result === MsModalResult.Cancel) {
    return;
  }

  if (entries.length === 1 && entries[0].isFile()) {
    await downloadEntry({
      entry: entries[0] as EntryStatFile,
      workspaceHandle: workspaceInfo.value.handle,
      workspaceId: workspaceInfo.value.id,
      informationManager: informationManager,
      fileOperationManager: fileOperationManager,
    });
  } else {
    await downloadArchive({
      entries: entries,
      archiveName: `${workspaceInfo.value.currentName}_${currentFolder.value === '/' ? 'ROOT' : currentFolder.value}.zip`,
      workspaceHandle: workspaceInfo.value.handle,
      workspaceId: workspaceInfo.value.id,
      informationManager: informationManager,
      fileOperationManager: fileOperationManager,
      relativePath: currentPath.value ?? '/',
    });
  }
  await onSelectionCancel();
}

async function showHistory(entries: EntryModel[]): Promise<void> {
  if (entries.length !== 1) {
    return;
  }
  if (!workspaceInfo.value) {
    window.electronAPI.log('error', 'No workspace info when trying to navigate to history');
    return;
  }

  await navigateTo(Routes.History, {
    query: {
      documentPath: entries[0].path,
      workspaceHandle: workspaceInfo.value.handle,
      selectFile: entries[0].isFile() ? entries[0].name : undefined,
    },
  });
  selectionEnabled.value = false;
}

async function openEntry(entryToOpen: EntryModel, options: OpenPathOptions): Promise<void> {
  if (!workspaceInfo.value) {
    window.electronAPI.log('warn', 'Trying to open an entry but missing workspace info.');
    return;
  } else if (!entryToOpen.isFile()) {
    window.electronAPI.log('warn', 'Trying to open an entry that is not a file.');
    return;
  }

  const entry = entryToOpen as EntryStatFile;
  const workspaceHandle = workspaceInfo.value.handle;

  if (workspaceAttributes.isHidden(workspaceInfo.value.id)) {
    const answer = await askQuestion(
      'WorkspacesPage.openInExplorerModal.file.title',
      'WorkspacesPage.openInExplorerModal.file.description',
      {
        yesText: 'WorkspacesPage.openInExplorerModal.actionConfirm',
        noText: 'WorkspacesPage.openInExplorerModal.actionCancel',
      },
    );
    if (answer === Answer.No) {
      return;
    }

    const result = await showWorkspace(workspaceInfo.value, workspaceAttributes, informationManager, eventDistributor);

    if (!result) {
      return;
    }
  }

  await pathOpener.openPath(workspaceHandle, entry.path, informationManager, options);
  selectionEnabled.value = false;
}

async function performFolderAction(action: FolderGlobalAction): Promise<void> {
  if (!workspaceInfo.value) {
    return;
  }
  switch (action) {
    case FolderGlobalAction.CreateFolder:
      return await createFolder();
    case FolderGlobalAction.ImportFiles:
      return await fileInputsRef.value?.importFiles();
    case FolderGlobalAction.ImportFolder:
      return await fileInputsRef.value?.importFolder();
    case FolderGlobalAction.OpenInExplorer:
      return await pathOpener.openPath(workspaceInfo.value.handle, currentPath.value, informationManager, { skipViewers: true });
    case FolderGlobalAction.ToggleSelect:
      return await toggleSelection();
    case FolderGlobalAction.SelectAll:
      return await selectAll();
    case FolderGlobalAction.Share:
      return await shareEntries();
  }
}

async function openGlobalContextMenu(event: Event): Promise<void> {
  const data = await _openGlobalContextMenu(
    event,
    ownRole.value,
    isLargeDisplay.value,
    folders.value.entriesCount() + files.value.entriesCount() === 0,
  );

  if (!data || !workspaceInfo.value) {
    return;
  }
  await performFolderAction(data.action);
}

async function getCurrentFolderEntry(): Promise<FolderModel | null> {
  if (!workspaceInfo.value || currentPath.value === '/') {
    return null;
  }

  const result = await entryStat(workspaceInfo.value.handle, currentPath.value);

  if (!result.ok || result.value.isFile()) {
    return null;
  }

  return result.value as FolderModel;
}

async function openFolderBreadcrumbContextMenu(event: Event, role: WorkspaceRole): Promise<void> {
  const data = await _openFolderBreadcrumbContextMenu(event, role);

  if (!data) {
    return;
  }

  const currentFolderEntry = currentPath.value ? await getCurrentFolderEntry() : null;

  if (!currentFolderEntry) {
    window.electronAPI.log('error', 'No current folder entry found when opening breadcrumb context menu');
    return;
  }

  const actions = new Map<FileAction, () => Promise<void>>([
    [FileAction.Rename, async (): Promise<void> => await renameEntries([currentFolderEntry])],
    [FileAction.ShowHistory, async (): Promise<void> => await showHistory([currentFolderEntry])],
    [FileAction.ShowDetails, async (): Promise<void> => await showDetails([currentFolderEntry])],
    [FileAction.CopyLink, async (): Promise<void> => await copyLink([currentFolderEntry])],
    [FileAction.SeeInExplorer, async (): Promise<void> => await seeInExplorer([currentFolderEntry])],
  ]);

  const fn = actions.get(data.action);
  if (fn) {
    await fn();
  }
}

async function openEntryContextMenu(event: Event, entry: EntryModel, onFinished?: () => void): Promise<void> {
  const selectedEntries = getSelectedEntries();
  const data = await _openEntryContextMenu(event, entry, selectedEntries, ownRole.value, isLargeDisplay.value);

  if (!data) {
    if (onFinished) {
      onFinished();
    }
    return;
  }

  const actions = new Map<FileAction, (file: EntryModel[]) => Promise<void>>([
    [
      FileAction.Preview,
      async (entries: EntryModel[]): Promise<void> =>
        await openEntry(entries[0], { disallowSystem: true, skipViewers: false, readOnly: true }),
    ],
    [FileAction.Rename, renameEntries],
    [
      FileAction.Edit,
      async (entries: EntryModel[]): Promise<void> =>
        await openEntry(entries[0], { readOnly: ownRole.value === parsec.WorkspaceRole.Reader }),
    ],
    [FileAction.MoveTo, moveEntriesTo],
    [FileAction.MakeACopy, copyEntries],
    [FileAction.Open, async (entries: EntryModel[]): Promise<void> => await openEntry(entries[0], { skipViewers: true })],
    [FileAction.ShowHistory, showHistory],
    [FileAction.Download, downloadEntries],
    [FileAction.ShowDetails, showDetails],
    [FileAction.CopyLink, copyLink],
    [FileAction.Delete, deleteEntries],
    [FileAction.SeeInExplorer, seeInExplorer],
  ]);

  const fn = actions.get(data.action);
  if (fn) {
    if (!selectedEntries.includes(entry)) {
      await fn([entry]);
    } else {
      await fn(selectedEntries);
    }
  }
  if (onFinished) {
    onFinished();
  }
}

async function toggleSelection(): Promise<void> {
  selectionEnabled.value = !selectionEnabled.value;
  if (selectionEnabled.value === false) {
    manualSelection.value = false;
    await unselectAll();
  } else {
    manualSelection.value = true;
  }
}

async function onSelectionCancel(): Promise<void> {
  await unselectAll();
  manualSelection.value = false;
}

async function selectAll(): Promise<void> {
  manualSelection.value = true;
  folders.value.selectAll(true);
  files.value.selectAll(true);
}

async function unselectAll(): Promise<void> {
  manualSelection.value = true;
  folders.value.selectAll(false);
  files.value.selectAll(false);
}

async function shareEntries(): Promise<void> {
  if (!workspaceInfo.value) {
    return;
  }
  copyPathLinkToClipboard(currentPath.value, workspaceInfo.value.handle, informationManager);
}

async function seeInExplorer(entries: EntryModel[]): Promise<void> {
  if (entries.length > 1 || !workspaceInfo.value || !isDesktop()) {
    return;
  }
  if (workspaceAttributes.isHidden(workspaceInfo.value.id)) {
    const answer = await askQuestion(
      'WorkspacesPage.openInExplorerModal.file.title',
      'WorkspacesPage.openInExplorerModal.file.description',
      {
        yesText: 'WorkspacesPage.openInExplorerModal.actionConfirm',
        noText: 'WorkspacesPage.openInExplorerModal.actionCancel',
      },
    );

    if (answer === Answer.No) {
      return;
    }

    const result = await showWorkspace(workspaceInfo.value, workspaceAttributes, informationManager, eventDistributor);

    if (!result) {
      return;
    }
  }

  await pathOpener.showInExplorer(workspaceInfo.value.handle, entries[0].path, informationManager);
}

async function onDropAsReader(): Promise<void> {
  await informationManager.present(
    new Information({
      message: 'FoldersPage.ImportFile.noDropForReader',
      level: InformationLevel.Error,
    }),
    PresentationMode.Toast,
  );
}

const actionBarOptionsFoldersPage = computed(() => {
  const actionArray = [];
  const selectedEntries = getSelectedEntries();

  if (selectedFilesCount.value === 0 && ownRole.value !== parsec.WorkspaceRole.Reader) {
    actionArray.push(
      {
        label: 'FoldersPage.createFolder',
        icon: folderOpen,
        onClick: async () => {
          await createFolder();
        },
      },
      {
        label: 'FoldersPage.import',
        image: DocumentImport,
        onClick: async (event: MouseEvent) => {
          await onImportClicked(event);
        },
      },
    );
  }
  if (selectedFilesCount.value === 1) {
    if (selectedEntries[0].isFile()) {
      actionArray.push({
        label: 'FoldersPage.fileContextMenu.actionPreview',
        icon: eye,
        onClick: async () => {
          await openEntry(getSelectedEntries()[0], { skipViewers: false, readOnly: true });
        },
      });
    }
    if (selectedEntries[0].isFile() && ownRole.value !== parsec.WorkspaceRole.Reader && isFileEditable(selectedEntries[0].name)) {
      actionArray.push({
        label: 'FoldersPage.fileContextMenu.actionEdit',
        icon: create,
        onClick: async () => {
          await openEntry(getSelectedEntries()[0], {});
        },
      });
    }
    if (ownRole.value !== parsec.WorkspaceRole.Reader) {
      actionArray.push(
        {
          label: 'FoldersPage.fileContextMenu.actionRename',
          image: RenameIcon,
          onClick: async () => {
            await renameEntries(getSelectedEntries());
          },
        },
        {
          label: 'FoldersPage.fileContextMenu.actionMoveTo',
          icon: arrowRedo,
          onClick: async () => {
            await moveEntriesTo(getSelectedEntries());
          },
        },
        {
          label: 'FoldersPage.fileContextMenu.actionMakeACopy',
          icon: duplicate,
          onClick: async () => {
            await copyEntries(getSelectedEntries());
          },
        },
        {
          label: 'FoldersPage.fileContextMenu.actionDelete',
          icon: trashBin,
          onClick: async () => {
            await deleteEntries(getSelectedEntries());
          },
        },
      );
    }
    if (isWeb()) {
      actionArray.push({
        label: 'FoldersPage.fileContextMenu.actionDownload',
        icon: download,
        onClick: async () => {
          await downloadEntries(getSelectedEntries());
        },
      });
    }
    actionArray.push(
      {
        label: 'FoldersPage.fileContextMenu.actionDetails',
        icon: informationCircle,
        onClick: async () => {
          await showDetails(getSelectedEntries());
        },
      },
      {
        label: 'FoldersPage.fileContextMenu.actionCopyLink',
        icon: link,
        onClick: async () => {
          await copyLink(getSelectedEntries());
        },
      },
    );
  }
  if (selectedFilesCount.value > 1) {
    if (ownRole.value !== parsec.WorkspaceRole.Reader) {
      actionArray.push(
        {
          label: 'FoldersPage.fileContextMenu.actionMoveTo',
          icon: arrowRedo,
          onClick: async () => {
            await moveEntriesTo(getSelectedEntries());
          },
        },
        {
          label: 'FoldersPage.fileContextMenu.actionMakeACopy',
          icon: duplicate,
          onClick: async () => {
            await copyEntries(getSelectedEntries());
          },
        },
        {
          label: 'FoldersPage.fileContextMenu.actionDelete',
          icon: trashBin,
          onClick: async () => {
            await deleteEntries(getSelectedEntries());
          },
        },
      );
    }
    if (isWeb()) {
      actionArray.push({
        label: 'FoldersPage.fileContextMenu.actionDownload',
        icon: download,
        onClick: async () => {
          await downloadEntries(getSelectedEntries());
        },
      });
    }
  }

  return actionArray;
});
</script>

<style scoped lang="scss">
.grid-list-container {
  display: flex;
  flex-direction: column;
  flex-grow: 0;
  overflow-y: auto;
  overflow-x: hidden;
}

.content-scroll {
  &::part(scroll) {
    -ms-overflow-style: -ms-autohiding-scrollbar;
  }

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

.folder-container {
  display: flex;
  flex-direction: column;

  @include ms.responsive-breakpoint('sm') {
    position: sticky;
    z-index: 10;
    background: var(--parsec-color-light-secondary-white);
    box-shadow: var(--parsec-shadow-strong);
    border-radius: var(--parsec-radius-18) var(--parsec-radius-18) 0 0;
  }
}

.folder-container div:not(.no-files-content div) {
  height: 100%;
}

.mobile-filters {
  @include ms.responsive-breakpoint('sm') {
    margin-top: 0 !important;
    height: auto !important;
  }

  &-buttons {
    @include ms.responsive-breakpoint('sm') {
      padding: 0.5rem 0.75rem;
      width: 100%;
      justify-content: flex-end;
      border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
    }
  }
}

.workspace-role-tag {
  background: var(--parsec-color-light-secondary-white);
  padding: 0.25rem;
  border-radius: var(--parsec-radius-32);
  border: 1px solid var(--parsec-color-light-secondary-disabled);
}

.no-files {
  width: 100%;

  &-content {
    width: 100%;
    height: 100%;
    margin: auto;
    color: var(--parsec-color-light-secondary-grey);
    border-radius: var(--parsec-radius-8);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 1rem;
    padding: 2rem 1rem;

    .ms-spinner {
      height: 1.5rem;
    }
  }
}

.error-list-folder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  padding: 2rem 1rem;
  max-width: 30rem;
  margin: auto;

  &__image {
    width: 10rem;
    height: 10rem !important;
  }

  &__text {
    height: fit-content !important;
  }
}
</style>

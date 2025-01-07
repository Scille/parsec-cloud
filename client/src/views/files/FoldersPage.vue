<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      class="content-scroll"
    >
      <ms-action-bar id="folders-ms-action-bar">
        <div v-if="selectedFilesCount === 0">
          <ms-action-bar-button
            id="button-new-folder"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="'FoldersPage.createFolder'"
            :icon="folderOpen"
            @click="createFolder()"
          />
          <ms-action-bar-button
            id="button-import"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="'FoldersPage.import'"
            :image="DocumentImport"
            @click="onImportClicked($event)"
          />
        </div>
        <div v-else-if="selectedFilesCount === 1">
          <ms-action-bar-button
            id="button-rename"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="'FoldersPage.fileContextMenu.actionRename'"
            :icon="pencil"
            @click="renameEntries(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-moveto"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="'FoldersPage.fileContextMenu.actionMoveTo'"
            :icon="arrowRedo"
            @click="moveEntriesTo(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-makeacopy"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="'FoldersPage.fileContextMenu.actionMakeACopy'"
            :icon="copy"
            @click="copyEntries(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-delete"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="'FoldersPage.fileContextMenu.actionDelete'"
            :icon="trashBin"
            @click="deleteEntries(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-details"
            :button-label="'FoldersPage.fileContextMenu.actionDetails'"
            :icon="informationCircle"
            @click="showDetails(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-copy-link"
            :button-label="'FoldersPage.fileContextMenu.actionCopyLink'"
            :icon="link"
            @click="copyLink(getSelectedEntries())"
          />
        </div>
        <div v-else>
          <ms-action-bar-button
            id="button-moveto"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="'FoldersPage.fileContextMenu.actionMoveTo'"
            :icon="arrowRedo"
            @click="moveEntriesTo(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-makeacopy"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="'FoldersPage.fileContextMenu.actionMakeACopy'"
            :icon="copy"
            @click="copyEntries(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-delete"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="'FoldersPage.fileContextMenu.actionDelete'"
            :icon="trashBin"
            @click="deleteEntries(getSelectedEntries())"
          />
        </div>
        <div class="right-side">
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
            :label="'FoldersPage.sort.byName'"
            :options="msSorterOptions"
            :default-option="SortProperty.Name"
            :sorter-labels="msSorterLabels"
            @change="onSortChange"
          />

          <ms-grid-list-toggle
            v-model="displayView"
            @update:model-value="onDisplayStateChange"
          />
        </div>
      </ms-action-bar>
      <div class="folder-container scroll">
        <file-inputs
          ref="fileInputsRef"
          :current-path="currentPath"
          @files-added="startImportFiles"
        />
        <div
          v-show="querying"
          class="body-lg"
        >
          <div class="no-files-content">
            <ms-spinner />
            <ion-text>
              {{ $msTranslate('FoldersPage.loading') }}
            </ion-text>
          </div>
        </div>

        <div
          v-if="!querying && itemsToShow === 0"
          class="no-files body-lg"
        >
          <file-drop-zone
            :current-path="currentPath"
            :show-drop-message="true"
            @files-added="startImportFiles"
            @global-menu-click="openGlobalContextMenu"
            :is-reader="ownRole === parsec.WorkspaceRole.Reader"
          >
            <div class="no-files-content">
              <ms-image :image="EmptyFolder" />
              <ion-text>
                {{ $msTranslate('FoldersPage.emptyFolder') }}
              </ion-text>
            </div>
          </file-drop-zone>
        </div>
        <div v-else-if="!querying">
          <div v-if="displayView === DisplayState.List">
            <file-list-display
              :files="files"
              :folders="folders"
              :operations-in-progress="fileOperationsCurrentDir"
              :current-path="currentPath"
              @click="onEntryClick"
              @menu-click="openEntryContextMenu"
              @files-added="startImportFiles"
              @global-menu-click="openGlobalContextMenu"
              :own-role="ownRole"
              @drop-as-reader="onDropAsReader"
            />
          </div>
          <div v-if="displayView === DisplayState.Grid">
            <file-grid-display
              :files="files"
              :folders="folders"
              :operations-in-progress="fileOperationsCurrentDir"
              :current-path="currentPath"
              @click="onEntryClick"
              @menu-click="openEntryContextMenu"
              @files-added="startImportFiles"
              @global-menu-click="openGlobalContextMenu"
              :own-role="ownRole"
              @drop-as-reader="onDropAsReader"
            />
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { entryNameValidator } from '@/common/validators';
import {
  Answer,
  MsModalResult,
  askQuestion,
  getTextFromUser,
  MsOptions,
  EmptyFolder,
  MsImage,
  DocumentImport,
  DisplayState,
  MsActionBar,
  MsActionBarButton,
  MsGridListToggle,
  MsSorter,
  MsSorterChangeEvent,
  Translatable,
  Clipboard,
  asyncComputed,
  MsSpinner,
} from 'megashark-lib';
import * as parsec from '@/parsec';

import {
  EntryCollection,
  FileDropZone,
  FileGridDisplay,
  FileImportPopover,
  FileOperationProgress,
  FileImportTuple,
  FileInputs,
  FileListDisplay,
  FileModel,
  FolderModel,
  ImportType,
  SortProperty,
  selectFolder,
  EntryModel,
} from '@/components/files';
import {
  Path,
  entryStat,
  WorkspaceCreateFolderErrorTag,
  listWorkspaces,
  WorkspaceID,
  WorkspaceRole,
  FsPath,
  EntryStat,
  WorkspaceStatFolderChildrenErrorTag,
  EntryStatFile,
  EntryName,
  isDesktop,
} from '@/parsec';
import { Routes, currentRouteIs, getCurrentRouteQuery, getDocumentPath, getWorkspaceHandle, navigateTo, watchRoute } from '@/router';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import {
  OperationProgressStateData,
  FolderCreatedStateData,
  ImportData,
  MoveData,
  FileOperationData,
  FileOperationManager,
  FileOperationManagerKey,
  FileOperationState,
  StateData,
  FileOperationDataType,
  CopyData,
} from '@/services/fileOperationManager';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { FileDetailsModal, FileContextMenu, FileAction, FolderGlobalContextMenu, FolderGlobalAction } from '@/views/files';
import { IonContent, IonPage, IonText, modalController, popoverController } from '@ionic/vue';
import { arrowRedo, copy, folderOpen, informationCircle, link, pencil, trashBin } from 'ionicons/icons';
import { Ref, computed, inject, onMounted, onUnmounted, ref } from 'vue';
import { EntrySyncedData, EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { openPath, showInExplorer } from '@/services/fileOpener';

interface FoldersPageSavedData {
  displayState?: DisplayState;
}

const msSorterOptions: MsOptions = new MsOptions([
  {
    label: 'FoldersPage.sort.byName',
    key: SortProperty.Name,
  },
  { label: 'FoldersPage.sort.byLastUpdate', key: SortProperty.LastUpdate },
  { label: 'FoldersPage.sort.bySize', key: SortProperty.Size },
]);

const msSorterLabels = {
  asc: 'FoldersPage.sort.asc',
  desc: 'FoldersPage.sort.desc',
};

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

const sortProperty = ref(SortProperty.Name);
const sortAsc = ref(true);

const fileOperations: Ref<Array<FileOperationProgress>> = ref([]);
const currentPath = ref('/');
const folders = ref(new EntryCollection<FolderModel>());
const files = ref(new EntryCollection<FileModel>());
const displayView = ref(DisplayState.List);
const workspaceInfo: Ref<parsec.StartedWorkspaceInfo | null> = ref(null);
// Init at true to avoid blinking while we're mounting the component
// but we're not loading the files yet.
const querying = ref(true);

const fileInputsRef = ref();
let eventCbId: string | null = null;

const selectedFilesCount = computed(() => {
  return files.value.selectedCount() + folders.value.selectedCount();
});

let hotkeys: HotkeyGroup | null = null;
let callbackId: string | null = null;

const ownRole = computed(() => {
  return workspaceInfo.value ? workspaceInfo.value.currentSelfRole : parsec.WorkspaceRole.Reader;
});

const itemsToShow = computed(() => {
  return (
    folders.value.entriesCount() + files.value.entriesCount() + (fileOperationsCurrentDir.value ? fileOperationsCurrentDir.value.length : 0)
  );
});

async function defineShortcuts(): Promise<void> {
  hotkeys = hotkeyManager.newHotkeys();
  hotkeys.add(
    { key: 'o', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop | Platforms.Web, disableIfModal: true, route: Routes.Documents },
    async () => {
      await fileInputsRef.value.importFiles();
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
      await fileInputsRef.value.importFolder();
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
      folders.value.selectAll(true);
      files.value.selectAll(true);
    },
  );
}

onMounted(async () => {
  displayView.value = (
    await storageManager.retrieveComponentData<FoldersPageSavedData>(FOLDERS_PAGE_DATA_KEY, {
      displayState: DisplayState.List,
    })
  ).displayState;

  await defineShortcuts();

  eventCbId = await eventDistributor.registerCallback(
    Events.EntryUpdated | Events.WorkspaceUpdated | Events.EntrySynced,
    async (event: Events, data?: EventData) => {
      if (event === Events.EntryUpdated) {
        await listFolder({ sameFolder: true });
      } else if (event === Events.WorkspaceUpdated && workspaceInfo.value) {
        await updateWorkspaceInfo(workspaceInfo.value.id);
      } else if (event === Events.EntrySynced) {
        const syncedData = data as EntrySyncedData;
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
          if (entry) {
            entry.needSync = false;
          }
        }
      }
    },
  );

  const workspaceHandle = getWorkspaceHandle();
  if (workspaceHandle) {
    const infoResult = await parsec.getWorkspaceInfo(workspaceHandle);
    if (infoResult.ok) {
      workspaceInfo.value = infoResult.value;
    }
  }
  callbackId = await fileOperationManager.registerCallback(onFileOperationState);
  currentPath.value = getDocumentPath();
  const query = getCurrentRouteQuery();
  await listFolder({ selectFile: query.selectFile });
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
  if (callbackId) {
    fileOperationManager.removeCallback(callbackId);
  }
  routeWatchCancel();
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
  await storageManager.storeComponentData<FoldersPageSavedData>(FOLDERS_PAGE_DATA_KEY, { displayState: displayView.value });
}

function onSortChange(event: MsSorterChangeEvent): void {
  folders.value.sort(event.option.key, event.sortByAsc);
  files.value.sort(event.option.key, event.sortByAsc);
  sortProperty.value = event.option.key;
  sortAsc.value = event.sortByAsc;
}

async function onFileOperationState(state: FileOperationState, operationData?: FileOperationData, stateData?: StateData): Promise<void> {
  if (state === FileOperationState.FileAdded && operationData) {
    fileOperations.value.push({ data: operationData, progress: 0 });
  } else if (state === FileOperationState.MoveAdded && operationData) {
    fileOperations.value.push({ data: operationData, progress: 0 });
  } else if (state === FileOperationState.CopyAdded && operationData) {
    fileOperations.value.push({ data: operationData, progress: 0 });
  } else if (state === FileOperationState.OperationProgress && operationData) {
    const op = fileOperations.value.find((item) => item.data.id === operationData.id);
    if (op) {
      op.progress = (stateData as OperationProgressStateData).progress;
    }
  } else if (
    [
      FileOperationState.Cancelled,
      FileOperationState.CreateFailed,
      FileOperationState.WriteError,
      FileOperationState.MoveFailed,
      FileOperationState.CopyFailed,
    ].includes(state) &&
    operationData
  ) {
    const index = fileOperations.value.findIndex((item) => item.data.id === operationData.id);
    if (index !== -1) {
      fileOperations.value.splice(index, 1);
    }
  } else if (state === FileOperationState.FileImported && operationData) {
    const index = fileOperations.value.findIndex((item) => item.data.id === operationData.id);
    if (index !== -1) {
      fileOperations.value.splice(index, 1);
    }
    const importData = operationData as ImportData;
    if (importData.workspaceHandle === workspaceInfo.value?.handle && parsec.Path.areSame(importData.path, currentPath.value)) {
      const importedFilePath = await parsec.Path.join(importData.path, importData.file.name);
      const statResult = await parsec.entryStat(importData.workspaceHandle, importedFilePath);
      if (statResult.ok && statResult.value.isFile()) {
        const existing = files.value.getEntries().find((entry) => entry.id === statResult.value.id);
        if (!existing) {
          (statResult.value as FileModel).isSelected = false;
          files.value.append(statResult.value as FileModel);
        } else {
          existing.name = statResult.value.name;
          existing.size = (statResult.value as parsec.EntryStatFile).size;
          existing.needSync = statResult.value.needSync;
          existing.updated = statResult.value.updated;
          existing.isSelected = false;
        }
        files.value.sort(sortProperty.value, sortAsc.value);
      }
    }
  } else if (state === FileOperationState.EntryMoved && operationData) {
    const index = fileOperations.value.findIndex((item) => item.data.id === operationData.id);
    if (index !== -1) {
      fileOperations.value.splice(index, 1);
    }
    const moveData = operationData as MoveData;
    const dstPathParent = await Path.parent(moveData.dstPath);
    const srcPathParent = await Path.parent(moveData.srcPath);
    if (
      moveData.workspaceHandle === workspaceInfo.value?.handle &&
      (Path.areSame(dstPathParent, currentPath.value) || Path.areSame(srcPathParent, currentPath.value))
    ) {
      await listFolder({ sameFolder: true });
    }
  } else if (state === FileOperationState.EntryCopied && operationData) {
    const index = fileOperations.value.findIndex((item) => item.data.id === operationData.id);
    if (index !== -1) {
      fileOperations.value.splice(index, 1);
    }
    const copyData = operationData as CopyData;

    if (copyData.workspaceHandle === workspaceInfo.value?.handle && Path.areSame(copyData.dstPath, currentPath.value)) {
      await listFolder({ sameFolder: true });
    }
  } else if (state === FileOperationState.FolderCreated) {
    const folderData = stateData as FolderCreatedStateData;
    if (folderData.workspaceHandle === workspaceInfo.value?.handle) {
      const parent = await parsec.Path.parent(folderData.path);
      if (parsec.Path.areSame(parent, currentPath.value)) {
        const statResult = await parsec.entryStat(folderData.workspaceHandle, folderData.path);
        if (statResult.ok && !statResult.value.isFile()) {
          if (!folders.value.getEntries().find((entry) => entry.id === statResult.value.id)) {
            (statResult.value as FolderModel).isSelected = false;
            folders.value.append(statResult.value as FolderModel);
            folders.value.sort(sortProperty.value, sortAsc.value);
          }
        }
      }
    }
  }
}

async function startImportFiles(imports: FileImportTuple[]): Promise<void> {
  const existing: FileImportTuple[] = [];

  if (!workspaceInfo.value) {
    return;
  }

  for (const imp of imports) {
    let importPath = imp.path;
    let fileName = imp.file.name;
    if (imp.file.webkitRelativePath) {
      const parsed = await Path.parse(`/${imp.file.webkitRelativePath}`);
      importPath = await Path.joinMultiple(imp.path, parsed.slice(0, -1));
      fileName = parsed[parsed.length - 1];
      imp.path = importPath;
    }
    const fullPath = await Path.join(importPath, fileName);
    const result = await entryStat(workspaceInfo.value.handle, fullPath);
    if (result.ok && result.value.isFile()) {
      existing.push(imp);
    }
  }

  if (existing.length > 0) {
    const answer = await askQuestion(
      { key: 'FoldersPage.importModal.replaceTitle', count: 1 },
      { key: 'FoldersPage.importModal.replaceQuestion', data: { file: existing[0].file.name, count: existing.length } },
      {
        yesText: { key: 'FoldersPage.importModal.replaceText', count: 1 },
        noText: { key: 'FoldersPage.importModal.skipText', count: 1 },
        backdropDismiss: false,
      },
    );
    if (answer === Answer.No) {
      imports = imports.filter((imp) => {
        return (
          existing.find((ex) => {
            return ex.file.name === imp.file.name && ex.path === imp.path;
          }) === undefined
        );
      });
    }
  }
  for (const imp of imports) {
    await fileOperationManager.importFile(workspaceInfo.value.handle, workspaceInfo.value.id, imp.file, imp.path);
  }
}

const fileOperationsCurrentDir = asyncComputed(async () => {
  const operations: Array<FileOperationProgress> = [];

  for (const op of fileOperations.value) {
    let path = '';
    if (op.data.getDataType() === FileOperationDataType.Import) {
      path = (op.data as ImportData).path;
    } else if (op.data.getDataType() === FileOperationDataType.Move) {
      path = await parsec.Path.parent((op.data as MoveData).dstPath);
    } else if (op.data.getDataType() === FileOperationDataType.Copy) {
      path = (op.data as CopyData).dstPath;
    }
    if (op.data.workspaceHandle === workspaceInfo.value?.handle && parsec.Path.areSame(path, currentPath.value)) {
      operations.push(op);
    }
  }
  return operations;
});

async function listFolder(options?: { selectFile?: EntryName; sameFolder?: boolean }): Promise<void> {
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
      // Excluding files currently being imported
      let foundInFileOps = false;
      for (const fileOp of fileOperationsCurrentDir.value) {
        if (fileOp.data.getDataType() === FileOperationDataType.Import) {
          foundInFileOps = (fileOp.data as ImportData).file.name === childName;
        } else if (fileOp.data.getDataType() === FileOperationDataType.Move) {
          const fileName = await Path.filename((fileOp.data as MoveData).dstPath);
          foundInFileOps = fileName === childName;
        } else if (fileOp.data.getDataType() === FileOperationDataType.Copy) {
          const fileName = await Path.filename((fileOp.data as CopyData).srcPath);
          foundInFileOps = fileName === childName;
        }
        if (foundInFileOps) {
          break;
        }
      }
      if (!foundInFileOps) {
        if (childStat.isFile()) {
          (childStat as FileModel).isSelected = Boolean(options && options.selectFile && options.selectFile === childName);
          newFiles.push(childStat as FileModel);
        } else {
          (childStat as FolderModel).isSelected = false;
          newFolders.push(childStat as FolderModel);
        }
      }
    }
    folders.value.smartUpdate(newFolders);
    files.value.smartUpdate(newFiles);
    folders.value.sort(sortProperty.value, sortAsc.value);
    files.value.sort(sortProperty.value, sortAsc.value);
  } else {
    // This happens when the handle becomes invalid (if we're logging out for example) and the app tries to refresh at the same
    // time. Logging out while importing files is a good example of that: logging out will cancel the imports which will trigger
    // a refresh.
    if (result.error.tag === WorkspaceStatFolderChildrenErrorTag.Internal && result.error.error === 'Invalid Handle') {
      console.log('Skipping entry stat error because of invalid handle');
      return;
    }
    informationManager.present(
      new Information({
        message: {
          key: 'FoldersPage.errors.listFailed',
          data: {
            path: currentPath.value,
          },
        },
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
  querying.value = false;
}

async function onEntryClick(entry: EntryModel, _event: Event): Promise<void> {
  if (!entry.isFile()) {
    const newPath = await parsec.Path.join(currentPath.value, entry.name);
    navigateTo(Routes.Documents, {
      query: { documentPath: newPath, workspaceHandle: workspaceInfo.value?.handle },
    });
  } else {
    await openEntries([entry]);
  }
}

async function createFolder(): Promise<void> {
  if (!workspaceInfo.value) {
    return;
  }
  const folderName = await getTextFromUser({
    title: 'FoldersPage.CreateFolderModal.title',
    trim: true,
    validator: entryNameValidator,
    inputLabel: 'FoldersPage.CreateFolderModal.label',
    placeholder: 'FoldersPage.CreateFolderModal.placeholder',
    okButtonText: 'FoldersPage.CreateFolderModal.create',
  });

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
    await fileInputsRef.value.importFiles();
  } else if (result.data.type === ImportType.Folder) {
    await fileInputsRef.value.importFolder();
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
  await listFolder({ sameFolder: true });
}

async function renameEntries(entries: EntryModel[]): Promise<void> {
  if (entries.length !== 1 || !workspaceInfo.value) {
    return;
  }
  const entry = entries[0];
  const ext = parsec.Path.getFileExtension(entry.name);
  const newName = await getTextFromUser({
    title: entry.isFile() ? 'FoldersPage.RenameModal.fileTitle' : 'FoldersPage.RenameModal.folderTitle',
    trim: true,
    validator: entryNameValidator,
    inputLabel: entry.isFile() ? 'FoldersPage.RenameModal.fileLabel' : 'FoldersPage.RenameModal.folderLabel',
    placeholder: entry.isFile() ? 'FoldersPage.RenameModal.filePlaceholder' : 'FoldersPage.RenameModal.folderPlaceholder',
    okButtonText: 'FoldersPage.RenameModal.rename',
    defaultValue: entry.name,
    selectionRange: [0, entry.name.length - (ext.length > 0 ? ext.length + 1 : 0)],
  });

  if (!newName) {
    return;
  }
  const filePath = await parsec.Path.join(currentPath.value, entry.name);
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
    entry.name = newName;
    entry.path = result.value;
  }
}

async function copyLink(entries: EntryModel[]): Promise<void> {
  if (entries.length !== 1 || !workspaceInfo.value) {
    return;
  }
  const entry = entries[0];
  const filePath = await parsec.Path.join(currentPath.value, entry.name);
  const result = await parsec.getPathLink(workspaceInfo.value.handle, filePath);
  if (result.ok) {
    if (!(await Clipboard.writeText(result.value))) {
      informationManager.present(
        new Information({
          message: 'FoldersPage.linkNotCopiedToClipboard',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.present(
        new Information({
          message: 'FoldersPage.linkCopiedToClipboard',
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    informationManager.present(
      new Information({
        message: { key: 'FoldersPage.getLinkError', data: { reason: result.error.tag } },
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
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
  // A bit complicated, but bear with me
  const existingEntries: Array<EntryStat> = [];
  // First, we try to detect if the destination already contains an entry with the same name
  for (const entry of entries) {
    const dstPath = await parsec.Path.join(folder, entry.name);
    const statsResult = await entryStat(workspaceInfo.value.handle, dstPath);
    if (statsResult.ok) {
      existingEntries.push(entry);
    }
  }
  if (existingEntries.length > 0) {
    const answer = await askQuestion(
      {
        key: 'FoldersPage.moveAlreadyExistTitle',
        count: existingEntries.length,
      },
      {
        key: 'FoldersPage.moveAlreadyExistQuestion',
        data: existingEntries.length === 1 ? { file: existingEntries[0].name } : undefined,
        count: existingEntries.length,
      },
      {
        yesText: { key: 'FoldersPage.moveAlreadyExistReplace', count: existingEntries.length },
        noText: { key: 'FoldersPage.moveAlreadyExistSkip', count: existingEntries.length },
        yesIsDangerous: true,
        backdropDismiss: false,
      },
    );
    // User chooses to skip, we remove the existing entries from the entries to move, and we clear existingEntries
    if (answer === Answer.No) {
      entries = entries.filter((e) => existingEntries.find((d) => d.id === e.id) === undefined);
      // Too difficult to add a .clear() method on an array, plus it wouldn't make any sense, why would you ever
      // want to clear an array?
      existingEntries.splice(0, existingEntries.length);
    }
  }
  for (const entry of entries) {
    await fileOperationManager.moveEntry(
      workspaceInfo.value.handle,
      workspaceInfo.value.id,
      await parsec.Path.join(currentPath.value, entry.name),
      await parsec.Path.join(folder, entry.name),
      // If the file is still in existingEntries, the user has given us
      // permission to replace it, otherwise we don't want to force.
      existingEntries.find((e) => e.id === entry.id) !== undefined,
    );
    entry.isSelected = false;
  }
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

  for (const entry of entries) {
    await fileOperationManager.copyEntry(workspaceInfo.value.handle, workspaceInfo.value.id, entry.path, folder);
    entry.isSelected = false;
  }
}

async function downloadEntries(entries: EntryModel[]): Promise<void> {
  console.log('Download', entries);
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
}

async function openEntries(entries: EntryModel[]): Promise<void> {
  if (entries.length !== 1 || !workspaceInfo.value || !entries[0].isFile()) {
    return;
  }

  const entry = entries[0] as EntryStatFile;
  const workspaceHandle = workspaceInfo.value.handle;

  const config = await storageManager.retrieveConfig();

  await openPath(workspaceHandle, entry.path, informationManager, { skipViewers: config.skipViewers });
}

async function openGlobalContextMenu(event: Event): Promise<void> {
  if (ownRole.value === WorkspaceRole.Reader) {
    return;
  }

  const popover = await popoverController.create({
    component: FolderGlobalContextMenu,
    cssClass: 'folder-global-context-menu',
    event: event,
    reference: event.type === 'contextmenu' ? 'event' : 'trigger',
    translucent: true,
    showBackdrop: false,
    dismissOnSelect: true,
    alignment: 'start',
    componentProps: {
      role: ownRole.value,
    },
  });
  await popover.present();

  const { data } = await popover.onDidDismiss();
  if (!data || !workspaceInfo.value) {
    return;
  }
  switch (data.action) {
    case FolderGlobalAction.CreateFolder:
      return await createFolder();
    case FolderGlobalAction.ImportFiles:
      return await fileInputsRef.value.importFiles();
    case FolderGlobalAction.ImportFolder:
      return await fileInputsRef.value.importFolder();
    case FolderGlobalAction.OpenInExplorer:
      return await openPath(workspaceInfo.value.handle, currentPath.value, informationManager, { skipViewers: true });
  }
}

async function openEntryContextMenu(event: Event, entry: EntryModel, onFinished?: () => void): Promise<void> {
  const selectedEntries = getSelectedEntries();

  const popover = await popoverController.create({
    component: FileContextMenu,
    cssClass: 'file-context-menu',
    event: event,
    reference: event.type === 'contextmenu' ? 'event' : 'trigger',
    translucent: true,
    showBackdrop: false,
    dismissOnSelect: true,
    alignment: 'start',
    componentProps: {
      role: ownRole.value,
      multipleFiles: selectedEntries.length > 1 && selectedEntries.includes(entry),
      isFile: entry.isFile(),
    },
  });
  await popover.present();

  const { data } = await popover.onDidDismiss();

  if (!data) {
    if (onFinished) {
      onFinished();
    }
    return;
  }

  const actions = new Map<FileAction, (file: EntryModel[]) => Promise<void>>([
    [FileAction.Rename, renameEntries],
    [FileAction.MoveTo, moveEntriesTo],
    [FileAction.MakeACopy, copyEntries],
    [FileAction.Open, openEntries],
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

async function seeInExplorer(entries: EntryModel[]): Promise<void> {
  if (entries.length > 1 || !workspaceInfo.value || !isDesktop()) {
    return;
  }
  if (entries[0].isFile()) {
    await showInExplorer(workspaceInfo.value.handle, entries[0].path, informationManager);
  } else {
    await openPath(workspaceInfo.value.handle, entries[0].path, informationManager, { skipViewers: true });
  }
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
</script>

<style scoped lang="scss">
.folder-container div:not(.no-files-content) {
  height: 100%;
}

.no-files {
  width: 100%;
  height: 100%;
  color: var(--parsec-color-light-secondary-grey);
  display: flex;
  margin: auto;
  align-items: center;

  &-content {
    border-radius: var(--parsec-radius-8);
    display: flex;
    height: fit-content;
    width: 100%;
    text-align: center;
    flex-direction: column;
    gap: 1rem;
    align-items: center;
    padding: 2rem 1rem;
  }
}
</style>

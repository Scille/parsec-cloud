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
            v-show="ownRole !== parsec.WorkspaceRole.Reader && false"
            :button-label="'FoldersPage.fileContextMenu.actionMoveTo'"
            :icon="arrowRedo"
            @click="moveEntriesTo(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-makeacopy"
            v-show="ownRole !== parsec.WorkspaceRole.Reader && false"
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
            v-show="ownRole !== parsec.WorkspaceRole.Reader && false"
            :button-label="'FoldersPage.fileContextMenu.actionMoveTo'"
            :icon="arrowRedo"
            @click="moveEntriesTo(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-makeacopy"
            v-show="ownRole !== parsec.WorkspaceRole.Reader && false"
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
          v-if="folders.entriesCount() + files.entriesCount() + fileImportsCurrentDir.length === 0"
          class="no-files body-lg"
        >
          <file-drop-zone
            ref="fileDropZoneRef"
            :current-path="currentPath"
            :show-drop-message="true"
            @files-added="startImportFiles"
          >
            <div class="no-files-content">
              <ms-image :image="EmptyFolder" />
              <ion-text>
                {{ $msTranslate('FoldersPage.emptyFolder') }}
              </ion-text>
            </div>
          </file-drop-zone>
        </div>
        <div v-else>
          <div v-if="displayView === DisplayState.List">
            <file-list-display
              :files="files"
              :folders="folders"
              :importing="fileImportsCurrentDir"
              :current-path="currentPath"
              @click="onEntryClick"
              @menu-click="openEntryContextMenu"
              @files-added="startImportFiles"
            />
          </div>
          <div v-if="displayView === DisplayState.Grid">
            <file-grid-display
              :files="files"
              :folders="folders"
              :importing="fileImportsCurrentDir"
              :current-path="currentPath"
              @click="onEntryClick"
              @menu-click="openEntryContextMenu"
              @files-added="startImportFiles"
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
} from 'megashark-lib';
import * as parsec from '@/parsec';

import {
  EntryCollection,
  FileDropZone,
  FileGridDisplay,
  FileImportPopover,
  FileImportProgress,
  FileImportTuple,
  FileInputs,
  FileListDisplay,
  FileModel,
  FolderModel,
  ImportType,
  SortProperty,
  selectFolder,
} from '@/components/files';
import { Path, entryStat, WorkspaceCreateFolderErrorTag } from '@/parsec';
import { Routes, currentRouteIs, getCurrentRouteQuery, getDocumentPath, getWorkspaceHandle, navigateTo, watchRoute } from '@/router';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import {
  FileProgressStateData,
  FolderCreatedStateData,
  ImportData,
  ImportManager,
  ImportManagerKey,
  ImportState,
  StateData,
} from '@/services/importManager';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import FileContextMenu, { FileAction } from '@/views/files/FileContextMenu.vue';
import FileDetailsModal from '@/views/files/FileDetailsModal.vue';
import { IonContent, IonPage, IonText, modalController, popoverController } from '@ionic/vue';
import { arrowRedo, copy, folderOpen, informationCircle, link, pencil, trashBin } from 'ionicons/icons';
import { Ref, computed, inject, onMounted, onUnmounted, ref } from 'vue';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';

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
  if (newPath !== currentPath.value) {
    currentPath.value = newPath;
  }
  const workspaceHandle = getWorkspaceHandle();
  if (workspaceHandle) {
    const infoResult = await parsec.getWorkspaceInfo(workspaceHandle);
    if (infoResult.ok) {
      workspaceInfo.value = infoResult.value;
    }
  }
  await listFolder();
});

const importManager: ImportManager = inject(ImportManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
const informationManager: InformationManager = inject(InformationManagerKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;

const FOLDERS_PAGE_DATA_KEY = 'FoldersPage';

const fileImports: Ref<Array<FileImportProgress>> = ref([]);
const currentPath = ref('/');
const folderInfo: Ref<parsec.EntryStatFolder | null> = ref(null);
const folders = ref(new EntryCollection<FolderModel>());
const files = ref(new EntryCollection<FileModel>());
const displayView = ref(DisplayState.List);
const workspaceInfo: Ref<parsec.StartedWorkspaceInfo | null> = ref(null);

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

  eventCbId = await eventDistributor.registerCallback(Events.EntryUpdated, async (event: Events, _data: EventData) => {
    if (event === Events.EntryUpdated) {
      await listFolder();
    }
  });

  const workspaceHandle = getWorkspaceHandle();
  if (workspaceHandle) {
    const infoResult = await parsec.getWorkspaceInfo(workspaceHandle);
    if (infoResult.ok) {
      workspaceInfo.value = infoResult.value;
    }
  }
  callbackId = await importManager.registerCallback(onFileImportState);
  currentPath.value = getDocumentPath();
  await listFolder();
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
  if (callbackId) {
    importManager.removeCallback(callbackId);
  }
  routeWatchCancel();
  if (eventCbId) {
    eventDistributor.removeCallback(eventCbId);
  }
});

async function onDisplayStateChange(): Promise<void> {
  await storageManager.storeComponentData<FoldersPageSavedData>(FOLDERS_PAGE_DATA_KEY, { displayState: displayView.value });
}

function onSortChange(event: MsSorterChangeEvent): void {
  folders.value.sort(event.option.key, event.sortByAsc);
  files.value.sort(event.option.key, event.sortByAsc);
}

async function onFileImportState(state: ImportState, importData?: ImportData, stateData?: StateData): Promise<void> {
  if (state === ImportState.FileAdded && importData) {
    fileImports.value.push({ data: importData, progress: 0 });
  } else if (state === ImportState.FileProgress && importData) {
    const index = fileImports.value.findIndex((item) => item.data.id === importData.id);
    if (index !== -1) {
      fileImports.value[index].progress = (stateData as FileProgressStateData).progress;
    }
  } else if ([ImportState.Cancelled, ImportState.CreateFailed, ImportState.WriteError].includes(state) && importData) {
    const index = fileImports.value.findIndex((item) => item.data.id === importData.id);
    if (index !== -1) {
      fileImports.value.splice(index, 1);
    }
  } else if (state === ImportState.FileImported && importData) {
    const index = fileImports.value.findIndex((item) => item.data.id === importData.id);
    if (index !== -1) {
      fileImports.value.splice(index, 1);
    }
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
      }
    }
  } else if (state === ImportState.FolderCreated) {
    const folderData = stateData as FolderCreatedStateData;
    if (folderData.workspaceHandle === workspaceInfo.value?.handle) {
      const parent = await parsec.Path.parent(folderData.path);
      if (parsec.Path.areSame(parent, currentPath.value)) {
        const statResult = await parsec.entryStat(folderData.workspaceHandle, folderData.path);
        if (statResult.ok && !statResult.value.isFile()) {
          if (!folders.value.getEntries().find((entry) => entry.id === statResult.value.id)) {
            (statResult.value as FolderModel).isSelected = false;
            folders.value.append(statResult.value as FolderModel);
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
      importPath = await Path.join(imp.path, parsed[0]);
      fileName = parsed[1];
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
    await importManager.importFile(workspaceInfo.value.handle, workspaceInfo.value.id, imp.file, imp.path);
  }
}

const fileImportsCurrentDir = computed(() => {
  return fileImports.value.filter(
    (item) => parsec.Path.areSame(item.data.path, currentPath.value) && item.data.workspaceHandle === workspaceInfo.value?.handle,
  );
});

async function listFolder(): Promise<void> {
  if (!workspaceInfo.value) {
    return;
  }
  if (!currentRouteIs(Routes.Documents)) {
    return;
  }
  const result = await parsec.entryStat(workspaceInfo.value.handle, currentPath.value);
  if (result.ok) {
    const newFolders: FolderModel[] = [];
    const newFiles: FileModel[] = [];
    folderInfo.value = result.value as parsec.EntryStatFolder;
    const query = getCurrentRouteQuery();
    for (const [childName] of (result.value as parsec.EntryStatFolder).children) {
      // Excluding files currently being imported
      if (fileImports.value.find((imp) => imp.data.file.name === childName) === undefined) {
        const childPath = await parsec.Path.join(currentPath.value, childName);
        const fileResult = await parsec.entryStat(workspaceInfo.value.handle, childPath);
        if (fileResult.ok) {
          if (fileResult.value.isFile()) {
            (fileResult.value as FileModel).isSelected = query.selectFile && query.selectFile === fileResult.value.name ? true : false;
            newFiles.push(fileResult.value as FileModel);
          } else {
            (fileResult.value as FolderModel).isSelected = false;
            newFolders.push(fileResult.value as FolderModel);
          }
        }
      }
    }
    folders.value.smartUpdate(newFolders);
    files.value.smartUpdate(newFiles);
  } else {
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
}

async function onEntryClick(entry: parsec.EntryStat, _event: Event): Promise<void> {
  if (!entry.isFile()) {
    const newPath = await parsec.Path.join(currentPath.value, entry.name);
    navigateTo(Routes.Documents, {
      params: { workspaceHandle: workspaceInfo.value?.handle },
      query: { documentPath: newPath },
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
  await listFolder();
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

function getSelectedEntries(): parsec.EntryStat[] {
  return [...folders.value.getSelectedEntries(), ...files.value.getSelectedEntries()];
}

async function deleteEntries(entries: parsec.EntryStat[]): Promise<void> {
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
  await listFolder();
}

async function renameEntries(entries: parsec.EntryStat[]): Promise<void> {
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
      case parsec.WorkspaceRenameEntryErrorTag.DestinationExists:
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
  }
}

async function copyLink(entries: parsec.EntryStat[]): Promise<void> {
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

async function moveEntriesTo(entries: parsec.EntryStat[]): Promise<void> {
  if (entries.length === 0 || !workspaceInfo.value) {
    return;
  }
  const folder = await selectFolder({
    title: { key: 'FoldersPage.moveSelectFolderTitle', data: { count: entries.length }, count: entries.length },
    startingPath: currentPath.value,
    workspaceHandle: workspaceInfo.value.handle,
  });
  if (!folder) {
    return;
  }
  let errorCount = 0;
  for (const entry of entries) {
    const currentEntryPath = await parsec.Path.join(currentPath.value, entry.name);
    const newEntryPath = await parsec.Path.join(folder, entry.name);
    const result = await parsec.moveEntry(currentEntryPath, newEntryPath);
    errorCount += Number(!result.ok);
  }
  if (errorCount > 0) {
    if (entries.length === 1) {
      informationManager.present(
        new Information({
          message: { key: 'FoldersPage.errors.moveOneFailed', data: { name: entries[0].name } },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.present(
        new Information({
          message: errorCount === entries.length ? 'FoldersPage.errors.moveMultipleAllFailed' : 'FoldersPage.errors.moveMultipleSomeFailed',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    informationManager.present(
      new Information({
        message: { key: 'FoldersPage.moveSuccess', data: { count: entries.length }, count: entries.length },
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  }
  await listFolder();
}

async function showDetails(entries: parsec.EntryStat[]): Promise<void> {
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

async function copyEntries(entries: parsec.EntryStat[]): Promise<void> {
  if (entries.length === 0 || !workspaceInfo.value) {
    return;
  }
  const folder = await selectFolder({
    title: { key: 'FoldersPage.copySelectFolderTitle', data: { count: entries.length }, count: entries.length },
    startingPath: currentPath.value,
    workspaceHandle: workspaceInfo.value.handle,
  });
  if (!folder) {
    return;
  }
  let errorCount = 0;
  for (const entry of entries) {
    const currentEntryPath = await parsec.Path.join(currentPath.value, entry.name);
    const newEntryPath = await parsec.Path.join(folder, entry.name);
    const result = await parsec.copyEntry(currentEntryPath, newEntryPath);
    errorCount += Number(!result.ok);
  }
  if (errorCount > 0) {
    if (entries.length === 1) {
      informationManager.present(
        new Information({
          message: { key: 'FoldersPage.errors.copyOneFailed', data: { name: entries[0].name } },
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.present(
        new Information({
          message: errorCount === entries.length ? 'FoldersPage.errors.copyMultipleAllFailed' : 'FoldersPage.errors.copyMultipleSomeFailed',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    informationManager.present(
      new Information({
        message: { key: 'FoldersPage.copySuccess', count: entries.length },
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  }
  await listFolder();
}

async function downloadEntries(entries: parsec.EntryStat[]): Promise<void> {
  console.log('Download', entries);
}

async function showHistory(entries: parsec.EntryStat[]): Promise<void> {
  if (entries.length !== 1) {
    return;
  }
  console.log('Show history', entries[0]);
}

async function openEntries(entries: parsec.EntryStat[]): Promise<void> {
  if (entries.length !== 1 || !workspaceInfo.value) {
    return;
  }
  if (parsec.isWeb()) {
    await informationManager.present(
      new Information({
        message: 'FoldersPage.open.unavailableOnWeb',
        level: InformationLevel.Warning,
      }),
      PresentationMode.Modal,
    );
    return;
  }
  const entry = entries[0];
  const entryPath = await parsec.Path.join(currentPath.value, entry.name);
  const result = await parsec.getSystemPath(workspaceInfo.value.handle, entryPath);

  if (!result.ok) {
    await informationManager.present(
      new Information({
        message: entry.isFile()
          ? { key: 'FoldersPage.open.fileFailed', data: { name: entry.name } }
          : { key: 'FoldersPage.open.folderFailed', data: { name: entry.name } },
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
  } else {
    window.electronAPI.openFile(result.value);
  }
}

async function openEntryContextMenu(event: Event, entry: parsec.EntryStat, onFinished?: () => void): Promise<void> {
  const popover = await popoverController.create({
    component: FileContextMenu,
    cssClass: 'file-context-menu',
    event: event,
    translucent: true,
    showBackdrop: false,
    dismissOnSelect: true,
    alignment: 'end',
    componentProps: {
      role: ownRole.value,
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

  const actions = new Map<FileAction, (file: parsec.EntryStat[]) => Promise<void>>([
    [FileAction.Rename, renameEntries],
    [FileAction.MoveTo, moveEntriesTo],
    [FileAction.MakeACopy, copyEntries],
    [FileAction.Open, openEntries],
    [FileAction.ShowHistory, showHistory],
    [FileAction.Download, downloadEntries],
    [FileAction.ShowDetails, showDetails],
    [FileAction.CopyLink, copyLink],
    [FileAction.Delete, deleteEntries],
  ]);

  const fn = actions.get(data.action);
  if (fn) {
    await fn([entry]);
  }
  if (onFinished) {
    onFinished();
  }
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

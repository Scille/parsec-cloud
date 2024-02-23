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
            :button-label="$t('FoldersPage.createFolder')"
            :icon="folderOpen"
            @click="createFolder()"
          />
          <ms-action-bar-button
            id="button-import"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="$t('FoldersPage.import')"
            :icon="document"
            @click="importFiles()"
          />
        </div>
        <div v-else-if="selectedFilesCount === 1">
          <ms-action-bar-button
            id="button-rename"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="$t('FoldersPage.fileContextMenu.actionRename')"
            :icon="pencil"
            @click="renameEntries(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-copy-link"
            :button-label="$t('FoldersPage.fileContextMenu.actionCopyLink')"
            :icon="link"
            @click="copyLink(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-moveto"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="$t('FoldersPage.fileContextMenu.actionMoveTo')"
            :icon="arrowRedo"
            @click="moveEntriesTo(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-makeacopy"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="$t('FoldersPage.fileContextMenu.actionMakeACopy')"
            :icon="copy"
            @click="copyEntries(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-delete"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="$t('FoldersPage.fileContextMenu.actionDelete')"
            :icon="trashBin"
            @click="deleteEntries(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-details"
            :button-label="$t('FoldersPage.fileContextMenu.actionDetails')"
            :icon="informationCircle"
            @click="showDetails(getSelectedEntries())"
          />
        </div>
        <div v-else>
          <ms-action-bar-button
            id="button-moveto"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="$t('FoldersPage.fileContextMenu.actionMoveTo')"
            :icon="arrowRedo"
            @click="moveEntriesTo(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-makeacopy"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="$t('FoldersPage.fileContextMenu.actionMakeACopy')"
            :icon="copy"
            @click="copyEntries(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-delete"
            v-show="ownRole !== parsec.WorkspaceRole.Reader"
            :button-label="$t('FoldersPage.fileContextMenu.actionDelete')"
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
                $t(
                  'FoldersPage.itemCount',
                  { count: folders.entriesCount() + files.entriesCount() },
                  folders.entriesCount() + files.entriesCount(),
                )
              }}
            </ion-text>
            <ion-text
              class="body item-selected"
              v-if="selectedFilesCount > 0"
            >
              {{ $t('FoldersPage.itemSelectedCount', { count: selectedFilesCount }, selectedFilesCount) }}
            </ion-text>
          </div>

          <ms-sorter
            :label="$t('FoldersPage.sort.byName')"
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
        <div
          v-if="folders.entriesCount() + files.entriesCount() === 0"
          class="no-files body-lg"
        >
          <div class="no-files-content">
            <ms-image :image="EmptyFolder" />
            <ion-text>
              {{ $t('FoldersPage.emptyFolder') }}
            </ion-text>
          </div>
        </div>
        <div v-else>
          <div v-if="displayView === DisplayState.List">
            <file-list-display
              :files="files"
              :folders="folders"
              :importing="fileImportsCurrentDir"
              @click="onEntryClick"
              @menu-click="openEntryContextMenu"
            />
          </div>
          <div v-if="displayView === DisplayState.Grid">
            <file-grid-display
              :files="files"
              :folders="folders"
              :importing="fileImportsCurrentDir"
              @click="onEntryClick"
              @menu-click="openEntryContextMenu"
            />
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { writeTextToClipboard } from '@/common/clipboard';
import { entryNameValidator } from '@/common/validators';
import {
  Answer,
  DisplayState,
  MsActionBar,
  MsActionBarButton,
  MsGridListToggle,
  askQuestion,
  getTextInputFromUser,
  selectFolder,
} from '@/components/core';
import { EmptyFolder, MsImage } from '@/components/core/ms-image';
import * as parsec from '@/parsec';

import { MsOptions, MsSorter, MsSorterChangeEvent } from '@/components/core';
import {
  EntryCollection,
  FileGridDisplay,
  FileImportProgress,
  FileListDisplay,
  FileModel,
  FolderModel,
  SortProperty,
} from '@/components/files';
import { Routes, getDocumentPath, getWorkspaceHandle, getWorkspaceId, navigateTo, watchRoute } from '@/router';
import { Groups, HotkeyManager, HotkeyManagerKey, Hotkeys, Modifiers, Platforms } from '@/services/hotkeyManager';
import { FileProgressStateData, ImportData, ImportManager, ImportManagerKey, ImportState, StateData } from '@/services/importManager';
import { Information, InformationKey, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { translate } from '@/services/translation';
import FileContextMenu, { FileAction } from '@/views/files/FileContextMenu.vue';
import FileDetailsModal from '@/views/files/FileDetailsModal.vue';
import FileUploadModal from '@/views/files/FileUploadModal.vue';
import { IonContent, IonPage, IonText, modalController, popoverController } from '@ionic/vue';
import { arrowRedo, copy, document, folderOpen, informationCircle, link, pencil, trashBin } from 'ionicons/icons';
import { Ref, computed, inject, onMounted, onUnmounted, ref } from 'vue';

interface FoldersPageSavedData {
  displayState?: DisplayState;
}

const msSorterOptions: MsOptions = new MsOptions([
  {
    label: translate('FoldersPage.sort.byName'),
    key: SortProperty.Name,
  },
  { label: translate('FoldersPage.sort.byLastUpdate'), key: SortProperty.LastUpdate },
  { label: translate('FoldersPage.sort.bySize'), key: SortProperty.Size },
]);

const msSorterLabels = {
  asc: translate('FoldersPage.sort.asc'),
  desc: translate('FoldersPage.sort.desc'),
};

const routeWatchCancel = watchRoute(async () => {
  const newPath = getDocumentPath();
  if (newPath === '') {
    return;
  }
  if (newPath !== currentPath.value) {
    await setCurrentPath(newPath);
  }
  ownRole.value = await parsec.getWorkspaceRole(getWorkspaceId());
});

const importManager: ImportManager = inject(ImportManagerKey)!;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;
const informationManager: InformationManager = inject(InformationKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;

const FOLDERS_PAGE_DATA_KEY = 'FoldersPage';

const fileImports: Ref<Array<FileImportProgress>> = ref([]);
const currentPath = ref('/');
const folderInfo: Ref<parsec.EntryStatFolder | null> = ref(null);
const folders = ref(new EntryCollection<FolderModel>());
const files = ref(new EntryCollection<FileModel>());
const displayView = ref(DisplayState.List);

const selectedFilesCount = computed(() => {
  return files.value.selectedCount() + folders.value.selectedCount();
});

let hotkeys: Hotkeys | null = null;
let callbackId: string | null = null;
let fileUploadModal: HTMLIonModalElement | null = null;
const ownRole: Ref<parsec.WorkspaceRole> = ref(parsec.WorkspaceRole.Reader);

onMounted(async () => {
  const savedData = await storageManager.retrieveComponentData<FoldersPageSavedData>(FOLDERS_PAGE_DATA_KEY);

  if (savedData && savedData.displayState !== undefined) {
    displayView.value = savedData.displayState;
  }
  hotkeys = hotkeyManager.newHotkeys(Groups.Documents);
  hotkeys.add('o', Modifiers.Ctrl, Platforms.Desktop | Platforms.Web, importFiles);
  hotkeys.add('enter', Modifiers.None, Platforms.MacOS, async () => await renameEntries(getSelectedEntries()));
  hotkeys.add('f2', Modifiers.None, Platforms.Windows | Platforms.Linux, async () => await renameEntries(getSelectedEntries()));
  hotkeys.add('i', Modifiers.Ctrl | Modifiers.Shift, Platforms.Desktop, async () => await showDetails(getSelectedEntries()));
  hotkeys.add('c', Modifiers.Ctrl, Platforms.Desktop, async () => await moveEntriesTo(getSelectedEntries()));
  hotkeys.add('l', Modifiers.Ctrl, Platforms.Desktop, async () => await copyLink(getSelectedEntries()));
  hotkeys.add(
    'delete',
    Modifiers.None,
    Platforms.Windows | Platforms.Linux | Platforms.Web,
    async () => await deleteEntries(getSelectedEntries()),
  );
  hotkeys.add('backspace', Modifiers.Ctrl, Platforms.MacOS, async () => await deleteEntries(getSelectedEntries()));
  hotkeys.add('g', Modifiers.Ctrl, Platforms.Desktop, async () => {
    displayView.value = displayView.value === DisplayState.Grid ? DisplayState.List : DisplayState.Grid;
  });

  ownRole.value = await parsec.getWorkspaceRole(getWorkspaceId());
  callbackId = await importManager.registerCallback(onFileImportState);
  const path = getDocumentPath();
  await setCurrentPath(path);
});

onUnmounted(async () => {
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
  if (callbackId) {
    importManager.removeCallback(callbackId);
  }
  routeWatchCancel();
});

async function setCurrentPath(path: parsec.FsPath): Promise<void> {
  if (path === '') {
    return;
  }
  let selectFile: parsec.EntryName | null = null;
  const result = await parsec.entryStat(currentPath.value);
  if (result.ok) {
    if (result.value.isFile()) {
      currentPath.value = await parsec.Path.parent(path);
      selectFile = await parsec.Path.filename(path);
    } else {
      currentPath.value = path;
    }
  }
  await listFolder(selectFile);
}

async function onDisplayStateChange(): Promise<void> {
  await storageManager.storeComponentData<FoldersPageSavedData>(FOLDERS_PAGE_DATA_KEY, { displayState: displayView.value });
}

function onSortChange(event: MsSorterChangeEvent): void {
  folders.value.sort(event.option.key, event.sortByAsc);
  files.value.sort(event.option.key, event.sortByAsc);
}

async function onFileImportState(state: ImportState, importData?: ImportData, stateData?: StateData): Promise<void> {
  if (fileUploadModal && state === ImportState.FileAdded) {
    await fileUploadModal?.dismiss();
  }
  if (!importData) {
    return;
  }
  if (state === ImportState.FileAdded) {
    fileImports.value.push({ data: importData, progress: 0 });
  } else if (state === ImportState.FileProgress) {
    const index = fileImports.value.findIndex((item) => item.data.id === importData.id);
    if (index !== -1) {
      fileImports.value[index].progress = (stateData as FileProgressStateData).progress;
    }
  } else if (
    [ImportState.FileImported, ImportState.Cancelled, ImportState.CreateFailed, ImportState.WriteError, ImportState].includes(state)
  ) {
    const index = fileImports.value.findIndex((item) => item.data.id === importData.id);
    if (index !== -1) {
      fileImports.value.splice(index, 1);
    }
    if (parsec.Path.areSame(importData.path, currentPath.value)) {
      await listFolder();
    }
  }
}

const fileImportsCurrentDir = computed(() => {
  return fileImports.value.filter((item) => parsec.Path.areSame(item.data.path, currentPath.value));
});

async function listFolder(selectFile: parsec.EntryName | null = null): Promise<void> {
  const result = await parsec.entryStat(currentPath.value);
  if (result.ok) {
    const newFolders: FolderModel[] = [];
    const newFiles: FileModel[] = [];
    folderInfo.value = result.value as parsec.EntryStatFolder;
    for (const childName of (result.value as parsec.EntryStatFolder).children) {
      // Excluding files currently being imported
      if (fileImports.value.find((imp) => imp.data.file.name === childName) === undefined) {
        const childPath = await parsec.Path.join(currentPath.value, childName);
        const fileResult = await parsec.entryStat(childPath);
        if (fileResult.ok) {
          if (fileResult.value.isFile()) {
            (fileResult.value as FileModel).isSelected = selectFile && selectFile === fileResult.value.name ? true : false;
            newFiles.push(fileResult.value as FileModel);
          } else {
            (fileResult.value as FolderModel).isSelected = false;
            newFolders.push(fileResult.value as FolderModel);
          }
        }
      }
      folders.value.replace(newFolders);
      files.value.replace(newFiles);
    }
  } else {
    informationManager.present(
      new Information({
        message: translate('FoldersPage.errors.listFailed', {
          path: currentPath.value,
        }),
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
      params: { workspaceHandle: getWorkspaceHandle() },
      query: { documentPath: newPath, workspaceId: getWorkspaceId() },
    });
  }
}

async function createFolder(): Promise<void> {
  hotkeyManager.disableGroup(Groups.Documents);
  const folderName = await getTextInputFromUser({
    title: translate('FoldersPage.CreateFolderModal.title'),
    trim: true,
    validator: entryNameValidator,
    inputLabel: translate('FoldersPage.CreateFolderModal.label'),
    placeholder: translate('FoldersPage.CreateFolderModal.placeholder'),
    okButtonText: translate('FoldersPage.CreateFolderModal.create'),
  });
  hotkeyManager.enableGroup(Groups.Documents);

  if (!folderName) {
    return;
  }
  const folderPath = await parsec.Path.join(currentPath.value, folderName);
  const result = await parsec.createFolder(folderPath);
  if (!result.ok) {
    informationManager.present(
      new Information({
        message: translate('FoldersPage.errors.createFolderFailed', {
          name: folderName,
        }),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    console.log(`New folder ${folderName} created`);
  }
  await listFolder();
}

async function importFiles(): Promise<void> {
  if (fileUploadModal) {
    return;
  }
  hotkeyManager.disableGroup(Groups.Documents);
  fileUploadModal = await modalController.create({
    component: FileUploadModal,
    cssClass: 'file-upload-modal',
    componentProps: {
      currentPath: currentPath.value.toString(),
      workspaceHandle: getWorkspaceHandle(),
      workspaceId: getWorkspaceId(),
    },
  });
  await fileUploadModal.present();
  await fileUploadModal.onDidDismiss();
  hotkeyManager.enableGroup(Groups.Documents);
  fileUploadModal = null;
}

function getSelectedEntries(): parsec.EntryStat[] {
  return [...folders.value.getSelectedEntries(), ...files.value.getSelectedEntries()];
}

async function deleteEntries(entries: parsec.EntryStat[]): Promise<void> {
  if (entries.length === 0) {
    return;
  } else if (entries.length === 1) {
    const entry = entries[0];
    const title = entry.isFile()
      ? translate('FoldersPage.deleteOneFileQuestionTitle')
      : translate('FoldersPage.deleteOneFolderQuestionTitle');
    const subtitle = entry.isFile()
      ? translate('FoldersPage.deleteOneFileQuestionSubtitle', { name: entry.name })
      : translate('FoldersPage.deleteOneFolderQuestionSubtitle', { name: entry.name });
    hotkeyManager.disableGroup(Groups.Documents);
    const answer = await askQuestion(title, subtitle, {
      yesIsDangerous: true,
      yesText: entry.isFile() ? translate('FoldersPage.deleteOneFileYes') : translate('FoldersPage.deleteOneFolderYes'),
      noText: entry.isFile() ? translate('FoldersPage.deleteOneFileNo') : translate('FoldersPage.deleteOneFolderNo'),
    });
    hotkeyManager.enableGroup(Groups.Documents);

    if (answer === Answer.No) {
      return;
    }
    const path = await parsec.Path.join(currentPath.value, entry.name);
    const result = entry.isFile() ? await parsec.deleteFile(path) : await parsec.deleteFolder(path);
    if (!result.ok) {
      informationManager.present(
        new Information({
          message: translate('FoldersPage.errors.deleteFailed', { name: entry.name }),
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      console.log(`File ${entry.name} deleted`);
    }
  } else {
    hotkeyManager.disableGroup(Groups.Documents);
    const answer = await askQuestion(
      translate('FoldersPage.deleteMultipleQuestionTitle'),
      translate('FoldersPage.deleteMultipleQuestionSubtitle', {
        count: entries.length,
      }),
      {
        yesIsDangerous: true,
        yesText: translate('FoldersPage.deleteMultipleYes', { count: entries.length }),
        noText: translate('FoldersPage.deleteMultipleNo', { count: entries.length }),
      },
    );
    hotkeyManager.enableGroup(Groups.Documents);
    if (answer === Answer.No) {
      return;
    }
    let errorsEncountered = 0;
    for (const entry of entries) {
      const path = await parsec.Path.join(currentPath.value, entry.name);
      const result = entry.isFile() ? await parsec.deleteFile(path) : await parsec.deleteFolder(path);
      if (!result.ok) {
        errorsEncountered += 1;
      }
    }
    if (errorsEncountered > 0) {
      informationManager.present(
        new Information({
          message:
            errorsEncountered === entries.length
              ? translate('FoldersPage.errors.deleteMultipleAllFailed')
              : translate('FoldersPage.errors.deleteMultipleSomeFailed'),
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      console.log(`${entries.length} entries deleted`);
    }
  }
  await listFolder();
}

async function renameEntries(entries: parsec.EntryStat[]): Promise<void> {
  if (entries.length !== 1) {
    return;
  }
  const entry = entries[0];
  const ext = parsec.Path.getFileExtension(entry.name);
  hotkeyManager.disableGroup(Groups.Documents);
  const newName = await getTextInputFromUser({
    title: entry.isFile() ? translate('FoldersPage.RenameModal.fileTitle') : translate('FoldersPage.RenameModal.folderTitle'),
    trim: true,
    validator: entryNameValidator,
    inputLabel: entry.isFile() ? translate('FoldersPage.RenameModal.fileLabel') : translate('FoldersPage.RenameModal.folderLabel'),
    placeholder: entry.isFile()
      ? translate('FoldersPage.RenameModal.filePlaceholder')
      : translate('FoldersPage.RenameModal.folderPlaceholder'),
    okButtonText: translate('FoldersPage.RenameModal.rename'),
    defaultValue: entry.name,
    selectionRange: [0, entry.name.length - (ext.length > 0 ? ext.length + 1 : 0)],
  });
  hotkeyManager.enableGroup(Groups.Documents);

  if (!newName) {
    return;
  }
  const filePath = await parsec.Path.join(currentPath.value, entry.name);
  const result = await parsec.rename(filePath, newName);
  if (!result.ok) {
    informationManager.present(
      new Information({
        message: translate('FoldersPage.errors.renameFailed', { name: entry.name }),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    console.log(`File ${entry.name} renamed to ${newName}`);
  }
  await listFolder();
}

async function copyLink(entries: parsec.EntryStat[]): Promise<void> {
  if (entries.length !== 1) {
    return;
  }
  const entry = entries[0];
  const filePath = await parsec.Path.join(currentPath.value, entry.name);
  const result = await parsec.getPathLink(getWorkspaceId(), filePath);
  if (result.ok) {
    if (!(await writeTextToClipboard(result.value))) {
      informationManager.present(
        new Information({
          message: translate('FoldersPage.linkNotCopiedToClipboard'),
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.present(
        new Information({
          message: translate('FoldersPage.linkCopiedToClipboard'),
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    informationManager.present(
      new Information({
        message: translate('FoldersPage.getLinkError', { reason: result.error.tag }),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

async function moveEntriesTo(entries: parsec.EntryStat[]): Promise<void> {
  if (entries.length === 0) {
    return;
  }
  hotkeyManager.disableGroup(Groups.Documents);
  const folder = await selectFolder({
    title: translate('FoldersPage.moveSelectFolderTitle', { count: entries.length }, entries.length),
    startingPath: currentPath.value,
    workspaceId: getWorkspaceId(),
  });
  hotkeyManager.enableGroup(Groups.Documents);
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
          message: translate('FoldersPage.errors.moveOneFailed', { name: entries[0].name }),
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.present(
        new Information({
          message:
            errorCount === entries.length
              ? translate('FoldersPage.errors.moveMultipleAllFailed')
              : translate('FoldersPage.errors.moveMultipleSomeFailed'),
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    informationManager.present(
      new Information({
        message: translate('FoldersPage.moveSuccess', { count: entries.length }, entries.length),
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  }
  await listFolder();
}

async function showDetails(entries: parsec.EntryStat[]): Promise<void> {
  if (entries.length !== 1) {
    return;
  }
  const entry = entries[0];
  hotkeyManager.disableGroup(Groups.Documents);
  const modal = await modalController.create({
    component: FileDetailsModal,
    cssClass: 'file-details-modal',
    componentProps: {
      entry: entry,
      path: await parsec.Path.join(currentPath.value, entry.name),
    },
  });
  await modal.present();
  await modal.onWillDismiss();
  hotkeyManager.enableGroup(Groups.Documents);
}

async function copyEntries(entries: parsec.EntryStat[]): Promise<void> {
  if (entries.length === 0) {
    return;
  }
  hotkeyManager.disableGroup(Groups.Documents);
  const folder = await selectFolder({
    title: translate('FoldersPage.copySelectFolderTitle', { count: entries.length }, entries.length),
    subtitle: translate('FoldersPage.copySelectFolderSubtitle', { location: currentPath.value }),
    startingPath: currentPath.value,
    workspaceId: getWorkspaceId(),
  });
  hotkeyManager.enableGroup(Groups.Documents);
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
          message: translate('FoldersPage.errors.copyOneFailed', { name: entries[0].name }),
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.present(
        new Information({
          message:
            errorCount === entries.length
              ? translate('FoldersPage.errors.copyMultipleAllFailed')
              : translate('FoldersPage.errors.copyMultipleSomeFailed'),
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    informationManager.present(
      new Information({
        message: translate('FoldersPage.copySuccess', {}, entries.length),
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
  hotkeyManager.disableGroup(Groups.Documents);
  console.log('Show history', entries[0]);
  hotkeyManager.enableGroup(Groups.Documents);
}

async function openEntries(entries: parsec.EntryStat[]): Promise<void> {
  if (entries.length !== 1) {
    return;
  }
  if (parsec.isWeb()) {
    hotkeyManager.disableGroup(Groups.Documents);
    await informationManager.present(
      new Information({
        message: translate('FoldersPage.open.unavailableOnWeb'),
        level: InformationLevel.Warning,
      }),
      PresentationMode.Modal,
    );
    hotkeyManager.enableGroup(Groups.Documents);
    return;
  }
  const entry = entries[0];
  const result = await parsec.getAbsolutePath(getWorkspaceHandle(), entry);

  if (!result.ok) {
    await informationManager.present(
      new Information({
        message: entry.isFile()
          ? translate('FoldersPage.open.fileFailed', { name: entry.name })
          : translate('FoldersPage.open.folderFailed', { name: entry.name }),
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
  } else {
    window.electronAPI.openFile(result.value);
  }
}

async function openEntryContextMenu(entry: parsec.EntryStat, event: Event, onFinished?: () => void): Promise<void> {
  hotkeyManager.disableGroup(Groups.Documents);
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
  hotkeyManager.enableGroup(Groups.Documents);

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

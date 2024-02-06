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
              {{ $t('FoldersPage.itemCount', { count: children.length }, children.length) }}
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

          <ms-grid-list-toggle v-model="displayView" />
        </div>
      </ms-action-bar>
      <div class="folder-container scroll">
        <div v-if="children.length === 0">
          {{ $t('FoldersPage.emptyFolder') }}
        </div>

        <div v-if="children.length && displayView === DisplayState.List">
          <ion-list class="list">
            <ion-list-header
              class="folder-list-header"
              lines="full"
            >
              <ion-label class="folder-list-header__label ion-text-nowrap label-selected">
                <ion-checkbox
                  class="checkbox"
                  @ion-change="selectAllFiles($event.detail.checked)"
                  v-model="allFilesSelected"
                />
              </ion-label>
              <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-name">
                {{ $t('FoldersPage.listDisplayTitles.name') }}
              </ion-label>
              <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-updatedBy">
                {{ $t('FoldersPage.listDisplayTitles.updatedBy') }}
              </ion-label>
              <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-lastUpdate">
                {{ $t('FoldersPage.listDisplayTitles.lastUpdate') }}
              </ion-label>
              <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-size">
                {{ $t('FoldersPage.listDisplayTitles.size') }}
              </ion-label>
              <ion-label class="folder-list-header__label cell-title ion-text-nowrap label-space" />
            </ion-list-header>
            <file-list-item
              v-for="child in files"
              :key="child.id"
              :file="child"
              :show-checkbox="selectedFilesCount > 0 || allFilesSelected"
              @click="onFileClick"
              @menu-click="openFileContextMenu"
              @select="onFileSelect"
              ref="fileListItemRefs"
            />
            <file-list-item-importing
              v-for="fileImport in fileImportsCurrentDir"
              :key="fileImport.data.id"
              :data="fileImport.data"
              :progress="fileImport.progress"
            />
          </ion-list>
        </div>
        <div
          v-if="children.length && displayView === DisplayState.Grid"
          class="folders-container-grid"
        >
          <file-card
            class="folder-grid-item"
            v-for="child in files"
            :key="child.id"
            :file="child"
            :show-checkbox="selectedFilesCount > 0"
            @click="onFileClick"
            @menu-click="openFileContextMenu"
            ref="fileGridItemRefs"
          />
          <file-card-importing
            v-for="fileImport in fileImportsCurrentDir"
            :key="fileImport.data.id"
            :data="fileImport.data"
            :progress="fileImport.progress"
          />
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
import FileCard from '@/components/files/FileCard.vue';
import FileListItem from '@/components/files/FileListItem.vue';
import * as parsec from '@/parsec';

import { MsOptions, MsSorter, MsSorterChangeEvent } from '@/components/core';
import FileCardImporting from '@/components/files/FileCardImporting.vue';
import FileListItemImporting from '@/components/files/FileListItemImporting.vue';
import { Routes, getDocumentPath, getWorkspaceHandle, getWorkspaceId, navigateTo, watchRoute } from '@/router';
import { FileProgressStateData, ImportData, ImportManager, ImportManagerKey, ImportState, StateData } from '@/services/importManager';
import { Information, InformationKey, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { translate } from '@/services/translation';
import FileContextMenu, { FileAction } from '@/views/files/FileContextMenu.vue';
import FileDetailsModal from '@/views/files/FileDetailsModal.vue';
import FileUploadModal from '@/views/files/FileUploadModal.vue';
import {
  IonCheckbox,
  IonContent,
  IonLabel,
  IonList,
  IonListHeader,
  IonPage,
  IonText,
  modalController,
  popoverController,
} from '@ionic/vue';
import { arrowRedo, copy, document, folderOpen, informationCircle, link, pencil, trashBin } from 'ionicons/icons';
import { Ref, computed, inject, onMounted, onUnmounted, ref } from 'vue';

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const informationManager: InformationManager = inject(InformationKey)!;
const fileListItemRefs: Ref<(typeof FileListItem)[]> = ref([]);
const fileGridItemRefs: Ref<(typeof FileCard)[]> = ref([]);
const allFilesSelected = ref(false);

enum SortProperty {
  Name,
  Size,
  LastUpdate,
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

interface FileImport {
  data: ImportData;
  progress: number;
}

const fileImports: Ref<Array<FileImport>> = ref([]);

const routeWatchCancel = watchRoute(async () => {
  const newPath = getDocumentPath();
  if (newPath === '') {
    return;
  }
  if (newPath !== currentPath.value) {
    currentPath.value = newPath;
  }
  if (currentPath.value) {
    await listFolder();
  }
  ownRole.value = await parsec.getWorkspaceRole(getWorkspaceId());
});
const currentPath = ref('/');
const folderInfo: Ref<parsec.EntryStatFolder | null> = ref(null);
const children: Ref<Array<parsec.EntryStat>> = ref([]);
const displayView = ref(DisplayState.List);
const selectedFilesCount = computed(() => {
  return getSelectedEntries().length;
});
const importManager: ImportManager = inject(ImportManagerKey)!;
let callbackId: string | null = null;
let fileUploadModal: HTMLIonModalElement | null = null;
const ownRole: Ref<parsec.WorkspaceRole> = ref(parsec.WorkspaceRole.Reader);
const sortProp = ref(SortProperty.Name);
const sortByAsc = ref(true);

const files = computed(() => {
  return children.value.slice().sort((item1, item2) => {
    // Because the difference between item1 and item2 will always be -1, 0 or 1, by setting
    // a folder with a score of 3 by default, we're ensuring that it will always be on top
    // of the list.
    const item1Score = item1.isFile() ? 3 : 0;
    const item2Score = item2.isFile() ? 3 : 0;
    let diff = 0;

    if (sortProp.value === SortProperty.Name) {
      diff = sortByAsc.value ? item2.name.localeCompare(item1.name) : item1.name.localeCompare(item2.name);
    } else if (sortProp.value === SortProperty.LastUpdate) {
      diff = sortByAsc.value ? (item1.updated > item2.updated ? 1 : 0) : item2.updated > item1.updated ? 1 : 0;
    } else if (sortProp.value === SortProperty.Size) {
      const size1 = item1.isFile() ? (item1 as parsec.EntryStatFile).size : 0;
      const size2 = item1.isFile() ? (item2 as parsec.EntryStatFile).size : 0;
      diff = sortByAsc.value ? (size1 < size2 ? 1 : 0) : size2 < size1 ? 1 : 0;
    }
    return item1Score - item2Score - diff;
  });
});

onMounted(async () => {
  ownRole.value = await parsec.getWorkspaceRole(getWorkspaceId());
  callbackId = await importManager.registerCallback(onFileImportState);
  const path = getDocumentPath();
  if (path !== '') {
    currentPath.value = path;
  }
  await listFolder();
});

onUnmounted(async () => {
  if (callbackId) {
    importManager.removeCallback(callbackId);
  }
  routeWatchCancel();
});

function onSortChange(event: MsSorterChangeEvent): void {
  sortProp.value = event.option.key;
  sortByAsc.value = event.sortByAsc;
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

async function listFolder(): Promise<void> {
  const result = await parsec.entryStat(currentPath.value);
  if (result.ok) {
    folderInfo.value = result.value as parsec.EntryStatFolder;
    children.value = [];
    allFilesSelected.value = false;
    for (const childName of (result.value as parsec.EntryStatFolder).children) {
      // Excluding files currently being imported
      if (fileImports.value.find((imp) => imp.data.file.name === childName) === undefined) {
        const childPath = await parsec.Path.join(currentPath.value, childName);
        const fileResult = await parsec.entryStat(childPath);
        if (fileResult.ok) {
          children.value.push(fileResult.value);
        }
      }
    }
  } else {
    informationManager.present(
      new Information({
        message: translate('FoldersPage.errors.listFailed.message', {
          path: currentPath.value,
        }),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

function onFileSelect(_file: parsec.EntryStat, _selected: boolean): void {
  if (selectedFilesCount.value === 0) {
    allFilesSelected.value = false;
    selectAllFiles(false);
  }
  // check global checkbox if all files are selected
  if (selectedFilesCount.value === children.value.length) {
    allFilesSelected.value = true;
  } else {
    allFilesSelected.value = false;
  }
}

async function onFileClick(_event: Event, file: parsec.EntryStat): Promise<void> {
  if (!file.isFile()) {
    const newPath = await parsec.Path.join(currentPath.value, file.name);
    navigateTo(Routes.Documents, {
      params: { workspaceHandle: getWorkspaceHandle() },
      query: { path: newPath, workspaceId: getWorkspaceId() },
    });
  }
}

async function createFolder(): Promise<void> {
  const folderName = await getTextInputFromUser({
    title: translate('FoldersPage.CreateFolderModal.title'),
    trim: true,
    validator: entryNameValidator,
    inputLabel: translate('FoldersPage.CreateFolderModal.label'),
    placeholder: translate('FoldersPage.CreateFolderModal.placeholder'),
    okButtonText: translate('FoldersPage.CreateFolderModal.create'),
  });

  if (!folderName) {
    return;
  }
  const folderPath = await parsec.Path.join(currentPath.value, folderName);
  const result = await parsec.createFolder(folderPath);
  if (!result.ok) {
    informationManager.present(
      new Information({
        message: translate('FoldersPage.errors.createFolderFailed.message', {
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
  fileUploadModal = null;
}

function selectAllFiles(checked: boolean): void {
  for (const item of displayView.value === DisplayState.List ? fileListItemRefs.value : fileGridItemRefs.value) {
    item.isSelected = checked;
    item.showCheckbox = checked;
  }
  allFilesSelected.value = checked;
}

function getSelectedEntries(): parsec.EntryStat[] {
  if (displayView.value === DisplayState.List) {
    return fileListItemRefs.value.filter((item) => item.isSelected).map((item) => item.props.file);
  } else {
    return fileGridItemRefs.value.filter((item) => item.isSelected).map((item) => item.props.file);
  }
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
    const answer = await askQuestion(title, subtitle, {
      yesIsDangerous: true,
      yesText: entry.isFile() ? translate('FoldersPage.deleteOneFileYes') : translate('FoldersPage.deleteOneFolderYes'),
      noText: entry.isFile() ? translate('FoldersPage.deleteOneFileNo') : translate('FoldersPage.deleteOneFolderNo'),
    });

    if (answer === Answer.No) {
      return;
    }
    const path = await parsec.Path.join(currentPath.value, entry.name);
    const result = entry.isFile() ? await parsec.deleteFile(path) : await parsec.deleteFolder(path);
    if (!result.ok) {
      informationManager.present(
        new Information({
          message: translate('FoldersPage.errors.deleteFailed.message', { name: entry.name }),
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      console.log(`File ${entry.name} deleted`);
    }
  } else {
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
  if (entries.length === 0) {
    return;
  }
  const entry = entries[0];
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
  });

  if (!newName) {
    return;
  }
  const filePath = await parsec.Path.join(currentPath.value, entry.name);
  const result = await parsec.rename(filePath, newName);
  if (!result.ok) {
    informationManager.present(
      new Information({
        message: translate('FoldersPage.errors.renameFailed.message', { name: entry.name }),
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
          message: translate('FoldersPage.linkNotCopiedToClipboard.message'),
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.present(
        new Information({
          message: translate('FoldersPage.linkCopiedToClipboard.message'),
          level: InformationLevel.Info,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    informationManager.present(
      new Information({
        message: translate('FoldersPage.getLinkError.message', { reason: result.error.tag }),
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
  const folder = await selectFolder({
    title: translate('FoldersPage.moveSelectFolderTitle', { count: entries.length }, entries.length),
    startingPath: currentPath.value,
    workspaceId: getWorkspaceId(),
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
          message: translate('FoldersPage.errors.moveOneFailed.message', { name: entries[0].name }),
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
        message: translate('FoldersPage.moveSuccess.message', { count: entries.length }, entries.length),
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
}

async function copyEntries(entries: parsec.EntryStat[]): Promise<void> {
  if (entries.length === 0) {
    return;
  }
  const folder = await selectFolder({
    title: translate('FoldersPage.copySelectFolderTitle', { count: entries.length }, entries.length),
    subtitle: translate('FoldersPage.copySelectFolderSubtitle', { location: currentPath.value }),
    startingPath: currentPath.value,
    workspaceId: getWorkspaceId(),
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
          message: translate('FoldersPage.errors.copyOneFailed.message', { name: entries[0].name }),
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
        message: translate('FoldersPage.copySuccess.message', {}, entries.length),
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
  if (entries.length !== 1) {
    return;
  }
  if (parsec.isWeb()) {
    await informationManager.present(
      new Information({
        message: translate('FoldersPage.open.unavailableOnWeb.message'),
        level: InformationLevel.Warning,
      }),
      PresentationMode.Modal,
    );
    return;
  }
  const entry = entries[0];
  const result = await parsec.getAbsolutePath(getWorkspaceHandle(), entry);

  if (!result.ok) {
    await informationManager.present(
      new Information({
        message: entry.isFile()
          ? translate('FoldersPage.open.fileFailedSubtitle', { name: entry.name })
          : translate('FoldersPage.open.folderFailedSubtitle', { name: entry.name }),
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
  } else {
    window.electronAPI.openFile(result.value);
  }
}

async function openFileContextMenu(event: Event, file: parsec.EntryStat, onFinished?: () => void): Promise<void> {
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
    await fn([file]);
  }
  if (onFinished) {
    onFinished();
  }
}
</script>

<style scoped lang="scss">
.folder-container {
  background-color: white;
}

.folder-list-header {
  &__label {
    padding: 0.75rem 1rem;
  }
  .label-selected {
    display: flex;
    align-items: center;
  }

  .label-space {
    min-width: 4rem;
    flex-grow: 0;
    margin-left: auto;
  }
}

.folders-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5em;
  overflow-y: auto;
}
</style>

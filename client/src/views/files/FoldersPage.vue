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
            :button-label="$t('FoldersPage.createFolder')"
            :icon="folderOpen"
            @click="createFolder()"
          />
          <ms-action-bar-button
            id="button-import"
            :button-label="$t('FoldersPage.import')"
            :icon="document"
            @click="importFiles()"
          />
        </div>
        <div v-else-if="selectedFilesCount === 1">
          <ms-action-bar-button
            id="button-rename"
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
            :button-label="$t('FoldersPage.fileContextMenu.actionMoveTo')"
            :icon="arrowRedo"
            @click="moveEntriesTo(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-makeacopy"
            :button-label="$t('FoldersPage.fileContextMenu.actionMakeACopy')"
            :icon="copy"
            @click="copyEntries(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-delete"
            :button-label="$t('FoldersPage.fileContextMenu.actionDelete')"
            :icon="trashBin"
            @click="deleteEntries(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-delete"
            :button-label="$t('FoldersPage.fileContextMenu.actionDetails')"
            :icon="informationCircle"
            @click="showDetails(getSelectedEntries())"
          />
        </div>
        <div v-else>
          <ms-action-bar-button
            id="button-moveto"
            :button-label="$t('FoldersPage.fileContextMenu.actionMoveTo')"
            :icon="arrowRedo"
            @click="moveEntriesTo(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-makeacopy"
            :button-label="$t('FoldersPage.fileContextMenu.actionMakeACopy')"
            :icon="copy"
            @click="copyEntries(getSelectedEntries())"
          />
          <ms-action-bar-button
            id="button-delete"
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
              <ion-label class="folder-list-header__label label-selected">
                <ion-checkbox
                  class="checkbox"
                  @ion-change="selectAllFiles($event.detail.checked)"
                  v-model="allFilesSelected"
                />
              </ion-label>
              <ion-label class="folder-list-header__label cell-title label-name">
                {{ $t('FoldersPage.listDisplayTitles.name') }}
              </ion-label>
              <ion-label class="folder-list-header__label cell-title label-updatedBy">
                {{ $t('FoldersPage.listDisplayTitles.updatedBy') }}
              </ion-label>
              <ion-label class="folder-list-header__label cell-title label-lastUpdate">
                {{ $t('FoldersPage.listDisplayTitles.lastUpdate') }}
              </ion-label>
              <ion-label class="folder-list-header__label cell-title label-size">
                {{ $t('FoldersPage.listDisplayTitles.size') }}
              </ion-label>
              <ion-label class="folder-list-header__label cell-title label-space" />
            </ion-list-header>
            <file-list-item
              v-for="child in children"
              :key="child.id"
              :file="child"
              :show-checkbox="selectedFilesCount > 0 || allFilesSelected"
              @click="onFileClick"
              @menu-click="openFileContextMenu"
              @select="onFileSelect"
              ref="fileListItemRefs"
            />
          </ion-list>
        </div>
        <div
          v-if="children.length && displayView === DisplayState.Grid"
          class="folders-container-grid"
        >
          <file-card
            class="folder-grid-item"
            v-for="child in children"
            :key="child.id"
            :file="child"
            :show-checkbox="selectedFilesCount > 0"
            @click="onFileClick"
            @menu-click="openFileContextMenu"
            ref="fileGridItemRefs"
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

import { Routes, getDocumentPath, getWorkspaceHandle, getWorkspaceId, navigateTo, watchRoute } from '@/router';
import { ImportData, ImportManager, ImportManagerKey, ImportState, StateData } from '@/services/importManager';
import { Notification, NotificationKey, NotificationLevel, NotificationManager } from '@/services/notificationManager';
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
const notificationManager: NotificationManager = inject(NotificationKey)!;
const fileListItemRefs: Ref<(typeof FileListItem)[]> = ref([]);
const fileGridItemRefs: Ref<(typeof FileCard)[]> = ref([]);
const allFilesSelected = ref(false);

const routeWatchCancel = watchRoute(async () => {
  const newPath = getDocumentPath();
  if (newPath !== currentPath.value) {
    currentPath.value = newPath;
  }
  if (currentPath.value) {
    await listFolder();
  }
});
const currentPath = ref('/');
const folderInfo: Ref<parsec.EntryStatFolder | null> = ref(null);
const children: Ref<Array<parsec.EntryStat>> = ref([]);
const displayView = ref(DisplayState.List);
const selectedFilesCount = computed(() => {
  return getSelectedEntries().length;
});
const importManager = inject(ImportManagerKey) as ImportManager;
let callbackId: string | null = null;
let fileUploadModal: HTMLIonModalElement | null = null;

onMounted(async () => {
  callbackId = await importManager.registerCallback(onFileImportState);
  await listFolder();
});

onUnmounted(async () => {
  if (callbackId) {
    importManager.removeCallback(callbackId);
  }
  routeWatchCancel();
});

async function onFileImportState(state: ImportState, _importData?: ImportData, _stateData?: StateData): Promise<void> {
  if (fileUploadModal && state === ImportState.FileAdded) {
    await fileUploadModal?.dismiss();
  }
}

async function listFolder(): Promise<void> {
  const result = await parsec.entryStat(currentPath.value);
  if (result.ok) {
    folderInfo.value = result.value as parsec.EntryStatFolder;
    children.value = [];
    allFilesSelected.value = false;
    for (const childName of (result.value as parsec.EntryStatFolder).children) {
      const childPath = await parsec.Path.join(currentPath.value, childName);
      const fileResult = await parsec.entryStat(childPath);
      if (fileResult.ok) {
        children.value.push(fileResult.value);
      }
    }
  } else {
    notificationManager.showToast(
      new Notification({
        title: translate('FoldersPage.errors.listFailed.title'),
        message: translate('FoldersPage.errors.listFailed.message', {
          path: currentPath.value,
        }),
        level: NotificationLevel.Error,
      }),
    );
  }
}

function onFileSelect(_file: parsec.EntryStat, _selected: boolean): void {
  console.log('File Selected');
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
    notificationManager.showToast(
      new Notification({
        title: translate('FoldersPage.errors.createFolderFailed.title'),
        message: translate('FoldersPage.errors.createFolderFailed.message', {
          name: folderName,
        }),
        level: NotificationLevel.Error,
      }),
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
      notificationManager.showToast(
        new Notification({
          title: translate('FoldersPage.errors.deleteFailed.title'),
          message: translate('FoldersPage.errors.deleteFailed.message', { name: entry.name }),
          level: NotificationLevel.Error,
        }),
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
      notificationManager.showToast(
        new Notification({
          title: translate('FoldersPage.errors.deleteMultipleFailed'),
          message:
            errorsEncountered === entries.length
              ? translate('FoldersPage.errors.deleteMultipleAllFailed')
              : translate('FoldersPage.errors.deleteMultipleSomeFailed'),
          level: NotificationLevel.Error,
        }),
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
    notificationManager.showToast(
      new Notification({
        title: translate('FoldersPage.errors.renameFailed.title'),
        message: translate('FoldersPage.errors.renameFailed.message', { name: entry.name }),
        level: NotificationLevel.Error,
      }),
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
      notificationManager.showToast(
        new Notification({
          title: translate('FoldersPage.linkNotCopiedToClipboard.title'),
          message: translate('FoldersPage.linkNotCopiedToClipboard.message'),
          level: NotificationLevel.Error,
        }),
      );
    } else {
      notificationManager.showToast(
        new Notification({
          title: translate('FoldersPage.linkCopiedToClipboard.title'),
          message: translate('FoldersPage.linkCopiedToClipboard.message'),
          level: NotificationLevel.Info,
        }),
      );
    }
  } else {
    notificationManager.showToast(
      new Notification({
        title: translate('FoldersPage.getLinkError.title'),
        message: translate('FoldersPage.getLinkError.message', { reason: result.error.tag }),
        level: NotificationLevel.Error,
      }),
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
      notificationManager.showToast(
        new Notification({
          title: translate('FoldersPage.errors.moveOneFailed.title'),
          message: translate('FoldersPage.errors.moveOneFailed.message', { name: entries[0].name }),
          level: NotificationLevel.Error,
        }),
      );
    } else {
      notificationManager.showToast(
        new Notification({
          title: translate('FoldersPage.errors.moveMultipleFailed'),
          message:
            errorCount === entries.length
              ? translate('FoldersPage.errors.moveMultipleAllFailed')
              : translate('FoldersPage.errors.moveMultipleSomeFailed'),
          level: NotificationLevel.Error,
        }),
      );
    }
  } else {
    notificationManager.showToast(
      new Notification({
        title: translate('FoldersPage.moveSuccess.title', {}, entries.length),
        message: translate('FoldersPage.moveSuccess.message', { count: entries.length }, entries.length),
        level: NotificationLevel.Success,
      }),
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
      notificationManager.showToast(
        new Notification({
          title: translate('FoldersPage.errors.copyOneFailed.title'),
          message: translate('FoldersPage.errors.copyOneFailed.message', { name: entries[0].name }),
          level: NotificationLevel.Error,
        }),
      );
    } else {
      notificationManager.showToast(
        new Notification({
          title: translate('FoldersPage.errors.copyMultipleFailed'),
          message:
            errorCount === entries.length
              ? translate('FoldersPage.errors.copyMultipleAllFailed')
              : translate('FoldersPage.errors.copyMultipleSomeFailed'),
          level: NotificationLevel.Error,
        }),
      );
    }
  } else {
    notificationManager.showToast(
      new Notification({
        title: translate('FoldersPage.copySuccess.title', { count: entries.length }, entries.length),
        message: translate('FoldersPage.copySuccess.message', {}, entries.length),
        level: NotificationLevel.Success,
      }),
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
    await notificationManager.showModal(
      new Notification({
        title: translate('FoldersPage.open.unavailableOnWeb.title'),
        message: translate('FoldersPage.open.unavailableOnWeb.message'),
        level: NotificationLevel.Warning,
      }),
    );
    return;
  }
  const entry = entries[0];
  const result = await parsec.getAbsolutePath(getWorkspaceHandle(), entry);

  if (!result.ok) {
    await notificationManager.showModal(
      new Notification({
        title: entry.isFile() ? translate('FoldersPage.open.fileFailedTitle') : translate('FoldersPage.open.folderFailedTitle'),
        message: entry.isFile()
          ? translate('FoldersPage.open.fileFailedSubtitle', { name: entry.name })
          : translate('FoldersPage.open.folderFailedSubtitle', { name: entry.name }),
        level: NotificationLevel.Error,
      }),
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
    min-width: 4rem;
    flex-grow: 0;
    display: flex;
    align-items: center;
    justify-content: end;
  }

  .label-name {
    width: 100%;
    min-width: 11.25rem;
    white-space: nowrap;
    overflow: hidden;
  }

  .label-updatedBy {
    min-width: 11.25rem;
    max-width: 10vw;
    flex-grow: 2;
  }

  .label-lastUpdate {
    min-width: 11.25rem;
    flex-grow: 0;
  }

  .label-size {
    min-width: 11.25rem;
  }

  .label-space {
    min-width: 4rem;
    flex-grow: 0;
    margin-left: auto;
    margin-right: 1rem;
  }
}

.folders-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5em;
  overflow-y: auto;
}
</style>

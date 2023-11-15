<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
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
            id="button-move-to"
            :button-label="$t('FoldersPage.fileContextMenu.actionMoveTo')"
            :icon="arrowRedo"
            @click="moveEntriesTo(getSelectedEntries())"
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
          <ms-grid-list-toggle
            v-model="displayView"
            @update:model-value="resetSelection()"
          />
        </div>
      </ms-action-bar>
      <div class="folder-container">
        <div v-if="children.length === 0">
          {{ $t('FoldersPage.emptyFolder') }}
        </div>

        <div v-if="children.length && displayView === DisplayState.List">
          <ion-list>
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
              <ion-label class="folder-list-header__label label-name">
                {{ $t('FoldersPage.listDisplayTitles.name') }}
              </ion-label>
              <ion-label class="folder-list-header__label label-updatedBy">
                {{ $t('FoldersPage.listDisplayTitles.updatedBy') }}
              </ion-label>
              <ion-label class="folder-list-header__label label-lastUpdate">
                {{ $t('FoldersPage.listDisplayTitles.lastUpdate') }}
              </ion-label>
              <ion-label class="folder-list-header__label label-size">
                {{ $t('FoldersPage.listDisplayTitles.size') }}
              </ion-label>
              <ion-label class="folder-list-header__label label-space" />
            </ion-list-header>
            <file-list-item
              v-for="child in children"
              :key="child.id"
              :file="child"
              :show-checkbox="selectedFilesCount > 0 || allFilesSelected"
              @click="onFileClick"
              @menu-click="openFileContextMenu($event, child)"
              @select="onFileSelect"
              ref="fileItemRefs"
            />
          </ion-list>
        </div>
        <div
          v-if="children.length && displayView === DisplayState.Grid"
          class="folders-container-grid"
        >
          <ion-item
            class="folder-grid-item"
            v-for="child in children"
            :key="child.id"
          >
            <file-card
              :file="child"
              @click="onFileClick"
              @menu-click="openFileContextMenu($event, child)"
            />
          </ion-item>
        </div>
      </div>
      <!-- number of item (selected or not) -->
      <div class="folder-footer">
        <div class="folder-footer__container">
          <ion-text
            class="text title-h5"
            v-if="selectedFilesCount === 0"
          >
            {{ $t('FoldersPage.itemCount', { count: children.length }, children.length) }}
          </ion-text>
          <ion-text
            class="text title-h5"
            v-if="selectedFilesCount > 0"
          >
            {{ $t('FoldersPage.itemSelectedCount', { count: selectedFilesCount }, selectedFilesCount) }}
          </ion-text>
          <ms-action-bar-button
            class="shortcuts-btn"
            id="button-move-to"
            :icon="arrowRedo"
            @click="moveEntriesTo(getSelectedEntries())"
            v-if="selectedFilesCount >= 1"
          />
          <ms-action-bar-button
            class="shortcuts-btn"
            id="button-delete"
            :icon="trashBin"
            @click="deleteEntries(getSelectedEntries())"
            v-if="selectedFilesCount >= 1"
          />
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonItem,
  IonPage,
  IonContent,
  IonList,
  IonListHeader,
  IonLabel,
  IonCheckbox,
  IonText,
  popoverController,
  modalController,
} from '@ionic/vue';
import { folderOpen, document, pencil, link, arrowRedo, trashBin, copy, informationCircle } from 'ionicons/icons';
import { useRoute } from 'vue-router';
import { computed, ref, Ref, inject, watch, onUnmounted, onMounted } from 'vue';
import MsActionBarButton from '@/components/core/ms-action-bar/MsActionBarButton.vue';
import MsGridListToggle from '@/components/core/ms-toggle/MsGridListToggle.vue';
import { DisplayState } from '@/components/core/ms-toggle/MsGridListToggle.vue';
import FileListItem from '@/components/files/FileListItem.vue';
import FileCard from '@/components/files/FileCard.vue';
import FileContextMenu from '@/views/files/FileContextMenu.vue';
import { routerNavigateTo } from '@/router';
import { FileAction } from '@/views/files/FileContextMenu.vue';
import MsActionBar from '@/components/core/ms-action-bar/MsActionBar.vue';
import FileUploadModal from '@/views/files/FileUploadModal.vue';
import { NotificationManager, Notification, NotificationKey, NotificationLevel } from '@/services/notificationManager';
import * as parsec from '@/parsec';
import { useI18n } from 'vue-i18n';
import { getTextInputFromUser } from '@/components/core/ms-modal/MsTextInputModal.vue';
import { entryNameValidator } from '@/common/validators';
import { Answer, askQuestion } from '@/components/core/ms-modal/MsQuestionModal.vue';
import FileDetailsModal from '@/views/files/FileDetailsModal.vue';
import { writeTextToClipboard } from '@/common/clipboard';
import { getPathLink, isWeb, WorkspaceHandle, WorkspaceID } from '@/parsec';
import { ImportManager, ImportManagerKey, StateData, ImportState } from '@/services/importManager';

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const notificationManager: NotificationManager = inject(NotificationKey)!;
const fileItemRefs: Ref<(typeof FileListItem)[]> = ref([]);
const allFilesSelected = ref(false);
const currentRoute = useRoute();
const unwatchRoute = watch(currentRoute, async (newRoute) => {
  if (newRoute.query && newRoute.query.path !== currentPath.value) {
    currentPath.value = newRoute.query.path as string;
  }
  if (currentPath.value) {
    await listFolder();
  }
});
const currentPath = ref('/');
const workspaceHandle = computed(() => {
  return parseInt(currentRoute.params.workspaceHandle as string) as WorkspaceHandle;
});
const workspaceId = computed(() => {
  return currentRoute.query.workspaceId as WorkspaceID;
});
const folderInfo: Ref<parsec.EntryStatFolder | null> = ref(null);
const children: Ref<Array<parsec.EntryStat>> = ref([]);
const selectedFilesCount = computed(() => {
  const count = fileItemRefs.value.filter((item) => item.isSelected).length;
  return count;
});
const displayView = ref(DisplayState.List);
const { t } = useI18n();
const importManager = inject(ImportManagerKey) as ImportManager;
let callbackId: string | null = null;

onMounted(async () => {
  callbackId = await importManager.registerCallback(onFileImportState);
  await listFolder();
});

onUnmounted(async () => {
  if (callbackId) {
    importManager.removeCallback(callbackId);
  }
  unwatchRoute();
});

async function onFileImportState(state: ImportState, data: StateData): Promise<void> {
  console.log(state, data);
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
        message: t('FoldersPage.errors.listFailed', {
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
    routerNavigateTo('folder', { workspaceHandle: workspaceHandle.value }, { path: newPath, workspaceId: currentRoute.query.workspaceId });
  }
}

async function createFolder(): Promise<void> {
  const folderName = await getTextInputFromUser({
    title: t('FoldersPage.CreateFolderModal.title'),
    trim: true,
    validator: entryNameValidator,
    inputLabel: t('FoldersPage.CreateFolderModal.label'),
    placeholder: t('FoldersPage.CreateFolderModal.placeholder'),
    okButtonText: t('FoldersPage.CreateFolderModal.create'),
  });

  if (!folderName) {
    return;
  }
  const folderPath = await parsec.Path.join(currentPath.value, folderName);
  const result = await parsec.createFolder(folderPath);
  if (!result.ok) {
    notificationManager.showToast(
      new Notification({
        message: t('FoldersPage.errors.createFolderFailed', {
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
  const modal = await modalController.create({
    component: FileUploadModal,
    cssClass: 'file-upload-modal',
    componentProps: {
      currentPath: currentPath.value.toString(),
      workspaceHandle: workspaceHandle.value,
      workspaceId: workspaceId.value,
    },
  });
  await modal.present();
  await modal.onWillDismiss();
  await modal.dismiss();
}

function selectAllFiles(checked: boolean): void {
  for (const item of fileItemRefs.value || []) {
    item.isSelected = checked;
    item.showCheckbox = checked;
  }
  allFilesSelected.value = checked;
}

function getSelectedEntries(): parsec.EntryStat[] {
  if (!fileItemRefs.value) {
    return [];
  }
  return fileItemRefs.value.filter((item) => item.isSelected).map((item) => item.props.file);
}

async function deleteEntries(entries: parsec.EntryStat[]): Promise<void> {
  if (entries.length === 0) {
    return;
  } else if (entries.length === 1) {
    const entry = entries[0];
    const title = entry.isFile() ? t('FoldersPage.deleteOneFileQuestionTitle') : t('FoldersPage.deleteOneFolderQuestionTitle');
    const subtitle = entry.isFile()
      ? t('FoldersPage.deleteOneFileQuestionSubtitle', { name: entry.name })
      : t('FoldersPage.deleteOneFolderQuestionSubtitle', { name: entry.name });
    const answer = await askQuestion(title, subtitle);

    if (answer === Answer.No) {
      return;
    }
    const path = await parsec.Path.join(currentPath.value, entry.name);
    const result = entry.isFile() ? await parsec.deleteFile(path) : await parsec.deleteFolder(path);
    if (!result.ok) {
      notificationManager.showToast(
        new Notification({
          message: t('FoldersPage.errors.deleteFailed', { name: entry.name }),
          level: NotificationLevel.Error,
        }),
      );
    } else {
      console.log(`File ${entry.name} deleted`);
    }
  } else {
    const answer = await askQuestion(
      t('FoldersPage.deleteMultipleQuestionTitle'),
      t('FoldersPage.deleteMultipleQuestionSubtitle', {
        count: entries.length,
      }),
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
          message:
            errorsEncountered === entries.length
              ? t('FoldersPage.errors.deleteMultipleAllFailed')
              : t('FoldersPage.errors.deleteMultipleSomeFailed'),
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
    title: entry.isFile() ? t('FoldersPage.RenameModal.fileTitle') : t('FoldersPage.RenameModal.folderTitle'),
    trim: true,
    validator: entryNameValidator,
    inputLabel: entry.isFile() ? t('FoldersPage.RenameModal.fileLabel') : t('FoldersPage.RenameModal.folderLabel'),
    placeholder: entry.isFile() ? t('FoldersPage.RenameModal.filePlaceholder') : t('FoldersPage.RenameModal.folderPlaceholder'),
    okButtonText: t('FoldersPage.RenameModal.rename'),
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
        message: t('FoldersPage.errors.renameFailed', { name: entry.name }),
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
  const result = await getPathLink(workspaceId.value, filePath);
  if (result.ok) {
    if (!(await writeTextToClipboard(result.value))) {
      notificationManager.showToast(
        new Notification({
          message: t('FoldersPage.linkNotCopiedToClipboard'),
          level: NotificationLevel.Error,
        }),
      );
    } else {
      notificationManager.showToast(
        new Notification({
          message: t('FoldersPage.linkCopiedToClipboard'),
          level: NotificationLevel.Info,
        }),
      );
    }
  } else {
    notificationManager.showToast(
      new Notification({
        message: t('FoldersPage.getLinkError', { reason: result.error.tag }),
        level: NotificationLevel.Error,
      }),
    );
  }
}

async function moveEntriesTo(entries: parsec.EntryStat[]): Promise<void> {
  console.log('Move entries', entries);
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
  console.log('Make a copy', entries);
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
  if (isWeb()) {
    await notificationManager.showModal(
      new Notification({
        message: t('FoldersPage.open.unavailableOnWeb'),
        level: NotificationLevel.Warning,
      }),
    );
    return;
  }
  const entry = entries[0];
  const result = await parsec.getAbsolutePath(workspaceHandle.value, entry);

  if (!result.ok) {
    await notificationManager.showModal(
      new Notification({
        title: entry.isFile() ? t('FoldersPage.open.fileFailedTitle') : t('FoldersPage.open.folderFailedTitle'),
        message: entry.isFile()
          ? t('FoldersPage.open.fileFailedSubtitle', { name: entry.name })
          : t('FoldersPage.open.folderFailedSubtitle', { name: entry.name }),
        level: NotificationLevel.Error,
      }),
    );
  } else {
    window.electronAPI.openFile(result.value);
  }
}

async function openFileContextMenu(event: Event, file: parsec.EntryStat): Promise<void> {
  const popover = await popoverController.create({
    component: FileContextMenu,
    cssClass: 'file-context-menu',
    event: event,
    translucent: true,
    showBackdrop: false,
    dismissOnSelect: true,
    reference: 'event',
  });
  await popover.present();

  const { data } = await popover.onDidDismiss();

  if (!data) {
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
}

async function resetSelection(): Promise<void> {
  fileItemRefs.value = [];
  allFilesSelected.value = false;
}
</script>

<style scoped lang="scss">
.folder-container {
  margin: 7em 2em 2em;
  background-color: white;
}

.folder-list-header {
  color: var(--parsec-color-light-secondary-grey);
  font-weight: 600;
  padding-inline-start: 0;

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

.right-side {
  margin-left: auto;
  display: flex;
}
</style>

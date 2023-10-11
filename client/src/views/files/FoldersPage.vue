<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <ms-action-bar
        id="folders-ms-action-bar"
      >
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
            @click="actionOnSelectedFile(renameFile)"
          />
          <ms-action-bar-button
            id="button-copy-link"
            :button-label="$t('FoldersPage.fileContextMenu.actionCopyLink')"
            :icon="link"
            @click="actionOnSelectedFile(copyLink)"
          />
          <ms-action-bar-button
            id="button-move-to"
            :button-label="$t('FoldersPage.fileContextMenu.actionMoveTo')"
            :icon="arrowRedo"
            @click="actionOnSelectedFile(moveTo)"
          />
          <ms-action-bar-button
            id="button-delete"
            :button-label="$t('FoldersPage.fileContextMenu.actionDelete')"
            :icon="trashBin"
            @click="actionOnSelectedFile(deleteFile)"
          />
          <ms-action-bar-button
            id="button-delete"
            :button-label="$t('FoldersPage.fileContextMenu.actionDetails')"
            :icon="informationCircle"
            @click="actionOnSelectedFile(showDetails)"
          />
        </div>
        <div v-else>
          <ms-action-bar-button
            id="button-moveto"
            :button-label="$t('FoldersPage.fileContextMenu.actionMoveTo')"
            :icon="arrowRedo"
            @click="actionOnSelectedFiles(moveTo)"
          />
          <ms-action-bar-button
            id="button-makeacopy"
            :button-label="$t('FoldersPage.fileContextMenu.actionMakeACopy')"
            :icon="copy"
            @click="actionOnSelectedFiles(makeACopy)"
          />
          <ms-action-bar-button
            id="button-delete"
            :button-label="$t('FoldersPage.fileContextMenu.actionDelete')"
            :icon="trashBin"
            @click="actionOnSelectedFiles(deleteFile)"
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
            {{ $t('FoldersPage.itemCount', { count: children.length }, children.length)
            }}
          </ion-text>
          <ion-text
            class="text title-h5"
            v-if="selectedFilesCount !== 0"
          >
            {{ $t('FoldersPage.itemSelectedCount', { count: selectedFilesCount }, selectedFilesCount) }}
          </ion-text>
          <ms-action-bar-button
            class="shortcuts-btn"
            id="button-move-to"
            :icon="arrowRedo"
            @click="actionOnSelectedFile(moveTo)"
            v-if="selectedFilesCount >= 1"
          />
          <ms-action-bar-button
            class="shortcuts-btn"
            id="button-delete"
            :icon="trashBin"
            @click="actionOnSelectedFile(deleteFile)"
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
import {
  folderOpen,
  document,
  pencil,
  link,
  arrowRedo,
  trashBin,
  copy,
  informationCircle,
} from 'ionicons/icons';
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
import { NotificationCenter, Notification, NotificationKey, NotificationLevel } from '@/services/notificationCenter';
import * as parsec from '@/parsec';
import { join as pathJoin } from '@/common/path';
import { useI18n } from 'vue-i18n';
import { getTextInputFromUser } from '@/components/core/ms-modal/MsTextInputModal.vue';
import { entryNameValidator } from '@/common/validators';
import { Answer, askQuestion } from '@/components/core/ms-modal/MsQuestionModal.vue';

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const notificationCenter: NotificationCenter = inject(NotificationKey)!;
const fileItemRefs: Ref<typeof FileListItem[]> = ref([]);
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
  return currentRoute.params.workspaceHandle;
});
const folderInfo: Ref<parsec.EntryStatFolder | null> = ref(null) ;
const children: Ref<Array<parsec.EntryStat>> = ref([]);
const selectedFilesCount = computed(() => {
  const count = fileItemRefs.value.filter((item) => item.isSelected).length;
  return count;
});
const displayView = ref(DisplayState.List);
const { t } = useI18n();

onMounted(async () => {
  await listFolder();
});

onUnmounted(async () => {
  unwatchRoute();
});

async function listFolder(): Promise<void> {
  const result = await parsec.entryStat(currentPath.value);
  if (result.ok) {
    folderInfo.value = result.value as parsec.EntryStatFolder;
    children.value = [];
    for (const childName of (result.value as parsec.EntryStatFolder).children) {
      const childPath = pathJoin(currentPath.value, childName);
      const fileResult = await parsec.entryStat(childPath);
      if (fileResult.ok) {
        children.value.push(fileResult.value);
      }
    }
  } else {
    notificationCenter.showToast(new Notification({
      message: t('FoldersPage.errors.listFailed', {path: currentPath.value}),
      level: NotificationLevel.Error,
    }));
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

function onFileClick(_event: Event, file: parsec.EntryStat): void {
  if (!file.isFile()) {
    const newPath = pathJoin(currentPath.value, file.name);
    routerNavigateTo('folder', {workspaceHandle: workspaceHandle.value}, {path: newPath, workspaceId: currentRoute.query.workspaceId});
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
  const folderPath = pathJoin(currentPath.value, folderName);
  const result = await parsec.createFolder(folderPath);
  if (!result.ok) {
    notificationCenter.showToast(new Notification({
      message: t('FoldersPage.errors.createFolderFailed', {name: folderName}),
      level: NotificationLevel.Error,
    }));
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
    },
  });
  await modal.present();
  await modal.onWillDismiss();
}

function selectAllFiles(checked: boolean): void {
  for (const item of fileItemRefs.value || []) {
    item.isSelected = checked;
    if (checked) {
      item.showCheckbox = true;
    } else {
      item.showCheckbox = false;
    }
  }
}

async function actionOnSelectedFile(action: (file: parsec.EntryStat) => Promise<void>): Promise<void> {
  const selected = fileItemRefs.value.find((item) => item.isSelected);

  if (!selected) {
    return;
  }
  await action(selected.props.file);
}

async function actionOnSelectedFiles(action: (file: parsec.EntryStat) => Promise<void>): Promise<void> {
  const selected = fileItemRefs.value.filter((item) => item.isSelected);

  for (const item of selected) {
    await action(item.props.file);
  }
}

async function renameFile(file: parsec.EntryStat): Promise<void> {
  const newName = await getTextInputFromUser({
    title: file.isFile() ? t('FoldersPage.RenameModal.fileTitle') : t('FoldersPage.RenameModal.folderTitle'),
    trim: true,
    validator: entryNameValidator,
    inputLabel: file.isFile() ? t('FoldersPage.RenameModal.fileLabel') : t('FoldersPage.RenameModal.folderLabel'),
    placeholder: file.isFile() ? t('FoldersPage.RenameModal.filePlaceholder') : t('FoldersPage.RenameModal.folderPlaceholder'),
    okButtonText: t('FoldersPage.RenameModal.rename'),
    defaultValue: file.name,
  });

  if (!newName) {
    return;
  }
  const filePath = pathJoin(currentPath.value, file.name);
  const result = await parsec.rename(filePath, newName);
  if (!result.ok) {
    notificationCenter.showToast(new Notification({
      message: t('FoldersPage.errors.renameFailed', {name: file.name}),
      level: NotificationLevel.Error,
    }));
  } else {
    console.log(`File ${file.name} renamed to ${newName}`);
  }
  await listFolder();
}

async function copyLink(file: parsec.EntryStat): Promise<void> {
  console.log('Get file link', file);
}

async function deleteFile(file: parsec.EntryStat): Promise<void> {
  const answer = await askQuestion(
    file.isFile() ?
      t('FoldersPage.deleteOneFileQuestion', {name: file.name}) :
      t('FoldersPage.deleteOneFolderQuestion', {name: file.name}),
  );

  if (answer === Answer.No) {
    return;
  }
  const filePath = pathJoin(currentPath.value, file.name);
  const result = file.isFile() ? await parsec.deleteFile(filePath) : await parsec.deleteFolder(filePath);
  if (!result.ok) {
    notificationCenter.showToast(new Notification({
      message: t('FoldersPage.errors.deleteFailed', {name: file.name}),
      level: NotificationLevel.Error,
    }));
  } else {
    console.log(`File ${file.name} deleted`);
  }
  await listFolder();
}

async function moveTo(file: parsec.EntryStat): Promise<void> {
  console.log('Move file', file);
}

async function showDetails(file: parsec.EntryStat): Promise<void> {
  console.log('Show details', file);
}

async function makeACopy(file: parsec.EntryStat): Promise<void> {
  console.log('Make a copy', file);
}

async function download(file: parsec.EntryStat): Promise<void> {
  console.log('Download', file);
}

async function showHistory(file: parsec.EntryStat): Promise<void> {
  console.log('Show history', file);
}

async function openInExplorer(file: parsec.EntryStat): Promise<void> {
  console.log('Open in explorer', file);
}

async function openFileContextMenu(event: Event, file: parsec.EntryStat): Promise<void> {
  console.log('File Menu');
  const popover = await popoverController
    .create({
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

  const actions = new Map<FileAction, (file: parsec.EntryStat) => Promise<void>>([
    [FileAction.Rename, renameFile],
    [FileAction.MoveTo, moveTo],
    [FileAction.MakeACopy, makeACopy],
    [FileAction.OpenInExplorer, openInExplorer],
    [FileAction.ShowHistory, showHistory],
    [FileAction.Download, download],
    [FileAction.ShowDetails, showDetails],
    [FileAction.CopyLink, copyLink],
    [FileAction.Delete, deleteFile],
  ]);

  const fn = actions.get(data.action);
  if (fn) {
    await fn(file);
  }
}

async function resetSelection(): Promise<void> {
  fileItemRefs.value = [];
  allFilesSelected.value = false;
}
</script>

<style scoped lang="scss">
.folder-container {
  margin: 2em;
  background-color: white;
}

.folder-list-header {
  color: var(--parsec-color-light-secondary-grey);
  font-weight: 600;
  padding-inline-start: 0;

  &__label {
    padding: .75rem 1rem;
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

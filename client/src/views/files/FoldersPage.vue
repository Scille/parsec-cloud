<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

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
        <div v-if="displayView === DisplayState.List">
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
              v-for="child in folderInfo.children"
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
          v-else
          class="folders-container-grid"
        >
          <ion-item
            class="folder-grid-item"
            v-for="child in folderInfo.children"
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
            {{ $t('FoldersPage.itemCount', { count: folderInfo.children.length }, folderInfo.children.length) }}
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
import { computed, ref, Ref } from 'vue';
import { MockFile, pathInfo } from '@/common/mocks';
import MsActionBarButton from '@/components/core/ms-action-bar/MsActionBarButton.vue';
import MsGridListToggle from '@/components/core/ms-toggle/MsGridListToggle.vue';
import { DisplayState } from '@/components/core/ms-toggle/MsGridListToggle.vue';
import FileListItem from '@/components/files/FileListItem.vue';
import FileCard from '@/components/files/FileCard.vue';
import FileContextMenu from '@/views/files/FileContextMenu.vue';
import router from '@/router';
import { FileAction } from '@/views/files/FileContextMenu.vue';
import MsActionBar from '@/components/core/ms-action-bar/MsActionBar.vue';
import FileUploadModal from '@/views/files/FileUploadModal.vue';

const fileItemRefs: Ref<typeof FileListItem[]> = ref([]);
const allFilesSelected = ref(false);
const currentRoute = useRoute();

// Use computed so the variables will automatically update when the route changes
const path = computed(() => {
  return currentRoute.query.path ?? '/';
});
const workspaceId = computed(() => {
  return currentRoute.params.workspaceId;
});
const folderInfo = computed((): MockFile => {
  return pathInfo(path.value.toString());
});
const selectedFilesCount = computed(() => {
  const count = fileItemRefs.value.filter((item) => item.isSelected).length;
  return count;
});
const displayView = ref(DisplayState.List);

function onFileSelect(_file: MockFile, _selected: boolean): void {
  console.log('File Selected');
  if (selectedFilesCount.value === 0) {
    allFilesSelected.value = false;
    selectAllFiles(false);
  }
  // check global checkbox if all files are selected
  if (selectedFilesCount.value === folderInfo.value.children.length) {
    allFilesSelected.value = true;
  } else {
    allFilesSelected.value = false;
  }
}

function onFileClick(_event: Event, file: MockFile): void {
  console.log('File Click');
  if (file.type === 'folder') {
    const newPath = (path.value.toString().endsWith('/')) ? `${path.value}${file.name}` : `${path.value}/${file.name}`;
    router.push({
      name: 'folder',
      params: { deviceId: currentRoute.params.deviceId, workspaceId: workspaceId.value },
      query: { path: newPath },
    });
  }
}

function createFolder(): void {
  console.log('Create folder clicked');
}

async function importFiles(): Promise<void> {
  const modal = await modalController.create({
    component: FileUploadModal,
    cssClass: 'file-upload-modal',
    componentProps: {
      currentPath: path.value.toString(),
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

function actionOnSelectedFile(action: (file: MockFile) => void): void {
  const selected = fileItemRefs.value.find((item) => item.isSelected);

  if (!selected) {
    return;
  }
  action(selected.props.file);
}

function actionOnSelectedFiles(action: (file: MockFile) => void): void {
  const selected = fileItemRefs.value.filter((item) => item.isSelected);

  for (const item of selected) {
    action(item.props.file);
  }
}

function renameFile(file: MockFile): void {
  console.log('Rename file', file);
}

function copyLink(file: MockFile): void {
  console.log('Get file link', file);
}

function deleteFile(file: MockFile): void {
  console.log('Delete file', file);
}

function moveTo(file: MockFile): void {
  console.log('Move file', file);
}

function showDetails(file: MockFile): void {
  console.log('Show details', file);
}

function makeACopy(file: MockFile): void {
  console.log('Make a copy', file);
}

function download(file: MockFile): void {
  console.log('Download', file);
}

function showHistory(file: MockFile): void {
  console.log('Show history', file);
}

function openInExplorer(file: MockFile): void {
  console.log('Open in explorer', file);
}

async function openFileContextMenu(event: Event, file: MockFile): Promise<void> {
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

  const actions = new Map<FileAction, (file: MockFile) => void>([
    [FileAction.Rename, renameFile],
    [FileAction.MoveTo, moveTo],
    [FileAction.MakeACopy, makeACopy],
    [FileAction.OpenInExplorer, openInExplorer],
    [FileAction.ShowHistory, showHistory],
    [FileAction.Download, download],
    [FileAction.ShowDetails, showDetails],
    [FileAction.CopyLink, copyLink],
  ]);

  const fn = actions.get(data.action);
  if (fn) {
    fn(file);
  }
}

function resetSelection(): void {
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

<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <ion-item-divider class="folder-toolbar ion-margin-bottom secondary">
        <div class="contextual-menu">
          <div v-if="selectedFilesCount === 0">
            <button-option
              id="button-new-folder"
              :button-label="$t('FoldersPage.createFolder')"
              :icon="folderOpen"
              @click="createFolder()"
            />
            <button-option
              id="button-import"
              :button-label="$t('FoldersPage.import')"
              :icon="document"
              @click="importFiles()"
            />
          </div>
          <div v-else-if="selectedFilesCount === 1">
            <button-option
              id="button-rename"
              :button-label="$t('FoldersPage.fileContextMenu.actionRename')"
              :icon="pencil"
              @click="actionOnSelectedFile(renameFile)"
            />
            <button-option
              id="button-copy-link"
              :button-label="$t('FoldersPage.fileContextMenu.actionCopyLink')"
              :icon="link"
              @click="actionOnSelectedFile(copyLink)"
            />
            <button-option
              id="button-move-to"
              :button-label="$t('FoldersPage.fileContextMenu.actionMoveTo')"
              :icon="arrowRedo"
              @click="actionOnSelectedFile(moveTo)"
            />
            <button-option
              id="button-delete"
              :button-label="$t('FoldersPage.fileContextMenu.actionDelete')"
              :icon="trashBin"
              @click="actionOnSelectedFile(deleteFile)"
            />
            <button-option
              id="button-delete"
              :button-label="$t('FoldersPage.fileContextMenu.actionDetails')"
              :icon="informationCircle"
              @click="actionOnSelectedFile(showDetails)"
            />
          </div>
          <div v-else>
            <button-option
              id="button-moveto"
              :button-label="$t('FoldersPage.fileContextMenu.actionMoveTo')"
              :icon="arrowRedo"
              @click="actionOnSelectedFiles(moveTo)"
            />
            <button-option
              id="button-makeacopy"
              :button-label="$t('FoldersPage.fileContextMenu.actionMakeACopy')"
              :icon="copy"
              @click="actionOnSelectedFiles(makeACopy)"
            />
            <button-option
              id="button-delete"
              :button-label="$t('FoldersPage.fileContextMenu.actionDelete')"
              :icon="trashBin"
              @click="actionOnSelectedFiles(deleteFile)"
            />
          </div>
          <div class="right-side">
            <list-grid-toggle
              v-model="displayView"
              @update:model-value="resetSelection()"
            />
          </div>
        </div>
      </ion-item-divider>
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
          <button-option
            class="shortcuts-btn"
            id="button-move-to"
            :icon="arrowRedo"
            @click="actionOnSelectedFile(moveTo)"
            v-if="selectedFilesCount >= 1"
          />
          <button-option
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
  IonItemDivider,
  IonPage,
  IonContent,
  IonList,
  IonListHeader,
  IonLabel,
  IonCheckbox,
  IonText,
  popoverController
} from '@ionic/vue';
import {
  folderOpen,
  document,
  pencil,
  link,
  arrowRedo,
  trashBin,
  copy,
  informationCircle
} from 'ionicons/icons';
import { useRoute } from 'vue-router';
import { computed, ref, Ref } from 'vue';
import { MockFile, pathInfo } from '@/common/mocks';
import ButtonOption from '@/components/ButtonOption.vue';
import ListGridToggle from '@/components/ListGridToggle.vue';
import { DisplayState } from '@/components/ListGridToggle.vue';
import FileListItem from '@/components/FileListItem.vue';
import FileCard from '@/components/FileCard.vue';
import FileContextMenu from '@/components/FileContextMenu.vue';
import router from '@/router';
import { FileAction } from '@/components/FileContextMenu.vue';

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
  if (file.type === 'folder') {
    const newPath = (path.value.toString().endsWith('/')) ? `${path.value}${file.name}` : `${path.value}/${file.name}`;
    router.push({
      name: 'folder',
      params: { deviceId: currentRoute.params.deviceId, workspaceId: workspaceId.value },
      query: { path: newPath }
    });
  }
}

function createFolder(): void {
  console.log('Create folder clicked');
}

function importFiles(): void {
  console.log('Import files clicked');
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
  const popover = await popoverController
    .create({
      component: FileContextMenu,
      cssClass: 'file-context-menu',
      event: event,
      translucent: true,
      showBackdrop: false,
      dismissOnSelect: true,
      reference: 'event'
    });
  await popover.present();

  const { data } = await popover.onDidDismiss();
  const actions = new Map<FileAction, (file: MockFile) => void>([
    [FileAction.Rename, renameFile],
    [FileAction.MoveTo, moveTo],
    [FileAction.MakeACopy, makeACopy],
    [FileAction.OpenInExplorer, openInExplorer],
    [FileAction.ShowHistory, showHistory],
    [FileAction.Download, download],
    [FileAction.ShowDetails, showDetails],
    [FileAction.CopyLink, copyLink]
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
.folder-toolbar {
  padding: 1em 2em;
  height: 6em;
  background-color: var(--parsec-color-light-secondary-background);
  border-top: 1px solid var(--parsec-color-light-secondary-light);
  position: sticky;
}

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

.folder-footer {
  width: 100%;
  position: fixed;
  bottom: 0;
  padding: 1em 0 2.5em;
  text-align: center;
  background: linear-gradient(360deg, #FFF 0%, rgba(255, 255, 255, 0.00) 100%);

  &__container {
    background: var(--parsec-color-light-secondary-background);
    width: fit-content;
    height: 3rem;
    padding: 0.5rem 1rem;
    margin: auto;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: var(--parsec-shadow-light);
    border-radius: var(--parsec-radius-4);
    gap: 1rem;
  }

  .shortcuts-btn {
    display: flex;
    align-items: center;
    gap: 1rem;

    &::before {
      background: var(--parsec-color-light-secondary-light);
      content: '';
      width: 1.5px;
      border-radius: var(--parsec-radius-4);
      height: calc(100% - .5rem);
      display: block;
    }
  }

  .text {
    color: var(--parsec-color-light-secondary-text);
  }
}

.right-side {
  margin-left: auto;
  display: flex;
}
</style>

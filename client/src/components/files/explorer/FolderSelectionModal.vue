<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="title"
    :close-button="{ visible: true }"
    :cancel-button="{
      label: 'TextInputModal.cancel',
      disabled: false,
      onClick: cancel,
    }"
    :confirm-button="{
      label: okButtonLabel || 'TextInputModal.moveHere',
      disabled: allowStartingPath ? false : selectedPath === startingPath,
      onClick: confirm,
    }"
  >
    <!-- :disabled="backStack.length === 0" -->
    <div
      class="navigation"
      ref="navigation"
    >
      <div
        v-if="isLargeDisplay"
        ref="buttons"
      >
        <div class="navigation-buttons">
          <ion-button
            fill="clear"
            @click="back()"
            class="navigation-back-button"
            :disabled="backStack.length === 0"
            :class="{ disabled: backStack.length === 0 }"
          >
            <ion-icon :icon="chevronBack" />
          </ion-button>
          <ion-button
            fill="clear"
            @click="forward()"
            :disabled="forwardStack.length === 0"
            :class="{ disabled: forwardStack.length === 0 }"
            class="navigation-forward-button"
          >
            <ion-icon :icon="chevronForward" />
          </ion-button>
        </div>
      </div>
      <header-breadcrumbs
        v-if="workspaceInfo"
        :path-nodes="headerPath"
        @change="onPathChange"
        class="navigation-breadcrumb"
        :items-before-collapse="1"
        :items-after-collapse="2"
        :available-width="breadcrumbsWidth"
        :workspace-name="workspaceInfo.currentName"
      />
      <div
        v-if="isSmallDisplay"
        @click="createNewFolder()"
        class="create-folder-button-small"
        :disabled="isCreatingFolder"
      >
        <ms-image
          :image="NewFolder"
          class="button-icon"
        />
      </div>
    </div>
    <ion-list class="folder-list">
      <ion-text
        class="current-folder button-medium"
        v-if="headerPath.length > 0 && pathLength > 1"
      >
        <ms-image
          :image="Folder"
          class="current-folder__icon"
        />
        <span class="current-folder__text">{{ `${headerPath[headerPath.length - 1].display}` }}</span>
      </ion-text>

      <ion-text
        class="folder-list__empty body"
        v-if="currentEntries.length === 0 && !isCreatingFolder"
      >
        {{ $msTranslate('FoldersPage.copyMoveFolderNoElement') }}
      </ion-text>
      <div
        class="folder-container"
        ref="folder-list"
        v-if="currentEntries.length > 0 || isCreatingFolder"
      >
        <div
          class="new-folder"
          v-if="isCreatingFolder"
        >
          <ms-image
            :image="Folder"
            class="new-folder-image"
          />
          <ms-input
            v-model="newFolderName"
            @update:model-value="onNewFolderNameChange"
            ref="newFolderInput"
            placeholder="FoldersPage.createFolder"
            @keydown.enter.prevent.stop="confirmNewFolder()"
            class="new-folder__input"
            autofocus
          />
          <ion-button
            fill="clear"
            @click="confirmNewFolder()"
            class="new-folder-confirm"
          >
            <ion-icon
              :icon="checkmark"
              class="button-icon"
            />
          </ion-button>
        </div>
        <ion-item
          class="file-item"
          v-for="entry in currentEntries"
          :key="entry[0].id"
          :disabled="entry[1]"
          @click="enterFolder(entry[0])"
        >
          <div class="file-item-image">
            <ms-image
              :image="entry[0].isFile() ? getFileIcon(entry[0].name) : Folder"
              class="file-item-image__icon"
            />
          </div>
          <ion-label class="file-item__name cell">
            {{ entry[0].name }}
          </ion-label>
          <!-- last update -->
          <div
            class="file-last-update"
            v-if="isLargeDisplay"
          >
            <ion-label class="label-last-update cell">
              {{ $msTranslate(formatTimeSince(entry[0].updated, '--', 'short')) }}
            </ion-label>
          </div>
        </ion-item>
      </div>
    </ion-list>
    <div
      class="navigation-buttons"
      v-if="isSmallDisplay"
    >
      <ion-button
        fill="clear"
        @click="back()"
        class="navigation-back-button"
        :disabled="backStack.length === 0"
        :class="{ disabled: backStack.length === 0 }"
      >
        <ion-icon :icon="chevronBack" />
      </ion-button>
      <ion-button
        fill="clear"
        @click="forward()"
        :disabled="forwardStack.length === 0"
        :class="{ disabled: forwardStack.length === 0 }"
        class="navigation-forward-button"
      >
        <ion-icon :icon="chevronForward" />
      </ion-button>
    </div>
    <ion-button
      slot="footer"
      class="create-folder-button button-default"
      @click="createNewFolder()"
      :disabled="isCreatingFolder"
      v-if="isLargeDisplay"
    >
      <ms-image
        :image="NewFolder"
        class="button-icon"
      />
      {{ $msTranslate('FoldersPage.createFolder') }}
    </ion-button>
    <div
      v-if="error"
      class="folder-selection-modal-error"
    >
      <ion-icon
        class="folder-selection-modal-error__icon"
        :icon="warning"
      />

      <ion-text class="folder-selection-modal-error__text button-medium">
        {{ $msTranslate(error) }}
      </ion-text>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import NewFolder from '@/assets/images/folder-new.svg?raw';
import { getFileIcon } from '@/common/file';
import { pxToRem } from '@/common/utils';
import { FolderSelectionOptions } from '@/components/files';
import HeaderBreadcrumbs, { RouterPathNode } from '@/components/header/HeaderBreadcrumbs.vue';
import {
  EntryStat,
  FsPath,
  Path,
  StartedWorkspaceInfo,
  WorkspaceCreateFolderErrorTag,
  createFolder,
  getWorkspaceInfo,
  statFolderChildren,
} from '@/parsec';
import { Routes } from '@/router';
import { IonButton, IonIcon, IonItem, IonLabel, IonList, IonText, modalController } from '@ionic/vue';
import { checkmark, chevronBack, chevronForward, home, warning } from 'ionicons/icons';
import { Folder, I18n, MsImage, MsInput, MsModal, MsModalResult, Translatable, formatTimeSince, useWindowSize } from 'megashark-lib';
import { Ref, nextTick, onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

const props = defineProps<FolderSelectionOptions>();
const selectedPath: Ref<FsPath> = ref(props.startingPath);
const error = ref<Translatable>('');
const headerPath: Ref<RouterPathNode[]> = ref([]);
const pathLength = ref(0);
const isCreatingFolder = ref(false);
const newFolderName = ref('');
const currentEntries: Ref<[EntryStat, boolean][]> = ref([]);
const workspaceInfo: Ref<StartedWorkspaceInfo | null> = ref(null);
const backStack: FsPath[] = [];
const forwardStack: FsPath[] = [];
const breadcrumbsWidth = ref(0);
const navigationRef = useTemplateRef<HTMLDivElement>('navigation');
const buttonsRef = useTemplateRef<HTMLDivElement>('buttons');
const folderListRef = useTemplateRef<HTMLElement>('folder-list');
const newFolderInput = useTemplateRef<InstanceType<typeof MsInput>>('newFolderInput');

const { windowWidth, isSmallDisplay, isLargeDisplay } = useWindowSize();

const topbarWidthWatchCancel = watch([windowWidth, pathLength], () => {
  if (navigationRef.value?.offsetWidth && buttonsRef.value?.offsetWidth) {
    breadcrumbsWidth.value = pxToRem(navigationRef.value.offsetWidth - buttonsRef.value.offsetWidth);
    if (isSmallDisplay.value) {
      breadcrumbsWidth.value += pathLength.value > 1 ? 2 : 1;
    }
  }
});

async function createNewFolder(): Promise<void> {
  isCreatingFolder.value = true;
  newFolderName.value = '';
  await nextTick();
  newFolderInput.value?.setFocus();

  if (folderListRef.value) {
    await nextTick();
    folderListRef.value.scrollTop = 0;
  }
}

async function confirmNewFolder(): Promise<void> {
  let trimmedName = newFolderName.value.trim();

  if (!workspaceInfo.value) {
    cancelNewFolder();
    return;
  }

  if (trimmedName.length === 0) {
    trimmedName = I18n.translate('FoldersPage.createFolder');
  }

  const path = await Path.join(selectedPath.value, trimmedName);
  const result = await createFolder(workspaceInfo.value.handle, path);

  if (!result.ok) {
    switch (result.error.tag) {
      case WorkspaceCreateFolderErrorTag.EntryExists: {
        error.value = { key: 'FoldersPage.errors.createFolderAlreadyExists', data: { name: trimmedName } };
        break;
      }
      default:
        error.value = { key: 'FoldersPage.errors.createFolderFailed', data: { name: trimmedName } };
        cancelNewFolder();
    }
  } else {
    await update();
    const entry = currentEntries.value.find((e) => e[0]?.name === trimmedName)?.[0] as EntryStat;
    await enterFolder(entry);
    isCreatingFolder.value = false;
  }
}

async function onNewFolderNameChange(): Promise<void> {
  newFolderName.value.trim();
  error.value = '';
}

async function cancelNewFolder(): Promise<void> {
  isCreatingFolder.value = false;
  newFolderName.value = '';
}

onMounted(async () => {
  const result = await getWorkspaceInfo(props.workspaceHandle);
  if (result.ok) {
    workspaceInfo.value = result.value;
  }
  await update();
});

onUnmounted(() => topbarWidthWatchCancel());

async function update(): Promise<void> {
  if (!workspaceInfo.value) {
    return;
  }
  const workspaceHandle = workspaceInfo.value.handle;
  const components = await Path.parse(selectedPath.value);

  const result = await statFolderChildren(workspaceHandle, selectedPath.value);
  if (result.ok) {
    const newEntries: [EntryStat, boolean][] = [];
    const filteredEntries = result.value
      .filter((entry) => !entry.isConfined())
      .sort((item1, item2) => {
        if (item1.isFile() !== item2.isFile()) {
          return Number(item1.isFile()) - Number(item2.isFile());
        }
        return item1.name.localeCompare(item2.name);
      });
    for (const entry of filteredEntries) {
      const isDisabled = await isEntryDisabled(entry);
      newEntries.push([entry, isDisabled]);
    }
    currentEntries.value = newEntries;
  }

  let path = '/';
  headerPath.value = [];
  headerPath.value.push({
    id: 0,
    display: workspaceInfo.value ? workspaceInfo.value.currentName : '',
    route: Routes.Documents,
    popoverIcon: home,
    query: { documentPath: path },
  });
  let id = 1;
  for (const comp of components) {
    path = await Path.join(path, comp);
    headerPath.value.push({
      id: id,
      display: comp === '/' ? '' : comp,
      route: Routes.Documents,
      query: { documentPath: path },
    });
    id += 1;
  }
  pathLength.value = headerPath.value.length;
}

async function isEntryDisabled(entry: EntryStat): Promise<boolean> {
  if (entry.isFile()) {
    return true;
  }
  for (const excludePath of props.excludePaths || []) {
    if (entry.path.startsWith(excludePath)) {
      return true;
    }
  }
  return false;
}

async function forward(): Promise<void> {
  const forwardPath = forwardStack.pop();

  if (!forwardPath) {
    return;
  }
  backStack.push(selectedPath.value);
  selectedPath.value = forwardPath;
  await update();
}

async function back(): Promise<void> {
  const backPath = backStack.pop();

  if (!backPath) {
    return;
  }
  forwardStack.push(selectedPath.value);
  selectedPath.value = backPath;
  await update();
}

async function onPathChange(node: RouterPathNode): Promise<void> {
  forwardStack.splice(0, forwardStack.length);
  await cancelNewFolder();
  if (node.query && node.query.documentPath) {
    selectedPath.value = node.query.documentPath;
    await update();
  }
}

async function enterFolder(entry: EntryStat): Promise<void> {
  cancelNewFolder();
  backStack.push(selectedPath.value);
  selectedPath.value = await Path.join(selectedPath.value, entry.name);
  await update();
}

async function confirm(): Promise<boolean> {
  return await modalController.dismiss(selectedPath.value, MsModalResult.Confirm);
}

async function cancel(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Cancel);
}
</script>

<style scoped lang="scss">
.navigation {
  display: flex;
  margin-bottom: 1rem;

  @include ms.responsive-breakpoint('md') {
    border-bottom: none;
    overflow: visible;
  }

  .disabled {
    pointer-events: none;
    color: var(--parsec-color-light-secondary-light);
    opacity: 1;
  }

  &-buttons {
    display: flex;
    align-items: center;
    margin-right: 0.5rem;

    @include ms.responsive-breakpoint('sm') {
      position: absolute;
      bottom: 3rem;
      z-index: 10;

      ion-icon {
        font-size: 1.5rem;
      }
    }
  }
}

.folder-list {
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  background: var(--parsec-color-light-secondary-background);
  box-shadow: var(--parsec-shadow-input);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  height: -webkit-fill-available;
  border-radius: var(--parsec-radius-8);
  height: 100%;
  padding: 0;

  &__empty {
    align-self: center;
    text-align: center;
    color: var(--parsec-color-light-secondary-soft-text);
    display: flex;
    align-items: center;
    height: 100%;
  }

  .current-folder {
    color: var(--parsec-color-light-secondary-text);
    background: var(--parsec-color-light-secondary-white);
    border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
    padding: 0.5rem 1rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    overflow: hidden;

    &__text {
      flex-grow: 1;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    &__icon {
      flex-shrink: 0;
      width: 1.25rem;
      height: 1.25rem;
    }
  }
}

.folder-container {
  overflow-y: auto;
  width: 100%;
  padding: 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  scroll-behavior: smooth;
}

.file-item {
  --show-full-highlight: 0;
  --background: var(--parsec-color-light-secondary-white);
  cursor: pointer;
  position: relative;
  overflow: visible;

  &::part(native) {
    --padding-start: 0px;
    padding: 0.125rem 0.75rem;
    border-radius: var(--parsec-radius-8);
  }

  &:hover {
    --background: var(--parsec-color-light-secondary-medium);
    box-shadow: var(--parsec-shadow-input);
  }

  &:focus,
  &:active {
    --background: var(--parsec-color-light-secondary-medium);
    --background-focused: var(--parsec-color-light-secondary-medium);
    --background-focused-opacity: 1;
    --border-width: 0;
  }

  &__name {
    color: var(--parsec-color-light-secondary-text);
    margin-left: 1rem;
  }

  &-image {
    width: 1.75rem;
    height: 1.75rem;
  }
}

.new-folder {
  display: flex;
  align-items: center;
  padding: 0.25rem 0.75rem;
  box-shadow: var(--parsec-shadow-input);
  border-radius: var(--parsec-radius-8);
  gap: 0.5rem;
  background: var(--parsec-color-light-secondary-white);
  height: fit-content;

  &-image {
    width: 1.75rem;
    min-width: 1.75rem;
    height: 1.75rem;
  }

  &__name {
    margin: 0.75rem 1rem;
    font-weight: 500;
  }

  &__input {
    flex-grow: 1;
  }
}

.create-folder-button {
  position: absolute;
  left: 1.5rem;
  bottom: 1.5rem;
  background: var(--parsec-color-light-secondary-premiere);
  border: 1px solid var(--parsec-color-light-secondary-medium);
  color: var(--parsec-color-light-secondary-text);
  cursor: pointer;
  border-radius: var(--parsec-radius-8);
  z-index: 20;
  transition:
    background 0.2s ease,
    box-shadow 0.2s ease;

  .button-icon {
    width: 1.125rem;
    margin-right: 0.5rem;
    --fill-color: var(--parsec-color-light-secondary-text);
  }

  &::part(native) {
    background: none;
    --background-hover: none;
    padding: 0.5rem 1rem;
    border-color: var(--parsec-color-light-secondary-text);
  }

  &:hover {
    background: var(--parsec-color-light-secondary-disabled);
    box-shadow: var(--parsec-shadow-input);
  }
}

.create-folder-button-small {
  background: var(--parsec-color-light-secondary-medium);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  border-radius: var(--parsec-radius-circle);
  box-shadow: var(--parsec-shadow-soft);
  padding: 0.5rem;
  cursor: pointer;

  .button-icon {
    width: 1.25rem;
    height: 1.25rem;
    --fill-color: var(--parsec-color-light-secondary-text);
  }

  &:hover {
    background: var(--parsec-color-light-secondary-disabled);
    box-shadow: var(--parsec-shadow-input);
  }

  &[disabled='true'] {
    --fill-color: var(--parsec-color-light-secondary-text);
    opacity: 0.3;
    cursor: not-allowed;
  }
}

.folder-selection-modal-error {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  margin-top: 1rem;

  &__icon {
    color: var(--parsec-color-light-danger-500);
    font-size: 1.25rem;
    flex-shrink: 0;

    @include ms.responsive-breakpoint('sm') {
      font-size: 1rem;
    }
  }

  &__text {
    color: var(--parsec-color-light-danger-500);
  }
}
</style>

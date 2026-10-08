<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="fileEditors.insertImage.title"
    :close-button="{ visible: true }"
    :cancel-button="{
      label: 'TextInputModal.cancel',
      disabled: false,
      onClick: cancel,
    }"
    :confirm-button="{
      label: 'fileEditors.insertImage.insert',
      disabled: !canImport,
      onClick: confirm,
    }"
  >
    <div class="toggle-view-container">
      <div
        class="toggle-view"
        role="tablist"
        aria-label="fileEditors.insertImage.tabs.label"
      >
        <ion-button
          class="toggle-view-button"
          role="tab"
          :aria-selected="currentTab === Tabs.FromWorkspace"
          :class="{ active: currentTab === Tabs.FromWorkspace }"
          @click="switchToTab(Tabs.FromWorkspace)"
          :disabled="currentTab === Tabs.FromWorkspace"
        >
          <ion-icon
            :icon="business"
            class="toggle-view-button__icon"
          />
          <ion-text class="toggle-view-button__label">
            {{ $msTranslate('fileEditors.insertImage.tabs.fromWorkspace') }}
          </ion-text>
        </ion-button>
        <ion-button
          class="toggle-view-button"
          role="tab"
          :aria-selected="currentTab === Tabs.FromPC"
          :class="{ active: currentTab === Tabs.FromPC }"
          @click="switchToTab(Tabs.FromPC)"
          :disabled="currentTab === Tabs.FromPC"
        >
          <ion-icon
            :icon="desktop"
            class="toggle-view-button__icon"
          />
          <ion-text class="toggle-view-button__label">
            {{ $msTranslate('fileEditors.insertImage.tabs.fromPC') }}
          </ion-text>
        </ion-button>
      </div>
    </div>

    <!-- workspace import -->
    <div
      class="tab-item"
      v-if="currentTab === Tabs.FromWorkspace"
    >
      <div
        class="navigation"
        ref="navigation"
      >
        <div
          v-if="isLargeDisplay"
          ref="buttons"
          class="navigation-buttons"
        >
          <ion-button
            slot="icon-only"
            fill="clear"
            @click="back()"
            class="navigation-back-button"
            :disabled="backStack.length === 0"
            :class="{ disabled: backStack.length === 0 }"
          >
            <ion-icon :icon="chevronBack" />
          </ion-button>
          <ion-button
            slot="icon-only"
            fill="clear"
            @click="forward()"
            :disabled="forwardStack.length === 0"
            :class="{ disabled: forwardStack.length === 0 }"
            class="navigation-forward-button"
          >
            <ion-icon :icon="chevronForward" />
          </ion-button>
        </div>
        <header-breadcrumbs
          v-if="workspaceInfo"
          :path-nodes="headerPath"
          @change="onPathChange"
          class="navigation-breadcrumb"
          :items-before-collapse="1"
          :items-after-collapse="2"
          :available-width="breadcrumbsWidth"
          :workspace-name="workspaceInfo.name"
        />
        <div
          class="workspace-info"
          v-if="workspaceInfo"
        >
          <ion-icon
            class="workspace-info__icon"
            :icon="business"
          />
          <ion-text class="workspace-info__title button-large">{{ workspaceInfo.name }}</ion-text>
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
          v-if="currentEntries.length === 0"
        >
          {{ $msTranslate('fileEditors.insertImage.emptyFolder') }}
        </ion-text>
        <div
          class="folder-container"
          ref="folder-list"
          v-if="currentEntries.length > 0"
        >
          <ion-item
            class="file-item"
            v-for="entry in currentEntries"
            :key="entry.id"
            :class="{ 'file-item--selected': selectedEntry !== null && selectedEntry.id === entry.id }"
            @click="onEntryClick(entry)"
          >
            <div class="file-item-image">
              <ms-image
                :image="entry.isFile() ? getFileIcon(entry.name) : Folder"
                class="file-item-image__icon"
              />
            </div>
            <ion-label class="file-item__name cell">
              {{ entry.name }}
            </ion-label>
          </ion-item>
        </div>
      </ion-list>
    </div>

    <!-- file import -->
    <div
      class="tab-item"
      v-if="currentTab === Tabs.FromPC"
    >
      <input
        type="file"
        hidden
        ref="hiddenInput"
        accept="image/*"
      />
      <file-drop-zone
        v-if="!importedImage"
        class="image-drop-zone"
        @files-added="onFilesAdded"
      >
        <div class="image-drop">
          <ms-image
            :image="DocumentImport"
            class="image-drop__image"
          />
          <ion-text class="image-drop__title title-h4">
            {{ $msTranslate('fileEditors.insertImage.dropZone.title') }}
          </ion-text>
          <ion-text class="image-drop__browse body">
            {{ $msTranslate('fileEditors.insertImage.dropZone.or') }}
            <ion-button
              fill="clear"
              class="image-drop__browse-link button-medium"
              @click="importButtonClick()"
            >
              {{ $msTranslate('fileEditors.insertImage.dropZone.browse') }}
            </ion-button>
          </ion-text>
        </div>
      </file-drop-zone>
      <div
        v-else
        class="file-added"
        role="status"
      >
        <div class="file-added-image">
          <ion-icon
            :icon="image"
            class="file-added-image__icon"
          />
          <ion-icon
            :icon="checkmarkCircle"
            class="file-added-image__check"
          />
        </div>
        <div class="file-added-text">
          <ion-text class="file-added-text__label body-sm">
            {{ $msTranslate('fileEditors.insertImage.selectedImage') }}
          </ion-text>
          <ion-text class="file-added-text__name button-medium">{{ importedImage.name }}</ion-text>
        </div>
        <ion-button
          class="file-added__update"
          fill="outline"
          @click="importButtonClick()"
        >
          {{ $msTranslate('fileEditors.insertImage.changeImage') }}
        </ion-button>
      </div>
    </div>
    <ms-report-text
      v-if="error"
      class="image-selection-error"
      :theme="MsReportTheme.Error"
    >
      {{ $msTranslate(error) }}
    </ms-report-text>
  </ms-modal>
</template>

<script setup lang="ts">
// Workspace file picker limited to image files, used to insert an image into
// an editics document (see `src/services/editics.ts`). Modeled after
// `FolderSelectionModal`, minus the folder-creation abilities, and returning
// the selected file's path instead of a folder.
import { getFileContent, getFileIcon } from '@/common/file';
import { FileContentType, detectOpenableFile } from '@/common/fileTypes';
import { pxToRem } from '@/common/utils';
import FileDropZone from '@/components/files/explorer/FileDropZone.vue';
import HeaderBreadcrumbs, { RouterPathNode } from '@/components/header/HeaderBreadcrumbs.vue';
import { EntryStat, FsPath, Path, WorkspaceHandle, WorkspaceInfo, getWorkspaceInfo, statFolderChildren } from '@/parsec';
import { Routes } from '@/router';
import { IonButton, IonIcon, IonItem, IonLabel, IonList, IonText, modalController } from '@ionic/vue';
import { business, checkmarkCircle, chevronBack, chevronForward, desktop, home, image } from 'ionicons/icons';
import {
  DocumentImport,
  Folder,
  MsImage,
  MsModal,
  MsModalResult,
  MsReportText,
  MsReportTheme,
  Translatable,
  useWindowSize,
} from 'megashark-lib';
import { Ref, computed, onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

enum Tabs {
  FromWorkspace = 'from-workspace',
  FromPC = 'from-pc',
}

const props = defineProps<{
  workspaceHandle: WorkspaceHandle;
}>();

const currentTab = ref<Tabs>(Tabs.FromPC);
const selectedPath = ref<FsPath>('/');
const selectedEntry: Ref<EntryStat | null> = ref(null);
const currentEntries: Ref<EntryStat[]> = ref([]);
const workspaceInfo: Ref<WorkspaceInfo | null> = ref(null);
const headerPath: Ref<RouterPathNode[]> = ref([]);
const pathLength = ref(0);
const backStack: FsPath[] = [];
const forwardStack: FsPath[] = [];
const breadcrumbsWidth = ref(0);
const navigationRef = useTemplateRef<HTMLDivElement>('navigation');
const buttonsRef = useTemplateRef<HTMLDivElement>('buttons');
const error = ref<Translatable | undefined>(undefined);
const hiddenInputRef = useTemplateRef<HTMLInputElement>('hiddenInput');
const importedImage: Ref<File | null> = ref(null);

const { windowWidth, isSmallDisplay, isLargeDisplay } = useWindowSize();

const topbarWidthWatchCancel = watch([windowWidth, pathLength], () => {
  if (navigationRef.value?.offsetWidth && buttonsRef.value?.offsetWidth) {
    breadcrumbsWidth.value = pxToRem(navigationRef.value.offsetWidth - buttonsRef.value.offsetWidth);
    if (isSmallDisplay.value) {
      breadcrumbsWidth.value += pathLength.value > 1 ? 2 : 1;
    }
  }
});

onMounted(async () => {
  const result = await getWorkspaceInfo(props.workspaceHandle);
  if (result.ok) {
    workspaceInfo.value = result.value;
  }
  await update();
});

onUnmounted(() => topbarWidthWatchCancel());

const canImport = computed(() => {
  return Boolean(
    (currentTab.value === Tabs.FromPC && importedImage.value) || (currentTab.value === Tabs.FromWorkspace && selectedEntry.value),
  );
});

async function update(): Promise<void> {
  if (!workspaceInfo.value) {
    return;
  }
  const workspaceHandle = workspaceInfo.value.handle;
  const components = await Path.parse(selectedPath.value);

  const result = await statFolderChildren(workspaceHandle, selectedPath.value);
  if (result.ok) {
    // Only show folders and image files: the picker is meant to select an
    // image to insert into the document.
    currentEntries.value = result.value
      .filter((entry) => !entry.isConfined() && (!entry.isFile() || isImageFile(entry)))
      .sort((item1, item2) => {
        if (item1.isFile() !== item2.isFile()) {
          return Number(item1.isFile()) - Number(item2.isFile());
        }
        return item1.name.localeCompare(item2.name);
      });
  }

  let path = '/';
  headerPath.value = [];
  headerPath.value.push({
    id: 0,
    display: workspaceInfo.value ? workspaceInfo.value.name : '',
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

async function forward(): Promise<void> {
  const forwardPath = forwardStack.pop();

  if (!forwardPath) {
    return;
  }
  backStack.push(selectedPath.value);
  selectedPath.value = forwardPath;
  selectedEntry.value = null;
  await update();
}

async function back(): Promise<void> {
  const backPath = backStack.pop();

  if (!backPath) {
    return;
  }
  forwardStack.push(selectedPath.value);
  selectedPath.value = backPath;
  selectedEntry.value = null;
  await update();
}

async function onPathChange(node: RouterPathNode): Promise<void> {
  forwardStack.splice(0, forwardStack.length);
  if (node.query && node.query.documentPath) {
    selectedPath.value = node.query.documentPath;
    selectedEntry.value = null;
    await update();
  }
}

function isImageFile(entry: EntryStat): boolean {
  return entry.isFile() && detectOpenableFile(entry.name).type === FileContentType.Image;
}

async function onEntryClick(entry: EntryStat): Promise<void> {
  error.value = '';
  if (!entry.isFile()) {
    await enterFolder(entry);
  } else if (isImageFile(entry)) {
    selectedEntry.value = entry;
  }
}

async function enterFolder(entry: EntryStat): Promise<void> {
  backStack.push(selectedPath.value);
  forwardStack.splice(0, forwardStack.length);
  selectedPath.value = await Path.join(selectedPath.value, entry.name);
  selectedEntry.value = null;
  await update();
}

async function confirm(): Promise<boolean> {
  if ((currentTab.value === Tabs.FromWorkspace && !selectedEntry.value) || (currentTab.value === Tabs.FromPC && !importedImage.value)) {
    return false;
  }
  let name: string | undefined;
  let content: Uint8Array | undefined;
  if (currentTab.value === Tabs.FromWorkspace) {
    content = await getFileContent(props.workspaceHandle, selectedEntry.value!.path);
    name = selectedEntry.value!.name;
  } else {
    const reader = importedImage.value!.stream().getReader();
    content = new Uint8Array(importedImage.value!.size);
    let offset = 0;
    let buffer = await reader.read();
    while (!buffer.done) {
      content.set(buffer.value, offset);
      offset += buffer.value.length;
      buffer = await reader.read();
    }
    if (buffer.value) {
      content.set(buffer.value, offset);
    }
    name = importedImage.value!.name;
  }
  if (!content || !name) {
    error.value = 'fileEditors.insertImage.errors.loadContent';
    return false;
  }

  return await modalController.dismiss({ name: name, content: content }, MsModalResult.Confirm);
}

function setImportedImage(file: File): void {
  if (detectOpenableFile(file.name).type !== FileContentType.Image) {
    error.value = 'fileEditors.insertImage.errors.notAnImage';
    return;
  }
  error.value = '';
  importedImage.value = file;
}

async function onInputChange(_event: Event): Promise<void> {
  if (hiddenInputRef.value!.files!.length === 1) {
    setImportedImage(hiddenInputRef.value!.files![0]);
  }
  hiddenInputRef.value!.removeEventListener('change', onInputChange);
}

async function onFilesAdded(gen: AsyncGenerator<File[]>): Promise<void> {
  const files: File[] = [];
  for await (const batch of gen) {
    files.push(...batch);
  }
  if (files.length === 0) {
    return;
  }
  if (files.length > 1) {
    error.value = 'fileEditors.insertImage.errors.tooManyFiles';
    return;
  }
  setImportedImage(files[0]);
}

async function importButtonClick(): Promise<void> {
  error.value = '';
  hiddenInputRef.value!.addEventListener('change', onInputChange);
  hiddenInputRef.value!.click();
}

async function switchToTab(tab: Tabs): Promise<void> {
  currentTab.value = tab;
  error.value = '';
}

async function cancel(): Promise<boolean> {
  return modalController.dismiss(undefined, MsModalResult.Cancel);
}
</script>

<style scoped lang="scss">
.toggle-view-container {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  margin-bottom: 1rem;

  @include ms.responsive-breakpoint('sm') {
    flex-direction: column;
    align-items: flex-start;
    gap: 0;
  }

  .toggle-view {
    display: flex;
    width: fit-content;
    background: var(--parsec-color-light-secondary-premiere);
    padding: 3px;
    border: 1px solid var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-8);
    box-shadow: var(--parsec-shadow-input);

    @include ms.responsive-breakpoint('sm') {
      width: 100%;
    }

    &-button {
      --color: var(--parsec-color-light-secondary-hard-grey);
      --background: none;
      --background-hover: var(--parsec-color-light-secondary-medium);
      margin: 0;

      @include ms.responsive-breakpoint('sm') {
        flex: 1;
      }

      &:hover {
        --color: var(--parsec-color-light-secondary-soft-text);
      }

      &::part(native) {
        display: flex;
        padding: 0.5rem 0.75rem;
      }

      &__icon {
        flex-shrink: 0;
        font-size: 1.125rem;
        margin-right: 0.5rem;
      }

      &__label {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }

      &.active {
        --background: var(--parsec-color-light-secondary-white);
        --border-radius: var(--parsec-radius-6);
        --color: var(--parsec-color-light-primary-700);
        cursor: default;
        opacity: 1;
      }
    }
  }
}

.tab-item {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  min-height: 0;
}

.image-drop-zone {
  display: flex;
  flex-grow: 1;
  height: auto;

  // The dashed overlay of FileDropZone is sized for the folder page, the
  // dashed border of `.image-drop` is used instead.
  :deep(.drop-zone-dashed) {
    display: none;
  }

  &.drop-zone-active .image-drop {
    background: var(--parsec-color-light-primary-30);
    border-color: var(--parsec-color-light-primary-400);
  }

  .image-drop {
    display: flex;
    width: 100%;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    flex-grow: 1;
    min-height: 14rem;
    padding: 2rem 1rem;
    text-align: center;
    background: var(--parsec-color-light-secondary-background);
    border: 1px dashed var(--parsec-color-light-secondary-light);
    border-radius: var(--parsec-radius-12);
    transition: all 150ms ease-in-out;

    &__image {
      width: 3rem;
      height: 3rem;
      margin-bottom: 0.5rem;
      pointer-events: none;
      --fill-color: var(--parsec-color-light-secondary-grey);
    }

    &__title {
      color: var(--parsec-color-light-secondary-text);
    }

    &__browse {
      display: flex;
      align-items: center;
      gap: 0.25rem;
      color: var(--parsec-color-light-secondary-soft-text);

      &-link {
        --color: var(--parsec-color-light-primary-500);
        --background-hover: none;
        margin: 0;

        &::part(native) {
          border-radius: 0;
          padding: 0;
          text-decoration: underline;
        }

        &:hover {
          --color: var(--parsec-color-light-primary-700);
        }
      }
    }
  }
}

.file-added {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  color: var(--parsec-color-light-secondary-text);
  background: var(--parsec-color-light-primary-100);
  box-shadow: var(--parsec-shadow-input);
  border-radius: var(--parsec-radius-12);
  padding: 0.625rem;
  overflow: hidden;

  &-image {
    position: relative;
    flex-shrink: 0;
    display: flex;

    &__icon {
      background: var(--parsec-color-light-secondary-white);
      color: var(--parsec-color-light-primary-600);
      font-size: 1.25rem;
      padding: 0.5rem;
      border-radius: var(--parsec-radius-8);
    }

    &__check {
      position: absolute;
      bottom: -0.25rem;
      right: -0.25rem;
      font-size: 1rem;
      color: var(--parsec-color-light-primary-600);
      background: var(--parsec-color-light-secondary-white);
      border-radius: var(--parsec-radius-circle);
    }
  }

  &-text {
    display: flex;
    flex-direction: column;
    flex-grow: 1;
    min-width: 0;

    &__label {
      color: var(--parsec-color-light-primary-600);
    }

    &__name {
      color: var(--parsec-color-light-secondary-text);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }

  &__update {
    --background: var(--parsec-color-light-secondary-white) !important;
    --color: var(--parsec-color-light-secondary-text) !important;
    --border-color: var(--parsec-color-light-secondary-medium) !important;
    --background-opacity: 0.7 !important;
    flex-shrink: 0;
    margin: 0;
    box-shadow: var(--parsec-shadow-input);

    &::part(native) {
      padding: 0.5rem 0.75rem;
    }
  }
}

.workspace-info {
  margin-left: auto;
  display: flex;
  width: fit-content;
  max-width: 35%;
  align-items: center;
  gap: 0.25rem;
  color: var(--parsec-color-light-primary-500);
  background: var(--parsec-color-light-primary-50);
  padding: 0.5rem;
  border-radius: var(--parsec-radius-8);

  @include ms.responsive-breakpoint('sm') {
    margin-top: 0.5rem;
  }

  &__title {
    font-weight: 600;
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }

  &__icon {
    color: var(--parsec-color-light-primary-500);
    font-size: 1rem;
    min-width: 1rem;
  }
}

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
  flex-shrink: 0;
  --show-full-highlight: 0;
  --background: var(--parsec-color-light-secondary-white);
  cursor: pointer;
  position: relative;

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

  &--selected,
  &--selected:hover {
    --background: var(--parsec-color-light-primary-100);
    box-shadow: var(--parsec-shadow-input);
  }

  &__name {
    color: var(--parsec-color-light-secondary-text);
    margin-left: 1rem;
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
  }

  &-image {
    width: 1.75rem;
    height: 1.75rem;
  }
}

.image-selection-error {
  margin-top: 1rem;
}
</style>

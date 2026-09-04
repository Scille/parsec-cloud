<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
            class="toggle-view-button__icon button-icon-left"
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
            class="toggle-view-button__icon button-icon-left"
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
            class="navigation-back-button color-secondary"
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
            class="navigation-forward-button color-secondary"
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
          <ion-text class="workspace-info__title">{{ workspaceInfo.name }}</ion-text>
        </div>
      </div>
      <ion-list class="folder-list">
        <ion-text
          class="current-folder"
          v-if="headerPath.length > 0 && pathLength > 1"
        >
          <ms-image
            :image="Folder"
            class="current-folder__icon"
          />
          <span class="current-folder__text">{{ `${headerPath[headerPath.length - 1].display}` }}</span>
        </ion-text>

        <ion-text
          class="folder-list__empty"
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
          <ion-text class="image-drop__title">
            {{ $msTranslate('fileEditors.insertImage.dropZone.title') }}
          </ion-text>
          <ion-text class="image-drop__browse">
            {{ $msTranslate('fileEditors.insertImage.dropZone.or') }}
            <ion-button
              fill="clear"
              class="image-drop__browse-link"
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
          <ion-text class="file-added-text__label">
            {{ $msTranslate('fileEditors.insertImage.selectedImage') }}
          </ion-text>
          <ion-text class="file-added-text__name">{{ importedImage.name }}</ion-text>
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
  gap: ms.spacing('gap-3xl');
  margin-bottom: 1rem;

  @include ms.responsive-breakpoint('sm') {
    flex-direction: column;
    align-items: flex-start;
    gap: ms.spacing('gap-none');
  }

  .toggle-view {
    display: flex;
    align-items: center;
    width: fit-content;
    padding: ms.spacing('padding-none');
    border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
    gap: ms.spacing('gap-xs');
    background: ms.color('surface-base-page-secondary');
    border-radius: ms.radius('lg');
    flex-shrink: 0;
    overflow: hidden;

    @include ms.responsive-breakpoint('sm') {
      width: 100%;
    }

    &-button {
      --color: #{ms.color('text-neutral-default')};
      --color-hover: #{ms.color('text-neutral-default-hover')};
      --background: none;
      --background-hover: #{ms.color('surface-base-default')};
      --background-hover-opacity: #{ms.opacity('10')};
      --background-activated: #{ms.color('surface-base-default')};
      --border-radius: #{ms.radius('md')};
      margin: 0;
      opacity: ms.opacity('disabled');

      @include ms.responsive-breakpoint('sm') {
        flex: 1;
      }

      &::part(native) {
        display: flex;
        padding: ms.spacing('padding-lg') ms.spacing('padding-2xl');
      }

      &__icon {
        flex-shrink: 0;
        font-size: 1.125rem;
        color: ms.color('icon-neutral-default');
      }

      &__label {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }

      &.active {
        --background: #{ms.color('surface-base-default')};
        border-radius: ms.radius('md');
        box-shadow: ms.shadow('toggle');
        cursor: default;
        opacity: ms.opacity('10');
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
    background: ms.color('surface-brand-default-subtle');
    border-color: ms.color('border-brand-default-subtle');
  }

  .image-drop {
    display: flex;
    width: 100%;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    flex-grow: 1;
    min-height: 14rem;
    padding: ms.spacing('padding-5xl') ms.spacing('padding-3xl');
    text-align: center;
    background: ms.color('surface-base-default-secondary');
    border: ms.border('thin') dashed ms.color('border-base-default');
    border-radius: ms.radius('2xl');
    transition: all 150ms ease-in-out;

    &__image {
      width: 3rem;
      height: 3rem;
      margin-bottom: 1rem;
      pointer-events: none;
      --fill-color: #{ms.color('icon-neutral-default')};
    }

    &__title {
      @include ms.font('heading-h5');
      color: ms.color('text-base-heading');
    }

    &__browse {
      @include ms.font('body-md-regular');
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-sm');
      color: ms.color('text-neutral-default');

      &-link {
        @include ms.font('label-md-medium');
        --color: #{ms.color('text-brand-default')};
        --background: none;
        --background-hover: none;
        margin: 0;

        &::part(native) {
          border-radius: ms.radius('none');
          padding: ms.spacing('padding-none');
          text-decoration: underline;
        }

        &:hover {
          --color: #{ms.color('text-brand-default-hover')};
        }
      }
    }
  }
}

.file-added {
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-2xl');
  color: ms.color('text-base-body');
  background: ms.color('surface-brand-default-subtle-hover');
  box-shadow: ms.shadow('input');
  border-radius: ms.radius('2xl');
  padding: ms.spacing('padding-xl');
  overflow: hidden;

  &-image {
    position: relative;
    flex-shrink: 0;
    display: flex;

    &__icon {
      background: ms.color('surface-base-default');
      color: ms.color('icon-brand-default');
      font-size: 1.25rem;
      padding: ms.spacing('padding-lg');
      border-radius: ms.radius('lg');
    }

    &__check {
      position: absolute;
      bottom: -0.25rem;
      right: -0.25rem;
      font-size: 1rem;
      color: ms.color('icon-brand-default');
      background: ms.color('surface-base-default');
      border-radius: ms.radius('full');
    }
  }

  &-text {
    display: flex;
    flex-direction: column;
    flex-grow: 1;
    min-width: 0;

    &__label {
      @include ms.font('body-sm-regular');
      color: ms.color('text-brand-default');
    }

    &__name {
      @include ms.font('label-md-medium');
      color: ms.color('text-base-body');
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }

  &__update {
    --background: #{ms.color('surface-base-default')} !important;
    --color: #{ms.color('text-base-body')} !important;
    --border-color: #{ms.color('border-neutral-default-subtle-hover')} !important;
    --background-opacity: #{ms.opacity('7')} !important;
    flex-shrink: 0;
    margin: 0;
    box-shadow: ms.shadow('input');

    &::part(native) {
      padding: ms.spacing('padding-lg') ms.spacing('padding-2xl');
    }
  }
}

.workspace-info {
  margin-left: auto;
  display: flex;
  width: fit-content;
  max-width: 35%;
  align-items: center;
  gap: ms.spacing('gap-sm');
  color: ms.color('text-brand-default');
  background: ms.color('surface-brand-default-subtle');
  padding: ms.spacing('padding-lg');
  border-radius: ms.radius('lg');

  @include ms.responsive-breakpoint('sm') {
    margin-top: 0.5rem;
  }

  &__title {
    @include ms.font('label-lg-medium');
    font-weight: 600;
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }

  &__icon {
    color: ms.color('icon-brand-default');
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
    color: ms.color('text-disabled-default');
    opacity: ms.opacity('10');
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
  background: ms.color('surface-base-default-secondary');
  box-shadow: ms.shadow('input');
  border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  height: -webkit-fill-available;
  border-radius: ms.radius('lg');
  height: 100%;
  padding: ms.spacing('padding-none');

  &__empty {
    @include ms.font('body-md-regular');
    align-self: center;
    text-align: center;
    color: ms.color('text-neutral-default');
    display: flex;
    align-items: center;
    height: 100%;
  }

  .current-folder {
    @include ms.font('label-md-medium');
    color: ms.color('text-base-body');
    background: ms.color('surface-base-default');
    border-bottom: ms.border('thin') solid ms.color('border-neutral-default-subtle-hover');
    padding: ms.spacing('padding-lg') ms.spacing('padding-3xl');
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
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
  padding: ms.spacing('padding-lg');
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-lg');
  scroll-behavior: smooth;
}

.file-item {
  flex-shrink: 0;
  --show-full-highlight: 0;
  --background: #{ms.color('surface-base-default')};
  cursor: pointer;
  position: relative;

  &::part(native) {
    --padding-start: #{ms.spacing('padding-none')};
    padding: ms.spacing('padding-xs') ms.spacing('padding-2xl');
    border-radius: ms.radius('lg');
  }

  &:hover {
    --background: #{ms.color('surface-neutral-default-subtle-hover')};
    box-shadow: ms.shadow('input');
  }

  &:focus,
  &:active {
    --background: #{ms.color('surface-neutral-default-subtle-pressed')};
    --background-focused: #{ms.color('surface-neutral-default-subtle-pressed')};
    --background-focused-opacity: 1;
    --border-width: #{ms.border('none')};
  }

  &--selected,
  &--selected:hover {
    --background: #{ms.color('surface-brand-default-subtle-hover')};
    box-shadow: ms.shadow('input');
  }

  &__name {
    @include ms.font('label-md-medium');
    color: ms.color('text-base-body');
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

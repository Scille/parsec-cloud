<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-drop-zone
    ref="fileDropZoneRef"
    :current-path="currentPath"
    :show-drop-message="true"
    @files-added="$emit('filesAdded', $event)"
    :is-reader="ownRole === WorkspaceRole.Reader"
    @drop-as-reader="$emit('dropAsReader')"
  >
    <div
      class="scroll"
      ref="containerScroll"
      @contextmenu="onContextMenu"
    >
      <div class="folders-container-grid">
        <file-card
          class="folder-grid-item"
          ref="folderItemsRef"
          v-for="folder in folders.getEntries()"
          :key="folder.id"
          :entry="folder"
          :show-checkbox="hasSelected() || selectionEnabled === true"
          @click="$emit('click', folder, $event)"
          @menu-click="onMenuClick"
          @files-added="onFilesAdded"
          :is-workspace-reader="ownRole === WorkspaceRole.Reader"
          @drop-as-reader="$emit('dropAsReader')"
          @select="$emit('checkboxClick')"
        />
        <file-card
          class="folder-grid-item"
          ref="fileItemsRef"
          v-for="file in files.getEntries()"
          :key="file.id"
          :entry="file"
          :show-checkbox="hasSelected() || selectionEnabled === true"
          @click="$emit('click', file, $event)"
          @menu-click="onMenuClick"
          @files-added="onFilesAdded"
          @drop-as-reader="$emit('dropAsReader')"
          @select="$emit('checkboxClick')"
        />

        <file-card-processing
          v-for="op in operationsInProgress"
          :key="op.data.id"
          :data="op.data"
          :progress="op.progress"
        />
      </div>
    </div>
  </file-drop-zone>
</template>

<script setup lang="ts">
import FileCard from '@/components/files/FileCard.vue';
import FileCardProcessing from '@/components/files/FileCardProcessing.vue';
import FileDropZone from '@/components/files/FileDropZone.vue';
import { EntryCollection, EntryModel, FileOperationProgress, FileModel, FolderModel } from '@/components/files/types';
import { FileImportTuple } from '@/components/files/utils';
import { FsPath, WorkspaceRole } from '@/parsec';
import { ref } from 'vue';

const props = defineProps<{
  operationsInProgress: Array<FileOperationProgress>;
  files: EntryCollection<FileModel>;
  folders: EntryCollection<FolderModel>;
  currentPath: FsPath;
  ownRole: WorkspaceRole;
  selectionEnabled?: boolean;
}>();

const fileDropZoneRef = ref();
const containerScroll = ref();
const fileItemsRef = ref<Array<typeof FileCard>>();
const folderItemsRef = ref<Array<typeof FileCard>>();

const emits = defineEmits<{
  (e: 'click', entry: EntryModel, event: Event): void;
  (e: 'menuClick', event: Event, entry: EntryModel, onFinished: () => void): void;
  (e: 'globalMenuClick', event: Event): void;
  (e: 'checkboxClick'): void;
  (e: 'filesAdded', imports: FileImportTuple[]): void;
  (e: 'dropAsReader'): void;
}>();

async function onContextMenu(event: Event): Promise<void> {
  event.preventDefault();
  emits('globalMenuClick', event);
}

async function onMenuClick(event: Event, entry: EntryModel, onFinished: () => void): Promise<void> {
  emits('menuClick', event, entry, onFinished);
}

defineExpose({
  scrollToSelected,
});

function onFilesAdded(imports: FileImportTuple[]): void {
  fileDropZoneRef.value.reset();
  emits('filesAdded', imports);
}

function hasSelected(): boolean {
  return props.files.hasSelected() || props.folders.hasSelected();
}

async function scrollToSelected(): Promise<void> {
  let selectedItem: typeof FileCard | undefined = undefined;

  for (const item of folderItemsRef.value ?? []) {
    if (item.props.entry.isSelected) {
      selectedItem = item;
      break;
    }
  }
  if (!selectedItem) {
    for (const item of fileItemsRef.value ?? []) {
      if (item.props.entry.isSelected) {
        selectedItem = item;
        break;
      }
    }
  }
  // Don't know how to handle it better.
  // If the component has been created recently, `selectedItem.offsetTop` doesn't have the
  // correct value yet.
  setTimeout(() => {
    if (selectedItem) {
      containerScroll.value.scrollTo({ top: selectedItem.$el.offsetTop, behavior: 'smooth' });
    }
  }, 500);
}
</script>

<style scoped lang="scss">
.scroll {
  padding: 0;
  margin-bottom: 0;
}

.folders-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5rem;
  overflow-y: auto;

  @include ms.responsive-breakpoint('sm') {
    padding-top: 2rem;
    gap: 1rem;
  }
}
</style>

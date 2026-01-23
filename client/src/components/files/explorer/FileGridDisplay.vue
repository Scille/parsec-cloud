<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-drop-zone
    ref="fileDropZone"
    :show-drop-message="true"
    @files-added="$emit('filesAdded', $event)"
    :is-reader="ownRole === WorkspaceRole.Reader"
    @drop-as-reader="$emit('dropAsReader')"
  >
    <div
      ref="containerScroll"
      @contextmenu="onContextMenu"
    >
      <div class="folders-container-grid">
        <file-card
          class="folder-grid-item"
          ref="folderItems"
          v-for="folder in folders.getEntries()"
          v-model="folder.isSelected"
          :key="folder.id"
          :entry="folder"
          :show-checkbox="hasSelected() || selectionEnabled === true"
          @open-item.stop="$emit('openItem', folder, $event)"
          @menu-click="onMenuClick"
          @files-added="onFilesAdded"
          :is-workspace-reader="ownRole === WorkspaceRole.Reader"
          @drop-as-reader="$emit('dropAsReader')"
        />
        <file-card
          class="folder-grid-item"
          ref="fileItems"
          v-model="file.isSelected"
          v-for="file in files.getEntries()"
          :key="file.id"
          :entry="file"
          :show-checkbox="hasSelected() || selectionEnabled === true"
          @open-item.stop="$emit('openItem', file, $event)"
          @menu-click="onMenuClick"
          @files-added="onFilesAdded"
          @drop-as-reader="$emit('dropAsReader')"
        />
        <file-card-processing
          v-for="op in fileOperations"
          :key="op.entryName"
          :operation="op"
        />
      </div>
    </div>
  </file-drop-zone>
</template>

<script setup lang="ts">
import FileCard from '@/components/files/explorer/FileCard.vue';
import FileCardProcessing from '@/components/files/explorer/FileCardProcessing.vue';
import FileDropZone from '@/components/files/explorer/FileDropZone.vue';
import { EntryCollection, EntryModel, FileModel, FileOperationCurrentFolder, FolderModel } from '@/components/files/types';
import { EntryName, WorkspaceRole } from '@/parsec';
import { useTemplateRef } from 'vue';

const props = defineProps<{
  files: EntryCollection<FileModel>;
  folders: EntryCollection<FolderModel>;
  ownRole: WorkspaceRole;
  selectionEnabled?: boolean;
  fileOperations: Array<FileOperationCurrentFolder>;
}>();

const fileDropZoneRef = useTemplateRef('fileDropZone');
const containerScrollRef = useTemplateRef('containerScroll');
const fileItemsRef = useTemplateRef<Array<typeof FileCard>>('fileItems');
const folderItemsRef = useTemplateRef<Array<typeof FileCard>>('folderItems');

const emits = defineEmits<{
  (e: 'openItem', entry: EntryModel, event: Event): void;
  (e: 'menuClick', event: Event, entry: EntryModel, onFinished: () => void): void;
  (e: 'globalMenuClick', event: Event): void;
  (e: 'filesAdded', files: Array<File>, destinationFolder?: EntryName): void;
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

function onFilesAdded(files: Array<File>, destinationFolder?: EntryName): void {
  fileDropZoneRef.value?.reset();
  emits('filesAdded', files, destinationFolder);
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
      containerScrollRef.value?.scrollTo({ top: selectedItem.$el.offsetTop, behavior: 'smooth' });
    }
  }, 500);
}
</script>

<style scoped lang="scss">
.folders-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5rem;
  overflow-y: auto;

  @include ms.responsive-breakpoint('sm') {
    padding: 1.5rem 1rem 0;
    gap: 1.5rem;
  }

  @include ms.responsive-breakpoint('xs') {
    gap: 1rem;
  }
}
</style>

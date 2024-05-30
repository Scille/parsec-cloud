<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-drop-zone
    ref="fileDropZoneRef"
    :current-path="currentPath"
    :show-drop-message="true"
    @files-added="$emit('filesAdded', $event)"
  >
    <div class="folders-container-grid">
      <file-card
        class="folder-grid-item"
        v-for="folder in folders.getEntries()"
        :key="folder.id"
        :entry="folder"
        :show-checkbox="hasSelected()"
        @click="$emit('click', folder, $event)"
        @menu-click="(event, entry, onFinished) => $emit('menuClick', event, entry, onFinished)"
        @files-added="onFilesAdded"
      />
      <file-card
        class="folder-grid-item"
        v-for="file in files.getEntries()"
        :key="file.id"
        :entry="file"
        :show-checkbox="hasSelected()"
        @click="$emit('click', file, $event)"
        @menu-click="(event, entry, onFinished) => $emit('menuClick', event, entry, onFinished)"
        @files-added="onFilesAdded"
      />

      <file-card-processing
        v-for="op in operationsInProgress"
        :key="op.data.id"
        :data="op.data"
        :progress="op.progress"
      />
    </div>
  </file-drop-zone>
</template>

<script setup lang="ts">
import FileCard from '@/components/files/FileCard.vue';
import FileCardProcessing from '@/components/files/FileCardProcessing.vue';
import FileDropZone from '@/components/files/FileDropZone.vue';
import { EntryCollection, EntryModel, FileOperationProgress, FileModel, FolderModel } from '@/components/files/types';
import { FileImportTuple } from '@/components/files/utils';
import { FsPath } from '@/parsec';
import { ref } from 'vue';

const props = defineProps<{
  operationsInProgress: Array<FileOperationProgress>;
  files: EntryCollection<FileModel>;
  folders: EntryCollection<FolderModel>;
  currentPath: FsPath;
}>();

const fileDropZoneRef = ref();

const emits = defineEmits<{
  (e: 'click', entry: EntryModel, event: Event): void;
  (e: 'menuClick', event: Event, entry: EntryModel, onFinished: () => void): void;
  (e: 'filesAdded', imports: FileImportTuple[]): void;
}>();

function onFilesAdded(imports: FileImportTuple[]): void {
  fileDropZoneRef.value.reset();
  emits('filesAdded', imports);
}

function hasSelected(): boolean {
  return props.files.hasSelected() || props.folders.hasSelected();
}
</script>

<style scoped lang="scss">
.folders-container-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 1.5em;
  overflow-y: auto;
}
</style>

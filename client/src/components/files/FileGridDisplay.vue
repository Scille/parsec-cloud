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

      <file-card-importing
        v-for="fileImport in importsInProgress"
        :key="fileImport.data.id"
        :data="fileImport.data as ImportData"
        :progress="fileImport.progress"
      />
    </div>
  </file-drop-zone>
</template>

<script setup lang="ts">
import FileCard from '@/components/files/FileCard.vue';
import FileCardImporting from '@/components/files/FileCardImporting.vue';
import FileDropZone from '@/components/files/FileDropZone.vue';
import { EntryCollection, EntryModel, FileOperationProgress, FileModel, FolderModel } from '@/components/files/types';
import { FileImportTuple } from '@/components/files/utils';
import { FsPath } from '@/parsec';
import { FileOperationDataType, ImportData } from '@/services/fileOperationManager';
import { computed, ref } from 'vue';

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

const importsInProgress = computed(() => {
  return props.operationsInProgress.filter((op) => op.data.getDataType() === FileOperationDataType.Import);
});

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

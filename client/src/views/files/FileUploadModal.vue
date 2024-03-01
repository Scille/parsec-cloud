<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="$t('FoldersPage.importModal.title')"
    :close-button="{ visible: true }"
  >
    <div class="modal-content inner-content">
      <file-drop-zone @files-drop="onFilesDrop">
        <file-import @files-import="onFilesImport" />
      </file-drop-zone>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { MsModal } from '@/components/core';
import { FileDropZone, FileImport } from '@/components/files';
import { Path, WorkspaceHandle, WorkspaceID } from '@/parsec';
import { ImportManager, ImportManagerKey } from '@/services/importManager';
import { inject } from 'vue';

const importManager = inject(ImportManagerKey) as ImportManager;

const props = defineProps<{
  currentPath: string;
  workspaceHandle: WorkspaceHandle;
  workspaceId: WorkspaceID;
}>();

async function uploadFileEntry(currentPath: string, fsEntry: FileSystemEntry): Promise<void> {
  const parsecPath = await Path.join(currentPath, fsEntry.name);

  if (fsEntry.isDirectory) {
    const reader = (fsEntry as FileSystemDirectoryEntry).createReader();
    reader.readEntries((entries) => {
      entries.forEach(async (entry) => {
        await uploadFileEntry(parsecPath, entry);
      });
    });
  } else {
    (fsEntry as FileSystemFileEntry).file(async (file) => {
      importManager.importFile(props.workspaceHandle, props.workspaceId, file, currentPath);
    });
  }
}

async function onFilesDrop(entries: FileSystemEntry[]): Promise<void> {
  for (const entry of entries) {
    await uploadFileEntry(props.currentPath, entry);
  }
}

async function onFilesImport(files: File[]): Promise<void> {
  for (const file of files) {
    importManager.importFile(props.workspaceHandle, props.workspaceId, file, props.currentPath);
  }
}
</script>

<style scoped lang="scss"></style>

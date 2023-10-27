<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="$t('FoldersPage.importModal.title')"
    :close-button="{visible: true}"
  >
    <div class="modal-content inner-content">
      <file-drop-zone
        @files-drop="onFilesDrop"
      >
        <file-import
          @files-import="onFilesImport"
        />
      </file-drop-zone>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import FileImport from '@/components/files/FileImport.vue';
import FileDropZone from '@/components/files/FileDropZone.vue';
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import { join as joinPath } from '@/common/path';
import { createFolder, WorkspaceHandle, WorkspaceID } from '@/parsec';
import { inject } from 'vue';
import { ImportManager, ImportKey } from '@/common/importManager';

const importManager = inject(ImportKey) as ImportManager;

const props = defineProps<{
  currentPath: string
  workspaceHandle: WorkspaceHandle
  workspaceId: WorkspaceID
}>();

async function uploadFileEntry(currentPath: string, fsEntry: FileSystemEntry): Promise<void> {
  const parsecPath = joinPath(currentPath, fsEntry.name);

  if (fsEntry.isDirectory) {
    console.log('Uploading folder', fsEntry.name, 'to', currentPath, 'create', parsecPath);
    const result = await createFolder(parsecPath);
    if (!result.ok) {
      console.log(`Failed to create folder ${parsecPath} (reason: ${result.error.tag}), cancelling...`);
      // No need to go further if the folder creation failed
      return;
    }
    const reader = (fsEntry as FileSystemDirectoryEntry).createReader();
    reader.readEntries((entries) => {
      entries.forEach(async (entry) => {
        await uploadFileEntry(parsecPath, entry);
      });
    });
  } else {
    (fsEntry as FileSystemFileEntry).file(async (file) => {
      importManager.importFile(props.workspaceHandle, props.workspaceId, file, parsecPath);
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
    importManager.importFile(props.workspaceHandle, props.workspaceId, file, joinPath(props.currentPath, file.name));
  }
}
</script>

<style scoped lang="scss">
</style>

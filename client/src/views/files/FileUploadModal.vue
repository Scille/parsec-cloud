<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="$t('FoldersPage.importModal.title')"
    :close-button-enabled="true"
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
import { createFile, createFolder } from '@/parsec';

const props = defineProps<{
  currentPath: string
}>();

let fd = 0;

async function mockParsecOpenFile(path: string, mode: string): Promise<number> {
  fd += 1;
  console.log('Opening', path, 'with mode', mode, 'as', fd);
  return fd;
}

async function mockParsecCloseFile(fd: number): Promise<void> {
  console.log('Closing', fd);
}

async function mockParsecWriteFile(fd: number, chunk: Uint8Array): Promise<number> {
  console.log('Writing to', fd, 'content of size', chunk.length);
  return chunk.length;
}

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
    console.log('Uploading file', fsEntry.name, 'to', currentPath);
    (fsEntry as FileSystemFileEntry).file(async (file) => {
      await uploadFile(file, parsecPath);
    });
  }
}

async function uploadFile(file: File, uploadPath: string): Promise<void> {
  const reader = file.stream().getReader();

  const result = await createFile(uploadPath);
  if (!result.ok) {
    console.log(`Failed to create file ${uploadPath} (reason: ${result.error.tag}), cancelling...`);
    // Failed to create the file, no need to go further
    return;
  }
  const fd = await mockParsecOpenFile(uploadPath, 'w+');
  // Would love to do it like in the documentation but it does not seem to be
  // supported in typescript
  // https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream

  // eslint-disable-next-line no-constant-condition
  while (true) {
    const data = await reader.read();
    if (data.done) {
      break;
    }
    await mockParsecWriteFile(fd, data.value);
  }
  await mockParsecCloseFile(fd);
}

async function onFilesDrop(entries: FileSystemEntry[]): Promise<void> {
  for (const entry of entries) {
    await uploadFileEntry(props.currentPath, entry);
  }
}

async function onFilesImport(files: File[]): Promise<void> {
  for (const file of files) {
    await uploadFile(file, joinPath(props.currentPath, file.name));
  }
}
</script>

<style scoped lang="scss">
</style>

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
        <file-import />
      </file-drop-zone>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import FileImport from '@/components/files/FileImport.vue';
import FileDropZone from '@/components/files/FileDropZone.vue';
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import { join as joinPath } from '@/common/path';

const props = defineProps<{
  currentPath: string
}>();

let fd = 0;

async function mockParsecUploadFile(currentPath: string, fsEntry: FileSystemEntry): Promise<void> {

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

  async function mockParsecMkdir(path: string): Promise<void> {
    console.log('Makedir', path);
  }

  const parsecPath = joinPath(currentPath, fsEntry.name);

  if (fsEntry.isDirectory) {
    console.log('Uploading folder', fsEntry.name, 'to', currentPath);
    mockParsecMkdir(fsEntry.name);
    const reader = (fsEntry as FileSystemDirectoryEntry).createReader();
    reader.readEntries((entries) => {
      entries.forEach(async (entry) => {
        await mockParsecUploadFile(parsecPath, entry);
      });
    });
  } else {
    console.log('Uploading file', fsEntry.name, 'to', currentPath);
    (fsEntry as FileSystemFileEntry).file(async (file) => {
      const reader = file.stream().getReader();
      const fd = await mockParsecOpenFile(parsecPath, 'w+');
      // eslint-disable-next-line no-constant-condition
      while (true) {
        const data = await reader.read();
        if (data.done) {
          break;
        }
        await mockParsecWriteFile(fd, data.value);
      }
      await mockParsecCloseFile(fd);
    });
  }
}

async function onFilesDrop(entries: FileSystemEntry[]): Promise<void> {
  for (const entry of entries) {
    await mockParsecUploadFile(props.currentPath, entry);
  }
}
</script>

<style scoped lang="scss">
</style>

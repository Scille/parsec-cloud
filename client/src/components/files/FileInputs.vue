<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <input
    type="file"
    directory
    webkitdirectory
    hidden
    ref="hiddenInputFolder"
    id="import-folder-input"
  />
  <input
    type="file"
    multiple
    hidden
    ref="hiddenInputFiles"
    id="import-file-input"
  />
</template>

<script setup lang="ts">
import { FileImportTuple } from '@/components/files/utils';
import { FsPath } from '@/parsec';
import { onBeforeUnmount, onMounted, ref } from 'vue';

defineExpose({
  importFiles,
  importFolder,
});

const props = defineProps<{
  currentPath: FsPath;
}>();

const emits = defineEmits<{
  (e: 'filesAdded', imports: FileImportTuple[]): void;
}>();

const hiddenInputFiles = ref();
const hiddenInputFolder = ref();

onMounted(() => {
  hiddenInputFiles.value.addEventListener('change', onInputChange);
  hiddenInputFolder.value.addEventListener('change', onInputChange);
});

onBeforeUnmount(() => {
  hiddenInputFiles.value.removeEventListener('change', onInputChange);
  hiddenInputFolder.value.removeEventListener('change', onInputChange);
});

async function onInputChange(event: any): Promise<void> {
  const elem = event.target;
  // Would love to use `hiddenInput.value.webkitEntries` instead but it returns
  // an empty list (may be browser dependant).
  // So we have to use `.files` instead, which is a worst API.
  if (elem.files && elem.files.length > 0) {
    const files: File[] = [];
    // Converting from `FileList` to `File[]`. No idea why the API is using
    // a worst type.
    for (let i = 0; i < elem.files.length; i++) {
      const item = elem.files.item(i);
      if (item) {
        files.push(item);
      }
    }
    if (files.length > 0) {
      await onFilesImport(files);
    }
  }
  elem.value = '';
}

async function importFiles(): Promise<void> {
  hiddenInputFiles.value.click();
}

async function importFolder(): Promise<void> {
  hiddenInputFolder.value.click();
}

async function onFilesImport(files: File[]): Promise<void> {
  const imports: FileImportTuple[] = files.map((file): FileImportTuple => {
    return { file: file, path: props.currentPath };
  });
  emits('filesAdded', imports);
}
</script>

<style scoped lang="scss"></style>

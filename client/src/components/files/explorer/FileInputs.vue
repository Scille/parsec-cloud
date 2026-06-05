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
import { onBeforeUnmount, onMounted, useTemplateRef } from 'vue';

defineExpose({
  importFiles,
  importFolder,
});

const emits = defineEmits<{
  (e: 'filesAdded', gen: AsyncGenerator<File[]>): void;
}>();

const hiddenInputFilesRef = useTemplateRef<HTMLInputElement>('hiddenInputFiles');
const hiddenInputFolderRef = useTemplateRef<HTMLInputElement>('hiddenInputFolder');

onMounted(() => {
  hiddenInputFilesRef.value?.addEventListener('change', onInputChange);
  hiddenInputFolderRef.value?.addEventListener('change', onInputChange);
});

onBeforeUnmount(() => {
  hiddenInputFilesRef.value?.removeEventListener('change', onInputChange);
  hiddenInputFolderRef.value?.removeEventListener('change', onInputChange);
});

async function onInputChange(event: any): Promise<void> {
  const elem = event.target;

  // Would love to use `hiddenInput.value.webkitEntries` instead but it returns
  // an empty list (may be browser dependant).
  // So we have to use `.files` instead, which is a worst API.
  if (elem.files && elem.files.length > 0) {
    const files: File[] = [];
    for (let i = 0; i < elem.files.length; i++) {
      const item: File | undefined = elem.files.item(i);
      if (!item) {
        continue;
      }
      (item as any).relativePath = item.webkitRelativePath ? `/${item.webkitRelativePath}` : `/${item.name}`;
      files.push(item);
    }
    emits('filesAdded', traverseFiles(files));
  }
  elem.value = '';
}

async function* traverseFiles(files: File[]): AsyncGenerator<File[]> {
  for (let i = 0; i < files.length; i += 200) {
    yield files.slice(i, i + 200);
  }
}

async function importFiles(): Promise<void> {
  hiddenInputFilesRef.value?.click();
}

async function importFolder(): Promise<void> {
  hiddenInputFolderRef.value?.click();
}
</script>

<style scoped lang="scss"></style>

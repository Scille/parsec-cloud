<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="file-item"
    v-for="(file, index) in files"
    :key="file.name"
  >
    <span class="file-name">{{ file.name }}</span>
    <ion-button
      class="remove-button"
      @click="removeFile(index)"
    >
      X
    </ion-button>
  </div>
  MAX FILE SIZE {{ $msTranslate(formatFileSize(MAX_FILE_SIZE)) }}
  <ion-button
    class="add-button"
    @click="addFile"
    :disabled="files.length >= limit"
  >
    {{ 'ADD A FILE' }}
  </ion-button>
  <input
    type="file"
    ref="inputRef"
    hidden
    @change="onInputChange"
  />
  <span>{{ `FILE ${files.length}/${limit}` }}</span>
</template>

<script setup lang="ts">
import { IonButton } from '@ionic/vue';
import { ref } from 'vue';
import { formatFileSize } from '@/common/file';

const MAX_FILE_SIZE = 3 * 1024 * 1024;

const props = withDefaults(
  defineProps<{
    limit?: number;
  }>(),
  {
    limit: 3,
  },
);

defineExpose({
  getFiles,
});

const files = ref<Array<File>>([]);
const inputRef = ref<HTMLInputElement>();

async function addFile(): Promise<void> {
  if (files.value.length >= props.limit || !inputRef.value) {
    return;
  }
  inputRef.value.click();
}

async function onInputChange(): Promise<void> {
  if (!inputRef.value) {
    return;
  }
  if (!inputRef.value.files || !inputRef.value.files.length || !inputRef.value.files.item(0)) {
    return;
  }
  const file = inputRef.value.files.item(0) as File;
  if (file.size > MAX_FILE_SIZE) {
    return;
  }
  files.value.push(file);
}

async function removeFile(index: number): Promise<void> {
  files.value.splice(index, 1);
}

function getFiles(): Array<File> {
  return files.value;
}
</script>

<style scoped lang="scss"></style>

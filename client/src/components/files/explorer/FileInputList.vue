<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="file-input-list">
    <ion-text class="body file-number">
      {{ $msTranslate({ key: 'browseFiles.currentFilesCount', data: { current: files.length, limit: limit } }) }}
    </ion-text>
    <div
      class="file-item"
      v-for="(file, index) in files"
      :key="file.name"
    >
      <ion-icon
        class="file-icon"
        :icon="documentOutline"
      />
      <span class="file-name subtitles-sm">{{ file.name }}</span>
      <ion-button
        fill="clear"
        class="remove-button"
        @click="removeFile(index)"
      >
        {{ $msTranslate('browseFiles.removeFile') }}
      </ion-button>
    </div>
    <div
      class="file-input"
      v-if="files.length < limit"
    >
      <ion-button
        class="file-input__button"
        fill="clear"
        @click="addFile"
      >
        <div class="button-content">
          <ion-icon
            class="button-icon"
            :icon="documentOutline"
          />
          <span class="button-medium button-label">{{ $msTranslate('browseFiles.addFile') }}</span>
          <span class="body button-file-size">
            {{ $msTranslate('browseFiles.maxFileSize') }} {{ $msTranslate(formatFileSize(MAX_FILE_SIZE)) }}
          </span>
        </div>
      </ion-button>
      <input
        type="file"
        ref="input"
        hidden
        @change="onInputChange"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatFileSize } from '@/common/file';
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { documentOutline } from 'ionicons/icons';
import { ref, useTemplateRef } from 'vue';

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
const inputRef = useTemplateRef<HTMLInputElement>('input');

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

<style scoped lang="scss">
.file-input-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  position: relative;
  width: 100%;
}

.file-number {
  position: absolute;
  top: -1.75rem;
  right: 0;
  color: var(--parsec-color-light-secondary-hard-grey);
  margin-bottom: 0.5rem;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  position: relative;
  background: var(--parsec-color-light-secondary-premiere);
  border-radius: var(--parsec-radius-8);
  padding: 0.625rem;
  color: var(--parsec-color-light-secondary-text);

  .file-name {
    width: fit-content;
  }

  .file-icon {
    color: var(--parsec-color-light-secondary-contrast);
    font-size: 1.125rem;
  }

  .remove-button {
    --color: var(--parsec-color-light-danger-500);
    --background: transparent;
    --background-hover: transparent;
    --color-hover: var(--parsec-color-light-danger-700);
    min-height: 1rem;
    margin-left: auto;
    margin-right: 0.5rem;

    &::part(native) {
      padding: 0.125rem;
    }
  }
}

.file-input {
  display: flex;
  align-items: center;
  border: 1px dashed var(--parsec-color-light-secondary-light);
  border-radius: var(--parsec-radius-8);
  gap: 0.5rem;

  &:hover {
    background: var(--parsec-color-light-secondary-background);
  }

  &__button {
    color: var(--parsec-color-light-secondary-contrast);
    width: 100%;

    &::part(native) {
      --background: transparent;
      --background-hover: transparent;
      --color-hover: var(--parsec-color-light-secondary-contrast);
    }

    .button-content {
      display: flex;
      align-items: center;
      gap: 0.5rem;

      .button-icon {
        color: var(--parsec-color-light-secondary-contrast);
        font-size: 1.125rem;
      }

      .button-label {
        color: var(--parsec-color-light-secondary-contrast);
      }

      .button-file-size {
        color: var(--parsec-color-light-secondary-grey);
      }
    }
  }
}
</style>

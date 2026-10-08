<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="file-input-list">
    <ion-text class="file-number">
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
      <span class="file-name">{{ file.name }}</span>
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
          <span class="button-label">{{ $msTranslate('browseFiles.addFile') }}</span>
          <span class="button-file-size">
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
  gap: ms.spacing('gap-lg');
  position: relative;
  width: 100%;
}

.file-number {
  @include ms.font('body-md-regular');
  position: absolute;
  top: -1.75rem;
  right: 0;
  color: ms.color('text-base-description');
  margin-bottom: 0.5rem;
}

.file-item {
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-lg');
  position: relative;
  background: ms.color('surface-base-page-secondary');
  border-radius: ms.radius('lg');
  padding: ms.spacing('padding-xl');
  color: ms.color('text-base-body');

  .file-name {
    @include ms.font('body-md-medium');
    width: fit-content;
  }

  .file-icon {
    color: ms.color('text-base-heading');
    font-size: 1.125rem;
  }

  .remove-button {
    --color: #{ms.color('text-error-default')};
    --background: transparent;
    --background-hover: transparent;
    --color-hover: #{ms.color('text-error-default-hover')};
    min-height: 1rem;
    margin-left: auto;
    margin-right: 0.5rem;

    &::part(native) {
      padding: ms.spacing('padding-xs');
    }
  }
}

.file-input {
  display: flex;
  align-items: center;
  border: ms.border('thin') dashed ms.color('border-base-default-hover');
  border-radius: ms.radius('lg');
  gap: ms.spacing('gap-lg');

  &:hover {
    background: ms.color('surface-base-default-secondary');
  }

  &__button {
    color: ms.color('text-base-heading');
    width: 100%;

    &::part(native) {
      --background: transparent;
      --background-hover: transparent;
      --color-hover: #{ms.color('text-base-heading')};
    }

    .button-content {
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-lg');

      .button-icon {
        color: ms.color('text-base-heading');
        font-size: 1.125rem;
      }

      .button-label {
        @include ms.font('label-md-medium');
        color: ms.color('text-base-heading');
      }

      .button-file-size {
        @include ms.font('body-md-regular');
        color: ms.color('text-base-description');
      }
    }
  }
}
</style>

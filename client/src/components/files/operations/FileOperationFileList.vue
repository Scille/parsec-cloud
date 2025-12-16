<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="multiples-file-list">
    <div
      class="multiples-file-item"
      v-for="file of filesList"
      :key="file.name"
    >
      <ms-image
        :image="getFileIcon(file.name)"
        class="file-icon"
      />
      <ion-text class="multiples-file-item__label body-sm">
        {{ file.name }}
      </ion-text>

      <!-- Waiting -->
      <ion-text
        v-if="status === FileOperationEvents.Added"
        class="waiting-text form-input"
      >
        {{ $msTranslate('FoldersPage.FileOperations.waiting') }}
      </ion-text>

      <!-- Cancelled -->
      <ion-icon
        v-if="status === FileOperationEvents.Cancelled"
        :icon="alert"
        class="icon--error"
      />

      <!-- done -->
      <ion-icon
        v-if="status === FileOperationEvents.Finished"
        :icon="checkmark"
        class="icon--success"
      />
    </div>
    <div
      v-if="files.length > MAX_DISPLAYED_FILES"
      class="multiples-file-item more-files"
    >
      <ms-image
        :image="MultiImport"
        class="file-icon"
      />
      <ion-text class="multiples-file-item__label body-sm">
        {{
          $msTranslate({
            key: 'FoldersPage.ConflictsFile.moreFiles',
            data: { count: otherFilesCount },
            count: otherFilesCount,
          })
        }}
      </ion-text>
    </div>
  </div>
</template>

<script lang="ts" setup>
import MultiImport from '@/assets/images/multi-import.svg?raw';
import { getFileIcon } from '@/common/file';
import { EntryStat, WorkspaceHistoryEntryStat } from '@/parsec';
import { FileOperationEvents } from '@/services/fileOperation';
import { IonIcon, IonText } from '@ionic/vue';
import { alert, checkmark } from 'ionicons/icons';
import { MsImage } from 'megashark-lib';
import { computed } from 'vue';

const props = defineProps<{
  files: Array<File> | Array<EntryStat> | Array<WorkspaceHistoryEntryStat>;
  status: FileOperationEvents;
}>();

const MAX_DISPLAYED_FILES = 99;

const filesList = computed(() => {
  if (props.files.length > MAX_DISPLAYED_FILES) {
    return props.files.slice(0, MAX_DISPLAYED_FILES);
  }
  return props.files;
});
const otherFilesCount = computed(() => {
  return props.files.length - MAX_DISPLAYED_FILES;
});
</script>

<style lang="scss" scoped>
.multiples-file-list {
  background: var(--parsec-color-light-secondary-premiere);
  max-height: 8rem;
  border-top: 1px solid var(--parsec-color-light-secondary-medium);
  max-height: 10rem;
  padding: 0.5rem;
  overflow-y: auto;
  overflow-x: hidden;
  z-index: 2;
  position: relative;

  .multiples-file-item {
    border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
    display: flex;
    align-items: center;
    padding: 0.5rem;
    gap: 0.5rem;

    &:hover {
      background: var(--parsec-color-light-secondary-medium);
      border-radius: var(--parsec-radius-8);
    }

    &:last-child {
      border-bottom: none;
    }

    .file-icon {
      min-width: 1.5rem;
      max-width: 1.5rem;
      min-height: 1.5rem;
      max-height: 1.5rem;
    }

    &__label {
      color: var(--parsec-color-light-secondary-text);
      flex-grow: 1;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .icon--error {
      font-size: 1.125rem;
      margin-left: auto;
      flex-shrink: 0;
    }

    .icon--success {
      font-size: 1.125rem;
      margin-left: auto;
      flex-shrink: 0;
      color: var(--parsec-color-light-primary-700);
    }

    &.more-files {
      .multiples-file-item__label {
        font-style: italic;
      }
    }
  }
}
</style>

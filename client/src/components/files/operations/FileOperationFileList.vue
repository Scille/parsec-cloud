<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="multiples-file-list">
    <div
      class="multiples-file-item"
      v-for="file of filesList"
      :key="file.name"
    >
      <ms-image
        :image="(file as File).webkitRelativePath !== undefined || (file as EntryStat).isFile() ? getFileIcon(file.name) : Folder"
        class="file-icon"
      />
      <ion-text class="multiples-file-item__label">
        {{ file.name }}
      </ion-text>

      <!-- Waiting -->
      <ion-text
        v-if="status === FileOperationEvents.Added"
        class="waiting-text"
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
      <ion-text class="multiples-file-item__label">
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
import { Folder, MsImage } from 'megashark-lib';
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
  background: ms.color('surface-base-default-secondary');
  border-top: ms.border('thin') solid ms.color('border-base-default');
  max-height: 10rem;
  padding: ms.spacing('padding-lg');
  overflow-y: auto;
  overflow-x: hidden;
  z-index: 2;
  position: relative;

  .multiples-file-item {
    border-bottom: ms.border('thin') solid ms.color('border-base-default');
    display: flex;
    align-items: center;
    padding: ms.spacing('padding-lg');
    gap: ms.spacing('gap-lg');
    cursor: default;

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
      @include ms.font('body-sm-regular');
      color: ms.color('text-base-body');
      flex-grow: 1;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .waiting-text {
      @include ms.font('label-md-medium');
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
      color: ms.color('icon-brand-default-hover');
    }

    &.more-files {
      .multiples-file-item__label {
        font-style: italic;
      }
    }
  }
}
</style>

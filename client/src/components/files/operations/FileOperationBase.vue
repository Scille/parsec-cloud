<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    v-if="info"
    class="file-operation-item"
    :class="{
      waiting: props.status === FileOperationEvents.Added,
      progress: props.status === FileOperationEvents.Progress || props.status === FileOperationEvents.Started,
      done: props.status === FileOperationEvents.Finished,
      cancelled: props.status === FileOperationEvents.Cancelled,
      failed: props.status === FileOperationEvents.Failed,
      multiple_elements: info.entries.length > 1,
    }"
  >
    <ion-item
      class="element ion-no-padding"
      :class="{ element_open: showFileList }"
    >
      <div class="element-content">
        <div class="element-type">
          <ms-image
            :image="info.icon"
            class="file-icon"
          />
        </div>
        <div class="element-details">
          <ion-text
            v-if="firstEntry"
            class="element-details__name form-input"
          >
            {{
              $msTranslate({
                key: info.translation,
                data: { name: firstEntry.name },
              })
            }}
          </ion-text>
          <ion-text
            v-else
            class="element-details__name form-input"
            :class="{ open: showFileList }"
            @click="toggleFileList"
          >
            {{
              $msTranslate({
                key: info.translationMultiple,
                data: { count: info.entries.length },
              })
            }}
            <ion-icon
              class="name-icon"
              :icon="chevronDown"
            />
          </ion-text>
          <div class="element-details-info body-sm">
            <ion-text v-if="props.status === FileOperationEvents.Cancelled">
              {{ $msTranslate('FoldersPage.FileOperations.cancelled') }}
            </ion-text>
            <ion-text v-if="props.status === FileOperationEvents.Failed">
              {{ $msTranslate('FoldersPage.FileOperations.failed') }}
            </ion-text>
            <ion-text class="element-details-info__workspace">
              <span v-if="props.status === FileOperationEvents.Failed || props.status === FileOperationEvents.Cancelled"> &bull; </span>
              {{ operationData.workspaceName }}
            </ion-text>
          </div>
        </div>

        <!-- waiting -->
        <div
          class="waiting-info"
          v-if="props.status === FileOperationEvents.Added"
        >
          <ion-text class="waiting-text body">
            {{ $msTranslate('FoldersPage.FileOperations.waiting') }}
          </ion-text>
          <ion-button
            fill="clear"
            size="small"
            class="cancel-button"
            @click="$emit('cancel', operationData)"
          >
            <ion-icon
              class="cancel-button__icon"
              slot="icon-only"
              :icon="closeCircle"
            />
          </ion-button>
        </div>

        <!-- in progress -->
        <div
          class="progress-info"
          v-if="props.status === FileOperationEvents.Progress && props.eventData"
        >
          <ion-text class="progress-percentage button-small default-state">
            {{
              getProgressPercent(
                (props.eventData as OperationProgressEventData).global.currentSize,
                (props.eventData as OperationProgressEventData).global.totalSize,
              )
            }}%
          </ion-text>
          <ms-spinner class="progress-spinner default-state" />
          <ion-button
            fill="clear"
            size="small"
            class="cancel-button hover-state"
            @click="$emit('cancel', operationData)"
          >
            {{ $msTranslate('FoldersPage.FileOperations.cancel') }}
          </ion-button>
        </div>

        <!-- done -->
        <ion-icon
          class="checkmark-icon"
          v-if="props.status === FileOperationEvents.Finished"
          :icon="checkmarkCircle"
        />

        <!-- cancelled -->
        <ion-icon
          class="cancel-icon"
          v-if="props.status === FileOperationEvents.Cancelled"
          :icon="alert"
        />

        <!-- failed -->
        <ion-icon
          class="failed-icon"
          v-if="props.status === FileOperationEvents.Failed"
          :icon="warning"
        />
      </div>
    </ion-item>
    <template v-if="props.operationData.type !== FileOperationDataType.Download">
      <transition name="file-list">
        <file-operation-file-list
          v-if="info.entries.length > 1 && showFileList"
          :files="info.entries"
          :status="props.status"
        />
      </transition>
    </template>
  </div>
</template>

<script setup lang="ts">
import CopyFile from '@/assets/images/copy-file.svg?raw';
import DownloadFile from '@/assets/images/download-file.svg?raw';
import MoveFile from '@/assets/images/move-file.svg?raw';
import RestoreFile from '@/assets/images/restore-file.svg?raw';
import { getProgressPercent } from '@/components/files';
import FileOperationFileList from '@/components/files/operations/FileOperationFileList.vue';
import {
  FileOperationCopyData,
  FileOperationData,
  FileOperationDataType,
  FileOperationDownloadData,
  FileOperationEventData,
  FileOperationEvents,
  FileOperationMoveData,
  FileOperationRestoreData,
  OperationProgressEventData,
} from '@/services/fileOperation';
import { IonButton, IonIcon, IonItem, IonText } from '@ionic/vue';
import { alert, checkmarkCircle, chevronDown, closeCircle, warning } from 'ionicons/icons';
import { asyncComputed, MsImage, MsSpinner } from 'megashark-lib';
import { computed, ref } from 'vue';

const props = defineProps<{
  operationData: FileOperationCopyData | FileOperationMoveData | FileOperationRestoreData | FileOperationDownloadData;
  status: FileOperationEvents;
  eventData?: FileOperationEventData;
}>();

const showFileList = ref(false);

const info = computed(() => {
  switch (props.operationData.type) {
    case FileOperationDataType.Copy:
      return {
        entries: (props.operationData as FileOperationCopyData).sources,
        icon: CopyFile,
        translation: 'FoldersPage.CopyFile.copy',
        translationMultiple: 'FoldersPage.CopyFile.copyMultiplesFiles',
      };
    case FileOperationDataType.Move:
      return {
        entries: (props.operationData as FileOperationMoveData).sources,
        icon: MoveFile,
        translation: 'FoldersPage.MoveFile.move',
        translationMultiple: 'FoldersPage.MoveFile.moveMultiplesFiles',
      };
    case FileOperationDataType.Restore:
      return {
        entries: (props.operationData as FileOperationRestoreData).entries,
        icon: RestoreFile,
        translation: 'FoldersPage.RestoreFile.restore',
        translationMultiple: 'FoldersPage.RestoreFile.restoreMultiplesFiles',
      };
    case FileOperationDataType.Download:
      return {
        entries: [(props.operationData as FileOperationDownloadData).entry],
        icon: DownloadFile,
        translation: 'FoldersPage.DownloadFile.download',
        translationMultiple: '',
      };
    default:
      return undefined;
  }
});

const firstEntry = asyncComputed(async () => {
  if (props.operationData.type === FileOperationDataType.Copy && (props.operationData as FileOperationCopyData).sources.length === 1) {
    return (props.operationData as FileOperationCopyData).sources.at(0);
  } else if (
    props.operationData.type === FileOperationDataType.Move &&
    (props.operationData as FileOperationMoveData).sources.length === 1
  ) {
    return (props.operationData as FileOperationCopyData).sources.at(0);
  } else if (
    props.operationData.type === FileOperationDataType.Restore &&
    (props.operationData as FileOperationRestoreData).entries.length === 1
  ) {
    return (props.operationData as FileOperationRestoreData).entries.at(0);
  } else if (props.operationData.type === FileOperationDataType.Download) {
    return (props.operationData as FileOperationDownloadData).entry;
  }
  return undefined;
});

defineEmits<{
  (event: 'cancel', operationData: FileOperationData): void;
  (event: 'click', operationData: FileOperationData, status: FileOperationEvents, eventData?: FileOperationEventData): void;
}>();

function toggleFileList() {
  showFileList.value = !showFileList.value;
}
</script>

<style scoped lang="scss">
.element {
  .file-icon {
    min-width: 2rem;
    max-width: 2rem;
    min-height: 2rem;
    max-height: 2rem;
  }
}

.failed {
  border: 1px solid var(--parsec-color-light-danger-500);

  .element {
    --background: var(--parsec-color-light-danger-50);
  }

  &-content {
    color: var(--parsec-color-light-danger-500);
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  .failed-icon {
    font-size: 1.25rem;
    color: var(--parsec-color-light-danger-500);
  }
}
</style>

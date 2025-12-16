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
          <div
            v-if="singleEntry"
            class="element-details-title"
          >
            <ion-text class="element-details-title__name form-input">
              <i18n-t
                :keypath="info.translation"
                scope="global"
              >
                <template #name>
                  <strong> {{ singleEntry.name }} </strong>
                </template>
              </i18n-t>
            </ion-text>
          </div>
          <div
            v-else
            class="element-details-title"
            :class="{ open: showFileList }"
            @click="toggleFileList"
          >
            <ion-text class="element-details-title__name form-input">
              <i18n-t
                :keypath="info.translationMultiple"
                scope="global"
              >
                <template #count>
                  <strong>{{ info.entries.length }} {{ $msTranslate('FoldersPage.FileOperations.files') }}</strong>
                </template>
              </i18n-t>
            </ion-text>
            <ion-icon
              class="name-icon"
              :icon="chevronDown"
            />
          </div>
          <div class="element-details-info body-sm">
            <ion-text v-if="props.status === FileOperationEvents.Cancelled">
              {{ $msTranslate('FoldersPage.FileOperations.cancelled') }}
            </ion-text>
            <ion-text v-if="props.status === FileOperationEvents.Failed">
              {{ $msTranslate('FoldersPage.FileOperations.failed') }}
            </ion-text>
            <ion-text
              class="element-details-info__workspace"
              v-if="!isHovered"
            >
              <span v-if="props.status === FileOperationEvents.Failed || props.status === FileOperationEvents.Cancelled"> &bull; </span>
              {{ operationData.workspaceName }}
            </ion-text>
            <ion-text
              class="hover-state"
              v-if="
                props.status === FileOperationEvents.Finished && isHovered && props.operationData.type !== FileOperationDataType.Download
              "
            >
              {{ $msTranslate('FoldersPage.ImportFile.browse') }}
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
        </div>

        <!-- finalizing -->
        <div
          class="finalizing-info"
          v-if="props.status === FileOperationEvents.Finalizing"
        >
          <ion-text class="waiting-text body">
            {{ $msTranslate('FoldersPage.FileOperations.finalizing') }}
          </ion-text>
        </div>

        <!-- in progress -->
        <div
          class="progress-info"
          v-if="props.status === FileOperationEvents.Progress && props.eventData"
        >
          <ion-text class="progress-percentage button-small default-state">
            {{ (props.eventData as OperationProgressEventData).global.progress }}%
          </ion-text>
          <ms-spinner class="progress-spinner default-state" />
          <ion-button
            fill="clear"
            size="small"
            class="cancel-button hover-state"
            :disabled="cancelling"
            @click="onCancelClicked"
          >
            {{ cancelling ? $msTranslate('FoldersPage.FileOperations.cancelling') : $msTranslate('FoldersPage.FileOperations.cancel') }}
          </ion-button>
        </div>

        <!-- done -->
        <ion-icon
          class="folder-icon"
          v-if="props.status === FileOperationEvents.Finished && showDestinationFolderIcon"
          :icon="folder"
          @click="$emit('click', operationData, props.status, eventData)"
          @mouseover="isHovered = true"
          @mouseleave="isHovered = false"
        />
        <ion-icon
          class="checkmark-icon"
          v-if="props.status === FileOperationEvents.Finished && !showDestinationFolderIcon"
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
import { alert, checkmarkCircle, chevronDown, folder, warning } from 'ionicons/icons';
import { MsImage, MsSpinner } from 'megashark-lib';
import { computed, ref } from 'vue';

const props = defineProps<{
  operationData: FileOperationCopyData | FileOperationMoveData | FileOperationRestoreData | FileOperationDownloadData;
  status: FileOperationEvents;
  eventData?: FileOperationEventData;
}>();

const isHovered = ref(false);
const showFileList = ref(false);
const cancelling = ref(false);

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

const singleEntry = computed(() => {
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

const showDestinationFolderIcon = computed(() => {
  return (
    props.operationData.type !== FileOperationDataType.Download &&
    !(props.operationData.type === FileOperationDataType.Restore && (props.operationData as FileOperationRestoreData).entries.length > 1)
  );
});

const emits = defineEmits<{
  (event: 'cancel', operationData: FileOperationData): void;
  (event: 'click', operationData: FileOperationData, status: FileOperationEvents, eventData?: FileOperationEventData): void;
}>();

function onCancelClicked(): void {
  emits('cancel', props.operationData);
}

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

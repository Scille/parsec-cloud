<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="file-operation-item"
    :class="{
      waiting: props.status === FileOperationEvents.Added,
      progress: props.status === FileOperationEvents.Progress || props.status === FileOperationEvents.Started,
      done: props.status === FileOperationEvents.Finished,
      cancelled: props.status === FileOperationEvents.Cancelled,
      failed: props.status === FileOperationEvents.Failed,
    }"
  >
    <ion-item class="element ion-no-padding">
      <div class="element-content">
        <div class="element-type">
          <ms-image
            :image="DownloadArchive"
            class="file-icon"
          />
        </div>
        <div class="element-details">
          <div class="element-details-title">
            <ion-text class="element-details-title__name form-input">
              {{ $msTranslate('FoldersPage.DownloadFile.archiveDownload') }}
            </ion-text>
          </div>
          <div class="element-details-info body-sm">
            <ion-text v-if="props.status === FileOperationEvents.Cancelled">
              {{ $msTranslate('FoldersPage.FileOperations.cancelled') }}
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
              v-if="props.status === FileOperationEvents.Finished && isHovered"
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
          <ion-text class="waiting-text form-input">
            {{ $msTranslate('FoldersPage.FileOperations.waiting') }}
          </ion-text>
          <ion-button
            fill="clear"
            size="small"
            class="cancel-button"
            :disabled="cancelling"
            @click="onCancelClicked"
          >
            {{ $msTranslate('FoldersPage.FileOperations.cancel') }}
          </ion-button>
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
            @click="onCancelClicked"
          >
            {{ cancelling ? $msTranslate('FoldersPage.FileOperations.cancelling') : $msTranslate('FoldersPage.FileOperations.cancel') }}
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
  </div>
</template>

<script setup lang="ts">
import DownloadArchive from '@/assets/images/download-archive.svg?raw';
import {
  FileOperationData,
  FileOperationDownloadArchiveData,
  FileOperationEventData,
  FileOperationEvents,
  OperationProgressEventData,
} from '@/services/fileOperation';
import { IonButton, IonIcon, IonItem, IonText } from '@ionic/vue';
import { alert, checkmarkCircle, warning } from 'ionicons/icons';
import { MsImage, MsSpinner } from 'megashark-lib';
import { ref } from 'vue';

const props = defineProps<{
  operationData: FileOperationDownloadArchiveData;
  status: FileOperationEvents;
  eventData?: FileOperationEventData;
}>();

const isHovered = ref(false);
const cancelling = ref(false);

const emits = defineEmits<{
  (event: 'cancel', operationData: FileOperationData): void;
  (event: 'click', operationData: FileOperationData, status: FileOperationEvents, eventData?: FileOperationEventData): void;
}>();

function onCancelClicked(): void {
  emits('cancel', props.operationData);
}
</script>

<style scoped lang="scss">
.element {
  .file-icon {
    min-width: 1.825rem;
    max-width: 1.825rem;
    min-height: 1.825rem;
    max-height: 1.825rem;
  }
}

.file-operation-item .element-details-title__name:hover {
  border-bottom-color: transparent;
  cursor: default;
}

.failed-content {
  color: var(--parsec-color-light-danger-500);
  display: flex;
  align-items: center;
  gap: 0.375rem;

  .information-icon {
    font-size: 1.375rem;

    &:hover {
      color: var(--parsec-color-light-danger-700);
    }
  }
}
</style>

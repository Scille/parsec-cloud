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
      multiple_elements: operationData.entries.length > 1,
    }"
  >
    <ion-item
      class="element ion-no-padding"
      :class="{ element_open: showFileList }"
    >
      <div class="element-content">
        <div class="element-type">
          <ms-image
            v-if="singleEntry"
            :image="getFileIcon(singleEntry.name)"
            class="file-icon"
          />
          <ms-image
            v-else
            :image="MultiImport"
            class="file-icon"
          />
        </div>
        <div class="element-details">
          <div
            v-if="singleEntry"
            class="element-details-title"
          >
            <ion-text class="element-details-title__name form-input">
              <strong> {{ singleEntry.name }} </strong>
            </ion-text>
          </div>
          <div
            v-else
            class="element-details-title"
            :class="{ open: showFileList }"
            @click="toggleFileList"
          >
            <ion-text class="element-details-title__name form-input">
              {{
                $msTranslate({
                  key:
                    props.status === FileOperationEvents.Finished
                      ? 'FoldersPage.DownloadFile.downloadMultipleDone'
                      : 'FoldersPage.DownloadFile.downloadMultiple',
                  data: { count: props.operationData.entries.length },
                  count: props.operationData.entries.length,
                })
              }}
            </ion-text>
            <ion-icon
              v-if="props.status === FileOperationEvents.Finished"
              class="name-icon"
              :icon="chevronDown"
            />
          </div>
          <div class="element-details-info button-small">
            <ion-text
              v-if="singleEntry"
              v-show="props.status === FileOperationEvents.Progress || props.status === FileOperationEvents.Finished"
            >
              {{ $msTranslate(formatFileSize(singleEntry.size)) }}
            </ion-text>
            <ion-text v-if="props.status === FileOperationEvents.Cancelled">
              {{ $msTranslate('FoldersPage.FileOperations.cancelled') }}
            </ion-text>
            <ion-text class="element-details-info__workspace">
              <!-- eslint-disable vue/html-indent -->
              <span
                v-if="
                  (singleEntry && [FileOperationEvents.Progress, FileOperationEvents.Finished].includes(props.status)) ||
                  [FileOperationEvents.Failed, FileOperationEvents.Cancelled].includes(props.status)
                "
              >
                <!-- eslint-enable vue/html-indent -->
                &bull;
              </span>
              <span v-if="status === FileOperationEvents.Started">
                {{ $msTranslate('FoldersPage.DownloadFile.preparing') }}
              </span>
              <span v-if="eventData && (eventData as OperationProgressEventData).global">
                {{
                  $msTranslate({
                    key: 'FoldersPage.DownloadFile.downloadedCount',
                    data: { count: Math.max((eventData as OperationProgressEventData).global.fileIndex - 1, 0) },
                  })
                }}
              </span>
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
        </div>

        <!-- finalizing -->
        <div
          class="finalizing-info"
          v-if="props.status === FileOperationEvents.Finalizing"
        >
          <ion-text class="waiting-text form-input">
            {{ $msTranslate('FoldersPage.FileOperations.finalizing') }}
          </ion-text>
        </div>
        <!-- in progress -->
        <div
          class="progress-info"
          v-if="props.status === FileOperationEvents.Progress || props.status === FileOperationEvents.Started"
        >
          <ms-spinner
            class="progress-spinner default-state"
            :size="18"
          />
          <ion-button
            fill="clear"
            size="small"
            class="cancel-button hover-state"
            @click="onCancelClicked"
            :disabled="cancelling"
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
    <transition name="file-list">
      <file-operation-file-list
        v-if="operationData.entries.length > 1 && showFileList && props.status === FileOperationEvents.Finished"
        :files="operationData.entries"
        :status="props.status"
        class="file-list-enter"
      />
    </transition>
  </div>
</template>

<script setup lang="ts">
import MultiImport from '@/assets/images/multi-import.svg?raw';
import { formatFileSize, getFileIcon } from '@/common/file';
import FileOperationFileList from '@/components/files/operations/FileOperationFileList.vue';
import { EntryStatFile } from '@/parsec';
import {
  FileOperationData,
  FileOperationDownloadFilesData,
  FileOperationEventData,
  FileOperationEvents,
  OperationProgressEventData,
} from '@/services/fileOperation';
import { IonButton, IonIcon, IonItem, IonText } from '@ionic/vue';
import { alert, checkmarkCircle, chevronDown, warning } from 'ionicons/icons';
import { MsImage, MsSpinner } from 'megashark-lib';
import { computed, ref } from 'vue';

const props = defineProps<{
  operationData: FileOperationDownloadFilesData;
  status: FileOperationEvents;
  eventData?: FileOperationEventData;
}>();

const showFileList = ref(false);
const cancelling = ref(false);

const emits = defineEmits<{
  (event: 'cancel', operationData: FileOperationData): void;
}>();

function onCancelClicked(): void {
  if (cancelling.value) {
    return;
  }
  cancelling.value = true;
  emits('cancel', props.operationData);
}

function toggleFileList() {
  showFileList.value = !showFileList.value;
}

const singleEntry = computed(() => {
  if (props.operationData.entries.length === 1 && props.operationData.entries.at(0)?.isFile()) {
    return props.operationData.entries.at(0) as EntryStatFile;
  }
  return undefined;
});
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

.file-operation-item:not(.multiple_elements) .element-details-title__name {
  display: block;
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

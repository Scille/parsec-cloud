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
      multiple_elements: operationData.files.length > 1,
    }"
  >
    <ion-item
      class="element ion-no-padding"
      :class="{ element_open: showFileList }"
    >
      <div class="element-content">
        <div class="element-type">
          <ms-image
            v-if="operationData.files.length === 1"
            :image="getFileIcon(operationData.files.at(0)?.name || '')"
            class="file-icon"
          />
          <ms-image
            v-else
            :image="MultiImport"
            class="file-icon"
          />
        </div>
        <div class="element-details">
          <ion-text
            v-if="operationData.files.length === 1"
            class="element-details__name form-input"
          >
            {{ operationData.files.at(0)?.name }}
          </ion-text>
          <ion-text
            v-else
            class="element-details__name form-input"
            :class="{ open: showFileList }"
            @click="toggleFileList"
          >
            {{
              $msTranslate({
                key: 'FoldersPage.ImportFile.importMultiplesFiles',
                data: { count: operationData.files.length },
              })
            }}
            <ion-icon
              class="name-icon"
              :icon="chevronDown"
            />
          </ion-text>
          <div class="element-details-info body-sm">
            <template v-if="operationData.files.length === 1">
              <ion-text
                v-for="file of operationData.files"
                :key="file.name"
                v-show="props.status === FileOperationEvents.Progress || (props.status === FileOperationEvents.Finished && !isHovered)"
              >
                {{ $msTranslate(formatFileSize(file.size)) }}
              </ion-text>
            </template>
            <ion-text v-if="props.status === FileOperationEvents.Cancelled">
              {{ $msTranslate('FoldersPage.FileOperations.cancelled') }}
            </ion-text>
            <ion-text
              class="element-details-info__workspace"
              v-if="!isHovered"
            >
              <!-- eslint-disable vue/html-indent -->
              <span
                v-if="
                  operationData.files.length === 1 ||
                  props.status === FileOperationEvents.Failed ||
                  props.status === FileOperationEvents.Cancelled
                "
              >
                <!-- eslint-enable vue/html-indent -->
                &bull;
              </span>
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
            @click="$emit('cancel', operationData)"
          >
            {{ $msTranslate('FoldersPage.FileOperations.cancel') }}
          </ion-button>
        </div>

        <!-- in progress -->
        <div
          class="progress-info"
          v-if="props.status === FileOperationEvents.Progress && props.eventData"
        >
          <ion-text
            v-if="operationData.files.length === 1"
            class="progress-percentage button-small default-state"
          >
            {{
              getProgressPercent(
                (props.eventData as OperationProgressEventData).currentFile.currentSize,
                (props.eventData as OperationProgressEventData).currentFile.totalSize,
              )
            }}%
          </ion-text>
          <ion-text
            v-else
            class="progress-percentage button-small default-state"
          >
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
          class="folder-icon"
          v-if="props.status === FileOperationEvents.Finished"
          :icon="folder"
          @click="$emit('click', operationData, props.status, eventData)"
          @mouseover="isHovered = true"
          @mouseleave="isHovered = false"
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
        v-if="operationData.files.length > 1 && showFileList"
        :files="operationData.files"
        :status="props.status"
        class="file-list-enter"
      />
    </transition>
  </div>
</template>

<script setup lang="ts">
import MultiImport from '@/assets/images/multi-import.svg?raw';
import { formatFileSize, getFileIcon } from '@/common/file';
import { getProgressPercent } from '@/components/files';
import FileOperationFileList from '@/components/files/operations/FileOperationFileList.vue';
import {
  FileOperationData,
  FileOperationEventData,
  FileOperationEvents,
  FileOperationImportData,
  OperationProgressEventData,
} from '@/services/fileOperation';
import { IonButton, IonIcon, IonItem, IonText } from '@ionic/vue';
import { alert, chevronDown, folder, warning } from 'ionicons/icons';
import { MsImage, MsSpinner } from 'megashark-lib';
import { ref } from 'vue';

const props = defineProps<{
  operationData: FileOperationImportData;
  status: FileOperationEvents;
  eventData?: FileOperationEventData;
}>();

const isHovered = ref(false);
const showFileList = ref(false);

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
    min-width: 1.825rem;
    max-width: 1.825rem;
    min-height: 1.825rem;
    max-height: 1.825rem;
  }
}

.file-operation-item:not(.multiple_elements) .element-details__name {
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

<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="element-container"
    :class="{
      waiting: state === FileOperationState.CopyAdded,
      progress: state === FileOperationState.OperationProgress,
      done: state === FileOperationState.EntryCopied,
      cancelled: state === FileOperationState.Cancelled,
      failed: state === FileOperationState.CopyFailed,
    }"
  >
    <ion-item class="element ion-no-padding">
      <div class="element-type">
        <div class="element-type__icon">
          <ms-image
            :image="Copy"
            class="file-icon"
          />
        </div>
      </div>
      <div class="element-details">
        <ion-text class="element-details__name body">
          {{
            $msTranslate({
              key: 'FoldersPage.CopyFile.copy',
              data: { name: shortenFileName(operationData.srcPath, { maxLength: 40, prefixLength: 8, suffixLength: 32 }) },
            })
          }}
        </ion-text>
        <ion-text
          class="element-details__workspace body-sm"
          v-if="workspaceName"
        >
          {{ workspaceName }}
        </ion-text>
        <div
          class="element-details-progress-container"
          v-show="state === FileOperationState.OperationProgress"
        >
          <ms-progress
            class="progress-bar"
            :progress="getProgress()"
            :appearance="MsProgressAppearance.Line"
          />
          <ion-icon
            class="cancel-icon"
            type="button"
            :icon="closeCircle"
            @click="$emit('cancel', operationData.id)"
          />
        </div>
      </div>

      <!-- waiting -->
      <div
        class="waiting-info"
        v-if="state === FileOperationState.CopyAdded"
      >
        <ion-text class="waiting-text body">
          {{ $msTranslate('FoldersPage.CopyFile.waiting') }}
        </ion-text>
        <ion-icon
          class="cancel-icon"
          @click="$emit('cancel', operationData.id)"
          type="button"
          :icon="closeCircle"
          slot="icon-only"
        />
      </div>

      <!-- done -->
      <ion-icon
        class="checkmark-icon"
        v-show="state === FileOperationState.EntryCopied"
        :icon="checkmarkCircle"
      />

      <!-- cancelled -->
      <ion-text
        class="cancel-text body"
        v-if="state === FileOperationState.Cancelled"
      >
        {{ $msTranslate('FoldersPage.CopyFile.cancelled') }}
      </ion-text>

      <!-- failed -->
      <div
        class="failed-content"
        v-if="state === FileOperationState.CopyFailed"
      >
        <ion-text class="failed-text body">
          {{ $msTranslate('FoldersPage.CopyFile.failed') }}
        </ion-text>
        <ms-information-tooltip
          v-show="getFailureReason() !== ''"
          :text="getFailureReason()"
          class="information-icon"
          slot="end"
        />
      </div>
    </ion-item>
  </div>
</template>

<script setup lang="ts">
import { shortenFileName } from '@/common/file';
import { getWorkspaceName } from '@/parsec';
import {
  CopyData,
  CopyFailedError,
  CopyFailedStateData,
  FileOperationState,
  OperationProgressStateData,
  StateData,
} from '@/services/fileOperationManager';
import { IonIcon, IonItem, IonText } from '@ionic/vue';
import { checkmarkCircle, closeCircle } from 'ionicons/icons';
import { Copy, MsImage, MsInformationTooltip, MsProgress, MsProgressAppearance, Translatable } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const props = defineProps<{
  operationData: CopyData;
  state: FileOperationState;
  stateData?: StateData;
}>();

const workspaceName = ref('');

defineExpose({
  props,
});

onMounted(async () => {
  workspaceName.value = await getWorkspaceName(props.operationData.workspaceHandle);
});

function getProgress(): number {
  if (props.state === FileOperationState.Cancelled || props.state === FileOperationState.EntryCopied) {
    return 100;
  } else if (props.state === FileOperationState.OperationProgress && props.stateData) {
    return (props.stateData as OperationProgressStateData).progress;
  }
  return 0;
}

defineEmits<{
  (event: 'cancel', id: string): void;
  (event: 'click', operationData: CopyData, state: FileOperationState): void;
}>();

function getFailureReason(): Translatable {
  if (props.state === FileOperationState.CopyFailed && props.stateData) {
    const data = props.stateData as CopyFailedStateData;
    switch (data.error) {
      case CopyFailedError.MaxFilesReached:
        return { key: 'FoldersPage.errors.copyFailedTooManyFiles' };
      case CopyFailedError.MaxRecursionReached:
        return { key: 'FoldersPage.errors.copyFailedMaxRecursion' };
      case CopyFailedError.OneFailed:
        return { key: 'FoldersPage.errors.copyFailedOneFailed' };
      case CopyFailedError.SourceDoesNotExist:
        return { key: 'FoldersPage.errors.copyFailedSourceDoesNotExist', data: { source: props.operationData.srcPath } };
    }
  }
  return '';
}
</script>

<style scoped lang="scss">
.element-container {
  background: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  transition: background 0.2s ease-in-out;
  border-radius: var(--parsec-radius-8);
  position: relative;
  margin: 0.5rem 0.5rem 0;

  &:hover {
    background: var(--parsec-color-light-secondary-premiere);
  }
}

.element {
  --background: none;
  padding: 0.75rem 1rem;
  --inner-padding-end: 0;

  .file-icon {
    width: 2rem;
    height: 2rem;
  }

  &-details {
    display: flex;
    flex-direction: column;
    position: relative;
    margin-left: 0.875rem;
    flex-grow: 1;
    overflow: hidden;

    &__name {
      color: var(--parsec-color-light-primary-700);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    &__workspace {
      color: var(--parsec-color-light-secondary-grey);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    &-progress-container {
      display: flex;
      align-items: center;
      margin-top: 0.25rem;
      gap: 0.5rem;

      .progress-bar {
        --progress-background: var(--parsec-color-light-primary-500);
        --background: var(--parsec-color-light-secondary-medium);
        border-radius: var(--parsec-radius-8);
        width: 100%;
      }

      .cancel-icon {
        color: var(--parsec-color-light-secondary-grey);
        font-size: 1.125rem;
        padding: 0.25rem;
        border-radius: var(--parsec-radius-4);

        &:hover {
          color: var(--parsec-color-light-secondary-text);
          background: var(--parsec-color-light-secondary-disabled);
        }
      }
    }
  }

  .checkmark-icon,
  .waiting-info,
  .failed-text,
  .cancel-text,
  .failed-content {
    margin-left: auto;
    padding-left: 0.5rem;
    flex-shrink: 0;
  }
}

// states of the element-container
.waiting,
.progress {
  .cancel-icon {
    color: var(--parsec-color-light-secondary-grey);
    font-size: 1.15rem;
    padding: 0.25rem;
    border-radius: var(--parsec-radius-4);
    cursor: pointer;

    &:hover {
      color: var(--parsec-color-light-secondary-text);
      background: var(--parsec-color-light-secondary-disabled);
    }
  }
}

.waiting {
  .waiting-info {
    display: flex;
    align-items: center;
  }

  .waiting-text {
    color: var(--parsec-color-light-primary-700);
  }

  .cancel-icon {
    display: none;
  }

  &:hover {
    .waiting-text {
      display: none;
    }

    .cancel-icon {
      display: block;
    }
  }
}

.done {
  .checkmark-icon {
    color: var(--parsec-color-light-success-700);
    font-size: 1.25rem;
  }
}

.cancelled {
  --background: var(--parsec-color-light-danger-500);

  .element {
    opacity: 0.8;
    filter: grayscale(100%);
  }

  .cancel-text {
    color: var(--parsec-color-light-secondary-hard-grey);
  }
}

.failed {
  &-content {
    color: var(--parsec-color-light-danger-500);
    display: flex;
    align-items: center;
    gap: 0.375rem;

    .information-icon {
      font-size: 1.25rem;

      &:hover {
        color: var(--parsec-color-light-danger-700);
      }
    }
  }
}
</style>
